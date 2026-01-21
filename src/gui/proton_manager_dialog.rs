// Proton Manager GUI
// Dialog for managing Proton versions

use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{Alignment, Element, Length, Task};
use std::path::PathBuf;

use crate::locale::I18n;
use crate::proton::proton_manager::{ProtonManager, ProtonRelease, PROTON_CONFIGS};

/// Messages for the Proton Manager
#[derive(Debug, Clone)]
pub enum ProtonManagerMessage {
    // Tab selection
    TabSelected(usize),

    // Fetch releases
    FetchReleases,
    ReleasesFetched(usize, Vec<ProtonRelease>),
    FetchError(String),

    // Download/Remove
    DownloadClicked(usize, String),
    RemoveClicked(usize, String),

    // Download progress
    DownloadProgress(String, u64, u64),
    DownloadComplete(String),
    DownloadError(String, String),

    // Refresh
    RefreshClicked,

    // Dialog
    Close,
}

/// Proton version entry state
#[derive(Debug, Clone)]
struct ProtonVersionEntry {
    tag_name: String,
    display_name: String,
    installed: bool,
    size: u64,
    downloading: bool,
    download_progress: f32,
}

impl ProtonVersionEntry {
    fn new(release: &ProtonRelease, is_installed: bool) -> Self {
        let tag_name = release.tag_name.clone();
        let display_name = if tag_name.starts_with("EM-") {
            format!("proton-{}", tag_name)
        } else {
            tag_name.clone()
        };

        let size = release
            .assets
            .iter()
            .find(|a| a.name.ends_with(".tar.gz") || a.name.ends_with(".tar.xz"))
            .map(|a| a.size)
            .unwrap_or(0);

        Self {
            tag_name,
            display_name,
            installed: is_installed,
            size,
            downloading: false,
            download_progress: 0.0,
        }
    }
}

/// State for the Proton Manager dialog
#[derive(Debug, Clone)]
pub struct ProtonManagerDialog {
    /// Manager instance
    manager: ProtonManager,

    /// Currently selected tab
    selected_tab: usize,

    /// Releases for each tab (GE-Proton, Proton-EM)
    releases: Vec<Vec<ProtonVersionEntry>>,

    /// Loading state
    loading: bool,

    /// Error message
    error_message: Option<String>,

    /// Progress label text
    progress_label: String,

    /// Progress bar value (0.0 to 1.0)
    progress_value: f32,

    /// Show progress bar
    show_progress: bool,

    /// Steam compat directory
    compat_dir: PathBuf,
}

impl ProtonManagerDialog {
    /// Create a new Proton Manager dialog
    pub fn new() -> (Self, Task<ProtonManagerMessage>) {
        let manager = ProtonManager::new();
        let compat_dir = manager.compat_dir.clone();

        (
            Self {
                manager,
                selected_tab: 0,
                releases: vec![Vec::new(), Vec::new()],
                loading: false,
                error_message: None,
                progress_label: String::new(),
                progress_value: 0.0,
                show_progress: false,
                compat_dir,
            },
            Task::done(ProtonManagerMessage::FetchReleases),
        )
    }

    /// Update the dialog state
    pub fn update(&mut self, message: ProtonManagerMessage) -> Task<ProtonManagerMessage> {
        match message {
            ProtonManagerMessage::TabSelected(index) => {
                if index < PROTON_CONFIGS.len() {
                    self.selected_tab = index;
                }
            }
            ProtonManagerMessage::FetchReleases => {
                if self.loading {
                    return Task::none();
                }

                self.loading = true;
                self.error_message = None;

                // Fetch releases for each Proton type
                let mut tasks = Vec::new();
                for (index, config) in PROTON_CONFIGS.iter().enumerate() {
                    let manager = self.manager.clone();
                    let index_clone = index;

                    // Create async task
                    tasks.push(Task::perform(
                        async move {
                            let releases = manager.get_all_releases(config).await.ok();
                            (index_clone, releases)
                        },
                        |(index, releases)| {
                            if let Some(releases) = releases {
                                ProtonManagerMessage::ReleasesFetched(index, releases)
                            } else {
                                ProtonManagerMessage::FetchError(
                                    "Failed to fetch releases".to_string(),
                                )
                            }
                        },
                    ));
                }

                return Task::batch(tasks);
            }
            ProtonManagerMessage::ReleasesFetched(tab_index, releases) => {
                self.loading = false;

                // Check which versions are installed
                let installed_versions = self.manager.get_installed_versions();

                for release in releases {
                    let tag_name = if release.tag_name.starts_with("EM-") {
                        format!("proton-{}", release.tag_name)
                    } else {
                        release.tag_name.clone()
                    };

                    let is_installed = installed_versions
                        .iter()
                        .any(|v| v.to_lowercase() == tag_name.to_lowercase());

                    let entry = ProtonVersionEntry::new(&release, is_installed);

                    if tab_index < self.releases.len() {
                        // Only add if not already in list
                        let exists = self.releases[tab_index]
                            .iter()
                            .any(|e| e.tag_name == release.tag_name);

                        if !exists {
                            self.releases[tab_index].push(entry);
                        }
                    }
                }
            }
            ProtonManagerMessage::FetchError(error) => {
                self.loading = false;
                self.error_message = Some(error);
            }
            ProtonManagerMessage::DownloadClicked(tab_index, tag_name) => {
                if tab_index >= self.releases.len() {
                    return Task::none();
                }

                // Find and update entry
                if let Some(entry) = self.releases[tab_index]
                    .iter_mut()
                    .find(|e| e.tag_name == tag_name)
                {
                    entry.downloading = true;
                    entry.download_progress = 0.0;
                }

                self.show_progress = true;
                self.progress_label = format!("Downloading {}...", tag_name);
                self.progress_value = 0.0;

                // Start download (this would be async in real implementation)
                // For now, just mark as downloading
            }
            ProtonManagerMessage::RemoveClicked(tab_index, tag_name) => {
                if tab_index >= self.releases.len() {
                    return Task::none();
                }

                // Find the installed path
                let display_name = if tag_name.starts_with("EM-") {
                    format!("proton-{}", tag_name)
                } else {
                    tag_name.clone()
                };

                // Try to find and remove
                if let Some(_entry) = self.releases[tab_index]
                    .iter()
                    .find(|e| e.tag_name == tag_name)
                {
                    let path = self.compat_dir.join(&display_name);

                    if path.exists() {
                        if let Err(e) = std::fs::remove_dir_all(&path) {
                            self.error_message = Some(format!("Failed to remove: {}", e));
                        } else {
                            // Update entry state
                            if let Some(entry) = self.releases[tab_index]
                                .iter_mut()
                                .find(|e| e.tag_name == tag_name)
                            {
                                entry.installed = false;
                            }
                        }
                    }
                }
            }
            ProtonManagerMessage::DownloadProgress(tag_name, downloaded, total) => {
                let progress = if total > 0 {
                    downloaded as f32 / total as f32
                } else {
                    0.0
                };

                self.progress_value = progress;
                self.progress_label =
                    format!("Downloading {}... {:.0}%", tag_name, progress * 100.0);

                // Update entry progress
                for releases in &mut self.releases {
                    if let Some(entry) = releases.iter_mut().find(|e| e.tag_name == tag_name) {
                        entry.download_progress = progress;
                    }
                }
            }
            ProtonManagerMessage::DownloadComplete(tag_name) => {
                self.show_progress = false;

                // Update entry state
                for releases in &mut self.releases {
                    if let Some(entry) = releases.iter_mut().find(|e| e.tag_name == tag_name) {
                        entry.installed = true;
                        entry.downloading = false;
                    }
                }
            }
            ProtonManagerMessage::DownloadError(tag_name, error) => {
                self.show_progress = false;
                self.error_message = Some(format!("Download failed: {}", error));

                // Reset downloading state
                for releases in &mut self.releases {
                    if let Some(entry) = releases.iter_mut().find(|e| e.tag_name == tag_name) {
                        entry.downloading = false;
                    }
                }
            }
            ProtonManagerMessage::RefreshClicked => {
                // Clear current releases and fetch new ones
                self.releases = vec![Vec::new(), Vec::new()];
                return Task::done(ProtonManagerMessage::FetchReleases);
            }
            ProtonManagerMessage::Close => {
                // Handled by caller
            }
        }

        Task::none()
    }

    /// View the dialog
    pub fn view(&self, i18n: &I18n) -> Element<'_, ProtonManagerMessage> {
        let tabs = self.view_tabs(i18n);
        let content = self.view_content(i18n);
        let progress_section = self.view_progress_section(i18n);
        let buttons = self.view_buttons(i18n);

        let error_section = if let Some(ref error) = self.error_message {
            column![
                Space::with_height(Length::Fixed(10.0)),
                text(error)
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::new(1.0, 0.0, 0.0, 1.0)),
                    }),
            ]
        } else {
            column![]
        };

        column![
            tabs,
            Space::with_height(Length::Fixed(10.0)),
            horizontal_rule(1),
            Space::with_height(Length::Fixed(10.0)),
            content,
            error_section,
            progress_section,
            Space::with_height(Length::Fixed(10.0)),
            buttons,
        ]
        .spacing(5)
        .padding(20)
        .into()
    }

    /// View tab buttons
    fn view_tabs(&self, _i18n: &I18n) -> Element<'_, ProtonManagerMessage> {
        let mut tabs = row![];

        for (index, config) in PROTON_CONFIGS.iter().enumerate() {
            let _is_selected = self.selected_tab == index;
            let tab_button = button(text(config.label).size(14))
                .on_press(ProtonManagerMessage::TabSelected(index));

            tabs = tabs.push(tab_button);
        }

        container(tabs.spacing(10)).padding(10).into()
    }

    /// View content area
    fn view_content(&self, i18n: &I18n) -> Element<'_, ProtonManagerMessage> {
        if self.loading {
            return text("Loading...").into();
        }

        if self.selected_tab >= self.releases.len() {
            return text("No releases available").into();
        }

        let releases = &self.releases[self.selected_tab];

        if releases.is_empty() {
            return column![
                text("No releases found").size(14),
                Space::with_height(Length::Fixed(10.0)),
                button(text("Refresh").size(14)).on_press(ProtonManagerMessage::RefreshClicked),
            ]
            .spacing(5)
            .into();
        }

        // Create list of releases
        let mut release_list = column![];

        for release in releases {
            let row = self.view_release_row(release, i18n);
            release_list = release_list.push(row);
        }

        scrollable(release_list.spacing(5))
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
            .into()
    }

    /// View a single release row
    fn view_release_row<'a>(
        &self,
        release: &'a ProtonVersionEntry,
        _i18n: &I18n,
    ) -> Element<'a, ProtonManagerMessage> {
        let version_text = text(&release.display_name).size(14);
        let size_text = text(format_size(release.size)).size(12);

        let button_text = if release.downloading {
            format!("{:.0}%", release.download_progress * 100.0)
        } else if release.installed {
            "Remove".to_string()
        } else {
            "Download".to_string()
        };

        let action_button = button(text(button_text).size(12)).width(Length::Fixed(120.0));

        let action_button = if release.downloading {
            action_button
        } else if release.installed {
            action_button.on_press(ProtonManagerMessage::RemoveClicked(
                self.selected_tab,
                release.tag_name.clone(),
            ))
        } else {
            action_button.on_press(ProtonManagerMessage::DownloadClicked(
                self.selected_tab,
                release.tag_name.clone(),
            ))
        };

        row![
            version_text,
            Space::with_width(Length::Fill),
            size_text,
            Space::with_width(Length::Fixed(10.0)),
            action_button,
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center)
        .into()
    }

    /// View progress section
    fn view_progress_section(&self, _i18n: &I18n) -> Element<'_, ProtonManagerMessage> {
        if !self.show_progress {
            return column![].into();
        }

        column![
            text(&self.progress_label).size(12),
            Space::with_height(Length::Fixed(5.0)),
            // Progress bar would go here - Iced doesn't have a native progress bar widget
            // We'll use text for now
            text(format!("{:.0}%", self.progress_value * 100.0)).size(12),
        ]
        .spacing(5)
        .into()
    }

    /// View buttons
    fn view_buttons(&self, i18n: &I18n) -> Element<'_, ProtonManagerMessage> {
        row![
            button(text("Refresh").size(14)).on_press(ProtonManagerMessage::RefreshClicked),
            Space::with_width(Length::Fill),
            button(text(i18n.t("Close")).size(14))
                .on_press(ProtonManagerMessage::Close)
                .width(Length::Fixed(100.0)),
        ]
        .spacing(10)
        .into()
    }
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

impl Default for ProtonManagerDialog {
    fn default() -> Self {
        Self::new().0
    }
}
