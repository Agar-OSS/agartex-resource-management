use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::{error, info};

use crate::domain::{
    crud::CrudInt,
    projects::{Project, ProjectData, ProjectMetaData},
};

pub enum ProjectInsertError {
    Unknown,
    TransactionFailure,
}
pub enum ProjectUpdateError {
    Unknown,
}
pub enum ProjectGetError {
    Missing,
    Unknown,
}

#[automock]
#[async_trait]
pub trait ProjectRepository {
    async fn get(&self, id: i32) -> Result<Vec<Project>, ProjectGetError>;
    async fn insert(&self, data: &ProjectData, owner: i32) -> Result<(), ProjectInsertError>;
    async fn update(&self, id: i32, data: &ProjectMetaData) -> Result<(), ProjectUpdateError>;
}

#[derive(Debug, Clone)]
pub struct PgProjectRepository {
    pub pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
    pub fn clone_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn get(&self, id: i32) -> Result<Vec<Project>, ProjectGetError> {
        let projects = sqlx::query_as::<_, Project>(
            "
            SELECT project_id, main_document_id, owner, name
            FROM projects
            WHERE projects.owner = $1
        ",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await;

        match projects {
            Ok(projects) => Ok(projects),
            Err(err) => {
                error!(%err);
                return Err(ProjectGetError::Unknown);
            }
        }
    }

    async fn update(
        &self,
        id: i32,
        project_metadata: &ProjectMetaData,
    ) -> Result<(), ProjectUpdateError> {
        let result = sqlx::query(
            "
            UPDATE projects 
            SET name = $1
            WHERE project_id = $2
        ",
        )
        .bind(&project_metadata.name)
        .bind(id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_result) => Ok(()),
            Err(err) => {
                error!(%err);
                return Err(ProjectUpdateError::Unknown);
            }
        }
    }
    async fn insert(
        &self,
        project_data: &ProjectData,
        owner: i32,
    ) -> Result<(), ProjectInsertError> {
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return Err(ProjectInsertError::TransactionFailure),
        };
        info!("transaction aquired");
        let document_id = match sqlx::query_as::<_, CrudInt>(
            "
            INSERT INTO documents (name) 
            VALUES ('main')
            ON CONFLICT DO NOTHING
            RETURNING document_id as id
        ",
        )
        .fetch_optional(&mut tx)
        .await
        {
            Ok(Some(document_id)) => document_id,
            Ok(None) => return Err(ProjectInsertError::TransactionFailure),
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::TransactionFailure);
            }
        };
        info!("first query done");

        let project_id = match sqlx::query_as::<_, CrudInt>(
            "
            INSERT INTO projects (main_document_id, owner, name)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            RETURNING project_id as id
        ",
        )
        .bind(document_id.id)
        .bind(owner)
        .bind(&project_data.name)
        .fetch_optional(&mut tx)
        .await
        {
            Ok(Some(project_id)) => project_id,
            Ok(None) => return Err(ProjectInsertError::TransactionFailure),
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::TransactionFailure);
            }
        };

        // Error was not handled but Ok was returned
        let _result_update_documents = sqlx::query(
            "
                UPDATE documents
                SET project_id = $1
                WHERE document_id = $2
            ",
        )
        .bind(project_id.id)
        .bind(document_id.id)
        .execute(&mut tx)
        .await;

        let result = tx.commit().await;
        match result {
            Ok(()) => Ok(()),
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::Unknown);
            }
        }
    }
}
