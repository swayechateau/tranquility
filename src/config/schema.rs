// Module: Schema/Config
// Location: cli/src/schema/config.rs
use crate::config::TranquilityConfig;
use crate::{log_info, log_warn};
use jsonschema::validator_for;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::SUPPORTED_EXTS;

pub fn validate_file(path: &PathBuf) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let raw = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            log_warn!(
                "validate",
                "config",
                &format!("❌ Failed to read file: {e}")
            );
            return false;
        }
    };

    let json_value: Value = match ext.as_str() {
        "json" => match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                log_warn!("validate", "config", &format!("❌ Invalid JSON: {e}"));
                return false;
            }
        },
        "yaml" | "yml" => {
            let yaml: serde_yaml::Value = match serde_yaml::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "config", &format!("❌ Invalid YAML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(yaml) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!(
                        "validate",
                        "config",
                        &format!("❌ Failed to convert YAML to JSON: {e}")
                    );
                    return false;
                }
            }
        }
        "xml" => {
            let parsed: TranquilityConfig = match quick_xml::de::from_str(&raw) {
                Ok(cfg) => cfg,
                Err(e) => {
                    log_warn!("validate", "config", &format!("❌ Invalid XML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(parsed) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!(
                        "validate",
                        "config",
                        &format!("❌ Failed to convert XML to JSON: {e}")
                    );
                    return false;
                }
            }
        }
        _ => {
            log_warn!(
                "validate",
                "config",
                &format!("❌ Unsupported file extension: .{ext}")
            );
            return false;
        }
    };

    let schema = schema_for!(TranquilityConfig);
    let schema_value = match serde_json::to_value(&schema) {
        Ok(s) => s,
        Err(e) => {
            log_warn!(
                "validate",
                "schema",
                &format!("❌ Failed to serialize full schema: {e}")
            );
            return false;
        }
    };

    let validator = match validator_for(&schema_value) {
        Ok(v) => v,
        Err(e) => {
            log_warn!("validate", "schema", &format!("❌ Invalid schema: {e}"));
            return false;
        }
    };

    if let Err(_) = validator.validate(&json_value) {
        for err in validator.iter_errors(&json_value) {
            log_warn!("validate", "schema", &format!("Schema violation: {err}"));
        }
        return false;
    }

    if let Err(custom_errors) = validate_custom(&json_value) {
        for err in custom_errors.lines() {
            log_warn!("validate", "custom", &format!("Custom check failed: {err}"));
        }
        return false;
    }

    log_info!("validate", "config", "✅ Config file is valid.");
    true
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
                // Field is optional — warn that default will be used
                log_warn!(
                    "validate",
                    "config",
                    &format!(
                        "ℹ️  Optional field '{}' is missing. Using default value.",
                        field
                    )
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
