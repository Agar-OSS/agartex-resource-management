use std::{str::FromStr, fmt::Debug, env, net::{SocketAddr, IpAddr, Ipv4Addr}};

use lazy_static::lazy_static;

fn load_env_or_default<T>(var: &str, default: T) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    match env::var(var) {
        Ok(val) => T::from_str(&val).unwrap(),
        Err(_) => default
    }
}

// implicit environment variables used:
// - PGHOST
// - PGPORT
// - PGDATABASE
// - PGUSER
// - PGPASSWORD
pub const FALLBACK_DB_URL: &str = "postgres://localhost:5432/agartex-db";

lazy_static! {
    pub static ref SERVER_URL: SocketAddr = load_env_or_default("SERVER_URL", SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3200));
    pub static ref SESSION_COOKIE_NAME: String = load_env_or_default("SESSION_COOKIE_NAME", String::from("RSESSID"));
}
