use axum::{Router, routing::{put, post}, Extension, extract::DefaultBodyLimit};

use crate::{repository::{resources::PgResourceRepository, projects::PgProjectRepository}, control::resources::{get_projects_resources, post_projects_resources, put_projects_resources}, constants::RESOURCE_SIZE_LIMIT_IN_BYTES};

pub fn resources_router(resources_repository: PgResourceRepository) -> Router {
    let root_handler = post(post_projects_resources::<PgProjectRepository, PgResourceRepository>)
        .get(get_projects_resources::<PgResourceRepository>);
    
    let resource_id_handler = put(put_projects_resources::<PgResourceRepository>)
        .layer(DefaultBodyLimit::max(*RESOURCE_SIZE_LIMIT_IN_BYTES));

    Router::new()
        .route("/", root_handler)
        .route("/:resource_id", resource_id_handler)
        .layer(Extension(resources_repository))
}
