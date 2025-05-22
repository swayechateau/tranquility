// src/schema/config.rs
use crate::model::config::TranquilityConfig;
use jsonschema::validator_for;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn validate_file(path: &str) -> Result<(), String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let raw = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {e}"))?;

    let json_value: Value = match ext.as_str() {
        "json" => serde_json::from_str(&raw).map_err(|e| format!("Invalid JSON: {e}"))?,
        "yaml" | "yml" => {
            let yaml: serde_yaml::Value =
                serde_yaml::from_str(&raw).map_err(|e| format!("Invalid YAML: {e}"))?;
            serde_json::to_value(yaml).unwrap()
        }
        "xml" => {
            let parsed: TranquilityConfig =
                quick_xml::de::from_str(&raw).map_err(|e| format!("Invalid XML: {e}"))?;
            serde_json::to_value(parsed).unwrap()
        }
        _ => return Err(format!("Unsupported file extension: .{}", ext)),
    };

    let schema = schema_for!(TranquilityConfig);
    let schema_value = serde_json::to_value(&schema.schema).unwrap();
    let validator = validator_for(&schema_value).map_err(|e| format!("Schema error: {e}"))?;

    validator.validate(&json_value).map_err(|_| {
        validator
            .iter_errors(&json_value)
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    validate_custom(&json_value)?;

    Ok(())
}

fn validate_custom(json: &Value) -> Result<(), String> {
    let mut errors = Vec::new();

    for field in ["applications_file", "vps_file", "log_file"] {
        if let Some(path_val) = json.get(field) {
            if let Some(path_str) = path_val.as_str() {
                if path_str.trim().is_empty() {
                    errors.push(format!("Field '{}' must not be empty.", field));
                } else if !Path::new(path_str).is_absolute() {
                    errors.push(format!("Field '{}' must be an absolute path.", field));
                }
            } else {
                errors.push(format!("Field '{}' must be a string.", field));
            }
        } else {
            errors.push(format!("Field '{}' is missing.", field));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
