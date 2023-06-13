use std::path::PathBuf;

use tokio::fs;
use tracing::{error, warn};

use crate::{constants::FILE_DIR_PATH, domain::{resources::Resource, documents::Document}};

pub enum FileWriteError {
    Missing,
    Unknown
}

#[tracing::instrument(skip(content))]
pub async fn write_file(path: PathBuf, content: impl AsRef<[u8]>, create_if_not_exists: bool) -> Result<(), FileWriteError> {
    if !path.exists() && !create_if_not_exists {
        warn!("Missing file");
        return Err(FileWriteError::Missing);
    }

    fs::write(path, content).await.map_err(|err| {
        error!(%err);
        FileWriteError::Unknown
    })
}

pub enum FileReadError {
    Missing,
    Unknown
}

#[tracing::instrument]
pub async fn read_file(path: PathBuf) -> Result<String, FileReadError> {
    if !path.exists() {
        warn!("Missing file");
        return Err(FileReadError::Missing);
    }

    fs::read_to_string(path).await.map_err(|err| {
        error!(%err);
        FileReadError::Unknown
    })
}

#[tracing::instrument]
pub fn get_document_path(document: &Document) -> PathBuf {
    let mut file_path = FILE_DIR_PATH.clone();
    file_path.extend([&document.project_id.to_string(), &document.name]);
    file_path
}

#[tracing::instrument]
pub fn get_resource_path(resource: &Resource) -> PathBuf {
    let mut file_path = FILE_DIR_PATH.clone();
    file_path.extend([&resource.project_id.to_string(), &resource.name]);
    file_path
}

#[tracing::instrument]
pub fn get_project_path(project_id: i32) -> PathBuf {
    let mut dir_path = FILE_DIR_PATH.clone();
    dir_path.push(project_id.to_string());
    dir_path
}