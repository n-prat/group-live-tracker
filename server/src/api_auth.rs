
// #[derive(Serialize)]
// pub(crate) struct LoginResponse {
//     status: String,
// }

// /// api/auth/login
// pub(crate) async fn api_auth_login(
//     payload: Json<LoginRequest>,
//     state: Arc<AppState>,
// ) -> Result<AppJson<LoginResponse>, AppError> {
//     let user = check_user(&payload.email, &state)?;
//     Ok(AppJson(user))
// }


