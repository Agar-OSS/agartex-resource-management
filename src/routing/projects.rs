use axum::{routing, Extension, Router};

use crate::{
    control::projects::{get_projects, post_projects, put_projects_metadata, get_project_document, put_project_document},
    repository::documents::PgDocumentRepository,
    repository::projects::PgProjectRepository,
    repository::resources::PgResourceRepository,
};

use super::documents::documents_router;
use super::resources::resources_router;

pub fn projects_router(
    projects_repository: PgProjectRepository, 
    documents_repository: PgDocumentRepository, 
    resources_repository: PgResourceRepository
) -> Router {
    let root_handler = routing::get(get_projects::<PgProjectRepository>)
        .post(post_projects::<PgProjectRepository>);

    let file_handler = routing::get(get_project_document::<PgProjectRepository, PgDocumentRepository>)
        .put(put_project_document::<PgProjectRepository, PgDocumentRepository>)
        .layer(Extension(documents_repository.clone()));

    let projects_router = Router::new()
        .route("/", root_handler)
        .route("/:project_id", file_handler)
        .route(
            "/:project_id/metadata",
            routing::put(put_projects_metadata::<PgProjectRepository>),
        )
        .layer(Extension(projects_repository));

    let lower_router = Router::new()
        .nest(
            "/:project_id/documents",
            documents_router(documents_repository),
        )
        .nest(
            "/:project_id/resources",
            resources_router(resources_repository),
        );
    projects_router.merge(lower_router)
}
