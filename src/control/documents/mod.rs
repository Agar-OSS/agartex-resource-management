use std::fmt::Debug;

use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::documents::{Document, DocumentData, DocumentMetaData},
    repository::crud::{CrudCheckError, CrudRepository},
    repository::documents::{
        DocumentGetError, DocumentPostError, DocumentPutError, DocumentRepository,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_documents<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Document>>, StatusCode> {
    info!("Received attempt to get a document");
    let user_idx: i32 = user_id.token().parse().unwrap();

    match repository.check(user_idx).await {
        Ok(value) => match repository.get(project_id).await {
            Ok(documents) => Ok(Json(documents)),
            Err(DocumentGetError::Missing) => Err(StatusCode::NOT_FOUND),
            Err(DocumentGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(CrudCheckError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(CrudCheckError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_documents<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document creation attempt");
    let user_idx: i32 = user_id.token().parse().unwrap();
    match repository.check(user_idx).await {
        Ok(_value) => match repository.insert(project_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(DocumentPostError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_documents_metadata<
    T: DocumentRepository + CrudRepository + Clone + Send + Sync,
>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    Path(document_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<DocumentMetaData>,
) -> StatusCode {
    info!("Received document update attempt");

    let user_idx: i32 = user_id.token().parse().unwrap();
    match repository.check(user_idx).await {
        Ok(_value) => match repository.update(project_id, document_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(DocumentPutError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_document_content<T: DocumentRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    Path(document_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
) -> StatusCode {
    info!("Received document content upload");
    StatusCode::NOT_IMPLEMENTED
}
