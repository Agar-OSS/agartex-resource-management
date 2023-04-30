use axum::Extension;
use http::StatusCode;
use mockall::predicate;

use crate::{domain::users::{UserData, User}, repository::users::MockUserRepository};

use super::*;

fn mock_email() -> String {
    String::from("email")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_user_data() -> UserData {
    UserData {
        email: mock_email(),
        password_hash: mock_password()
    }
}

fn mock_user() -> User {
    User { 
        id: 1, 
        email: mock_email(), 
        password_hash: mock_password() 
    }
}

#[tokio::test]
async fn post_users_normal() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_insert()
        .with(predicate::eq(mock_user_data()))
        .times(1)
        .returning(|_| Ok(()));

    assert_eq!(Ok(StatusCode::CREATED), post_users(Extension(user_repository), Json(mock_user_data())).await)
}

#[tokio::test]
async fn post_users_duplicate_error() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_insert()
        .with(predicate::eq(mock_user_data()))
        .times(1)
        .returning(|_| Err(UserInsertError::Duplicate));

    assert_eq!(Err(StatusCode::CONFLICT), post_users(Extension(user_repository), Json(mock_user_data())).await)
}

#[tokio::test]
async fn post_users_service_unknown_error() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_insert()
        .with(predicate::eq(mock_user_data()))
        .times(1)
        .returning(|_| Err(UserInsertError::Unknown));

    assert_eq!(Err(StatusCode::INTERNAL_SERVER_ERROR), post_users(Extension(user_repository), Json(mock_user_data())).await)
}

#[tokio::test]
async fn get_users_normal() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_get_by_email()
        .with(predicate::eq(mock_email()))
        .times(1)
        .returning(|_| Ok(mock_user()));

    let res = get_users(Extension(user_repository), Path(mock_email())).await;

    assert!(res.is_ok());
    assert_eq!(mock_user(), res.unwrap().0)
}

#[tokio::test]
async fn get_users_missing_error() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_get_by_email()
        .with(predicate::eq(mock_email()))
        .times(1)
        .returning(|_| Err(UserGetError::Missing));

    let res = get_users(Extension(user_repository), Path(mock_email())).await;

    assert!(res.is_err());
    assert_eq!(StatusCode::NOT_FOUND, res.unwrap_err())
}

#[tokio::test]
async fn get_users_unknown_error() {
    let mut user_repository = MockUserRepository::new();

    user_repository
        .expect_get_by_email()
        .with(predicate::eq(mock_email()))
        .times(1)
        .returning(|_| Err(UserGetError::Unknown));

    let res = get_users(Extension(user_repository), Path(mock_email())).await;

    assert!(res.is_err());
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, res.unwrap_err())
}
