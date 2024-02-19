use axum::{extract, Json};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct LoginResponse {
    status: String,
}

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    email: String,
    password: String,
}

/// api/auth/login
pub(crate) async fn api_auth_login(
    extract::Json(payload): extract::Json<LoginRequest>,
) -> Json<LoginResponse> {
    let user = check_user(payload.email).await;
    Json(user)
}

// TODO cf `check_username` in server/src/ws_handler.rs
async fn check_user(_username: String) -> LoginResponse {
    // ...

    LoginResponse {
        status: "ok".to_string(),
    }
}
