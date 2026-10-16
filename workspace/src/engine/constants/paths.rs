use crate::engine::{
    constants::{CACHE_DIR_NAME, CONFIG_DIR_NAME, STATE_DIR_NAME},
    error::{ErrorCode, Result},
};
use std::{env, path::PathBuf};

pub fn app_config_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|p| p.join(CONFIG_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "APPDATA is not set")
            })
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
            return Ok(PathBuf::from(xdg_config_home).join(CONFIG_DIR_NAME));
        }

        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|p| p.join(".config").join(CONFIG_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "neither XDG_CONFIG_HOME nor HOME is set")
            })
    }
}

pub fn app_state_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|p| p.join(STATE_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "LOCALAPPDATA is not set")
            })
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(xdg_state_home) = env::var_os("XDG_STATE_HOME") {
            return Ok(PathBuf::from(xdg_state_home).join(STATE_DIR_NAME));
        }

        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|p| p.join(".local").join("state").join(STATE_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "neither XDG_STATE_HOME nor HOME is set")
            })
    }
}

pub fn app_cache_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|p| p.join(CACHE_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "LOCALAPPDATA is not set")
            })
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(xdg_cache_home) = env::var_os("XDG_CACHE_HOME") {
            return Ok(PathBuf::from(xdg_cache_home).join(CACHE_DIR_NAME));
        }

        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|p| p.join(".cache").join(CACHE_DIR_NAME))
            .ok_or_else(|| {
                ErrorCode::ConfigInvalid
                    .error()
                    .with_context("reason", "neither XDG_CACHE_HOME nor HOME is set")
            })
    }
}
