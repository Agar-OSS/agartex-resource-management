use std::path::PathBuf;

use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tokio::fs;
use tracing::error;

use crate::{domain::documents::{Document, DocumentData}, constants::FILE_DIR_PATH};

pub enum DocumentInsertError {
    Unknown,
}
pub enum DocumentUpdateError {
    Unknown,
}
pub enum DocumentGetError {
    Missing,
    Unknown,
}

#[automock]
#[async_trait]
pub trait DocumentRepository {
    async fn get(&self, project_id: i32) -> Result<Vec<Document>, DocumentGetError>;
    async fn insert(&self, project_id: i32, data: &DocumentData)
        -> Result<(), DocumentInsertError>;
    async fn update(
        &self,
        document_id: i32,
        data: &DocumentData,
    ) -> Result<(), DocumentUpdateError>;
    async fn read_file(&self, project_id: i32) -> Result<PathBuf, DocumentGetError>;
    async fn write_file(&self, project_id: i32, content: &str) -> Result<(), DocumentUpdateError>;
}

#[derive(Debug, Clone)]
pub struct PgDocumentRepository {
    pub pool: PgPool,
}

impl PgDocumentRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

fn get_document_path(document_id: i32) -> PathBuf {
    let mut file_path = FILE_DIR_PATH.clone();
    file_path.push(document_id.to_string());
    file_path
}

#[async_trait]
impl DocumentRepository for PgDocumentRepository {
    async fn get(&self, project_id: i32) -> Result<Vec<Document>, DocumentGetError> {
        let documents = sqlx::query_as::<_, Document>(
            "
            SELECT document_id, project_id, name
            FROM documents
            WHERE documents.project_id = $1
        ",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await;

        match documents {
            Ok(documents) if !documents.is_empty() => Ok(documents),
            Ok(_documents) => Err(DocumentGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(DocumentGetError::Unknown);
            }
        }
    }

    async fn update(
        &self,
        document_id: i32,
        document_metadata: &DocumentData,
    ) -> Result<(), DocumentUpdateError> {
        let result = sqlx::query(
            "
            UPDATE documents 
            SET name = $1
            WHERE document_id = $2
        ",
        )
        .bind(&document_metadata.name)
        .bind(document_id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_result) => Ok(()),
            Err(err) => {
                error!(%err);
                return Err(DocumentUpdateError::Unknown);
            }
        }
    }
    async fn insert(
        &self,
        project_id: i32,
        document_data: &DocumentData,
    ) -> Result<(), DocumentInsertError> {
        let result = sqlx::query(
            "
            INSERT_INTO documents (project_id, name)
            VALUES ($1, $2)
        ",
        )
        .bind(project_id)
        .bind(&document_data.name)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_result) => Ok(()),
            Err(err) => {
                error!(%err);
                Err(DocumentInsertError::Unknown)
            }
        }
    }

    async fn read_file(&self, document_id: i32) -> Result<PathBuf, DocumentGetError> {
        // We don't check any access privileges for now
        let file_path = get_document_path(document_id);

        if !file_path.exists() {
            if let Err(err) = fs::write(&file_path, "").await {
                error!(%err);
                return Err(DocumentGetError::Unknown);
            }
        }
        Ok(file_path)
    }

    async fn write_file(&self, document_id: i32, content: &str) -> Result<(), DocumentUpdateError> {
        // We don't check any access privileges for now
        let file_path = get_document_path(document_id);

        fs::write(file_path, content).await.map_err(|err| {
            error!(%err);
            DocumentUpdateError::Unknown
        })
    }
}
