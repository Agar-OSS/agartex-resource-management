use axum::{extract::Path, Extension, Json, TypedHeader};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::projects::{Project, ProjectMetadata},
    extractors::headers::XUserId,
    repository::projects::ProjectUpdateError,
    repository::projects::{ProjectGetError, ProjectInsertError, ProjectRepository},
};

#[tracing::instrument(skip(repository))]
pub async fn get_projects<T: ProjectRepository>(
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

#[tracing::instrument(skip(repository))]
pub async fn post_projects<T: ProjectRepository>(
    Extension(repository): Extension<T>,
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>,
    Json(data): Json<ProjectMetadata>,
) -> Result<(StatusCode, Json<Project>), StatusCode> {
    info!("Received project creation attempt");
    
    match repository.insert(&data, user_id).await {
        Ok(project) => Ok((StatusCode::CREATED, Json(project))),
        Err(ProjectInsertError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_projects_metadata<T: ProjectRepository>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    Json(data): Json<ProjectMetadata>
) -> StatusCode {
    info!("Received project update attempt");

    match repository.update(project_id, &data).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(ProjectUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
