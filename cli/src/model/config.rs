// src/model/config.rs
use dirs::config_dir;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

use crate::{log_info, log_warn};
use crate::logger::{default_log_path};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Primary,
    Stdout,
}

impl Default for LogOutput {
    fn default() -> Self {
        LogOutput::Primary
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TranquilityConfig {
    pub applications_file: PathBuf,
    pub vps_file: PathBuf,
    pub log_file: PathBuf,
    pub log_output: LogOutput,
}

impl TranquilityConfig {
    /// Gets the user's config directory, e.g. ~/.config/tranquility
    pub fn config_dir() -> io::Result<PathBuf> {
        config_dir().map(|p| p.join("tranquility")).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not locate config directory")
        })
    }

    /// Constructs a default configuration, always placing logs in ~/.config/tranquility/logs
    pub fn default() -> io::Result<Self> {
        let base_dir = Self::config_dir()?;
        let logs_dir = base_dir.join("logs");
        fs::create_dir_all(&logs_dir)?;

        let log_file = logs_dir.join(format!(
            "{}-tranquility.log",
            chrono::Local::now().format("%Y-%m-%d")
        ));

        Ok(TranquilityConfig {
            applications_file: base_dir.join("applications.json"),
            vps_file: base_dir.join("vps.json"),
            log_file,
            log_output: LogOutput::Primary,
        })
    }

    /// Loads config if it exists, otherwise creates a default one
    pub fn load_or_init() -> io::Result<Self> {
        let path = Self::config_dir()?.join("config.json");

        if path.exists() {
            log_info!("load", "config", "✅ Config file exists.");
            let content = fs::read_to_string(&path)?;
            Self::validate_schema(&content)?;

            let mut cfg: TranquilityConfig = serde_json::from_str(&content)?;

            if cfg.log_file.as_os_str().is_empty() {
                cfg.log_file = default_log_path();

                let patched_json = serde_json::to_string_pretty(&cfg)?;
                fs::write(&path, patched_json)?;
                log_warn!("patch", "config", "⚠️  Patched missing log_file in config.");
            }

            Ok(cfg)
        } else {
            log_warn!(
                "load",
                "config",
                "⚠️  Config file not found. Creating default config."
            );
            let default = Self::default()?;
            let json = serde_json::to_string_pretty(&default)?;
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, json)?;
            log_info!("create", "config", "✅ Default config created.");
            Ok(default)
        }
    }

    /// Force resets the config to default
    pub fn reset() -> io::Result<()> {
        let path = Self::config_dir()?.join("config.json");
        let default = Self::default()?;
        let json = serde_json::to_string_pretty(&default)?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, json)?;
        Ok(())
    }

    /// Validates config schema and required fields
    pub fn validate_schema(content: &str) -> io::Result<()> {
        let parsed: Result<TranquilityConfig, _> = serde_json::from_str(content);

        match parsed {
            Ok(cfg) => {
                if cfg.applications_file.as_os_str().is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "applications_file path is missing",
                    ));
                }
                if cfg.vps_file.as_os_str().is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vps_file path is missing",
                    ));
                }
                Ok(())
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid config format: {e}"),
            )),
        }
    }
}
