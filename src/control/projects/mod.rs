use axum::{extract::Path, Extension, Json, TypedHeader};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::projects::{Project, ProjectData, ProjectMetaData},
    extractors::headers::XUserId,
    repository::projects::ProjectUpdateError,
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
        Err(ProjectInsertError::TransactionFailure) => StatusCode::INTERNAL_SERVER_ERROR,
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
        Ok(()) => StatusCode::CREATED,
        Err(ProjectUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
