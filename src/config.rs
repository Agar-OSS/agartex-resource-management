use std::{io::BufReader, fs::File, path::Path, net::SocketAddr};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub database_addr: String
}

const CONFIG_PATH: &str = "config.json";

lazy_static! {
    pub static ref CONFIG: Config = serde_json::from_reader(BufReader::new(File::open(Path::new(CONFIG_PATH)).unwrap())).unwrap();
}
