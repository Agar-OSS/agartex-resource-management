use axum::{
    extract::Path,
    Extension, Json, TypedHeader,
};
use http::{StatusCode};
use tracing::info;

use crate::{
    domain::resources::{Resource, ResourceData},
    domain::headers::XUserId,
    repository::resources::{
        ResourceGetError, ResourceInsertError, ResourceRepository, ResourceUpdateError,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_resources<T: ResourceRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
) -> Result<Json<Vec<Resource>>, StatusCode> {
    info!("Received attempt to get a resource");

    match repository.get(project_id).await {
        Ok(resources) => Ok(Json(resources)),
        Err(ResourceGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(ResourceGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_resources<T: ResourceRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
    Json(data): Json<ResourceData>,
) -> StatusCode {
    info!("Received resource creation attempt");
    match repository.insert(project_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(ResourceInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_resources_metadata<T: ResourceRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    Path(resource_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
    Json(data): Json<ResourceData>,
) -> StatusCode {
    info!("Received resource update attempt");

    match repository.update(project_id, resource_id, &data).await {
        Ok(()) => StatusCode::CREATED,
        Err(ResourceUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_resources_content<T: ResourceRepository + Clone + Send + Sync>(
    Extension(_repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(_resource_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<XUserId>,
) -> StatusCode {
    info!("Received resource content upload");
    StatusCode::NOT_IMPLEMENTED
}
