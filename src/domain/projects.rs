use serde::{Deserialize, Serialize};
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    #[sqlx(rename = "project_id")]
    pub id: i32,
    pub main_document_id: i32,
    pub owner: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub main_document_id: i32,
    pub owner: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetaData {
    pub name: String,
}
