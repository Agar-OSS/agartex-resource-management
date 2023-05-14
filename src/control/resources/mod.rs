use std::fmt::Debug;

use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::resources::{Resource, ResourceData, ResourceMetaData},
    repository::crud::{CrudCheckError, CrudRepository},
    repository::resources::{
        ResourceGetError, ResourcePostError, ResourcePutError, ResourceRepository,
    },
};

#[tracing::instrument(skip_all)]
pub async fn get_resources<T: ResourceRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Resource>>, StatusCode> {
    info!("Received attempt to get a resource");
    let user_idx: i32 = user_id.token().parse().unwrap();

    match repository.check(user_idx).await {
        Ok(_value) => match repository.get(project_id).await {
            Ok(resources) => Ok(Json(resources)),
            Err(ResourceGetError::Missing) => Err(StatusCode::NOT_FOUND),
            Err(ResourceGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(CrudCheckError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(CrudCheckError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_resources<T: ResourceRepository + CrudRepository + Clone + Send + Sync>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ResourceData>,
) -> StatusCode {
    info!("Received resource creation attempt");
    let user_idx: i32 = user_id.token().parse().unwrap();
    match repository.check(user_idx).await {
        Ok(_value) => match repository.insert(project_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(ResourcePostError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn put_resources_metadata<
    T: ResourceRepository + CrudRepository + Clone + Send + Sync,
>(
    Extension(repository): Extension<T>,
    Path(project_id): Path<i32>,
    Path(resource_id): Path<i32>,
    TypedHeader(user_id): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ResourceMetaData>,
) -> StatusCode {
    info!("Received resource update attempt");

    let user_idx: i32 = user_id.token().parse().unwrap();
    match repository.check(user_idx).await {
        Ok(_value) => match repository.update(project_id, resource_id, &data).await {
            Ok(()) => StatusCode::CREATED,
            Err(ResourcePutError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Err(CrudCheckError::Missing) => StatusCode::NOT_FOUND,
        Err(CrudCheckError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn post_resources_content<
    T: ResourceRepository + CrudRepository + Clone + Send + Sync,
>(
    Extension(_repository): Extension<T>,
    Path(_project_id): Path<i32>,
    Path(_resource_id): Path<i32>,
    TypedHeader(_user_id): TypedHeader<Authorization<Bearer>>,
) -> StatusCode {
    info!("Received resource content upload");
    StatusCode::NOT_IMPLEMENTED
}
