/// SHOULD match server/migrations/20240226_1558_initial.sql
#[derive(PartialEq, Debug)]
pub(crate) struct User {
    pub(crate) username: String,
    pub(crate) password_hash: String,
    pub(crate) is_super_user: bool,
}
