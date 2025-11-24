// Module: Model/VPS
// Location: cli/src/model/vps/xml.rs

use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct VpsConfigXml {
    pub vps: Vec<VpsEntryXml>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct VpsEntryXml {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_connect_script: Option<String>,
}
