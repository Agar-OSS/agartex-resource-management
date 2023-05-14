use std::result;

use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::error;
pub enum CrudCheckError {
    Missing,
    Unknown,
}

use crate::domain::crud::CrudInt;

use crate::repository::projects::PgProjectRepository;

use super::documents::PgDocumentRepository;
use super::resources::PgResourceRepository;

#[automock]
#[async_trait]
pub trait CrudRepository {
    fn get_pool(&self) -> &PgPool;
    async fn check(&self, id: i32) -> Result<CrudInt, CrudCheckError> {
        let user = sqlx::query_as::<_, CrudInt>(
            "
            SELECT sessions.user_id as id 
            FROM sessions
            WHERE sessions.user_id = $1
        ",
        )
        .bind(id)
        .fetch_optional(self.get_pool())
        .await;

        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => return Err(CrudCheckError::Missing),
            Err(err) => {
                error!(%err);
                return Err(CrudCheckError::Unknown);
            }
        }
    }
}

impl CrudRepository for PgProjectRepository {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl CrudRepository for PgDocumentRepository {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl CrudRepository for PgResourceRepository {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
