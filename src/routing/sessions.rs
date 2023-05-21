use axum::{routing, Extension, Router};

use crate::{
    control::sessions::{delete_sessions, get_sessions, post_sessions},
    repository::sessions::PgSessionRepository,
};

pub fn sessions_router(sessions_repository: PgSessionRepository) -> Router {
    let root_handler = routing::get(get_sessions::<PgSessionRepository>)
        .post(post_sessions::<PgSessionRepository>)
        .delete(delete_sessions::<PgSessionRepository>);

    Router::new()
        .route("/", root_handler)
        .layer(Extension(sessions_repository))
}
