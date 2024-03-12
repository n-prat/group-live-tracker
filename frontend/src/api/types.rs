/// `https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/api/types.rs`
///
use serde::{Deserialize, Serialize};

/// SHOULD roughly match `server/src/user.rs`
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    // pub id: String,
    // pub name: String,
    // pub email: String,
    // pub role: String,
    // pub photo: String,
    // pub verified: bool,
    // pub createdAt: DateTime<Utc>,
    // pub updatedAt: DateTime<Utc>,
    pub(crate) username: String,
    // pub(crate) password_hash: String,
    pub(crate) is_super_user: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

/// MUST match `struct AuthBody` in `server/src/auth_jwt.rs`
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

/// SHOULD roughly match `server/src/api_user.rs`
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ListUsers {
    pub(crate) users: Vec<User>,
}
