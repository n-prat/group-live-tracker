use std::sync::Arc;

use crate::errors_and_responses::AppError;
use crate::AppState;


/// Use a AppState to get a new unique username
pub(crate) fn check_user(username_to_check: &str, state: &Arc<AppState>) -> Result<(), AppError> {
    let mut users_set = state.users_set.lock().unwrap();

    if !users_set.contains(username_to_check) {
        users_set.insert(username_to_check.to_owned());

        Ok(())
    } else {
        Err(AppError::LoginError)
    }
}