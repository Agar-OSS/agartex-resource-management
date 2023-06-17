use crate::extractors::time::json_time;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    pub project_id: i32,
    pub main_document_id: i32,

    pub owner_id: i32,
    #[sqlx(rename = "email")]
    pub owner_email: String,
    pub project_name: String,

    #[serde(with = "json_time")]
    pub created_at: NaiveDateTime,
    #[serde(with = "json_time")]
    pub last_modified: NaiveDateTime,
}
#[derive(FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
}
