use axum::{routing, Extension, Router};

use crate::{
    control::{
        documents::{get_projects_documents, put_projects_documents},
        projects::{get_projects, get_projects_metadata, post_projects, put_projects_metadata},
        sharing::{post_projects_sharing, put_projects_sharing},
    },
    repository::documents::PgDocumentRepository,
    repository::resources::PgResourceRepository,
    repository::{projects::PgProjectRepository, sharing::PgProjectSharingRepository},
};

use super::resources::resources_router;

pub fn projects_router(
    projects_repository: PgProjectRepository,
    documents_repository: PgDocumentRepository,
    resources_repository: PgResourceRepository,
    sharing_repository: PgProjectSharingRepository,
) -> Router {
    let root_handler = routing::get(get_projects::<PgProjectRepository>)
        .post(post_projects::<PgProjectRepository>);

    let documents_router =
        routing::get(get_projects_documents::<PgProjectRepository, PgDocumentRepository>)
            .put(put_projects_documents::<PgProjectRepository, PgDocumentRepository>)
            .layer(Extension(documents_repository));
    let sharing_router = Router::new()
        .route(
            "/sharing/:token",
            routing::post(post_projects_sharing::<PgProjectSharingRepository>),
        )
        .route(
            "/:project_id/sharing",
            routing::put(put_projects_sharing::<PgProjectSharingRepository>),
        )
        .layer(Extension(sharing_repository));

    let metadata_handler = routing::put(put_projects_metadata::<PgProjectRepository>)
        .get(get_projects_metadata::<PgProjectRepository>);

    Router::new()
        .route("/", root_handler)
        .merge(sharing_router)
        .route("/:project_id", documents_router)
        .route("/:project_id/metadata", metadata_handler)
        .nest(
            "/:project_id/resources",
            resources_router(resources_repository),
        )
        .layer(Extension(projects_repository))
}
