use crate::errors_and_responses::AppError;
use crate::state::SharedState;

/// Use a AppState to get a new unique username
pub(crate) fn check_user(username_to_check: &str, state: &SharedState) -> Result<(), AppError> {
    let users_set = &mut state.write().unwrap().users_set;

    if !users_set.contains(username_to_check) {
        users_set.insert(username_to_check.to_owned());

        Ok(())
    } else {
        Err(AppError::LoginError)
    }
}
