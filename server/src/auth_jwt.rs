/// https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/jwt/src/main.rs#L1
// TODO use a turnkey JWT crate/lib from https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/ECOSYSTEM.md?plain=1#L13 ?
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

// Quick instructions
//
// - get an authorization token:
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -d '{"client_id":"foo","client_secret":"bar"}' \
//     http://localhost:3000/authorize
//
// - visit the protected area using the authorized token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
//
// - try to visit the protected area using an invalid token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer blahblahblah' \
//     http://localhost:3000/protected

pub(crate) static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "example_jwt=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     let app = Router::new()
//         .route("/protected", get(protected))
//         .route("/authorize", post(authorize));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

// async fn protected(claims: Claims) -> Result<String, AuthError> {
//     // Send the protected data to the user
//     Ok(format!(
//         "Welcome to the protected area :)\nYour data:\n{claims}",
//     ))
// }

pub(crate) async fn authorize(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    // TODO? || payload.password.is_empty()
    if payload.email.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // TODO? Here you can check the user credentials from a database
    // if payload.client_id != "foo" || payload.client_secret != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }
    let claims = Claims {
        sub: payload.email.to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub(crate) struct Keys {
    encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthBody {
    access_token: String,
    token_type: String,
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct AuthPayload {
//     client_id: String,
//     client_secret: String,
// }

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    pub(crate) email: String,
}

#[derive(Debug)]
pub(crate) enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::Request;
    use axum::http::{self};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::util::ServiceExt;

    fn init() {
        // https://docs.rs/crate/env_logger/latest
        let _ = env_logger::builder().is_test(true).try_init();
    }

    /// Generate a Auth token that can be used in the various "#[tokio::test]"
    pub(crate) fn generate_token(email: &str) -> String {
        let claims = Claims {
            sub: email.to_owned(),
            company: "ACME".to_owned(),
            // Mandatory expiry time as UTC timestamp
            exp: 2000000000, // May 2033
        };
        // Create the authorization token
        let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();

        token
    }

    /// We WANT a random user to be able to "login"
    #[tokio::test]
    async fn test_authorize_without_user_in_db_should_work() {
        init();

        let f = async {
            let app = crate::new_app().await.unwrap();

            // `Router` implements `tower::Service<Request<Body>>` so we can
            // call it like any tower service, no need to run an HTTP server.
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/authorize")
                        .method(http::Method::POST)
                        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                        .body(Body::from(json!({ "email": "aaa" }).to_string()))
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
        assert_eq!(body["token_type"], "Bearer");
        assert_eq!(body["access_token"].to_string().len(), 146);
    }
}
