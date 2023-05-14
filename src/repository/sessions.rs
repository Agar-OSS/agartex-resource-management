use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::{error, info};

use crate::domain::sessions::{Session, SessionData};

pub enum SessionGetError {
    Missing,
    Unknown,
}

pub enum SessionDeleteError {
    Unknown,
}

pub enum SessionInsertError {
    Duplicate,
    Unknown,
}

#[automock]
#[async_trait]
pub trait SessionRepository {
    async fn insert(&self, session: &SessionData) -> Result<(), SessionInsertError>;
    async fn get(&self, id: &str) -> Result<Session, SessionGetError>;
    async fn delete(&self, id: &str) -> Result<(), SessionDeleteError>;
}

#[derive(Debug, Clone)]
pub struct PgSessionRepository {
    pub pool: PgPool,
}

impl PgSessionRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    #[tracing::instrument(skip_all, fields(user_id = session_data.user_id))]
    async fn insert(&self, session_data: &SessionData) -> Result<(), SessionInsertError> {
        let result = sqlx::query(
            "
            INSERT INTO sessions (session_id, user_id, expires) 
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
        ",
        )
        .bind(&session_data.id)
        .bind(session_data.user_id)
        .bind(session_data.expires)
        .execute(&self.pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(SessionInsertError::Duplicate)
                }
            }
            Err(err) => {
                error!(%err);
                Err(SessionInsertError::Unknown)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn get(&self, id: &str) -> Result<Session, SessionGetError> {
        info!(id);
        let session = sqlx::query_as::<_, Session>(
            "
            SELECT session_id, users.user_id, expires, email, password_hash
            FROM sessions JOIN users
            ON sessions.user_id = users.user_id
            WHERE sessions.session_id = $1
        ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await;

        match session {
            Ok(Some(session)) => Ok(session),
            Ok(None) => return Err(SessionGetError::Missing),
            Err(err) => {
                error!(%err);
                return Err(SessionGetError::Unknown);
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn delete(&self, id: &str) -> Result<(), SessionDeleteError> {
        match sqlx::query("DELETE FROM sessions WHERE session_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(%err);
                Err(SessionDeleteError::Unknown)
            }
        }
    }
}
