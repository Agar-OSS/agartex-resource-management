use axum::{extract::Path, Extension, Json, TypedHeader};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::documents::{Document, DocumentData},
    extractors::headers::XUserId,
    repository::{
        documents::{
            DocumentGetError, DocumentInsertError, DocumentRepository, DocumentUpdateError,
        },
        projects::{ProjectGetError, ProjectRepository},
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_documents<T: DocumentRepository>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(XUserId(_user_id)): TypedHeader<XUserId>,
) -> Result<Json<Vec<Document>>, StatusCode> {
    info!("Received attempt to get a document");

    match repository.get(project_id).await {
        Ok(documents) => Ok(Json(documents)),
        Err(DocumentGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_documents<T: DocumentRepository>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(XUserId(_user_id)): TypedHeader<XUserId>,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document creation attempt");

    match repository.insert(project_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_documents_metadata<T: DocumentRepository>(
    Extension(repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(document_id): Path<i32>,
    TypedHeader(XUserId(_user_id)): TypedHeader<XUserId>,
    Json(data): Json<DocumentData>,
) -> StatusCode {
    info!("Received document update attempt");

    match repository.update(document_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(DocumentUpdateError::Missing) => StatusCode::NOT_FOUND,
        Err(DocumentUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_document_content<T: DocumentRepository>(
    Extension(_repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(_document_id): Path<i32>,
    TypedHeader(XUserId(_user_id)): TypedHeader<XUserId>,
) -> StatusCode {
    info!("Received document content upload");
    StatusCode::NOT_IMPLEMENTED
}

#[tracing::instrument(skip(project_repository, document_repository, content))]
pub async fn put_projects_documents<P: ProjectRepository, D: DocumentRepository>(
    Extension(project_repository): Extension<P>,
    Extension(document_repository): Extension<D>,
    Path(project_id): Path<i32>,
    content: String,
) -> StatusCode {
    info!("Received attempt to update document text");

    let document_id = match project_repository.get_meta(project_id).await {
        Ok(project) => project.main_document_id,
        Err(ProjectGetError::Missing) => return StatusCode::NOT_FOUND,
        Err(ProjectGetError::Unknown) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    info!("Retrieved document id: {}", document_id);

    let document = match document_repository.get_meta(project_id, document_id).await {
        Ok(document) => document,
        Err(DocumentGetError::Missing) => return StatusCode::NOT_FOUND,
        Err(DocumentGetError::Unknown) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    match document_repository.write_file(&document, &content).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip(project_repository, document_repository))]
pub async fn get_projects_documents<P: ProjectRepository, D: DocumentRepository>(
    Extension(project_repository): Extension<P>,
    Extension(document_repository): Extension<D>,
    Path(project_id): Path<i32>,
) -> Result<String, StatusCode> {
    info!("Received attempt to get document text");

    let document_id = match project_repository.get_meta(project_id).await {
        Ok(project) => project.main_document_id,
        Err(ProjectGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    info!("Retrieved document id: {}", document_id);

    let document = match document_repository.get_meta(project_id, document_id).await {
        Ok(document) => document,
        Err(DocumentGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let content = match document_repository.read_file(&document).await {
        Ok(content) => content,
        Err(DocumentGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    info!("Successfully retrieved file content");

    Ok(content)
}
