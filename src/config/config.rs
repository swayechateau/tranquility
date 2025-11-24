// Module: Model/Config
// Location: cli/src/model/config.rs

use crate::SUPPORTED_EXTS;
use dirs::config_dir;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf}
};

use once_cell::sync::OnceCell;
pub static CONFIG: OnceCell<TranquilityConfig> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct TranquilityConfig {
    pub applications_file: PathBuf,
    pub vps_file: PathBuf,
    pub log_directory: PathBuf,
    pub log_output: LogOutput,
}

impl TranquilityConfig {
    fn config_file_base_name() -> &'static str {
        "config"
    }

    fn file_with_ext(base: &Path, name: &str, ext: &str) -> PathBuf {
        base.join(format!("{name}.{ext}"))
    }

    pub fn config_path() -> io::Result<PathBuf> {
        let base = Self::config_dir()?;
        Ok(
            resolve_config_file_with_extensions(&base, Self::config_file_base_name())
                .unwrap_or_else(|| {
                    Self::file_with_ext(&base, Self::config_file_base_name(), "yaml")
                }),
        )
    }

    pub fn config_dir() -> io::Result<PathBuf> {
        config_dir().map(|p| p.join("tranquility")).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not locate config directory")
        })
    }

    fn default_file_path(base_dir: &Path, name: &str) -> PathBuf {
        resolve_config_file_with_extensions(base_dir, name)
            .unwrap_or_else(|| base_dir.join(format!("{name}.yaml")))
    }

    pub fn log_file(&self) -> PathBuf {
        self.log_directory.join(format!(
            "{}-tranquility.log",
            chrono::Local::now().format("%Y-%m-%d")
        ))
    }

    pub fn default() -> io::Result<Self> {
        let base_dir = Self::config_dir()?;
        let log_directory = base_dir.join("logs");
        fs::create_dir_all(&log_directory)?;

        Ok(Self {
            applications_file: Self::default_file_path(&base_dir, "applications"),
            vps_file: Self::default_file_path(&base_dir, "vps"),
            log_directory,
            log_output: LogOutput::Primary,
        })
    }

    pub fn load_once() -> &'static TranquilityConfig {
        CONFIG.get_or_init(|| match Self::load_internal() {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("⚠️ Failed to load config: {err}");
                std::process::exit(1);
            }
        })
    }

    fn load_internal() -> io::Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Self::validate_schema(&content, &path)?;
            let mut cfg = deserialize_config(&path, &content)?;

            if cfg.log_directory.as_os_str().is_empty() {
                cfg.log_directory = Self::config_dir()?.join("logs");
                write_config(&path, &cfg)?;
            }

            Ok(cfg)
        } else {
            let default = Self::default()?;
            write_config(&path, &default)?;
            Ok(default)
        }
    }

    pub fn reset() -> io::Result<()> {
        let path = Self::config_path()?;
        let default = Self::default()?;
        write_config(&path, &default)
    }

    pub fn validate_schema(_content: &str, path: &PathBuf) -> io::Result<()> {
        if !crate::config::schema::validate_file(path) {
            eprintln!("⚠️ Schema did not match for {}", path.display());
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
                    let base = Self::config_dir()?;

                    if cfg.log_directory.as_os_str().is_empty() {
                        cfg.log_directory = base.join("logs");
                        changed = true;
                    }

                    if cfg.applications_file.as_os_str().is_empty() {
                        cfg.applications_file = Self::default_file_path(&base, "applications");
                        changed = true;
                    }

                    if cfg.vps_file.as_os_str().is_empty() {
                        cfg.vps_file = Self::default_file_path(&base, "vps");
                        changed = true;
                    }

                    if changed {
                        write_config(&path, &cfg)?;
                        eprintln!("⚠️  Config was missing fields and has been patched.");
                    } else {
                        eprintln!("✅ Config file is complete.");
                    }
                }
                Err(e) => {
                    eprintln!("⚠️  Config is invalid. Recreating default. Error: {e}");
                    let default = Self::default()?;
                    write_config(&path, &default)?;
                    eprintln!("✅ Default config recreated.");
                }
            }
        } else {
            eprintln!("⚠️  Config not found. Creating default.");
            let default = Self::default()?;
            write_config(&path, &default)?;
            eprintln!("✅ Default config created.");
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

fn resolve_config_file_with_extensions(base: &Path, name: &str) -> Option<PathBuf> {
    for ext in &SUPPORTED_EXTS {
        let candidate = base.join(format!("{name}.{ext}"));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}
