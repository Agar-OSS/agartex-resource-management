use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    pub project_id: i32,
    pub main_document_id: i32,
    pub owner: i32,
    pub name: String,
}

#[derive(FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
}
