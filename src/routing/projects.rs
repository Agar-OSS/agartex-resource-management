use axum::{routing, Extension, Router};

use crate::{
    control::{projects::{get_projects, post_projects, put_projects_metadata}, documents::{get_projects_documents, put_projects_documents}},
    repository::documents::PgDocumentRepository,
    repository::projects::PgProjectRepository,
    repository::resources::PgResourceRepository,
};

use super::resources::resources_router;

pub fn projects_router(
    projects_repository: PgProjectRepository, 
    documents_repository: PgDocumentRepository, 
    resources_repository: PgResourceRepository
) -> Router {
    let root_handler = routing::get(get_projects::<PgProjectRepository>)
        .post(post_projects::<PgProjectRepository>);

    let documents_router = routing::get(get_projects_documents::<PgProjectRepository, PgDocumentRepository>)
        .put(put_projects_documents::<PgProjectRepository, PgDocumentRepository>)
        .layer(Extension(documents_repository));

    Router::new()
        .route("/", root_handler)
        .route("/:project_id", documents_router)
        .route(
            "/:project_id/metadata",
            routing::put(put_projects_metadata::<PgProjectRepository>),
        )
        .nest(
            "/:project_id/resources",
            resources_router(resources_repository)
        )
        .layer(Extension(projects_repository))
}
