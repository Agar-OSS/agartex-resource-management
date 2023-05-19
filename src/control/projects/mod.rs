use std::fmt::Debug;

use axum::{extract::Path, headers::HeaderMap, Extension, Json};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::projects::{Project, ProjectData, ProjectMetaData},
    repository::projects::ProjectUpdateError,
    repository::projects::{ProjectGetError, ProjectInsertError, ProjectRepository},
};

#[tracing::instrument(skip_all)]
pub async fn get_projects<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    headers: HeaderMap,
) -> Result<Json<Vec<Project>>, StatusCode> {
    info!("Received attempt to get a project");
    let user_id = headers.get("X-User-Id").unwrap();
    let user_idx = user_id.to_str().unwrap().parse::<i32>().unwrap();

    match repository.get(user_idx).await {
        Ok(projects) => Ok(Json(projects)),
        Err(ProjectGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_projects<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    headers: HeaderMap,
    Json(data): Json<ProjectData>,
) -> StatusCode {
    info!("Received project creation attempt");
    let user_id = headers.get("X-User-Id").unwrap();
    let user_idx = user_id.to_str().unwrap().parse::<i32>().unwrap();
    match repository.insert(&data, user_idx).await {
        Ok(()) => StatusCode::CREATED,
        Err(ProjectInsertError::TransactionFailure) => StatusCode::INTERNAL_SERVER_ERROR,
        Err(ProjectInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_projects_metadata<T: ProjectRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    _headers: HeaderMap,
    Json(data): Json<ProjectMetaData>,
) -> StatusCode {
    info!("Received project update attempt");

    match repository.update(project_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(ProjectUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
