use std::{io::BufReader, fs::File, path::Path, net::SocketAddr};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub database_addr: String
}

const CONFIG_PATH: &str = "config.json";

pub const SERVER_ADDR_ENV_VAR: &str = "SERVER_ADDR";
pub const DATABASE_ADDR_ENV_VAR: &str = "DATABASE_ADDR";

fn load_config() -> Config {
    let mut cfg: Config = serde_json::from_reader(BufReader::new(File::open(Path::new(CONFIG_PATH)).unwrap())).unwrap();

    if let Ok(server_addr) = std::env::var(SERVER_ADDR_ENV_VAR) {
        cfg.server_addr = server_addr.parse().unwrap();
    }

    if let Ok(database_addr) = std::env::var(DATABASE_ADDR_ENV_VAR) {
        cfg.database_addr = database_addr.parse().unwrap();
    }

    cfg
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}
