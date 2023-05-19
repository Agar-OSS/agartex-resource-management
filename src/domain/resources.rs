use serde::{Deserialize, Serialize};
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Resource {
    #[sqlx(rename = "resource_id")]
    pub id: i32,
    pub project_id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceData {
    pub name: String,
}
