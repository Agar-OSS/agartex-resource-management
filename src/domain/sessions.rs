use serde::{Deserialize, Serialize};
use sqlx;

use super::users::User;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Session {
    #[sqlx(rename = "session_id")]
    pub id: String,
    #[sqlx(flatten)]
    pub user: User,
    pub expires: i64,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct SessionData {
    #[sqlx(rename = "session_id")]
    pub id: String,
    pub user_id: i32,
    pub expires: i64,
}
