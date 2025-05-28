// src/model/config.rs
use dirs::config_dir;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use tranquility::{log_info, log_warn};

use crate::schema::SUPPORTED_EXTS;

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
    pub logs_directory: PathBuf,
    pub log_output: LogOutput,
}

impl TranquilityConfig {
    pub fn config_path() -> io::Result<PathBuf> {
        let base = Self::config_dir()?;
        for ext in &SUPPORTED_EXTS {
            let candidate = base.join(format!("config.{ext}"));
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        Ok(base.join("config.yaml"))
    }

    pub fn config_dir() -> io::Result<PathBuf> {
        config_dir().map(|p| p.join("tranquility")).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not locate config directory")
        })
    }

    pub fn log_file(&self) -> PathBuf {
        self.logs_directory.join(format!(
            "{}-tranquility.log",
            chrono::Local::now().format("%Y-%m-%d")
        ))
    }

    pub fn default() -> io::Result<Self> {
        let base_dir = Self::config_dir()?;
        let logs_directory = base_dir.join("logs");
        fs::create_dir_all(&logs_directory)?;

        Ok(TranquilityConfig {
            applications_file: base_dir.join("applications.json"),
            vps_file: base_dir.join("vps.json"),
            logs_directory,
            log_output: LogOutput::Primary,
        })
    }

    pub fn load_or_init() -> io::Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            log_info!(
                "load",
                "config",
                &format!("✅ Config file exists at {:?}", path)
            );
            let content = fs::read_to_string(&path)?;
            Self::validate_schema(&content, &path)?;

            let mut cfg = deserialize_config(&path, &content)?;

            if cfg.logs_directory.as_os_str().is_empty() {
                cfg.logs_directory = Self::config_dir()?.join("logs");
                write_config(&path, &cfg)?;
                log_warn!(
                    "patch",
                    "config",
                    "⚠️  Patched missing logs_directory in config."
                );
            }

            Ok(cfg)
        } else {
            log_warn!(
                "load",
                "config",
                "⚠️  Config file not found. Creating default config."
            );
            let default = Self::default()?;
            let path = Self::config_path()?;
            write_config(&path, &default)?;
            log_info!("create", "config", "✅ Default config created.");
            Ok(default)
        }
    }

    pub fn reset() -> io::Result<()> {
        let path = Self::config_path()?;
        let default = Self::default()?;
        write_config(&path, &default)
    }

    pub fn validate_schema(_content: &str, path: &PathBuf) -> io::Result<()> {
        if !crate::schema::config::validate_file(path) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("⚠️ Config validation failed: {}", path.display()),
            ));
        }
        Ok(())
    }

    pub fn fix() -> io::Result<()> {
        let path = Self::config_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)?;

            match deserialize_config(&path, &content) {
                Ok(mut cfg) => {
                    let mut changed = false;

                    if cfg.logs_directory.as_os_str().is_empty() {
                        cfg.logs_directory = Self::config_dir()?.join("logs");
                        changed = true;
                    }

                    if cfg.applications_file.as_os_str().is_empty() {
                        cfg.applications_file = Self::config_dir()?.join("applications.yaml");
                        changed = true;
                    }

                    if cfg.vps_file.as_os_str().is_empty() {
                        cfg.vps_file = Self::config_dir()?.join("vps.yaml");
                        changed = true;
                    }

                    if changed {
                        write_config(&path, &cfg)?;
                        log_warn!(
                            "patch",
                            "config",
                            "⚠️  Config was missing fields and has been patched."
                        );
                    } else {
                        log_info!("check", "config", "✅ Config file is complete.");
                    }
                }
                Err(e) => {
                    log_warn!(
                        "invalid",
                        "config",
                        &format!("⚠️  Config is invalid. Recreating default. Error: {e}")
                    );
                    let default = Self::default()?;
                    write_config(&path, &default)?;
                    log_info!("create", "config", "✅ Default config recreated.");
                }
            }
        } else {
            log_warn!(
                "missing",
                "config",
                "⚠️  Config not found. Creating default."
            );
            let default = Self::default()?;
            write_config(&path, &default)?;
            log_info!("create", "config", "✅ Default config created.");
        }

        Ok(())
    }
}

pub fn deserialize_config(path: &PathBuf, content: &str) -> io::Result<TranquilityConfig> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    match ext.as_str() {
        "json" => serde_json::from_str(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("JSON error: {e}"))),
        "yaml" | "yml" => serde_yaml::from_str(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("YAML error: {e}"))),
        "xml" => quick_xml::de::from_str(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("XML error: {e}"))),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unsupported config format",
        )),
    }
}

fn write_config(path: &PathBuf, config: &TranquilityConfig) -> io::Result<()> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    let content = match ext.as_str() {
        "json" => serde_json::to_string_pretty(config).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("JSON write error: {e}"))
        })?,
        "yaml" | "yml" => serde_yaml::to_string(config).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("YAML write error: {e}"))
        })?,
        "xml" => quick_xml::se::to_string(config).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("XML write error: {e}"))
        })?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported config format",
            ))
        }
    };

    fs::write(path, content)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to write config: {e}")))
}
