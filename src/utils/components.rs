// Component downloader
// Downloads and manages anti-cheat components (EAC, BattlEye)

use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use reqwest::Client;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::config::paths::Paths;

/// UMU Launcher URLs
/// TODO: Use for UMU auto-update feature
#[allow(dead_code)]
const UMU_VERSION_API: &str = "https://api.github.com/repos/Faugus/umu-launcher/releases";
/// TODO: Use for UMU auto-update feature
#[allow(dead_code)]
const UMU_URL_TEMPLATE: &str =
    "https://github.com/Faugus/umu-launcher/releases/download/{}/umu-run";

/// Component manager
/// TODO: Implement anti-cheat component management (EAC, BattlEye)
#[allow(dead_code)]
pub struct ComponentManager {
    client: Client,
    install_dir: PathBuf,
}

impl ComponentManager {
    /// Create a new component manager
    /// TODO: Use for component initialization
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("Faugus-Launcher")
                .build()
                .unwrap_or_default(),
            install_dir: Paths::user_data("faugus-launcher"),
        }
    }

    /// Get latest UMU version
    /// TODO: Use for UMU update checking
    #[allow(dead_code)]
    pub async fn get_latest_umu_version(&self) -> Result<String> {
        let response = self
            .client
            .get(UMU_VERSION_API)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .context("Failed to fetch UMU version")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch UMU version: {}", response.status());
        }

        let releases: Vec<serde_json::Value> = response.json().await?;

        if let Some(release) = releases.first() {
            if let Some(tag) = release.get("tag_name").and_then(|v| v.as_str()) {
                return Ok(tag.to_string());
            }
        }

        anyhow::bail!("No releases found");
    }

    /// Get installed UMU version
    /// TODO: Use for UMU update checking
    #[allow(dead_code)]
    pub fn get_installed_umu_version(&self) -> Option<String> {
        let version_file = self.install_dir.join("version.txt");

        if !version_file.exists() {
            return None;
        }

        fs::read_to_string(version_file)
            .ok()
            .map(|v| v.trim().to_string())
    }

    /// Download and install UMU runner
    /// TODO: Use for UMU auto-update feature
    #[allow(dead_code)]
    pub async fn download_umu(
        &self,
        version: &str,
        on_progress: impl Fn(u64, u64),
    ) -> Result<PathBuf> {
        info!("Downloading UMU runner version: {}", version);

        let url = UMU_URL_TEMPLATE.replace("{}", version);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to download UMU runner")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download UMU runner: {}", response.status());
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded = 0u64;

        let install_path = self.install_dir.join("umu-run");
        let mut file = File::create(&install_path).await?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.try_next().await? {
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            if total_size > 0 {
                on_progress(downloaded, total_size);
            }
        }

        file.flush().await?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&install_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&install_path, perms)?;
        }

        // Save version
        let version_file = self.install_dir.join("version.txt");
        fs::write(&version_file, version)?;

        info!("UMU runner installed successfully");

        Ok(install_path)
    }

    /// Update UMU runner to latest version
    /// TODO: Use for UMU auto-update feature
    #[allow(dead_code)]
    pub async fn update_umu(&self, on_progress: impl Fn(u64, u64)) -> Result<bool> {
        let latest = self.get_latest_umu_version().await?;
        let installed = self.get_installed_umu_version();

        if installed.as_deref() == Some(&latest) {
            info!("UMU runner is up to date: {}", latest);
            return Ok(false);
        }

        info!("Updating UMU runner from {:?} to {}", installed, latest);

        self.download_umu(&latest, on_progress).await?;

        Ok(true)
    }

    /// Get EAC component directory
    /// TODO: Use for EAC installation/management
    #[allow(dead_code)]
    pub fn eac_dir() -> PathBuf {
        Paths::user_config("faugus-launcher/components/eac")
    }

    /// Get BattlEye component directory
    /// TODO: Use for BattlEye installation/management
    #[allow(dead_code)]
    pub fn be_dir() -> PathBuf {
        Paths::user_config("faugus-launcher/components/be")
    }

    /// Check if EAC is installed
    /// TODO: Use for EAC status display
    #[allow(dead_code)]
    pub fn is_eac_installed() -> bool {
        Self::eac_dir().exists()
    }

    /// Check if BattlEye is installed
    /// TODO: Use for BattlEye status display
    #[allow(dead_code)]
    pub fn is_be_installed() -> bool {
        Self::be_dir().exists()
    }

    /// Install EAC component
    /// TODO: Implement EAC download/installation
    #[allow(dead_code)]
    pub fn install_eac() -> Result<()> {
        let eac_dir = Self::eac_dir();
        fs::create_dir_all(&eac_dir).context("Failed to create EAC directory")?;

        // TODO: Download EAC components
        info!("EAC component directory created at: {:?}", eac_dir);

        Ok(())
    }

    /// Install BattlEye component
    /// TODO: Implement BattlEye download/installation
    #[allow(dead_code)]
    pub fn install_be() -> Result<()> {
        let be_dir = Self::be_dir();
        fs::create_dir_all(&be_dir).context("Failed to create BattlEye directory")?;

        // TODO: Download BattlEye components
        info!("BattlEye component directory created at: {:?}", be_dir);

        Ok(())
    }

    /// Check if components need updating
    /// TODO: Use for component update notifications
    #[allow(dead_code)]
    pub async fn check_components_update(&self) -> Result<bool> {
        let installed = self.get_installed_umu_version();
        let latest = self.get_latest_umu_version().await?;

        Ok(installed.as_deref() != Some(&latest))
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}
