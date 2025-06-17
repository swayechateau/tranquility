// Module: Schema
// Location:cli/src/schema/mod.rs
pub mod application;
pub mod config;
pub mod vps;

/// List of supported file extensions for schema validation.
pub const SUPPORTED_EXTS: [&str; 4] = ["yaml", "yml", "json", "xml"];