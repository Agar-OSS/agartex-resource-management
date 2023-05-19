use serde::{Deserialize, Serialize};
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Document {
    #[sqlx(rename = "document_id")]
    pub id: i32,
    pub project_id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentData {
    pub name: String,
}
