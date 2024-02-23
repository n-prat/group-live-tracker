use axum::extract::Multipart;
use axum::Extension;
use geozero::gpx::GpxReader;
use geozero::ProcessToJson;

use crate::auth_jwt::Claims;
use crate::errors_and_responses::AppError;
use crate::state::SharedState;

/// see https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/multipart-form/src/main.rs#L64
#[axum::debug_handler]
pub(crate) async fn handle_gpx_upload(
    Extension(state): Extension<SharedState>,
    _claims: Claims,
    mut multipart: Multipart,
) -> Result<(), AppError> {
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
        http::{HeaderName, HeaderValue},
        Router,
    };
    use axum_test::{
        multipart::{MultipartForm, Part},
        TestServer,
    };
    use serde_json::Value;

    use crate::new_state;

    use super::*;

    #[tokio::test]
    async fn test_handle_gpx_upload() {
        let f = async {
            let app_state = new_state();

            let my_app = Router::new()
                .route("/api/gpx", axum::routing::post(handle_gpx_upload))
                .layer(Extension(app_state.clone()));

            // Create a TestServer with your application
            let server = TestServer::new(my_app).unwrap();

            let token = crate::auth_jwt::tests::generate_token("aaa");

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
}
