use serde::{Deserialize, Serialize};
<<<<<<< HEAD
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use sqlx;
=======
use sqlx::FromRow;
>>>>>>> main

#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
<<<<<<< HEAD
    #[sqlx(rename = "project_id")]
    pub id: i32,
=======
    pub project_id: i32,
    pub main_document_id: i32,
>>>>>>> main
    pub owner: i32,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
}

<<<<<<< HEAD
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetaData {
=======
#[derive(FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetadata {
>>>>>>> main
    pub name: String,
}
