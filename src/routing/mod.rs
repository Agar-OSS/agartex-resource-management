mod users;

use axum::Router;
use sqlx::PgPool;

use crate::{repository::users::PgUserRepository};

use self::users::users_router;

pub fn main_router(pool: &PgPool) -> Router {
    let users_repository = PgUserRepository::new(pool);

    Router::new()
        .nest("/users", users_router(users_repository))
}
