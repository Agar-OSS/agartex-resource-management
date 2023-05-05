use std::fmt::Debug;

use axum::{Extension, Json, extract::Path};
use http::StatusCode;
use tracing::info;

use crate::{domain::{sessions::{Session, SessionData}}, repository::{sessions::{SessionRepository, SessionGetError, SessionInsertError, SessionDeleteError}}};

#[tracing::instrument(skip_all, fields(user_id = data.user_id))]
pub async fn post_sessions<T: SessionRepository + Debug>(Extension(repository): Extension<T>, Json(data): Json<SessionData>) -> StatusCode {
    info!("Received session creation attempt");
    match repository.insert(&data).await {
        Ok(()) => StatusCode::CREATED,
        Err(SessionInsertError::Duplicate) => StatusCode::CONFLICT,
        Err(SessionInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[tracing::instrument(skip_all)]
pub async fn get_sessions<T: SessionRepository + Debug>(Extension(repository): Extension<T>, Path(session_id): Path<String>) -> Result<Json<Session>, StatusCode> {
    info!("Received attempt to get session");
    match repository.get(&session_id).await {
        Ok(session) => Ok(Json(session)),
        Err(SessionGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(SessionGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[tracing::instrument(skip_all)]
pub async fn delete_sessions<T: SessionRepository + Debug>(Extension(repository): Extension<T>, Path(session_id): Path<String>) -> StatusCode {
    info!("Received attempt to delete session");
    match repository.delete(&session_id).await {
        Ok(()) => StatusCode::CREATED,
        Err(SessionDeleteError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[cfg(test)]
mod tests;
