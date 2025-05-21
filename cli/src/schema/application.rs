// src/schema/application.rs
use crate::models::ApplicationFile;
use jsonschema::{Draft, JSONSchema};
use schemars::schema_for;
use serde_json::Value;
use std::fs;

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
            let parsed: ApplicationFile =
                quick_xml::de::from_str(&raw).map_err(|e| format!("Invalid XML: {e}"))?;
            serde_json::to_value(parsed).unwrap()
        }
        _ => return Err(format!("Unsupported file extension: .{}", ext)),
    };

    let schema = schema_for!(ApplicationFile);
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&serde_json::to_value(&schema.schema).unwrap())
        .map_err(|e| format!("Schema error: {e}"))?;

    compiled
        .validate(&json_value)
        .map_err(|errs| errs.map(|e| e.to_string()).collect::<Vec<_>>().join("\n"))?;

    validate_custom(&json_value)?;

    Ok(())
}

fn validate_custom(json: &Value) -> Result<(), String> {
    let mut errors = Vec::new();

    if let Some(apps) = json.get("applications").and_then(|v| v.as_array()) {
        for (i, app) in apps.iter().enumerate() {
            if let Some(versions) = app.get("versions").and_then(|v| v.as_array()) {
                for (j, ver) in versions.iter().enumerate() {
                    if let Some(methods) = ver.get("install_methods").and_then(|v| v.as_array()) {
                        for (k, method) in methods.iter().enumerate() {
                            let steps = method.get("steps");

                            let has_cmd_install = steps
                                .and_then(|s| s.get("install"))
                                .map_or(false, |v| !v.is_null());

                            let has_cmd_uninstall = steps
                                .and_then(|s| s.get("uninstall"))
                                .map_or(false, |v| !v.is_null());

                            let has_steps = steps.is_some();
                            let has_pkg_manager = method.get("package_manager").is_some();
                            let has_pkg_name = method.get("package_name").is_some();

                            if !has_steps && !has_pkg_manager {
                                errors.push(format!(
                                    "App[{}] Version[{}] Method[{}]: Must define at least one installation method — either 'steps' or 'package_manager'",
                                    i, j, k
                                ));
                            }
                            if has_pkg_manager {
                                let install_missing = !has_cmd_install;
                                let uninstall_missing = !has_cmd_uninstall;

                                // If any CLI step is missing, we fall back to PM and need package_name
                                if (install_missing || uninstall_missing || !has_steps)
                                    && !has_pkg_name
                                {
                                    errors.push(format!(
                                        "App[{}] Version[{}] Method[{}]: 'package_manager' requires 'package_name' if any of install/uninstall steps are missing or steps is entirely absent.",
                                        i, j, k
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
