// src/models/vps.rs
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct VPSConfig {
    pub name: Option<String>,
    pub username: Option<String>,
    pub host: String,
    pub port: Option<String>,
    pub private_key: Option<std::path::PathBuf>,
    pub post_connect_script: Option<String>,
}
