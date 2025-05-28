// Module: Schema/VPS
// Location:cli/src/schema/vps.rs
use crate::model::vps::VPSList;
use crate::schema::SUPPORTED_EXTS;
use jsonschema::validator_for;
use schemars::schema_for;
use serde_json::Value;
use std::{fs, path::Path};
use tranquility::{log_info, log_warn};

pub fn validate_file(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        log_warn!("validate", "vps", &format!("❌ Unsupported extension: .{ext}"));
        return false;
    }

    let raw = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            log_warn!("validate", "vps", &format!("❌ Failed to read file: {e}"));
            return false;
        }
    };

    let json_value: Value = match ext.as_str() {
        "json" => match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                log_warn!("validate", "vps", &format!("❌ Invalid JSON: {e}"));
                return false;
            }
        },
        "yaml" | "yml" => {
            let yaml = match serde_yaml::from_str::<serde_yaml::Value>(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "vps", &format!("❌ Invalid YAML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(yaml) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "vps", &format!("❌ Failed to convert YAML to JSON: {e}"));
                    return false;
                }
            }
        }
        "xml" => {
            let parsed: VPSList = match quick_xml::de::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "vps", &format!("❌ Invalid XML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(parsed) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "vps", &format!("❌ Failed to convert XML to JSON: {e}"));
                    return false;
                }
            }
        }
        _ => unreachable!(),
    };

    let schema = schema_for!(VPSList);
    let schema_json = match serde_json::to_value(&schema.schema) {
        Ok(s) => s,
        Err(e) => {
            log_warn!("validate", "schema", &format!("❌ Failed to convert schema: {e}"));
            return false;
        }
    };

    let validator = match validator_for(&schema_json) {
        Ok(v) => v,
        Err(e) => {
            log_warn!("validate", "schema", &format!("❌ Failed to compile schema: {e}"));
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
            log_warn!("validate", "custom", &format!("❌ {err}"));
        }
        return false;
    }

    log_info!("validate", "vps", "✅ VPS config is valid.");
    true
}

fn validate_custom(json: &Value) -> Result<(), String> {
    let mut errors = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    if let Some(vps_list) = json.get("vps").and_then(|v| v.as_array()) {
        for (i, vps) in vps_list.iter().enumerate() {
            // name: required, non-empty
            if vps.get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
            {
                errors.push(format!("VPS[{}]: 'name' is required and must not be empty.", i));
            }

            // host: required, non-empty
            if vps.get("host")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
            {
                errors.push(format!("VPS[{}]: 'host' is required and must not be empty.", i));
            }

            // id: optional, but must be unique if present
            if let Some(id_val) = vps.get("id").and_then(|v| v.as_str()) {
                if !seen_ids.insert(id_val.to_string()) {
                    errors.push(format!("VPS[{}]: Duplicate id '{}'", i, id_val));
                }
            }

            // port: optional, but if provided must be valid u16
            if let Some(port_val) = vps.get("port") {
                let is_valid = match port_val {
                    Value::Number(num) => num.as_u64().map_or(false, |n| n <= 65535),
                    Value::String(s) => s.parse::<u16>().is_ok(),
                    _ => false,
                };

                if !is_valid {
                    errors.push(format!(
                        "VPS[{}]: 'port' must be a number between 0 and 65535 if provided.",
                        i
                    ));
                }
            }

            // user: optional — no validation needed; app defaults to script user
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
