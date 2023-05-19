use std::fmt::Debug;

use axum::{extract::Path, headers::HeaderMap, Extension, Json};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::projects::{Project, ProjectData, ProjectMetaData},
    repository::projects::{ProjectGetError, ProjectPostError, ProjectRepository},
    repository::{
        crud::{CrudCheckError, CrudRepository},
        projects::ProjectPutError,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_projects<T: ProjectRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    headers: HeaderMap,
) -> Result<Json<Vec<Project>>, StatusCode> {
    info!("Received attempt to get a project");
    let user_id = headers.get("X-User-Id").unwrap();
    let user_idx = user_id.to_str().unwrap().parse().unwrap();

    match repository.check(user_idx).await {
        Ok(value) => match repository.get(value.id).await {
            Ok(projects) => Ok(Json(projects)),
            Err(ProjectGetError::Missing) => Err(StatusCode::NOT_FOUND),
            Err(ProjectGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(CrudCheckError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(CrudCheckError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_projects<T: ProjectRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    headers: HeaderMap,
    Json(data): Json<ProjectData>,
) -> StatusCode {
    info!("Received project creation attempt");
    let user_id = headers.get("X-User-Id").unwrap();
    let user_idx = user_id.to_str().unwrap().parse().unwrap();

    match repository.check(user_idx).await {
        Ok(_value) => match repository.insert(&data).await {
            Ok(()) => StatusCode::CREATED,
            Err(ProjectPostError::TransactionFailure) => StatusCode::INTERNAL_SERVER_ERROR,
            Err(ProjectPostError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_projects_metadata<T: ProjectRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    headers: HeaderMap,
    Json(data): Json<ProjectMetaData>,
) -> StatusCode {
    info!("Received project update attempt");
    let user_id = headers.get("X-User-Id").unwrap();
    let user_idx = user_id.to_str().unwrap().parse().unwrap();

    match repository.check(user_idx).await {
        Ok(_value) => match repository.update(project_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(ProjectPutError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
