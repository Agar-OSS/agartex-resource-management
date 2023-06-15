use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Document {
    pub document_id: i32,
    pub project_id: i32,
    pub name: String,
}

#[derive(FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentData {
    pub name: String,
}
