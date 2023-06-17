use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tokio::fs;
use tracing::{error, info, warn};

use crate::{
    domain::{
        crud::CrudInt,
        documents::Document,
        projects::{Project, ProjectMetaData},
    },
    filesystem::{get_document_path, get_project_path, write_file},
};

pub enum ProjectInsertError {
    Unknown,
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
    async fn get_meta(&self, project_id: i32) -> Result<Project, ProjectGetError>;
    async fn insert(
        &self,
        data: &ProjectMetaData,
        owner_id: i32,
    ) -> Result<Project, ProjectInsertError>;
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
}

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    #[tracing::instrument(skip(self))]
    async fn get(&self, id: i32) -> Result<Vec<Project>, ProjectGetError> {
        let projects = sqlx::query_as::<_, Project>(
            "
            SELECT project_id, projcet_name, created_at, last_modified, owner_id_id
            FROM projects
            WHERE projects.owner_id = $1
        ",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await;

        match projects {
            Ok(projects) if !projects.is_empty() => Ok(projects),
            Ok(_projects) => Err(ProjectGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(ProjectGetError::Unknown);
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_meta(&self, project_id: i32) -> Result<Project, ProjectGetError> {
        let sql = "
            SELECT project_id, main_document_id, owner_id_id, project_name
            FROM projects
            WHERE project_id = $1
        ";

        let project = sqlx::query_as::<_, Project>(sql)
            .bind(project_id)
            .fetch_optional(&self.pool)
            .await;

        match project {
            Ok(Some(project)) => Ok(project),
            Ok(None) => {
                warn!("Missing project");
                Err(ProjectGetError::Missing)
            }
            Err(err) => {
                error!(%err);
                Err(ProjectGetError::Unknown)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn update(
        &self,
        id: i32,
        project_metadata: &ProjectMetaData,
    ) -> Result<(), ProjectUpdateError> {
        let result = sqlx::query(
            "
            UPDATE projects 
            SET project_name = $1
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

    #[tracing::instrument(skip(self))]
    async fn insert(
        &self,
        project_data: &ProjectMetaData,
        owner_id: i32,
    ) -> Result<Project, ProjectInsertError> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::Unknown);
            }
        };
        info!("Transaction aquired");

        let insert_document_sql = "
            INSERT INTO documents (name) 
            VALUES ('main.tex')
            RETURNING document_id as id
        ";
        let insert_document_result =
            sqlx::query_as::<_, CrudInt>(insert_document_sql).fetch_one(&mut tx);

        let document_id = match insert_document_result.await {
            Ok(document_id) => document_id.id,
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::Unknown);
            }
        };

        info!("Created document {}", document_id);

        let insert_project_sql = "
            INSERT INTO projects (main_document_id, owner_id_id, project_name)
            VALUES ($1, $2, $3)
            RETURNING project_id, main_document_id, created_at, last_modified, owner_id_id, project_name
        ";
        let insert_project_result = sqlx::query_as::<_, Project>(insert_project_sql)
            .bind(document_id)
            .bind(owner_id)
            .bind(&project_data.name)
            .fetch_one(&mut tx);

        let project = match insert_project_result.await {
            Ok(project) => project,
            Err(err) => {
                error!(%err);
                return Err(ProjectInsertError::Unknown);
            }
        };

        if let Err(err) = fs::create_dir(get_project_path(project.project_id)).await {
            error!(%err);
            return Err(ProjectInsertError::Unknown);
        }

        let update_document_sql = "
            UPDATE documents
            SET project_id = $1
            WHERE document_id = $2
        ";
        let update_document_result = sqlx::query(update_document_sql)
            .bind(project.project_id)
            .bind(document_id)
            .execute(&mut tx);

        if let Err(err) = update_document_result.await {
            error!(%err);
            return Err(ProjectInsertError::Unknown);
        }

        let document = Document {
            document_id,
            project_id: project.project_id,
            name: String::from("main.tex"),
        };

        if write_file(get_document_path(&document), "", true)
            .await
            .is_err()
        {
            return Err(ProjectInsertError::Unknown);
        }

        match tx.commit().await {
            Ok(_) => Ok(project),
            Err(err) => {
                error!(%err);
                Err(ProjectInsertError::Unknown)
            }
        }
    }
}
