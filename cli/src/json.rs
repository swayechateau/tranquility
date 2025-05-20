use std::fs;
use crate::models::ApplicationConfig;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;

fn validate_against_schema(json: &str, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let instance: Value = serde_json::from_str(json)?;
    let schema_json = std::fs::read_to_string(schema_path)?;
    let schema: Value = serde_json::from_str(&schema_json)?;
    let compiled = JSONSchema::options().with_draft(Draft::Draft202012).compile(&schema)?;
    if let Err(errors) = compiled.validate(&instance) {
        for error in errors {
            eprintln!("❌ Schema error: {}", error);
        }
        return Err("Validation failed".into());
    }
    Ok(())
}

pub fn load_applications_from_file(path: &str) -> Result<ApplicationConfig, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let config: ApplicationConfig = serde_json::from_str(&data)?;
    Ok(config)
}