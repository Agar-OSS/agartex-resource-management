use axum::{Router, Extension};
use sqlx::PgPool;

use crate::{service::{users::HashUserService, hash::BcryptHashService}, control::users::post_users, repository::users::PgUserRepository};

pub fn main_router(pool: &PgPool) -> Router {
    let users_service = HashUserService::new(PgUserRepository::new(pool), BcryptHashService::new());

    let users_handler = axum::routing::post(post_users::<HashUserService<PgUserRepository, BcryptHashService>>);

    Router::new()
        .route("/users", users_handler)
        .layer(Extension(users_service))
}
