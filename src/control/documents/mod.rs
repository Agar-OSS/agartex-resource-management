use std::fmt::Debug;

use axum::{extract::Path, Extension, Json};
use http::{HeaderMap, StatusCode};
use tracing::info;

use crate::{
    domain::documents::{Document, DocumentData},
    repository::crud::CrudRepository,
    repository::documents::{
        DocumentGetError, DocumentInsertError, DocumentRepository, DocumentUpdateError,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_documents<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    _headers: HeaderMap,
) -> Result<Json<Vec<Document>>, StatusCode> {
    info!("Received attempt to get a document");

    match repository.get(project_id).await {
        Ok(documents) => Ok(Json(documents)),
        Err(DocumentGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_documents<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    _headers: HeaderMap,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document creation attempt");

    match repository.insert(project_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_documents_metadata<
    T: DocumentRepository + CrudRepository + Clone + Send + Sync,
>(
    Extension(repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(document_id): Path<i32>,
    _headers: HeaderMap,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document update attempt");

    match repository.update(document_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_document_content<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(_repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(_document_id): Path<i32>,
    _headers: HeaderMap,
) -> StatusCode {
    info!("Received document content upload");
    StatusCode::NOT_IMPLEMENTED
}
