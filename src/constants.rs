use std::{str::FromStr, fmt::Debug, env, net::{SocketAddr, IpAddr, Ipv4Addr}};

use lazy_static::lazy_static;
use regex::Regex;

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
pub const PASSWORD_SPECIAL_CHARS: &str = "!@#$%^&*";

lazy_static! {
    pub static ref PASSWORD_REGEX: Regex = Regex::new(format!("^[A-Za-z0-9{}]*$", PASSWORD_SPECIAL_CHARS).as_str()).unwrap();

    pub static ref SERVER_URL: SocketAddr = load_env_or_default("SERVER_URL", SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3200));
    pub static ref HASH_COST: u32 = load_env_or_default("BCRYPT_HASH_COST", 12);
}
