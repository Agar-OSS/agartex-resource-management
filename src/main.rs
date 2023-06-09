use std::fs;

use tracing::{error, info};

mod constants;
mod control;
mod database;
mod domain;
mod extractors;
mod filesystem;
mod repository;
mod routing;
mod validation;

use constants::{FILE_DIR_PATH, SERVER_URL};

#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    fs::create_dir_all(FILE_DIR_PATH.as_path())?;

    let pool = match database::create_conn_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}", err);
            return Err(anyhow::Error::from(err));
        }
    };

    info!("Running server!");
    axum::Server::try_bind(&SERVER_URL)?
        .serve(routing::main_router(&pool).into_make_service())
        .await
        .map_err(anyhow::Error::from)
}

#[tokio::main]
#[tracing::instrument]
async fn main() {
    if let Err(err) = run().await {
        error!(%err);
    }
}
