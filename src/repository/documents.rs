use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::error;

use crate::{
    domain::documents::{Document, DocumentData},
    filesystem::{get_document_path, read_file, write_file, FileReadError, FileWriteError},
};

pub enum DocumentInsertError {
    Unknown,
}
pub enum DocumentUpdateError {
    Missing,
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
    async fn get_meta(
        &self,
        project_id: i32,
        document_id: i32,
    ) -> Result<Document, DocumentGetError>;
    async fn insert(&self, project_id: i32, data: &DocumentData)
        -> Result<(), DocumentInsertError>;
    async fn update(
        &self,
        document_id: i32,
        data: &DocumentData,
    ) -> Result<(), DocumentUpdateError>;
    async fn read_file(&self, document: &Document) -> Result<String, DocumentGetError>;
    async fn write_file(
        &self,
        document: &Document,
        content: &str,
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
            Ok(documents) if !documents.is_empty() => Ok(documents),
            Ok(_documents) => Err(DocumentGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(DocumentGetError::Unknown);
            }
        }
    }

    async fn get_meta(
        &self,
        project_id: i32,
        document_id: i32,
    ) -> Result<Document, DocumentGetError> {
        let get_document_sql = "
            SELECT document_id, project_id, name
            FROM documents
            WHERE project_id = $1 AND document_id = $2
        ";

        let result = sqlx::query_as::<_, Document>(get_document_sql)
            .bind(project_id)
            .bind(document_id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(document)) => Ok(document),
            Ok(None) => Err(DocumentGetError::Missing),
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
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!(%err);
                return Err(DocumentInsertError::Unknown);
            }
        };

        let insert_document_sql = "
            INSERT_INTO documents (project_id, name)
            VALUES ($1, $2)
            RETURNING document_id, project_id, name
        ";

        let result = sqlx::query_as::<_, Document>(insert_document_sql)
            .bind(project_id)
            .bind(&document_data.name)
            .fetch_one(&mut tx);

        let document = match result.await {
            Ok(document) => document,
            Err(err) => {
                error!(%err);
                return Err(DocumentInsertError::Unknown);
            }
        };

        if write_file(get_document_path(&document), "", true)
            .await
            .is_err()
        {
            return Err(DocumentInsertError::Unknown);
        }

        tx.commit().await.map_err(|err| {
            error!(%err);
            DocumentInsertError::Unknown
        })
    }

    #[tracing::instrument(skip(self))]
    async fn read_file(&self, document: &Document) -> Result<String, DocumentGetError> {
        // We don't check any access privileges for now
        read_file(get_document_path(document))
            .await
            .map_err(|err| match err {
                FileReadError::Missing => DocumentGetError::Missing,
                FileReadError::Unknown => DocumentGetError::Unknown,
            })
    }

    async fn write_file(
        &self,
        document: &Document,
        content: &str,
    ) -> Result<(), DocumentUpdateError> {
        // We don't check any access privileges for now
        write_file(get_document_path(document), content, false)
            .await
            .map_err(|err| match err {
                FileWriteError::Missing => DocumentUpdateError::Missing,
                FileWriteError::Unknown => DocumentUpdateError::Unknown,
            })
    }
}
