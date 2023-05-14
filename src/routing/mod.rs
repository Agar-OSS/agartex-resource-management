mod documents;
mod projects;
mod resources;
mod sessions;
mod users;

use axum::Router;
use sqlx::PgPool;

use crate::repository::{
    projects::PgProjectRepository, sessions::PgSessionRepository, users::PgUserRepository,
};

use self::{projects::projects_router, sessions::sessions_router, users::users_router};

pub fn main_router(pool: &PgPool) -> Router {
    let users_repository = PgUserRepository::new(pool);
    let sessions_repository = PgSessionRepository::new(pool);
    let projects_repository = PgProjectRepository::new(pool);
    Router::new()
        .nest("/users", users_router(users_repository))
        .nest("/sessions", sessions_router(sessions_repository))
        .nest("/projects", projects_router(projects_repository))
}
