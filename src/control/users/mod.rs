use std::fmt::Debug;

use axum::{Extension, Json, extract::Path};
use http::StatusCode;
use tracing::info;

use crate::{domain::users::{UserData, User}, repository::users::{UserRepository, UserInsertError, UserGetError}};

#[tracing::instrument(skip(repository))]
pub async fn post_users<T: UserRepository + Debug>(Extension(repository): Extension<T>, Json(data): Json<UserData>) -> StatusCode {
    info!("Received user creation attempt");
    match repository.insert(data).await {
        Ok(()) => StatusCode::CREATED,
        Err(UserInsertError::Duplicate) => StatusCode::CONFLICT,
        Err(UserInsertError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[tracing::instrument(skip(repository))]
pub async fn get_users<T: UserRepository + Debug>(Extension(repository): Extension<T>, Path(user_email): Path<String>) -> Result<Json<User>, StatusCode> {
    info!("Received attempt to get user");
    match repository.get_by_email(&user_email).await {
        Ok(user) => Ok(Json(user)),
        Err(UserGetError::Missing) => Err(StatusCode::NOT_FOUND),
        Err(UserGetError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[cfg(test)]
mod tests;
