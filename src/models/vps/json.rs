// Module: Model/VPS
// Location: cli/src/model/vps/json.rs

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use quick_xml::de::from_str as from_xml;
use quick_xml::se::to_string as to_xml;
use serde_yaml;
use std::{fmt, fs, io, path::PathBuf};

use crate::{log_info, core::expand_home, models::vps::generate_id};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VpsConfig {
    pub vps: Vec<VpsEntry>,
}

impl From<super::xml::VpsConfigXml> for VpsConfig {
    fn from(xml_config: super::xml::VpsConfigXml) -> Self {
        VpsConfig {
            vps: xml_config.vps.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<VpsConfig> for super::xml::VpsConfigXml {
    fn from(config: VpsConfig) -> Self {
        super::xml::VpsConfigXml {
            vps: config
                .vps
                .into_iter()
                .map(|entry| super::xml::VpsEntryXml {
                    id: entry.id,
                    name: entry.name,
                    host: entry.host,
                    user: entry.user,
                    port: entry.port.map(|p| String::from(p)), // convert FlexibleValue to String
                    private_key: entry.private_key,
                    post_connect_script: entry.post_connect_script,
                })
                .collect(),
        }
    }
}

impl From<&VpsConfig> for super::xml::VpsConfigXml {
    fn from(config: &VpsConfig) -> Self {
        super::xml::VpsConfigXml {
            vps: config
                .vps
                .iter()
                .map(|entry| super::xml::VpsEntryXml {
                    id: entry.id.clone(),
                    name: entry.name.clone(),
                    host: entry.host.clone(),
                    user: entry.user.clone(),
                    port: entry.port.as_ref().map(|p| String::from(p)),
                    private_key: entry.private_key.clone(),
                    post_connect_script: entry.post_connect_script.clone(),
                })
                .collect(),
        }
    }
}

impl VpsConfig {
    pub fn fix(&mut self) -> bool {
        let mut changed = false;
        let mut seen_ids = std::collections::HashSet::new();

        for vps in &mut self.vps {
            changed |= vps.fix(&mut seen_ids);
        }

        changed
    }

    pub fn validate(&self) -> Result<(), String> {
        for vps in &self.vps {
            vps.validate()?;
        }
        Ok(())
    }

    pub fn push(&mut self, vps: VpsEntry) {
        self.vps.push(vps);
    }

    pub fn load_from_file(path: &PathBuf) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mut config = match ext.as_str() {
            "xml" => {
                let xml: super::xml::VpsConfigXml = from_xml(&content)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                xml.into()
            }
            "yaml" | "yml" => serde_yaml::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            _ => {
                // Try parsing as full config
                match serde_json::from_str::<Self>(&content) {
                    Ok(config) => config,
                    Err(_) => {
                        // Try fallback: array of entries
                        let entries: Vec<VpsEntry> = serde_json::from_str(&content)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                        VpsConfig { vps: entries }
                    }
                }
            }
        };

        // Fix any missing fields/IDs and persist back
        config.fix_and_save(path)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> io::Result<()> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let content = match ext.as_str() {
            "xml" => {
                let xml_config: super::xml::VpsConfigXml = self.into();
                to_xml(&xml_config).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            }
            "yaml" | "yml" => serde_yaml::to_string(self)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            _ => serde_json::to_string_pretty(self)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        };

        fs::create_dir_all(path.parent().unwrap_or_else(|| path.as_path()))?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn fix_and_save(&mut self, path: &PathBuf) -> io::Result<()> {
        let changed = self.fix();
        if changed {
            self.save_to_file(path)?;
            log_info!("reset", "vps-config-file", "vps config file updated");
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VpsEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_connect_script: Option<String>,
}

// Conversion from XML to the flexible model
impl From<super::xml::VpsEntryXml> for VpsEntry {
    fn from(xml: super::xml::VpsEntryXml) -> Self {
        VpsEntry {
            id: xml.id,
            name: xml.name,
            host: xml.host,
            user: xml.user,
            port: xml.port.map(FlexibleValue::String),
            private_key: xml.private_key,
            post_connect_script: xml.post_connect_script,
        }
    }
}

impl VpsEntry {
    /// Returns the username, defaulting to the system user if missing
    pub fn effective_user(&self) -> String {
        self.user
            .as_deref()
            .map(str::to_string)
            .or_else(super::get_username)
            .unwrap_or_else(|| "user".to_string())
    }
    /// Returns the port, defaulting to 22 if missing
    pub fn effective_port(&self) -> u16 {
        self.port
            .as_ref()
            .and_then(|p| p.to_u16())
            .unwrap_or(22)
    }
    /// Returns the ID, defaulting to "<generated>" if missing
    pub fn effective_id(&self) -> &str {
        self.id.as_deref().unwrap_or("<generated>")
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self
            .name
            .as_ref()
            .map_or(true, |name| name.trim().is_empty())
        {
            return Err("Missing VPS name".into());
        }

        if self.host.trim().is_empty() {
            return Err(format!(
                "VPS `{}` has no host specified",
                self.name.as_deref().unwrap_or("<unknown>")
            ));
        }

        if let Some(ref port) = self.port {
            if let Some(p) = port.to_u16() {
                if !(1..=65535).contains(&p) {
                    return Err(format!(
                        "VPS `{}` has invalid port: {}",
                        self.name.as_deref().unwrap_or("<unknown>"),
                        p
                    ));
                }
            } else {
                return Err(format!(
                    "VPS `{}` has non-numeric port: {:?}",
                    self.name.as_deref().unwrap_or("<unknown>"),
                    port
                ));
            }
        }

        Ok(())
    }

    /// Expands `~` and `$HOME` in relevant fields
    pub fn expand_paths(&mut self) {
        if let Some(pk) = &self.private_key {
            self.private_key = Some(expand_home(pk));
        }
        if let Some(script) = &self.post_connect_script {
            self.post_connect_script = Some(expand_home(script));
        }
    }

    pub fn fix(&mut self, seen_ids: &mut std::collections::HashSet<String>) -> bool {
        let mut changed = false;

        if self.id.is_none() {
            let generated = generate_id(self.name.as_deref(), &self.host, self.user.as_deref());
            if seen_ids.insert(generated.clone()) {
                self.id = Some(generated);
                changed = true;
            }
        } else {
            let id = self.id.as_ref().unwrap();
            if !seen_ids.insert(id.clone()) {
                let mut count = 1;
                let mut new_id = format!("{}-{}", id, count);
                while !seen_ids.insert(new_id.clone()) {
                    count += 1;
                    new_id = format!("{}-{}", id, count);
                }
                self.id = Some(new_id);
                changed = true;
            }
        }

        self.expand_paths();

        changed
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum FlexibleValue {
    Number(u16),
    String(String),
}

impl FlexibleValue {
    pub fn to_u16(&self) -> Option<u16> {
        match self {
            FlexibleValue::Number(n) => Some(*n),
            FlexibleValue::String(s) => s.parse().ok(),
        }
    }
}

impl fmt::Display for FlexibleValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexibleValue::Number(n) => write!(f, "{}", n),
            FlexibleValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<FlexibleValue> for String {
    fn from(p: FlexibleValue) -> Self {
        match p {
            FlexibleValue::Number(n) => n.to_string(),
            FlexibleValue::String(s) => s,
        }
    }
}

impl From<&FlexibleValue> for String {
    fn from(p: &FlexibleValue) -> Self {
        match p {
            FlexibleValue::Number(n) => n.to_string(),
            FlexibleValue::String(s) => s.clone(),
        }
    }
}

impl From<&str> for FlexibleValue {
    fn from(s: &str) -> Self {
        FlexibleValue::String(s.to_string())
    }
}

impl From<String> for FlexibleValue {
    fn from(s: String) -> Self {
        FlexibleValue::String(s)
    }
}

impl From<u16> for FlexibleValue {
    fn from(n: u16) -> Self {
        FlexibleValue::Number(n)
    }
}
