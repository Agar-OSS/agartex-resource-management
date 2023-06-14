use axum::{body::Bytes, extract::Path, Extension, Json};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::resources::{Resource, ResourceMetadata},
    repository::{
        projects::{ProjectGetError, ProjectRepository},
        resources::{
            ResourceGetError, ResourceInsertError, ResourceRepository, ResourceUpdateError,
        },
    },
    validation::ValidatedJson,
};

#[tracing::instrument(skip(project_repository, resource_repository))]
pub async fn get_projects_resources<P: ProjectRepository, R: ResourceRepository>(
    Extension(project_repository): Extension<P>,
    Extension(resource_repository): Extension<R>,
    Path(project_id): Path<i32>,
) -> Result<Json<Vec<Resource>>, StatusCode> {
    info!("Received attempt to get project resources");

    match project_repository.get_meta(project_id).await {
        Err(ProjectGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(_) => (),
    }

    match resource_repository.get(project_id).await {
        Ok(resources) => Ok(Json(resources)),
        Err(ResourceGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(ResourceGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip(project_repository, resource_repository))]
pub async fn post_projects_resources<P: ProjectRepository, R: ResourceRepository>(
    Extension(project_repository): Extension<P>,
    Extension(resource_repository): Extension<R>,
    Path(project_id): Path<i32>,
    ValidatedJson(data): ValidatedJson<ResourceMetadata>,
) -> Result<(StatusCode, Json<Resource>), StatusCode> {
    info!("Received resource creation attempt");

    match project_repository.get_meta(project_id).await {
        Err(ProjectGetError::Missing) => return Err(StatusCode::NOT_FOUND),
        Err(ProjectGetError::Unknown) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(_) => (),
    }

    match resource_repository.insert(project_id, &data).await {
        Ok(resource) => Ok((StatusCode::CREATED, Json(resource))),
        Err(ResourceInsertError::Duplicate) => Err(StatusCode::CONFLICT),
        Err(ResourceInsertError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip(repository))]
pub async fn put_projects_resources<T: ResourceRepository>(
    Extension(repository): Extension<T>,
    Path((project_id, resource_id)): Path<(i32, i32)>,
    body: Bytes,
) -> StatusCode {
    info!("Received resource content update attempt");
    // TODO: some mime type checking

    match repository
        .update(project_id, resource_id, body.as_ref())
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(ResourceUpdateError::Missing) => StatusCode::NOT_FOUND,
        Err(ResourceUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
