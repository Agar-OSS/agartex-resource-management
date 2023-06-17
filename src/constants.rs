use std::{
    env,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use http::HeaderName;
use lazy_static::lazy_static;
use regex::Regex;

fn load_env_or_default<T>(var: &str, default: T) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    match env::var(var) {
        Ok(val) => T::from_str(&val).unwrap(),
        Err(_) => default,
    }
}

// implicit environment variables used:
// - PGHOST
// - PGPORT
// - PGDATABASE
// - PGUSER
// - PGPASSWORD
pub const FALLBACK_DB_URL: &str = "postgres://localhost:5432/agartex-db";
pub static XUSERID_HEADER_NAME: HeaderName = HeaderName::from_static("x-user-id");

lazy_static! {
    pub static ref SERVER_URL: SocketAddr = load_env_or_default(
        "SERVER_URL",
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3200)
    );
    pub static ref SESSION_COOKIE_NAME: String =
        load_env_or_default("SESSION_COOKIE_NAME", String::from("RSESSID"));
    pub static ref FILE_DIR_PATH: PathBuf =
        load_env_or_default("FILE_DIR_PATH", PathBuf::from(r"blobs"));
    pub static ref RESOURCE_SIZE_LIMIT_IN_BYTES: usize =
        load_env_or_default("RESOURCE_SIZE_LIMIT", 10 * 1024 * 1024);
    pub static ref NAME_REGEX: Regex = Regex::from_str(r"^[\w\s.-]+$").unwrap();
}
