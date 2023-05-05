use axum::{Router, routing, Extension};

use crate::{repository::sessions::PgSessionRepository, control::sessions::{get_sessions, post_sessions, delete_sessions}};

pub fn sessions_router(sessions_repository: PgSessionRepository) -> Router {
    let root_handler = routing
        ::get(get_sessions::<PgSessionRepository>)
        .post(post_sessions::<PgSessionRepository>)
        .delete(delete_sessions::<PgSessionRepository>);
    
    Router::new()
        .route("/", root_handler)
        .layer(Extension(sessions_repository))
}
