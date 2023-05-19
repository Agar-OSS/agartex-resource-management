use axum::{body::Body, routing, Extension, Router};

use crate::{
    control::projects::{get_projects, post_projects, put_projects_metadata},
    repository::documents::PgDocumentRepository,
    repository::projects::PgProjectRepository,
    repository::resources::PgResourceRepository,
};

use super::documents::documents_router;
use super::resources::resources_router;

pub fn projects_router(projects_repository: PgProjectRepository) -> Router {
    let pool = projects_repository.clone_pool();

    let root_handler = routing::get(get_projects::<PgProjectRepository>)
        .post(post_projects::<PgProjectRepository>);

    let projects_router = Router::new()
        .route("/", root_handler)
        .route(
            "/:project_id/metadata",
            routing::put(put_projects_metadata::<PgProjectRepository>),
        )
        .layer(Extension(projects_repository));

    let lower_router = Router::new()
        .nest(
            "/:project_id/documents",
            documents_router(PgDocumentRepository::new(&pool)),
        )
        .nest(
            "/:project_id/resources",
            resources_router(PgResourceRepository::new(&pool)),
        );
    projects_router.merge(lower_router)
}
