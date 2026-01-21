use crate::config::paths::Paths;
use std::path::PathBuf;
use thiserror::Error;

pub const UMU_PROTON_LATEST: &str = "UMU-Proton Latest";
pub const GE_PROTON_LATEST: &str = "GE-Proton Latest (default)";
pub const PROTON_EM_LATEST: &str = "Proton-EM Latest";

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("Runner '{name}' is not installed. Please install it via Proton Manager.")]
    NotInstalled { name: String },
    /// TODO: Use for custom path validation
    #[allow(dead_code)]
    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },
}

/// Resolves a display name to a PROTONPATH value.
pub fn resolve_runner(name: &str) -> Result<String, RunnerError> {
    match name {
        UMU_PROTON_LATEST => Ok(String::new()),
        GE_PROTON_LATEST => Ok("Proton-GE Latest".to_string()),
        PROTON_EM_LATEST => Ok("Proton-EM Latest".to_string()),
        _ => {
            // Check if it's a known system proton that needs full path
            if name == "Proton-CachyOS" {
                let system_path =
                    PathBuf::from("/usr/share/steam/compatibilitytools.d/Proton-CachyOS");
                if system_path.exists() {
                    return Ok(system_path.to_string_lossy().to_string());
                }
            }
            Ok(name.to_string())
        }
    }
}

/// Validates if the resolved runner exists.
pub fn validate_runner(name: &str) -> Result<(), RunnerError> {
    let resolved = resolve_runner(name)?;
    if resolved.is_empty() {
        return Ok(()); // UMU-Proton Latest is handled by umu-run auto
    }

    if resolved.starts_with('/') {
        if PathBuf::from(&resolved).exists() {
            return Ok(());
        } else {
            return Err(RunnerError::NotInstalled {
                name: name.to_string(),
            });
        }
    }

    // Check user compatibilitytools.d
    let user_path = Paths::steam_compat_tools_dir().join(&resolved);
    if user_path.exists() {
        return Ok(());
    }

    // Check system compatibilitytools.d
    let system_path = PathBuf::from("/usr/share/steam/compatibilitytools.d").join(&resolved);
    if system_path.exists() {
        return Ok(());
    }

    Err(RunnerError::NotInstalled {
        name: name.to_string(),
    })
}
