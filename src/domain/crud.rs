use serde::Serialize;
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct CrudInt {
    pub id: i32,
}
