// Tray Menu
// Manages the right-click context menu for the system tray icon

use anyhow::Result;
use muda::Menu;
use tracing::info;

use crate::locale::I18n;

/// Tray menu manager
pub struct TrayMenu {
    menu: Menu,
    show_item: muda::MenuItem,
    hide_item: muda::MenuItem,
}

impl TrayMenu {
    /// Create a new tray menu
    pub fn new(i18n: &I18n) -> Result<Self> {
        let menu = Menu::new();

        let show_item = muda::MenuItem::with_id("show", i18n.t("Show"), true, None);
        let hide_item = muda::MenuItem::with_id("hide", i18n.t("Hide"), true, None);

        menu.append(&show_item)?;
        menu.append(&hide_item)?;
        menu.append(&muda::PredefinedMenuItem::separator())?;

        let _quit_item = muda::MenuItem::with_id("quit", i18n.t("Quit"), true, None);
        menu.append(&_quit_item)?;

        info!("Tray menu created");

        Ok(Self {
            menu,
            show_item,
            hide_item,
        })
    }

    /// Get the menu
    pub fn menu(&self) -> &Menu {
        &self.menu
    }

    /// Set the enabled state of show/hide items
    pub fn set_window_visible(&self, visible: bool) {
        self.show_item.set_enabled(!visible);
        self.hide_item.set_enabled(visible);
    }
}
