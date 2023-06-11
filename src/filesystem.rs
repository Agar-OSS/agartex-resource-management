use std::path::PathBuf;

use tokio::fs;
use tracing::{error, warn};

use crate::constants::FILE_DIR_PATH;

pub enum FileWriteError {
    Missing,
    Unknown
}

#[tracing::instrument]
pub async fn write_file(path: PathBuf, content: &str, create_if_not_exists: bool) -> Result<(), FileWriteError> {
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

pub fn get_document_path(document_id: i32) -> PathBuf {
    let mut file_path = FILE_DIR_PATH.clone();
    file_path.push(document_id.to_string());
    file_path
}
