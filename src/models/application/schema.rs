use crate::{
    SUPPORTED_EXTS, log_info, log_warn, models::application::ApplicationList as ApplicationFile,
};
use jsonschema::validator_for;
use schemars::schema_for;
use serde_json::Value;
use std::{fs, path::Path};

pub fn validate_file(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        log_warn!(
            "validate",
            "application",
            &format!("❌ Unsupported extension: .{ext}")
        );
        return false;
    }
    let raw = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            log_warn!(
                "validate",
                "application",
                &format!("❌ Failed to read file: {e}")
            );
            return false;
        }
    };

    let json_value: Value = match ext.as_str() {
        "json" => match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                log_warn!("validate", "application", &format!("❌ Invalid JSON: {e}"));
                return false;
            }
        },
        "yaml" | "yml" => {
            let yaml: serde_yaml::Value = match serde_yaml::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "application", &format!("❌ Invalid YAML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(yaml) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!(
                        "validate",
                        "application",
                        &format!("❌ Failed to convert YAML to JSON: {e}")
                    );
                    return false;
                }
            }
        }
        "xml" => {
            let parsed: ApplicationFile = match quick_xml::de::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!("validate", "application", &format!("❌ Invalid XML: {e}"));
                    return false;
                }
            };

            match serde_json::to_value(parsed) {
                Ok(v) => v,
                Err(e) => {
                    log_warn!(
                        "validate",
                        "application",
                        &format!("❌ Failed to convert XML to JSON: {e}")
                    );
                    return false;
                }
            }
        }
        _ => {
            log_warn!(
                "validate",
                "application",
                &format!("❌ Unsupported file extension: .{ext}")
            );
            return false;
        }
    };

    let schema = schema_for!(ApplicationFile);
    let schema_value = match serde_json::to_value(&schema) {
        Ok(s) => s,
        Err(e) => {
            log_warn!(
                "validate",
                "schema",
                &format!("❌ Failed to serialize schema: {e}")
            );
            return false;
        }
    };

    let validator = match validator_for(&schema_value) {
        Ok(v) => v,
        Err(e) => {
            log_warn!(
                "validate",
                "schema",
                &format!("❌ Schema generation failed: {e}")
            );
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

    log_info!("validate", "application", "✅ Application config is valid.");
    true
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
