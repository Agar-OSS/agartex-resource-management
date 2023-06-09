use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    #[sqlx(rename = "project_id")]
    pub id: i32,
    pub owner: i32,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetaData {
    pub name: String,
}
