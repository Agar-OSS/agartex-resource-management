use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UserData {
    pub email: String,
    pub password_hash: String
}
