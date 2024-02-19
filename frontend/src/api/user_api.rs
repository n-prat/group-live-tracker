/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/api/user_api.rs
///
use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse};
use reqwasm::http;

const API_ROOT: &str = "http://localhost:8081";

pub async fn api_register_user(user_data: &str) -> Result<User, String> {
    let response = match http::Request::post(&format!("{API_ROOT}/api/auth/register"))
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    let response = match http::Request::post(&format!("{API_ROOT}/api/auth/login"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_user_info() -> Result<User, String> {
    let response = match http::Request::get(&format!("{API_ROOT}/api/users/me"))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let response = match http::Request::get(&format!("{API_ROOT}/api/auth/logout"))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}
