use axum::{Router, routing, Extension};

use crate::{repository::users::PgUserRepository, control::users::{get_users, post_users}};

pub fn users_router(users_repository: PgUserRepository) -> Router {
    Router::new()
        .route("/:user_email", routing::get(get_users::<PgUserRepository>))
        .route("/", routing::post(post_users::<PgUserRepository>))
        .layer(Extension(users_repository))
}
