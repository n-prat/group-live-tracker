use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::AppState;

/// https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/error-handling/src/main.rs#L129C1-L152C2
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub(crate) struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
pub(crate) enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    LoginError,
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
            //
            AppError::LoginError => (StatusCode::BAD_REQUEST, "login error".to_owned()),
            // AppError::TimeError(err) => {
            //     // Because `TraceLayer` wraps each request in a span that contains the request
            //     // method, uri, etc we don't need to include those details here
            //     tracing::error!(%err, "error from time_library");

            //     // Don't expose any details about the error to the client
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "Something went wrong".to_owned(),
            //     )
            // }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

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
    payload: Json<LoginRequest>,
    state: Arc<AppState>,
) -> Result<AppJson<LoginResponse>, AppError> {
    let user = check_user(&payload.email, &state)?;
    Ok(AppJson(user))
}

/// Use a AppState to get a new unique username
fn check_user(username_to_check: &str, state: &Arc<AppState>) -> Result<LoginResponse, AppError> {
    let mut users_set = state.users_set.lock().unwrap();

    if !users_set.contains(username_to_check) {
        users_set.insert(username_to_check.to_owned());

        Ok(LoginResponse {
            status: "ok".to_string(),
        })
    } else {
        Err(AppError::LoginError)
    }
}
