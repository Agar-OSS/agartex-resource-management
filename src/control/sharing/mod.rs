use axum::{extract::Path, Extension, Json, TypedHeader};
use http::StatusCode;
use tracing::info;

use crate::{
    extractors::headers::XUserId,
    repository::sharing::{
        ProjectSharingCreateError, ProjectSharingRepository, ProjectSharingUpdateError,
    },
};

//creates a collaboration entry in collaboration table, returns collaboration_id
#[tracing::instrument(skip_all)]
pub async fn post_projects_sharing<T: ProjectSharingRepository>(
    Extension(repository): Extension<T>,
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>,
    Path(token): Path<String>,
) -> StatusCode {
    info!("Receive sharing entry creation attempt");

    match repository.update(token, user_id).await {
        Ok(_) => StatusCode::CREATED,
        Err(ProjectSharingUpdateError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

//overwrites friend_id to a number
#[tracing::instrument(skip_all)]
pub async fn put_projects_sharing<T: ProjectSharingRepository>(
    Extension(repository): Extension<T>,
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>,
    Path(project_id): Path<i32>,
) -> Result<String, StatusCode> {
    info!("Receive sharing entry creation attempt");

    match repository.create(project_id, user_id).await {
        Ok(token) => Ok(token),
        Err(ProjectSharingCreateError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
