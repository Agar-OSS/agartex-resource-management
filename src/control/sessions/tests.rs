use axum::{Extension, headers::Authorization};
use http::StatusCode;
use mockall::predicate;
use sqlx::types::chrono::Utc;

use crate::{domain::{sessions::{SessionData, Session}, users::User}, repository::sessions::MockSessionRepository};

use super::*;

fn mock_email() -> String {
    String::from("email")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_header() -> TypedHeader<Authorization<Bearer>> {
    TypedHeader(Authorization::bearer(mock_session_id().as_str()).unwrap())
}

fn mock_user() -> User {
    User {
        id: 1, 
        email: mock_email(), 
        password_hash: mock_password() 
    }
}

fn mock_session_id() -> String {
    String::from_iter(std::iter::repeat('1').take(64))
}

fn mock_session_data() -> SessionData {
    SessionData {
        id: mock_session_id(),
        user_id: 1,
        expires: Utc::now().timestamp()
    }
}

fn mock_session() -> Session {
    Session { 
        id: mock_session_id(), 
        user: mock_user(),
        expires: Utc::now().timestamp()
    }
}

#[tokio::test]
async fn post_sessions_normal() {
    let mut session_repository = MockSessionRepository::new();

    session_repository
        .expect_insert()
        .with(predicate::eq(mock_session_data()))
        .times(1)
        .returning(|_| Ok(()));

    assert_eq!(StatusCode::CREATED, post_sessions(Extension(session_repository), Json(mock_session_data())).await)
}

#[tokio::test]
async fn post_sessions_duplicate_error() {
    let mut session_repository = MockSessionRepository::new();

    let session_data = mock_session_data();
    

    session_repository
        .expect_insert()
        .with(predicate::eq(session_data.clone()))
        .times(1)
        .returning(|_| Err(SessionInsertError::Duplicate));

    assert_eq!(StatusCode::CONFLICT, post_sessions(Extension(session_repository), Json(session_data)).await)
}

#[tokio::test]
async fn post_sessions_service_unknown_error() {
    let mut session_repository = MockSessionRepository::new();

    let session_data = mock_session_data();

    session_repository
        .expect_insert()
        .with(predicate::eq(session_data.clone()))
        .times(1)
        .returning(|_| Err(SessionInsertError::Unknown));

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, post_sessions(Extension(session_repository), Json(session_data)).await)
}

#[tokio::test]
async fn get_sessions_normal() {
    let mut session_repository = MockSessionRepository::new();

    let session_id = mock_session_id();
    let session = mock_session();
    let session_cpy = session.clone();

    session_repository
        .expect_get()
        .with(predicate::eq(session_id.clone()))
        .times(1)
        .return_once(|_| Ok(session_cpy));

    let res = get_sessions(Extension(session_repository), mock_header()).await;

    assert!(res.is_ok());
    assert_eq!(session, res.unwrap().0)
}

#[tokio::test]
async fn get_sessions_missing_error() {
    let mut session_repository = MockSessionRepository::new();

    let session_id = mock_session_id();

    session_repository
        .expect_get()
        .with(predicate::eq(session_id.clone()))
        .times(1)
        .returning(|_| Err(SessionGetError::Missing));

    let res = get_sessions(Extension(session_repository), mock_header()).await;

    assert!(res.is_err());
    assert_eq!(StatusCode::NOT_FOUND, res.unwrap_err())
}

#[tokio::test]
async fn get_sessions_unknown_error() {
    let mut session_repository = MockSessionRepository::new();

    let session_id = mock_session_id();

    session_repository
        .expect_get()
        .with(predicate::eq(session_id.clone()))
        .times(1)
        .returning(|_| Err(SessionGetError::Unknown));

    let res = get_sessions(Extension(session_repository), mock_header()).await;

    assert!(res.is_err());
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, res.unwrap_err())
}

#[tokio::test]
async fn delete_sessions_normal() {
    let mut session_repository = MockSessionRepository::new();

    let session_id = mock_session_id();

    session_repository
        .expect_delete()
        .with(predicate::eq(session_id.clone()))
        .times(1)
        .returning(|_| Ok(()));

    assert_eq!(StatusCode::NO_CONTENT, delete_sessions(Extension(session_repository), mock_header()).await)
}

#[tokio::test]
async fn delete_sessions_unknown_error() {
    let mut session_repository = MockSessionRepository::new();

    let session_id = mock_session_id();

    session_repository
        .expect_delete()
        .with(predicate::eq(session_id.clone()))
        .times(1)
        .returning(|_| Err(SessionDeleteError::Unknown));

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, delete_sessions(Extension(session_repository), mock_header()).await)
}
