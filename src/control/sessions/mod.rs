use std::fmt::Debug;

use axum::{
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
use http::StatusCode;
use tracing::info;

use crate::{
    domain::sessions::{Session, SessionData},
    repository::sessions::{
        SessionDeleteError, SessionGetError, SessionInsertError, SessionRepository,
    },
};

#[tracing::instrument(skip_all, fields(user_id = data.user_id))]
pub async fn post_sessions<T: SessionRepository + Debug>(
    Extension(repository): Extension<T>,
    Json(data): Json<SessionData>,
) -> StatusCode {
    info!("Received session creation attempt");
    match repository.insert(&data).await {
        Ok(()) => StatusCode::CREATED,
        Err(SessionInsertError::Duplicate) => StatusCode::CONFLICT,
        Err(SessionInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all)]
pub async fn get_sessions<T: SessionRepository + Debug>(
    Extension(repository): Extension<T>,
    TypedHeader(session_id): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Session>, StatusCode> {
    info!("Received attempt to get session");

    match repository.get(session_id.token()).await {
        Ok(session) => Ok(Json(session)),
        Err(SessionGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(SessionGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(skip_all)]
pub async fn delete_sessions<T: SessionRepository + Debug>(
    Extension(repository): Extension<T>,
    TypedHeader(session_id): TypedHeader<Authorization<Bearer>>,
) -> StatusCode {
    info!("Received attempt to delete session");

    match repository.delete(session_id.token()).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(SessionDeleteError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests;
