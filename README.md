# Faugus Launcher

A simple and lightweight app for running Windows games using [UMU-Launcher](https://github.com/Open-Wine-Components/umu-launcher)

> **Note:** This is the **Rust rewrite** of Faugus Launcher. The original Python (GTK3) version is available in the `legacy` branch.

### Support the project
<a href='https://ko-fi.com/K3K210EMDU' target='_blank'><img src=https://github.com/Faugus/faugus-launcher/blob/main/assets/ko-fi.png width="155" height="35"/></a>&nbsp;&nbsp;
<a href='https://www.paypal.com/donate/?business=57PP9DVD3VWAN&no_recurring=0&currency_code=USD' target='_blank'><img src=https://github.com/Faugus/faugus-launcher/blob/main/assets/paypal.png width="155" height="35"/></a>

# Installation

## Arch-based distributions (AUR)
```
yay -S --noconfirm faugus-launcher
```

## Fedora / Nobara (Copr)
```
sudo dnf -y copr enable faugus/faugus-launcher
sudo dnf -y install faugus-launcher
```

## Bazzite (Copr)
```
sudo dnf5 -y copr enable faugus/faugus-launcher
sudo rpm-ostree -y install faugus-launcher
```
Restart your system.

## Debian-based distributions
### PPA (Ubuntu, Mint, KDE Neon...)
```
sudo dpkg --add-architecture i386
sudo add-apt-repository -y ppa:faugus/faugus-launcher
sudo apt update
sudo apt install -y faugus-launcher
```

### .deb package
```
sudo dpkg --add-architecture i386
sudo apt update
sudo apt install -y wget
mkdir -p ~/faugus-launcher
wget -P ~/faugus-launcher https://github.com/Faugus/faugus-launcher/releases/download/1.13.5/faugus-launcher_1.13.5-1_all.deb
sudo apt install -y ~/faugus-launcher/*.deb
sudo rm -r ~/faugus-launcher
```

## openSUSE
The openSUSE package will no longer be updated. Please use the Flatpak.

## [Flatpak](https://flathub.org/apps/io.github.Faugus.faugus-launcher)
### Installation:
```
flatpak install flathub io.github.Faugus.faugus-launcher
```

### Running:
```
flatpak run io.github.Faugus.faugus-launcher
```

### MangoHud installation:
```
flatpak install org.freedesktop.Platform.VulkanLayer.MangoHud/x86_64/25.08
```

### Steam Flatpak integration
Allow Faugus Launcher to detect Steam users:
```
sudo flatpak override io.github.Faugus.faugus-launcher --filesystem=~/.var/app/com.valvesoftware.Steam/.steam/steam/userdata/
```
Allow Steam to run Faugus Launcher's shortcuts:
```
sudo flatpak override com.valvesoftware.Steam --talk-name=org.freedesktop.Flatpak
```
Allow Steam to see the game's icon:
```
sudo flatpak override com.valvesoftware.Steam --filesystem=~/.var/app/io.github.Faugus.faugus-launcher/config/faugus-launcher/
```

### Known issues:
- The 'stop' button won't close games/apps
- Gamescope doesn't work
- It may not use the system theme in some DEs

## Build from source

### Requirements
- Rust 1.75 or later
- Cargo
- System libraries (on Debian/Ubuntu): `libssl-dev pkg-config libatk1.0-dev libgtk-3-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libx11-dev`

### Building
```bash
cargo build --release
```

The binary will be located at `target/release/faugus-launcher-rs`.

### Installing (optional)
```bash
sudo cp target/release/faugus-launcher-rs /usr/local/bin/faugus-launcher
```

# Usage
[![YouTube](http://i.ytimg.com/vi/Ay6C2f55Pc8/hqdefault.jpg)](https://www.youtube.com/watch?v=Ay6C2f55Pc8)

# Information
### Default prefixes location
```
~/Faugus/
```

### Runners location
```
~/.local/share/Steam/compatibilitytools.d/
```

### Shortcut locations
For Desktop Environments that support icons on the Desktop
```
~/Desktop/
```
For Application Launchers
```
~/.local/share/applications/
```

# Technology Stack (Rust Version)
- **GUI Framework**: [Iced](https://github.com/iced-rs/iced) 0.13
- **Runtime**: [Tokio](https://tokio.rs/) async runtime
- **System Integration**:
  - Steam VDF parsing: [new-vdf-parser](https://github.com/V0ldek/new-vdf-parser)
  - Process management: [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
  - System tray: [tray-icon](https://github.com/tauri-apps/tray-icon)
- **Networking**: [reqwest](https://github.com/seanmonstar/reqwest) with rustls-tls
- **Configuration**: [serde](https://serde.rs/) for JSON/VDF

# Development

## Project Structure
```
src/
├── main.rs              # Application entry point
├── config/              # Configuration management
├── gui/                 # Iced UI components
├── launcher/            # Game launching logic
├── steam/               # Steam integration (VDF)
├── tray/                # System tray functionality
├── icons/               # Icon management
└── utils/               # Utilities (anti-cheat, paths)
```

## Running tests
```bash
cargo test
```

## Checking code
```bash
cargo check
cargo clippy
cargo fmt
```

# Screenshots
### Main window
<img src=screenshots/main-list.png/><br><br>
<img src=screenshots/main-blocks.png/><br><br>
<img src=screenshots/main-banners.png/><br>

### Add/Edit game
<img src=screenshots/add-main.png/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src=screenshots/add-tools.png/><br>

### Settings
<img src=screenshots/settings.png/><br>

### Proton Manager
<img src=screenshots/proton-manager.png/><br>

### Create shortcut from .exe file
<img src=screenshots/shortcut-file.png/><br>
