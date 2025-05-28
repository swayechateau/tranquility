// Module: Schema
// Location:cli/src/schema/mod.rs
pub mod application;
pub mod config;
pub mod vps;

pub const SUPPORTED_EXTS: [&str; 4] = ["json", "yaml", "yml", "xml"];