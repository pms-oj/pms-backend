use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Host {
    pub host: String,
    pub host_pass: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web {
    pub host: String,
    pub enable_gql_playground: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct General {
    pub timezone: Tz,
    pub default_language: Uuid,
    pub db_threads: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub host: Host,
    pub web: Web,
    pub redis: Redis,
}
