//! # Tranquility
//!
//! A system management tool for installing apps, fonts, and managing VPS servers.
//!
//! ## Modules
//!
//! - [`cli`]: Command-line interface implementation
//! - [`core`]: Core application logic including bootstrap, context management, and use cases
//! - [`engine`]: Business logic layer with traits, models, capabilities, and configuration
//! - [`infra`]: Infrastructure layer for UI interactions and file system access

pub mod cli;
pub mod core;
pub mod engine;
pub mod infra;
