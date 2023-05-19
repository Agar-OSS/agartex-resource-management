use axum::{extract::Path, Extension, Json, TypedHeader};
use http::{StatusCode};
use tracing::info;

use crate::{
    domain::documents::{Document, DocumentData},
    domain::headers::XUserId,
    repository::documents::{
        DocumentGetError, DocumentInsertError, DocumentRepository, DocumentUpdateError,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_documents<T: DocumentRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
) -> Result<Json<Vec<Document>>, StatusCode> {
    info!("Received attempt to get a document");

    match repository.get(project_id).await {
        Ok(documents) => Ok(Json(documents)),
        Err(DocumentGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_documents<T: DocumentRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document creation attempt");

    match repository.insert(project_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_documents_metadata<T: DocumentRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(document_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document update attempt");

    match repository.update(document_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_document_content<T: DocumentRepository + Clone + Send + Sync>(
    Extension(_repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(_document_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
) -> StatusCode {
    info!("Received document content upload");
    StatusCode::NOT_IMPLEMENTED
}
