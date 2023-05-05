mod users;
mod sessions;

use axum::Router;
use sqlx::PgPool;

use crate::{repository::{users::PgUserRepository, sessions::PgSessionRepository}};

use self::{users::users_router, sessions::sessions_router};

pub fn main_router(pool: &PgPool) -> Router {
    let users_repository = PgUserRepository::new(pool);
    let sessions_repository = PgSessionRepository::new(pool);

    Router::new()
        .nest("/users", users_router(users_repository))
        .nest("/sessions", sessions_router(sessions_repository))
}
