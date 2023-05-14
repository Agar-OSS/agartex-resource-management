use std::fmt::Debug;

use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
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
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    info!("Received attempt to get a project");
    let user_idx: i32 = user_id.token().parse().unwrap();

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
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ProjectData>,
) -> StatusCode {
    info!("Received project creation attempt");
    let user_idx: i32 = user_id.token().parse().unwrap();
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
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ProjectMetaData>,
) -> StatusCode {
    info!("Received project update attempt");

    let user_idx: i32 = user_id.token().parse().unwrap();
    match repository.check(user_idx).await {
        Ok(_value) => match repository.update(project_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(ProjectPutError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
