use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{NaiveDateTime};
use crate::extractors::time::json_time;
#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    pub project_id: i32,
    pub main_document_id: i32,
    #[serde(with = "json_time")]
    pub created_at: NaiveDateTime,
    #[serde(with = "json_time")]
    pub last_modified: NaiveDateTime,
    pub owner: i32,
    pub name: String,
}

#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct ProjectCoreData {
    pub project_id: i32,
    #[serde(with = "json_time")]
    pub created_at: NaiveDateTime,
    #[serde(with = "json_time")]
    pub last_modified: NaiveDateTime,
    pub owner: i32,
    pub name: String,
}
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub name: String,
}
