// Module: Model/VPS
// Location: cli/src/model/vps.rs
use crate::{
    config::TranquilityConfig,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::{PathBuf}};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VPSList {
    pub vps: Vec<VPSConfig>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct VPSConfig {
    pub id: Option<String>,
    pub name: Option<String>,
    pub user: Option<String>,
    pub host: String,
    pub port: Option<String>,
    pub private_key: Option<PathBuf>,
    pub post_connect_script: Option<String>,
}

impl VPSList {
    /// Fills in missing fields like id, port, and user; ensures unique ids
    pub fn fix(&mut self) -> bool {
        use std::collections::HashSet;

        let mut seen_ids = HashSet::new();
        let mut changed = false;

        for vps in &mut self.vps {
            let original_id = vps.id.clone();

            // Generate or ensure unique ID
            if vps.id.is_none() {
                let base_id = generate_id(vps.name.as_deref(), &vps.host, vps.user.as_deref());
                let mut candidate = base_id.clone();
                let mut count = 1;
                while !seen_ids.insert(candidate.clone()) {
                    count += 1;
                    candidate = format!("{}-{}", base_id, count);
                }
                vps.id = Some(candidate);
                changed = true;
            } else {
                let mut candidate = original_id.clone().unwrap();
                let mut count = 1;
                while !seen_ids.insert(candidate.clone()) {
                    count += 1;
                    candidate = format!("{}-{}", original_id.clone().unwrap(), count);
                }
                if candidate != original_id.unwrap() {
                    vps.id = Some(candidate);
                    changed = true;
                }
            }

            // Default port
            if vps.port.as_deref().unwrap_or("").trim().is_empty() {
                vps.port = Some("22".into());
                changed = true;
            }

            // Default user
            if vps.user.is_none() {
                vps.user = Some("user".into());
                changed = true;
            }
        }

        changed
    }

    pub fn load_from_file(path: &PathBuf) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let list = serde_json::from_str(&content)?;
        Ok(list)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, json)?;
        Ok(())
    }
}

pub fn generate_id(name: Option<&str>, host: &str, user: Option<&str>) -> String {
    let raw = name
        .or_else(|| Some(host))
        .unwrap_or("vps")
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-");

    let user = user.unwrap_or("user").to_lowercase();
    format!("{}-{}", raw.trim_matches('-'), user)
}



pub fn schema_example() -> VPSList {
    let vps = VPSConfig {
        id: Some("example-vps-user".into()),
        name: Some("Example VPS".into()),
        user: Some("ubuntu".into()),
        host: "example.com".into(),
        port: Some("22".into()),
        private_key: Some("/home/user/.ssh/id_rsa".into()),
        post_connect_script: Some("uptime && echo $USER".into()),
    };

    VPSList {
        vps: vec![vps],
    }
}


pub fn vps_config_schema() {
    let example = schema_example();
    println!("{}", serde_json::to_string_pretty(&example).unwrap());
}

pub fn load_vps_entries(path: &PathBuf) -> io::Result<Vec<VPSConfig>> {
    let content = fs::read_to_string(path)?;
    let entries = serde_json::from_str(&content)?;
    Ok(entries)
}

pub fn get_vps_entries() -> io::Result<Vec<VPSConfig>> {
    let config = TranquilityConfig::load_or_init()?;
    load_vps_entries(&config.vps_file)
}