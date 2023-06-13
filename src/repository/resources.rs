use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::{error};

use crate::{domain::resources::{Resource, ResourceMetadata}, filesystem::{write_file, get_resource_path, FileWriteError}};

pub enum ResourceInsertError {
    Duplicate,
    Unknown,
}
pub enum ResourceUpdateError {
    Missing,
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
    async fn get_meta(&self, project_id: i32, resource_id: i32) -> Result<Resource, ResourceGetError>;
    async fn insert(&self, project_id: i32, data: &ResourceMetadata)
        -> Result<Resource, ResourceInsertError>;
    async fn update(
        &self,
        project_id: i32,
        resource_id: i32,
        content: &[u8]
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
    #[tracing::instrument(skip(self))]
    async fn get(&self, project_id: i32) -> Result<Vec<Resource>, ResourceGetError> {
        let resource_get_sql = "
            SELECT resource_id, project_id, name
            FROM resources
            WHERE project_id = $1
        ";
        
        let resources = sqlx::query_as::<_, Resource>(resource_get_sql)
            .bind(project_id)
            .fetch_all(&self.pool)
            .await;

        match resources {
            Ok(resources) => Ok(resources),
            Err(err) => {
                error!(%err);
                return Err(ResourceGetError::Unknown);
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_meta(&self, project_id: i32, resource_id: i32) -> Result<Resource, ResourceGetError> {
        let resource_get_sql = "
            SELECT resource_id, project_id, name
            FROM resources
            WHERE project_id = $1 AND resource_id = $2
        ";
        
        let result = sqlx::query_as::<_, Resource>(resource_get_sql)
            .bind(project_id)
            .bind(resource_id)
            .fetch_optional(&self.pool);

        match result.await {
            Ok(Some(resource)) => Ok(resource),
            Ok(None) => Err(ResourceGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(ResourceGetError::Unknown);
            }
        }
    }

    #[tracing::instrument(skip(self, content))]
    async fn update(
        &self,
        project_id: i32,
        resource_id: i32,
        content: &[u8],
    ) -> Result<(), ResourceUpdateError> {
        let resource = match self.get_meta(project_id, resource_id).await {
            Ok(resource) => resource,
            Err(ResourceGetError::Missing) => return Err(ResourceUpdateError::Missing),
            Err(ResourceGetError::Unknown) => return Err(ResourceUpdateError::Unknown)
        };

        write_file(get_resource_path(&resource), content, false).await.map_err(|err| {
            match err {
                FileWriteError::Missing => ResourceUpdateError::Missing,
                FileWriteError::Unknown => ResourceUpdateError::Unknown
            }
        })
    }

    #[tracing::instrument(skip(self))]
    async fn insert(
        &self,
        project_id: i32,
        resource_data: &ResourceMetadata,
    ) -> Result<Resource, ResourceInsertError> {
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!(%err);
                return Err(ResourceInsertError::Duplicate);
            }
        };

        let insert_resource_sql = r#"
            INSERT INTO resources (project_id, name)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING resource_id, project_id, name
        "#;
        
        let result = sqlx::query_as::<_, Resource>(insert_resource_sql)
            .bind(project_id)
            .bind(&resource_data.name)
            .fetch_optional(&mut tx);

        let resource = match result.await {
            Ok(Some(resource)) => resource,
            Ok(None) => return Err(ResourceInsertError::Duplicate),
            Err(err) => {
                error!(%err);
                return Err(ResourceInsertError::Unknown);
            }
        };

        if write_file(get_resource_path(&resource), b"", true).await.is_err() {
            return Err(ResourceInsertError::Unknown);
        }

        match tx.commit().await {
            Ok(_) => Ok(resource),
            Err(err) => {
                error!(%err);
                Err(ResourceInsertError::Unknown)
            }
        }
    }
}
