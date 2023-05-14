use axum::{routing, Extension, Router};

use crate::{
    control::resources::{
        get_resources, post_resources, post_resources_content, put_resources_metadata,
    },
    repository::resources::PgResourceRepository,
};

pub fn resources_router(documents_repository: PgResourceRepository) -> Router {
    let root_handler = routing::get(get_resources::<PgResourceRepository>)
        .post(post_resources::<PgResourceRepository>);

    let documents_router = Router::new()
        .route("/", root_handler)
        .route(
            "/:document_id",
            routing::post(post_resources_content::<PgResourceRepository>),
        )
        .route(
            "/:document_id/metadata",
            routing::put(put_resources_metadata::<PgResourceRepository>),
        )
        .layer(Extension(documents_repository));

    documents_router
}
