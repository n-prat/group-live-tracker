use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api_authorize_jwt::Claims,
    db::{get_user_from_db, list_users_from_db, update_user_to_superuser},
    errors_and_responses::AppError,
    state::SharedState,
    user::User,
};

#[derive(Debug, Serialize)]
pub(crate) struct ListUsers {
    users: Vec<User>,
}

/// List all users
/// MUST be called by a superuser
#[axum::debug_handler]
pub(crate) async fn list_users(
    Extension(state): Extension<SharedState>,
    claims: Claims,
) -> Result<Json<ListUsers>, AppError> {
    // check the user credentials from a database
    let db_pool = match state.read() {
        Ok(state) => state.db_pool.clone(),
        Err(err) => {
            tracing::error!("list_users: state read lock error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };
    let _user = match get_user_from_db(&db_pool, &claims.sub).await {
        Ok(Some(user)) => {
            // Handle the case when the user is found in the database
            if !user.is_super_user {
                tracing::error!(
                    "list_users: user found but NOT a superuser: {:?}",
                    claims.sub,
                );
                return Err(AppError::NotFound);
            }

            user
        }
        Ok(None) => {
            // Handle the case when the user is not found in the database
            tracing::error!("list_users: user not found: {:?}", claims.sub,);
            return Err(AppError::NotFound);
        }
        Err(err) => {
            // Handle the case when an error occurs during the database query
            tracing::error!("list_users: db error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };

    let all_users = match list_users_from_db(&db_pool).await {
        Ok(users) => users,
        Err(err) => {
            tracing::error!("list_users: db error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };

    Ok(Json(ListUsers { users: all_users }))
}

#[derive(Deserialize)]
pub(crate) struct SetSuperuserRequest {
    pub(crate) username: String,
}

/// Mark a given user as superuser
/// MUST be called by a superuser
#[axum::debug_handler]
pub(crate) async fn set_superuser(
    Extension(state): Extension<SharedState>,
    claims: Claims,
    Json(payload): Json<SetSuperuserRequest>,
) -> Result<(), AppError> {
    // check the user credentials from a database
    let db_pool = match state.read() {
        Ok(state) => state.db_pool.clone(),
        Err(err) => {
            tracing::error!("list_users: state read lock error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };
    let _user = match get_user_from_db(&db_pool, &claims.sub).await {
        Ok(Some(user)) => {
            // Handle the case when the user is found in the database
            if !user.is_super_user {
                tracing::error!(
                    "set_superuser: user found but NOT a superuser: {:?}",
                    claims.sub,
                );
                return Err(AppError::NotFound);
            }

            user
        }
        Ok(None) => {
            // Handle the case when the user is not found in the database
            tracing::error!("set_superuser: user not found: {:?}", claims.sub,);
            return Err(AppError::NotFound);
        }
        Err(err) => {
            // Handle the case when an error occurs during the database query
            tracing::error!("set_superuser: db error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };

    match update_user_to_superuser(&db_pool, &payload.username).await {
        Ok(()) => {}
        Err(err) => {
            tracing::error!("set_superuser: db error: {:?}", err,);
            return Err(AppError::InternalError);
        }
    };

    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::db::{insert_user, setup_db, update_user_to_superuser};

    use super::*;

    use axum::body::Body;
    use axum::http::{self};
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use http_body_util::BodyExt;
    use serde_json::json;
    use serde_json::Value;
    use sqlx::SqlitePool;
    use tower::util::ServiceExt;

    async fn init(username: Option<&str>, should_set_superuser: bool) -> (Router, SqlitePool) {
        // https://docs.rs/crate/env_logger/latest
        let _ = env_logger::builder().is_test(true).try_init();

        let db_pool = setup_db("sqlite::memory:", None, None).await.unwrap();
        let app = crate::new_app(db_pool.clone()).unwrap();

        // INSERT a user if asked
        if let Some(username) = username {
            insert_user(&db_pool, username, "bbb").await.unwrap();

            if should_set_superuser {
                update_user_to_superuser(&db_pool, username).await.unwrap();
            }
        }

        (app, db_pool)
    }

    #[tokio::test]
    async fn test_list_users_non_existent_user_404() {
        let username = "aaa";
        let (app, _db_pool) = init(None, false).await;

        let f = async {
            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // `Router` implements `tower::Service<Request<Body>>` so we can
            // call it like any tower service, no need to run an HTTP server.
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/users")
                        .method(http::Method::GET)
                        .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            response
        };

        let response = temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;
        let response_status = response.status();
        let _response_body = response.into_body().collect().await.unwrap().to_bytes();
        // println!("response_body: {:?}", response_body);
        // println!("status: {:?}", response_status);

        assert_eq!(response_status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_users_must_be_superuser_else_404() {
        let username = "aaa";
        let (app, _db_pool) = init(Some(username), false).await;

        let f = async {
            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // `Router` implements `tower::Service<Request<Body>>` so we can
            // call it like any tower service, no need to run an HTTP server.
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/users")
                        .method(http::Method::GET)
                        .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            response
        };

        let response = temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;
        let response_status = response.status();
        let _response_body = response.into_body().collect().await.unwrap().to_bytes();
        // println!("response_body: {:?}", response_body);
        // println!("status: {:?}", response_status);

        assert_eq!(response_status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_users_superuser_ok() {
        let username = "aaa";
        let (app, _db_pool) = init(Some(username), true).await;

        let f = async {
            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // `Router` implements `tower::Service<Request<Body>>` so we can
            // call it like any tower service, no need to run an HTTP server.
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/users")
                        .method(http::Method::GET)
                        .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            response
        };

        let response = temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;
        let response_status = response.status();
        let response_body = response.into_body().collect().await.unwrap().to_bytes();
        // println!("response_body: {:?}", response_body);
        // println!("status: {:?}", response_status);

        assert_eq!(response_status, StatusCode::OK);
        let body: Value = serde_json::from_slice(&response_body).unwrap();
        // assert body["users"] is an empty vec
        let resp_users: Vec<Value> = body["users"].as_array().unwrap().to_vec();
        assert_eq!(resp_users.len(), 1);
    }

    #[tokio::test]
    async fn test_set_superuser_superuser_ok() {
        let username = "aaa";
        let (app, db_pool) = init(Some(username), true).await;
        let username_to_update = "bbb";
        insert_user(&db_pool, username_to_update, "bbb")
            .await
            .unwrap();

        let f = async {
            let token = crate::api_authorize_jwt::tests::generate_token(username);

            // `Router` implements `tower::Service<Request<Body>>` so we can
            // call it like any tower service, no need to run an HTTP server.
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/user/set_superuser")
                        .method(http::Method::POST)
                        .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                        .body(Body::from(json!({ "username": "bbb" }).to_string()))
                        .unwrap(),
                )
                .await
                .unwrap();

            response
        };

        let response = temp_env::async_with_vars([("JWT_SECRET", Some("0123456789"))], f).await;
        let response_status = response.status();
        let _response_body = response.into_body().collect().await.unwrap().to_bytes();
        // println!("response_body: {:?}", response_body);
        // println!("status: {:?}", response_status);

        assert_eq!(response_status, StatusCode::OK);

        let updated_user = get_user_from_db(&db_pool, username_to_update)
            .await
            .unwrap()
            .unwrap();
        assert!(updated_user.is_super_user);
    }
}
