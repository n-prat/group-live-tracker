use axum::extract::Multipart;
use axum::Extension;
use geozero::gpx::GpxReader;
use geozero::ProcessToJson;

use crate::api_authorize_jwt::Claims;
use crate::db::get_user_from_db;
use crate::errors_and_responses::AppError;
use crate::state::SharedState;

/// see https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/multipart-form/src/main.rs#L64
#[axum::debug_handler]
pub(crate) async fn handle_gpx_upload(
    Extension(state): Extension<SharedState>,
    claims: Claims,
    mut multipart: Multipart,
) -> Result<(), AppError> {
    // check the user credentials from a database
    let db_pool = &state.read().unwrap().db_pool.clone();
    match get_user_from_db(db_pool, &claims.sub).await {
        Ok(Some(user)) => {
            // Handle the case when the user is found in the database
            if !user.is_super_user {
                tracing::error!(
                    "handle_gpx_upload: user found but NOT a superuser: {:?}",
                    claims.sub,
                );
                return Err(AppError::NotFound);
            }
        }
        Ok(None) => {
            // Handle the case when the user is not found in the database
            tracing::error!("handle_gpx_upload: user not found: {:?}", claims.sub,);
            return Err(AppError::NotFound);
        }
        Err(err) => {
            // Handle the case when an error occurs during the database query
            tracing::error!("authorize: db error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        // let field = field.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // TODO? check field.name() == "file" ?

        // let mut file = File::create("path/to/your/temp.gpx")
        //     .await
        //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // while let Some(chunk) = field.next().await {
        //     let chunk = chunk.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        //     file.write_all(&chunk)
        //         .await
        //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // }
        let _name = field.name().unwrap().to_string();
        let _file_name = field.file_name().unwrap().to_string();
        let _content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        // // let reader = BufReader::from(data);
        let mut reader = std::io::Cursor::new(data.as_ref());

        // // Read the GPX file and convert to GeoJSON
        // let gpx: Gpx = read(reader).unwrap();
        // info!("gpx.tracks : {}", gpx.tracks.len());
        // let track: &Track = &gpx.tracks[0];
        // let segment: &TrackSegment = &track.segments[0];
        // let points = &segment.points;

        // let mut features = Vec::new();
        // for point in points {
        //     let coordinates = vec![point.point().y(), point.point().x()];
        //     let geometry = Geometry::new(Value::Point(coordinates));
        //     let properties = json!({
        //         "elevation": point.elevation,
        //         "time": point.time,
        //     });
        //     let feature = Feature {
        //         bbox: None,
        //         geometry: Some(geometry),
        //         id: None,
        //         properties: Some(properties),
        //         foreign_members: None,
        //     };
        //     features.push(feature);
        // }
        //
        // let feature_collection = GeoJson::FeatureCollection(FeatureCollection::from(features));

        // cf https://github.com/georust/geozero/blob/52a4d2d3c11f02e734274fcb6ee4b88b94b5b53d/geozero/tests/gpx.rs#L91
        let mut reader = GpxReader(&mut reader);
        let geojson_str = reader.to_json().unwrap();

        state.write().unwrap().geojson = Some(geojson_str);

        // return Ok(Json(json!({ "status": "success" })));
        return Ok(());
    }
    Err(AppError::BadRequest)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::{
        http::{HeaderName, HeaderValue, StatusCode},
        Router,
    };
    use axum_test::{
        multipart::{MultipartForm, Part},
        TestServer,
    };
    use serde_json::Value;

    use crate::db::{insert_user, setup_db, update_user_to_superuser};
    use crate::new_state;

    use super::*;

    #[tokio::test]
    async fn test_handle_gpx_upload_superuser_ok() {
        let f = async {
            let db_pool = setup_db("sqlite::memory:").await.unwrap();
            let username = "aaa";
            insert_user(&db_pool, username, "password").await.unwrap();
            update_user_to_superuser(&db_pool, username).await.unwrap();
            let app_state = new_state(db_pool);

            let my_app = Router::new()
                .route("/api/gpx", axum::routing::post(handle_gpx_upload))
                .layer(Extension(app_state.clone()));

            // Create a TestServer with your application
            let server = TestServer::new(my_app).unwrap();

            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // Create a multipart form data payload
            let bytes = include_bytes!("../tests/data/2024-02-19_1444960792_MJ 19_02.gpx");
            let file_part = Part::bytes(bytes.as_slice())
                .file_name("file.gpx")
                .mime_type("text/plain");

            // Build a TestRequest
            let response = server
                .post("/api/gpx")
                .add_header(
                    HeaderName::from_str("Authorization").unwrap(),
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                )
                .multipart(MultipartForm::new().add_part("file", file_part))
                .await;

            (response, app_state)
        };

        let (response, app_state) =
            temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;

        let response_body = response.text();

        // Assert the response is as expected
        assert_eq!(response.status_code(), 200);
        assert_eq!(response_body, "");
        // FIXME
        let geojson_str: String = app_state
            .write()
            .unwrap()
            .geojson
            .as_ref()
            .unwrap()
            .to_string();
        assert_eq!(geojson_str.len(), 15830);
        // we use this reference file in the frontend tests
        let geojson_res: Value = serde_json::from_str(&geojson_str).unwrap();
        let geojson_ref: Value = serde_json::from_str(include_str!(
            "../tests/data/2024-02-19_1444960792_MJ 19_02.geojson"
        ))
        .unwrap();
        assert_eq!(geojson_res, geojson_ref);
    }

    /// test that ONLY a superuser can upload a GPX file
    #[tokio::test]
    async fn test_handle_gpx_upload_must_be_superuser_else_404() {
        let f = async {
            let db_pool = setup_db("sqlite::memory:").await.unwrap();
            let username = "aaa";
            insert_user(&db_pool, username, "password").await.unwrap();
            let app_state = new_state(db_pool);

            let my_app = Router::new()
                .route("/api/gpx", axum::routing::post(handle_gpx_upload))
                .layer(Extension(app_state.clone()));

            // Create a TestServer with your application
            let server = TestServer::new(my_app).unwrap();

            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // Create a multipart form data payload
            let bytes = include_bytes!("../tests/data/2024-02-19_1444960792_MJ 19_02.gpx");
            let file_part = Part::bytes(bytes.as_slice())
                .file_name("file.gpx")
                .mime_type("text/plain");

            // Build a TestRequest
            let response = server
                .post("/api/gpx")
                .add_header(
                    HeaderName::from_str("Authorization").unwrap(),
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                )
                .multipart(MultipartForm::new().add_part("file", file_part))
                .await;

            (response, app_state)
        };

        let (response, _app_state) =
            temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;

        let _response_body = response.text();

        // Assert the response is as expected
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    /// test that ONLY a superuser can upload a GPX file
    #[tokio::test]
    async fn test_handle_gpx_upload_user_not_in_db_should_fail_404() {
        let f = async {
            let db_pool = setup_db("sqlite::memory:").await.unwrap();
            let username = "aaa";
            let app_state = new_state(db_pool);

            let my_app = Router::new()
                .route("/api/gpx", axum::routing::post(handle_gpx_upload))
                .layer(Extension(app_state.clone()));

            // Create a TestServer with your application
            let server = TestServer::new(my_app).unwrap();

            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // Create a multipart form data payload
            let bytes = include_bytes!("../tests/data/2024-02-19_1444960792_MJ 19_02.gpx");
            let file_part = Part::bytes(bytes.as_slice())
                .file_name("file.gpx")
                .mime_type("text/plain");

            // Build a TestRequest
            let response = server
                .post("/api/gpx")
                .add_header(
                    HeaderName::from_str("Authorization").unwrap(),
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                )
                .multipart(MultipartForm::new().add_part("file", file_part))
                .await;

            (response, app_state)
        };

        let (response, _app_state) =
            temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;

        let _response_body = response.text();

        // Assert the response is as expected
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
