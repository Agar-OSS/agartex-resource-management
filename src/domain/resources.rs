use serde::{Deserialize, Serialize};
use sqlx;
use validator::Validate;

use crate::constants::NAME_REGEX;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Resource {
    pub resource_id: i32,
    pub project_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Validate)]
pub struct ResourceMetadata {
    #[validate(length(min = 1, max = 128), regex = "NAME_REGEX")]
    pub name: String
}
