use axum::{Router, Extension};
use sqlx::PgPool;

use crate::{repository::users::PgUserRepository, control::users::post_users};

pub fn main_router(pool: &PgPool) -> Router {
    let users_repository = PgUserRepository::new(pool);

    let users_handler = axum::routing::post(post_users::<PgUserRepository>);

    Router::new()
        .route("/users", users_handler)
        .layer(Extension(users_repository))
}
