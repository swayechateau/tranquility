use crate::{
    SUPPORTED_EXTS, log_info, log_warn,
    models::vps::{json::VpsConfig, xml::VpsConfigXml},
};
use jsonschema::validator_for;
use schemars::{Schema, schema_for};
use serde_json::Value;
use std::{fs, path::Path};

pub fn validate_file(path: &Path) -> bool {
    let ext = get_file_extension(path);

    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        log_warn!(
            "validate",
            "vps",
            &format!("❌ Unsupported extension: .{ext}")
        );
        return false;
    }

    let raw = match read_file_to_string(path) {
        Ok(content) => content,
        Err(err_msg) => {
            log_warn!("validate", "vps", &format!("❌ {err_msg}"));
            return false;
        }
    };

    let (json_value, schema_value) = match parse_and_convert_to_json(&ext, &raw) {
        Ok(result) => result,
        Err(err_msg) => {
            log_warn!("validate", "vps", &format!("❌ {err_msg}"));
            return false;
        }
    };

    let schema_json = match serde_json::to_value(&schema_value) {
        Ok(s) => s,
        Err(e) => {
            log_warn!(
                "validate",
                "schema",
                &format!("❌ Failed to convert schema: {e}")
            );
            return false;
        }
    };

    if let Err(errors) = validate_against_schema(&json_value, &schema_json) {
        for err in errors {
            log_warn!("validate", "schema", &format!("Schema violation: {err}"));
        }
        return false;
    }

    if let Err(custom_errors) = validate_custom(&json_value, &ext) {
        for err in custom_errors.lines() {
            log_warn!("validate", "custom", &format!("❌ {err}"));
        }
        return false;
    }

    log_info!("validate", "vps", "✅ VPS config is valid.");
    true
}

fn get_file_extension(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn read_file_to_string(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {e}"))
}

fn parse_and_convert_to_json(ext: &str, raw: &str) -> Result<(Value, Schema), String> {
    match ext {
        "json" => {
            let value =
                serde_json::from_str::<Value>(raw).map_err(|e| format!("Invalid JSON: {e}"))?;
            Ok((value, schema_for!(VpsConfig)))
        }
        "yaml" | "yml" => {
            let yaml = serde_yaml::from_str::<serde_yaml::Value>(raw)
                .map_err(|e| format!("Invalid YAML: {e}"))?;
            let json_value = serde_json::to_value(yaml)
                .map_err(|e| format!("Failed to convert YAML to JSON: {e}"))?;
            Ok((json_value, schema_for!(VpsConfig)))
        }
        "xml" => {
            let parsed_xml = quick_xml::de::from_str::<VpsConfigXml>(raw)
                .map_err(|e| format!("Invalid XML: {e}"))?;

            let config: VpsConfig = parsed_xml.into(); // Convert XML -> Unified VpsConfig
            let json_value = serde_json::to_value(&config)
                .map_err(|e| format!("Failed to convert XML to JSON: {e}"))?;

            Ok((json_value, schema_for!(VpsConfig)))
        }
        _ => Err("Unsupported extension".into()),
    }
}

fn validate_against_schema(json_value: &Value, schema_json: &Value) -> Result<(), Vec<String>> {
    match validator_for(schema_json) {
        Ok(validator) => {
            if let Err(_) = validator.validate(json_value) {
                let errors: Vec<String> = validator
                    .iter_errors(json_value)
                    .map(|e| e.to_string())
                    .collect();
                Err(errors)
            } else {
                Ok(())
            }
        }
        Err(e) => Err(vec![format!("Failed to compile schema: {e}")]),
    }
}

fn validate_custom(json: &Value, ext: &str) -> Result<(), String> {
    println!("validate_custom: {ext}");
    let config: Result<VpsConfig, _> = serde_json::from_value(json.clone());
    if config.is_err() {
        return Err("Failed to convert JSON to VpsConfig for custom validation.".into());
    }
    let vps_config = config.unwrap();
    vps_config.validate()
}
