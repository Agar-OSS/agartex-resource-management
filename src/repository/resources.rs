use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::{error};

use crate::domain::resources::{Resource, ResourceData};

pub enum ResourceInsertError {
    Unknown,
}
pub enum ResourceUpdateError {
    Unknown,
}
pub enum ResourceGetError {
    Missing,
    Unknown,
}

#[automock]
#[async_trait]
pub trait ResourceRepository {
    async fn get(&self, project_id: i32) -> Result<Vec<Resource>, ResourceGetError>;
    async fn insert(&self, project_id: i32, data: &ResourceData)
        -> Result<(), ResourceInsertError>;
    async fn update(
        &self,
        project_id: i32,
        resource_id: i32,
        data: &ResourceData,
    ) -> Result<(), ResourceUpdateError>;
}

#[derive(Debug, Clone)]
pub struct PgResourceRepository {
    pub pool: PgPool,
}

impl PgResourceRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl ResourceRepository for PgResourceRepository {
    async fn get(&self, project_id: i32) -> Result<Vec<Resource>, ResourceGetError> {
        let resources = sqlx::query_as::<_, Resource>(
            "
            SELECT resource_id, project_id, name
            FROM resources
            WHERE resources.project_id = $1
        ",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await;

        match resources {
            Ok(resources) if resources.len() > 0 => Ok(resources),
            Ok(_resources) => Err(ResourceGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(ResourceGetError::Unknown);
            }
        }
    }

    async fn update(
        &self,
        _project_id: i32,
        resource_id: i32,
        resource_metadata: &ResourceData,
    ) -> Result<(), ResourceUpdateError> {
        let result = sqlx::query(
            "
            UPDATE resources 
            SET name = $1
            WHERE resource_id = $2
        ",
        )
        .bind(&resource_metadata.name)
        .bind(resource_id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_result) => Ok(()),
            Err(err) => {
                error!(%err);
                return Err(ResourceUpdateError::Unknown);
            }
        }
    }
    async fn insert(
        &self,
        project_id: i32,
        resource_data: &ResourceData,
    ) -> Result<(), ResourceInsertError> {
        let result = sqlx::query(
            "
            INSERT_INTO resources (project_id, name)
            VALUES ($1, $2)
            ON CONFLICT do DO NOTHING
        ",
        )
        .bind(project_id)
        .bind(&resource_data.name)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_result) => Ok(()),
            Err(err) => {
                error!(%err);
                return Err(ResourceInsertError::Unknown);
            }
        }
    }
}
