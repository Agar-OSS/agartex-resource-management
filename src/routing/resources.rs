use axum::{extract::DefaultBodyLimit, routing, Extension, Router};

use crate::{
    constants::RESOURCE_SIZE_LIMIT_IN_BYTES,
    control::resources::{get_projects_resources, post_projects_resources, put_projects_resources},
    repository::{projects::PgProjectRepository, resources::PgResourceRepository},
};

pub fn resources_router(resources_repository: PgResourceRepository) -> Router {
    let root_handler =
        routing::post(post_projects_resources::<PgProjectRepository, PgResourceRepository>)
            .get(get_projects_resources::<PgResourceRepository>);

    let resource_id_handler = routing::put(put_projects_resources::<PgResourceRepository>)
        .layer(DefaultBodyLimit::max(*RESOURCE_SIZE_LIMIT_IN_BYTES));

    Router::new()
        .route("/", root_handler)
        .route("/:resource_id", resource_id_handler)
        .layer(Extension(resources_repository))
}
