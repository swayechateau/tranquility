// src/model/config.rs
use dirs::config_dir;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

use crate::logger::log_event;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TranquilityConfig {
    pub applications_file: PathBuf,
    pub vps_file: PathBuf,
    pub log_file: PathBuf,
}

impl TranquilityConfig {
    pub fn config_dir() -> io::Result<PathBuf> {
        config_dir().map(|p| p.join("tranquility")).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not locate config directory")
        })
    }

    pub fn default() -> io::Result<Self> {
        let base_dir = Self::config_dir()?;
        let logs_dir = base_dir.join("logs");
        fs::create_dir_all(&logs_dir)?;

        let log_file = logs_dir.join(format!(
            "tranquility-{}.log",
            chrono::Local::now().format("%Y-%m-%d")
        ));

        Ok(TranquilityConfig {
            applications_file: base_dir.join("applications.json"),
            vps_file: base_dir.join("vps.json"),
            log_file: log_file,
        })
    }

    pub fn load_or_init() -> io::Result<Self> {
        let path = Self::config_dir()?.join("config.json");

        if path.exists() {
            log_event("info", "load", "config", "✅ Config file exists.", None);
            let content = fs::read_to_string(&path)?;
            Self::validate_schema(&content)?;

            let mut cfg: TranquilityConfig = serde_json::from_str(&content)?;

            // Patch: If log_file is missing, assign a default path
            if cfg.log_file.as_os_str().is_empty() {
                let base_dir = Self::config_dir()?;
                let logs_dir = base_dir.join("logs");
                fs::create_dir_all(&logs_dir)?;
                cfg.log_file = logs_dir.join(format!(
                    "tranquility-{}.log",
                    chrono::Local::now().format("%Y-%m-%d")
                ));

                // Save patched config back to file
                let patched_json = serde_json::to_string_pretty(&cfg)?;
                fs::write(&path, patched_json)?;
                log_event("warn", "patch", "config", "⚠️  Patched missing log_file in config.", None);
                
            }

            Ok(cfg)
        } else {
            log_event("warn", "load", "config", "⚠️  Config file not found. Creating default config.", None);
            let default = Self::default()?;
            let json = serde_json::to_string_pretty(&default)?;
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, json)?;
            log_event("info", "create", "config", "✅ Default config created.", None);
            Ok(default)
        }
    }

    pub fn reset() -> io::Result<()> {
        let path = Self::config_dir()?.join("config.json");
        let default = Self::default()?;
        let json = serde_json::to_string_pretty(&default)?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn validate_schema(content: &str) -> io::Result<()> {
        let parsed: Result<TranquilityConfig, _> = serde_json::from_str(content);

        match parsed {
            Ok(cfg) => {
                // Perform additional validation checks manually
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
