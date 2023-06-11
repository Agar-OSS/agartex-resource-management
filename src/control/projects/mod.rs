use axum::{extract::Path, Extension, Json, TypedHeader, response::IntoResponse};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::projects::{Project, ProjectData, ProjectMetaData},
    extractors::headers::XUserId,
    repository::{projects::ProjectUpdateError, documents::{DocumentRepository, DocumentGetError}},
    repository::projects::{ProjectGetError, ProjectInsertError, ProjectRepository},
};

#[tracing::instrument(skip_all)]
pub async fn get_projects<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>
) -> Result<Json<Vec<Project>>, StatusCode> {
    info!("Received attempt to get a project");
    
    match repository.get(user_id).await {
        Ok(projects) => Ok(Json(projects)),
        Err(ProjectGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_projects<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>,
    Json(data): Json<ProjectData>,
) -> StatusCode {
    info!("Received project creation attempt");
    
    match repository.insert(&data, user_id).await {
        Ok(()) => StatusCode::CREATED,
        Err(ProjectInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_projects_metadata<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(XUserId(_user_id)): TypedHeader<XUserId>,
    Json(data): Json<ProjectMetaData>,
) -> StatusCode {
    info!("Received project update attempt");

    match repository.update(project_id, &data).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(ProjectUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip(project_repository, document_repository))]
pub async fn get_project_document<P: ProjectRepository, D: DocumentRepository>(
    Extension(project_repository): Extension<P>,
    Extension(document_repository): Extension<D>,
    Path(project_id): Path<i32>
) -> Result<impl IntoResponse, StatusCode> {
    info!("Received attempt to get document text");

    let document_id = match project_repository.get_metadata(project_id).await {
        Ok(project) => project.main_document_id,
        Err(ProjectGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    info!("Retrieved document id: {}", document_id);

    let content = match document_repository.read_file(document_id).await {
        Ok(content) => content,
        Err(DocumentGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(DocumentGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    info!("Successfully retrieved file content");

    Ok(content)
}

#[tracing::instrument(skip(project_repository, document_repository))]
pub async fn put_project_document<P: ProjectRepository, D: DocumentRepository>(
    Extension(project_repository): Extension<P>,
    Extension(document_repository): Extension<D>,
    Path(project_id): Path<i32>,
    content: String
) -> StatusCode {
    info!("Received attempt to update document text");

    let document_id = match project_repository.get_metadata(project_id).await {
        Ok(project) => project.main_document_id,
        Err(ProjectGetError::Missing) => return StatusCode::NOT_FOUND,
        Err(ProjectGetError::Unknown) => return StatusCode::INTERNAL_SERVER_ERROR
    };

    info!("Retrieved document id: {}", document_id);

    match document_repository.write_file(document_id, &content).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}
