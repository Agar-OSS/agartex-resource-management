use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::{error, info};

use crate::domain::documents::{Document, DocumentData};

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
    async fn get(&self, projcet_id: i32) -> Result<Vec<Document>, DocumentGetError>;
    async fn insert(&self, project_id: i32, data: &DocumentData)
        -> Result<(), DocumentInsertError>;
    async fn update(
        &self,
        document_id: i32,
        data: &DocumentData,
    ) -> Result<(), DocumentUpdateError>;
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
            Ok(documents) => Ok(documents),
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
            ON CONFLICT DO NOTHING
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
                return Err(DocumentInsertError::Unknown);
            }
        }
    }
}
