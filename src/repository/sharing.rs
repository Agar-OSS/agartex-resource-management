use axum::async_trait;
use mockall::automock;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use tracing::{error, info, warn};

use crate::domain::crud::CrudInt;

pub enum ProjectSharingCreateError {
    Unknown,
}
pub enum ProjectSharingUpdateError {
    Unknown,
}

#[automock]
#[async_trait]
pub trait ProjectSharingRepository {
    async fn create(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<String, ProjectSharingCreateError>;
    async fn update(&self, token: String, user_id: i32) -> Result<(), ProjectSharingUpdateError>;
}

#[derive(Debug, Clone)]
pub struct PgProjectSharingRepository {
    pub pool: PgPool,
}

impl PgProjectSharingRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
    pub fn generate_random_token(&self, len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

#[async_trait]
impl ProjectSharingRepository for PgProjectSharingRepository {
    async fn create(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<String, ProjectSharingCreateError> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!(%err);
                return Err(ProjectSharingCreateError::Unknown);
            }
        };
        info!("Transaction acquired");

        let check_if_project_exists = "
            SELECT project_id
            FROM projects
            WHERE projects.owner_id= $1 and projects.project_id = $2 
        ";
        let insert_document_sql = "
            INSERT INTO tokens (token, project_id) 
            VALUES ($1, $2)
        ";

        let check_project_result = sqlx::query(check_if_project_exists)
            .bind(user_id)
            .bind(project_id)
            .fetch_optional(&mut tx);

        if let Err(err) = check_project_result.await {
            error!(%err);
            return Err(ProjectSharingCreateError::Unknown);
        }

        let token = self.generate_random_token(64);
        let insert_document_result = sqlx::query(insert_document_sql)
            .bind(&token)
            .bind(project_id)
            .execute(&mut tx);

        if let Err(err) = insert_document_result.await {
            error!(%err);
            return Err(ProjectSharingCreateError::Unknown);
        }
        match tx.commit().await {
            Ok(_) => Ok(token),
            Err(err) => {
                error!(%err);
                Err(ProjectSharingCreateError::Unknown)
            }
        }
    }

    async fn update(&self, token: String, user_id: i32) -> Result<(), ProjectSharingUpdateError> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!(%err);
                return Err(ProjectSharingUpdateError::Unknown);
            }
        };
        info!("Transaction acquired");
        info!(token);

        let get_project_id_sql = "
            SELECT project_id as id
            FROM tokens
            WHERE token = $1
        ";
        let get_project_id = sqlx::query_as::<_, CrudInt>(get_project_id_sql)
            .bind(&token)
            .fetch_one(&mut tx);

        info!("token parsing done");
        let project_id = match get_project_id.await {
            Ok(project_id) => project_id,
            Err(err) => {
                error!(%err);
                return Err(ProjectSharingUpdateError::Unknown);
            }
        };

        let update_document_sql = "
            INSERT INTO sharing (friend_id, project_id)
            VALUES ($1, $2)
        ";

        let insert_document_result = sqlx::query(update_document_sql)
            .bind(user_id)
            .bind(project_id.id)
            .execute(&mut tx);

        if let Err(err) = insert_document_result.await {
            error!(%err);
            return Err(ProjectSharingUpdateError::Unknown);
        }

        match tx.commit().await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(%err);
                Err(ProjectSharingUpdateError::Unknown)
            }
        }
    }
}
