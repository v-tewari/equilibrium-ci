use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub(super) struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: Option<String>,
    pub(crate) password_less_login: Option<i8>,
    pub(crate) can_login: Option<i8>,
    pub(crate) is_active: Option<i8>,
    pub(crate) created: u32,
    pub(crate) created_by: Option<String>,
    pub(crate) updated: u32,
    pub(crate) updated_by: Option<String>,
}
