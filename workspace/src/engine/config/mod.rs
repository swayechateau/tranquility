use dirs::config_dir;
use jsonschema::validator_for;
use once_cell::sync::OnceCell;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
};

use crate::engine::constants::SUPPORTED_EXTS;

pub static CONFIG: OnceCell<BaseConfig> = OnceCell::new();

#[derive(Default, Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    #[default]
    Primary,
    Stdout,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct BaseConfig {
    pub applications_file: PathBuf,
    pub vps_file: PathBuf,
    pub log_directory: PathBuf,
    pub log_output: LogOutput,
}

impl BaseConfig {
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

    pub fn load_default() -> io::Result<Self> {
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

    pub fn load_once() -> &'static BaseConfig {
        CONFIG.get_or_init(|| match Self::load_internal() {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!(
                    "⚠️ Failed to load config: {err}. Using in-memory defaults for this run — run `tquil doctor fix` to repair the config file."
                );
                Self::fallback_default()
            }
        })
    }

    /// Load and validate a config file at an explicit path (used for `--config`).
    /// Unlike `load_once`, this never auto-creates or patches the file — an
    /// explicitly-named config is expected to already exist and be valid.
    pub fn load_from(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::validate_schema(&content, path)?;
        let mut cfg = deserialize_config(path, &content)?;
        if cfg.log_directory.as_os_str().is_empty() {
            cfg.log_directory = Self::config_dir()?.join("logs");
        }
        Ok(cfg)
    }

    /// Built-in defaults used when even `load_default` (which needs a
    /// resolvable, writable config directory) fails.
    pub fn fallback_default() -> Self {
        Self::load_default().unwrap_or_else(|err| {
            eprintln!(
                "⚠️ Failed to prepare default config directories: {err}. Falling back to built-in defaults."
            );
            Self {
                applications_file: PathBuf::from("applications.yaml"),
                vps_file: PathBuf::from("vps.yaml"),
                log_directory: PathBuf::from("."),
                log_output: LogOutput::Primary,
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
            let default = Self::load_default()?;
            write_config(&path, &default)?;
            Ok(default)
        }
    }

    pub fn reset() -> io::Result<()> {
        let path = Self::config_path()?;
        let default = Self::load_default()?;
        write_config(&path, &default)
    }

    pub fn validate_schema(_content: &str, path: &std::path::Path) -> io::Result<()> {
        if !validate_file(path) {
            eprintln!("⚠️ Schema did not match for {}", path.display());
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("⚠️ Config validation failed: {}", path.display()),
            ));
        }
        Ok(())
    }

    pub fn fix() -> io::Result<()> {
        Self::fix_at(&Self::config_path()?)
    }

    /// Same as `fix`, but against an explicit path (e.g. from `--config`)
    /// instead of always resolving the default config location.
    pub fn fix_at(path: &Path) -> io::Result<()> {
        let path = path.to_path_buf();

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
                    let default = Self::load_default()?;
                    write_config(&path, &default)?;
                    eprintln!("✅ Default config recreated.");
                }
            }
        } else {
            eprintln!("⚠️  Config not found. Creating default.");
            let default = Self::load_default()?;
            write_config(&path, &default)?;
            eprintln!("✅ Default config created.");
        }

        Ok(())
    }
}

pub fn deserialize_config(path: &Path, content: &str) -> io::Result<BaseConfig> {
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

fn write_config(path: &PathBuf, config: &BaseConfig) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| io::Error::other(format!("Failed to create config directory: {e}")))?;
    }

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
            ));
        }
    };

    fs::write(path, content).map_err(|e| io::Error::other(format!("Failed to write config: {e}")))
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

/// Validate a config file, returning every problem found as a message.
pub fn check_file(path: &Path) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let raw = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("failed to read file: {e}"));
        }
    };

    let json_value: Value = match ext.as_str() {
        "json" => match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("invalid JSON: {e}"));
            }
        },
        "yaml" | "yml" => {
            let yaml: serde_yaml::Value = match serde_yaml::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("invalid YAML: {e}"));
                }
            };

            match serde_json::to_value(yaml) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("failed to convert YAML to JSON: {e}"));
                }
            }
        }
        "xml" => {
            let parsed: BaseConfig = match quick_xml::de::from_str(&raw) {
                Ok(cfg) => cfg,
                Err(e) => {
                    return Err(format!("invalid XML: {e}"));
                }
            };

            match serde_json::to_value(parsed) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("failed to convert XML to JSON: {e}"));
                }
            }
        }
        _ => {
            return Err(format!("unsupported file extension: .{ext}"));
        }
    };

    let schema = schema_for!(BaseConfig);
    let schema_value = match serde_json::to_value(&schema) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("failed to serialize full schema: {e}"));
        }
    };

    let validator = match validator_for(&schema_value) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("invalid schema: {e}"));
        }
    };

    if validator.validate(&json_value).is_err() {
        let msgs: Vec<String> = validator
            .iter_errors(&json_value)
            .map(|err| format!("schema violation: {err}"))
            .collect();
        return Err(msgs.join("\n"));
    }

    validate_custom(&json_value)
}

pub fn validate_file(path: &Path) -> bool {
    match check_file(path) {
        Ok(()) => true,
        Err(msg) => {
            tracing::warn!("validate config: {msg}");
            false
        }
    }
}

fn validate_custom(json: &Value) -> Result<(), String> {
    let mut errors = Vec::new();
    for field in ["applications_file", "vps_file", "log_directory"] {
        match json.get(field) {
            Some(path_val) => {
                // Must be a string
                let path_str = match path_val.as_str() {
                    Some(s) if s.trim().is_empty() => {
                        errors.push(format!("Field '{}' must not be empty if provided.", field));
                        continue;
                    }
                    Some(s) => s,
                    None => {
                        errors.push(format!("Field '{}' must be a string if provided.", field));
                        continue;
                    }
                };

                // Must be an absolute path
                if !Path::new(path_str).is_absolute() {
                    errors.push(format!("Field '{}' must be an absolute path.", field));
                }

                // Check extension for applications_file and vps_file
                if field != "log_directory" {
                    let ext = Path::new(path_str)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
                        errors.push(format!(
                            "Field '{}' must have a supported file extension (json, yaml, yml, xml). Found: .{}",
                            field, ext
                        ));
                    }
                }
            }
            None => {
                tracing::info!(
                    "validate config: optional field '{field}' is missing, using default"
                );
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
