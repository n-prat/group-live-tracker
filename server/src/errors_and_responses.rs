/// https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/error-handling/src/main.rs#L129C1-L152C2
use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;

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
    BadRequest,
    /// NOTE: for security reasons, this is ALSO used when trying to access protected routes (eg not a superuser, etc)
    NotFound,
    /// eg DB error, etc
    InternalError,
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
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "bad request".to_owned()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_owned()),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal error".to_owned(),
            ),
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
