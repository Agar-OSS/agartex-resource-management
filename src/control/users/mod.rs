use std::fmt::Debug;

use axum::{Extension, Json};
use http::StatusCode;
use tracing::info;

use crate::{domain::users::UserData, repository::users::{UserRepository, UserInsertError}};

#[tracing::instrument(skip(repository))]
pub async fn post_users<T: UserRepository + Debug>(Extension(repository): Extension<T>, Json(data): Json<UserData>) -> Result<StatusCode, StatusCode> {
    info!("Received user creation attempt");
    match repository.insert(data).await {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(UserInsertError::Duplicate) => Err(StatusCode::CONFLICT),
        Err(UserInsertError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod tests;
