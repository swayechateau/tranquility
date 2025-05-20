use std::fs;
use std::io::{self};
use std::path::{PathBuf};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranquilityConfig {
    pub applications_file: PathBuf,
}

impl TranquilityConfig {
    pub fn config_dir() -> io::Result<PathBuf> {
        config_dir()
            .map(|p| p.join("tranquility"))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not locate config directory"))
    }

    pub fn default() -> Self {
        let apps_file = Self::config_dir().unwrap().join("applications.json");
        TranquilityConfig {
            applications_file: apps_file,
        }
    }

    pub fn load_or_init() -> io::Result<Self> {
        let path = Self::config_dir()?.join("config.json");

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Self::validate_schema(&content)?;
            let cfg: Self = serde_json::from_str(&content)?;
            Ok(cfg)
        } else {
            let default = Self::default();
            let json = serde_json::to_string_pretty(&default)?;
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, json)?;
            Ok(default)
        }
    }

    pub fn reset() -> io::Result<()> {
        let path = Self::config_dir()?.join("config.json");
        let default = Self::default();
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
                return Err(io::Error::new(io::ErrorKind::InvalidData, "applications_file path is missing"));
            }
            Ok(())
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid config format: {e}"))),
    }
}
}
