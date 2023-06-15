mod documents;
mod projects;
mod resources;
mod sessions;
mod users;

use axum::Router;
use sqlx::PgPool;

use crate::repository::{
    documents::PgDocumentRepository, projects::PgProjectRepository,
    resources::PgResourceRepository, sessions::PgSessionRepository, users::PgUserRepository,
};

use self::{projects::projects_router, sessions::sessions_router, users::users_router};

pub fn main_router(pool: &PgPool) -> Router {
    let users_repository = PgUserRepository::new(pool);
    let sessions_repository = PgSessionRepository::new(pool);
    let projects_repository = PgProjectRepository::new(pool);
    let documents_repository = PgDocumentRepository::new(pool);
    let resources_repository = PgResourceRepository::new(pool);

    Router::new()
        .nest("/users", users_router(users_repository))
        .nest("/sessions", sessions_router(sessions_repository))
        .nest(
            "/projects",
            projects_router(
                projects_repository,
                documents_repository,
                resources_repository,
            ),
        )
}
