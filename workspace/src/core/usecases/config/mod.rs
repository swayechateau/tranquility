//! Configuration management use cases.
//!
//! Provides Reset, Override, and Validate operations for config, applications, and VPS files.

use crate::{
    core::Context,
    core::CoreResult,
    engine::config,
    engine::error::{Error, ErrorCode},
};
use std::fs;
use std::path::PathBuf;

/// Reset the main config file to default.
pub async fn reset_config(ctx: &Context) -> CoreResult<()> {
    if let Some(path) = ctx.project_config_path() {
        if ctx.dry_run() {
            ctx.ui().logger().info(&format!(
                "[dry run] would reset config file: {}",
                path.display()
            ));
            return Ok(());
        }
        fs::remove_file(&path)?;
        ctx.ui()
            .logger()
            .info(&format!("Config file reset: {}", path.display()));
    }
    Ok(())
}

/// Reset the applications file to default (create empty).
pub async fn reset_applications(ctx: &Context) -> CoreResult<()> {
    let app_file = ctx.base_config().applications_file.clone();
    if app_file.exists() {
        if ctx.dry_run() {
            ctx.ui().logger().info(&format!(
                "[dry run] would reset applications file: {}",
                app_file.display()
            ));
            return Ok(());
        }
        fs::remove_file(&app_file)?;
        ctx.ui()
            .logger()
            .info(&format!("Applications file reset: {}", app_file.display()));
    }
    Ok(())
}

/// Reset the VPS file to default (create empty).
pub async fn reset_vps(ctx: &Context) -> CoreResult<()> {
    let vps_file = ctx.base_config().vps_file.clone();
    if vps_file.exists() {
        if ctx.dry_run() {
            ctx.ui().logger().info(&format!(
                "[dry run] would reset VPS file: {}",
                vps_file.display()
            ));
            return Ok(());
        }
        fs::remove_file(&vps_file)?;
        ctx.ui()
            .logger()
            .info(&format!("VPS file reset: {}", vps_file.display()));
    }
    Ok(())
}

/// Override the main config file with a custom file.
pub async fn override_config(ctx: &Context, source: &PathBuf) -> CoreResult<()> {
    config::check_file(source).map_err(|msg| {
        Error::from_code(ErrorCode::ConfigInvalid)
            .with_context("file", source.display().to_string())
            .with_context("error", msg)
    })?;

    let dest = ctx
        .project_config_path()
        .ok_or_else(|| Error::from_code(ErrorCode::ConfigInvalid))?;

    if ctx.dry_run() {
        ctx.ui().logger().info(&format!(
            "[dry run] would override config with: {}",
            source.display()
        ));
        return Ok(());
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, &dest)?;
    ctx.ui()
        .logger()
        .info(&format!("Config overridden with: {}", source.display()));
    Ok(())
}

/// Override the applications file with a custom file.
pub async fn override_applications(ctx: &Context, source: &PathBuf) -> CoreResult<()> {
    crate::engine::capabilities::apps::validate_apps_file(source)?;
    let dest = ctx.base_config().applications_file.clone();

    if ctx.dry_run() {
        ctx.ui().logger().info(&format!(
            "[dry run] would override applications file with: {}",
            source.display()
        ));
        return Ok(());
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, &dest)?;
    ctx.ui().logger().info(&format!(
        "Applications file overridden with: {}",
        source.display()
    ));
    Ok(())
}

/// Override the VPS file with a custom file.
pub async fn override_vps(ctx: &Context, source: &PathBuf) -> CoreResult<()> {
    crate::engine::capabilities::vps::validate_file(source)?;
    let dest = ctx.base_config().vps_file.clone();

    if ctx.dry_run() {
        ctx.ui().logger().info(&format!(
            "[dry run] would override VPS file with: {}",
            source.display()
        ));
        return Ok(());
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, &dest)?;
    ctx.ui()
        .logger()
        .info(&format!("VPS file overridden with: {}", source.display()));
    Ok(())
}

/// Validate the main config file.
pub async fn validate_config(_ctx: &Context, source: &std::path::Path) -> CoreResult<()> {
    config::check_file(source).map_err(|msg| {
        Error::from_code(ErrorCode::ValidationFailed)
            .with_context("file", source.display().to_string())
            .with_context("error", msg)
    })?;
    println!("✓ Config file is valid");
    Ok(())
}

/// Validate the applications file.
pub async fn validate_applications(_ctx: &Context, source: &std::path::Path) -> CoreResult<()> {
    let n = crate::engine::capabilities::apps::validate_apps_file(source)?;
    println!("✓ Applications file is valid ({n} entries)");
    Ok(())
}

/// Validate the VPS file.
pub async fn validate_vps(_ctx: &Context, source: &PathBuf) -> CoreResult<()> {
    let n = crate::engine::capabilities::vps::validate_file(source)?;
    println!("✓ VPS file is valid ({n} entries)");
    Ok(())
}
