use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use chrono_tz::Tz;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Host {
    pub host: SocketAddr,
    pub host_pass: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web {
    pub host: SocketAddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct General {
    pub timezone: Tz,
    pub default_language: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: Host,
    pub web: Web,
    pub database: Database,
    pub redis: Redis,
}
