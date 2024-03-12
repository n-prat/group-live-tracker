/// `https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/api/user_api.rs`
///
use super::types::{ErrorResponse, ListUsers, UserLoginResponse};
use reqwasm::http;

use crate::app::API_ROOT;

// pub async fn api_register_user(user_data: &str) -> Result<User, String> {
//     let response = match http::Request::post(&format!("{API_ROOT}/api/auth/register"))
//         .header("Content-Type", "application/json")
//         .body(user_data)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data.data.user),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    let response = http::Request::post(&format!("{API_ROOT}/authorize"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
        .map_err(|_| "Failed to make request".to_string())?;

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

// pub async fn api_user_info() -> Result<User, String> {
//     let response = match http::Request::get(&format!("{API_ROOT}/api/users/me"))
//         .credentials(http::RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data.data.user),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }

// pub async fn api_logout_user() -> Result<(), String> {
//     let response = match http::Request::get(&format!("{API_ROOT}/api/auth/logout"))
//         .credentials(http::RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     Ok(())
// }

/// cf `server/src/api_user.rs`
pub async fn api_list_users(auth_token: &str) -> Result<ListUsers, String> {
    let response = http::Request::get(&format!("{API_ROOT}/users"))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {auth_token}",))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
        .map_err(|_| "Failed to make request".to_string())?;

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    let res_json = response.json::<ListUsers>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
