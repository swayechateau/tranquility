use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Configuration for all VPS entries.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VpsConfig {
    pub vps: Vec<VpsEntry>,
}

impl VpsConfig {
    /// Check if the configuration is valid.
    pub fn validate(&self) -> Result<(), String> {
        for vps in &self.vps {
            vps.validate()?;
        }
        Ok(())
    }

    /// Fix missing IDs and expand home paths.
    pub fn fix(&mut self) -> bool {
        let mut changed = false;
        let mut seen_ids = HashSet::new();

        for vps in &mut self.vps {
            changed |= vps.fix(&mut seen_ids);
        }

        changed
    }

    /// Add a VPS entry to the config.
    pub fn push(&mut self, vps: VpsEntry) {
        self.vps.push(vps);
    }
}

/// A single VPS entry.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

impl VpsEntry {
    /// Get the effective username (defaults to system user or "user").
    pub fn effective_user(&self) -> String {
        self.user
            .as_deref()
            .map(str::to_string)
            .or_else(get_current_user)
            .unwrap_or_else(|| "user".to_string())
    }

    /// Get the effective port (defaults to 22).
    pub fn effective_port(&self) -> u16 {
        self.port.as_ref().and_then(|p| p.to_u16()).unwrap_or(22)
    }

    /// Get the effective ID (or `<generated>` if missing).
    pub fn effective_id(&self) -> &str {
        self.id.as_deref().unwrap_or("<generated>")
    }

    /// Validate the VPS entry.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(id) = &self.id {
            validate_id_format(id)?;
        }
        if self.name.as_ref().is_none_or(|name| name.trim().is_empty()) {
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

    /// Fix missing ID and expand home paths.
    pub fn fix(&mut self, seen_ids: &mut HashSet<String>) -> bool {
        let mut changed = false;

        if self.id.is_none() {
            let generated = generate_vps_id(self.name.as_deref(), &self.host, self.user.as_deref());
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

        changed
    }
}

/// A flexible value that can be a number or string.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

/// Generate an ID for a VPS based on name, host, and user.
pub fn generate_vps_id(name: Option<&str>, host: &str, _user: Option<&str>) -> String {
    let base = slugify_id(name.filter(|n| !n.trim().is_empty()).unwrap_or(host));
    if base.is_empty() {
        "vps".to_string()
    } else {
        base
    }
}

/// Reduce text to a one-word id: lowercase letters and digits only
/// (`"Temp Server"` becomes `"tempserver"`).
pub fn slugify_id(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Make `base` unique among `taken` by appending a number (`tempserver2`, ...).
pub fn unique_id<'a>(base: &str, taken: impl Iterator<Item = &'a str> + Clone) -> String {
    let exists = |candidate: &str| taken.clone().any(|t| t == candidate);
    if !exists(base) {
        return base.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{base}{n}");
        if !exists(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

/// An explicit id must be a single word: letters, digits, `-` or `_`.
pub fn validate_id_format(id: &str) -> Result<(), String> {
    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(format!(
            "invalid id `{id}`: use a single word of letters, digits, '-' or '_'"
        ));
    }
    Ok(())
}

/// Get the current system user.
#[cfg(target_family = "unix")]
pub fn get_current_user() -> Option<String> {
    std::env::var("USER").ok()
}

#[cfg(target_family = "windows")]
pub fn get_current_user() -> Option<String> {
    std::env::var("USERNAME").ok()
}
