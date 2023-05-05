use axum::{Router, routing, Extension};

use crate::{repository::sessions::PgSessionRepository, control::sessions::{get_sessions, post_sessions, delete_sessions}};

pub fn sessions_router(sessions_repository: PgSessionRepository) -> Router {
    Router::new()
        .route(
            "/:session_id", 
            routing::get(get_sessions::<PgSessionRepository>)
                .delete(delete_sessions::<PgSessionRepository>)
        )
        .route("/", routing::post(post_sessions::<PgSessionRepository>))
        .layer(Extension(sessions_repository))
}
