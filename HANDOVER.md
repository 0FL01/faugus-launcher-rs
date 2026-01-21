# Handover Notes: Rust faugus-run Final (Iterations 1-2 Complete)

## Summary
Implemented Rust `faugus-run` CLI binary that fully replaces Python `faugus_run.py` for game launching, removing Python runtime dependency from the launch path while preserving complete compatibility with existing configurations and prefixes.

## What Was Removed / Decoupled
- **Python runtime dependency for launching**: Games no longer require `faugus_run.py` or Python 3 interpreter
- **Python shortcuts generation**: Desktop entry generation moved to Rust (`src/shortcuts/desktop_entry.rs`)
- **Legacy launch path**: All game launches now go through `GameLauncher::launch()` with UMU integration
- **Python-era fragility**: Removed `contains()` string matching that caused false positives from config comments

## What Was Added / Replaced
- **Rust `faugus-run` binary**: `src/bin/faugus-run.rs` — drop-in CLI replacement for Python launcher
  - `faugus-run --game <gameid>` — launches games via Rust launcher
  - Unit-tested CLI parsing, error handling, game lookup
- **envar.txt support**: `src/config/envar.rs` — global environment variables
  - Python-compatible `KEY=value` format
  - Precedence: envar.txt → game-specific → launcher overrides
  - Robust parsing with comment skipping, partial error recovery
- **AppConfig typed parsing**: Replaced fragile string matching with structured `AppConfig::load()`
  - No false positives from config comments or substrings
  - Proper boolean parsing for wayland, HDR, WOW64, GPU, logging settings
- **GameMode fix**: Changed from incorrect `LD_PRELOAD=gamemoderun` to correct `gamemoderun umu-run` wrapper
- **Lossless Scaling fix**: Uses Linux-native `LSFG_*` env vars instead of Windows DLL LD_PRELOAD
- **Config caching**: `config.ini` read once per launch (was 5+ reads)
- **Library target**: `src/lib.rs` — exposes core functionality for external binaries

## Compatibility Guarantees
**On-disk API fully preserved**:
- **`games.json`**: Format unchanged, full Python → Rust compatibility via `legacy_compat` flag
- **`config.ini`**: Format unchanged, settings read by `AppConfig::load()`
- **`envar.txt`**: Python-compatible format, newly integrated
- **WINEPREFIX**: No changes to existing prefixes, launcher only sets environment
- **Steam shortcuts**: `.desktop` files can reference Rust binary: `Exec=/usr/local/bin/faugus-run --game %u`

## How to Run
```bash
# GUI (default binary)
cargo run

# CLI launcher
cargo run --bin faugus-run -- --game <gameid>

# Release build
cargo build --release
./target/release/faugus-launcher-rs          # GUI
./target/release/faugus-run --game <gameid>  # CLI
```

## Verification Checklist
```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release
```

## Known Limitations / Out-of-Scope
- **Flatpak not required**: Assumes standard Linux environment, no Flatpak detection
- **Rust→Python backwards not required**: Legacy handles Python→Rust migration only
- **Process detachment**: Game launches detached (as before), no PID monitoring in CLI
- **No game validation**: Binary doesn't check executable existence before launch
- **UMU-Launcher required**: Fails if `umu-run` not installed/in PATH

## Dead Code Cleanup (Iteration 3)

### Removed Items
- **`src/config/paths.rs`**:
  - `faugus_proton_manager()` — legacy Python binary integration (dead)
  - 8 unused TODO functions: `temp_dir()`, `lock_file()`, `share_dir()`, `is_steam_flatpak()`, `steam_common_dirs()`, `find_lossless_dll()`, `is_flatpak()`, `lsfgvk_so()`
  - Removed obsolete `#[allow(dead_code)]` from `default_prefix()` (now actively used)

- **`src/utils/components.rs`** — entire module removed
  - `ComponentManager` struct and all methods (all marked dead_code, never used)

- **`src/steam/shortcuts.rs`**:
  - `SteamShortcut` struct (marked dead_code, never used)
  - `get_all()` method (dead_code, only returned SteamShortcut)
  - `value_to_shortcut()` method (dead_code, only converted to SteamShortcut)
  - Related test `test_steam_shortcut_default()`

### Directory Changes
- Removed empty `src/utils/` directory and `src/utils/mod.rs`
- Removed `mod utils;` from `src/main.rs`

### Preserved Items (Not Removed)
- `Paths::faugus_run()` — actively used in `src/shortcuts/desktop_entry.rs`
- `Paths::envar_txt()` — actively used in launcher
- `AppConfig.lossless_location` — active config field
- All working Steam VDF parsing code and new-vdf-parser usage

### Verification
- ✅ `cargo fmt`
- ✅ `cargo clippy --all-targets -- -D warnings`
- ✅ `cargo test` (29 tests pass)

## Next Steps
1. ~~**Packaging**: Install `faugus-run` to `/usr/local/bin/` or `/usr/bin/` via package manager~~ ✅ Verification script added (see "Packaging Verification" section below)
2. **Optional**: Move tracing initialization from main.rs to library entry point for better reusability
3. **Desktop integration**: Update `.desktop` files and Steam shortcuts to reference Rust binary

## Technical Details

### envar.txt (NEW)
**Location**: `~/.config/faugus-launcher/envar.txt`
**Format**: `KEY=value` (Python-compatible)
**Precedence**: envar.txt → game-specific → launcher overrides
**Parsing**:
- Skips comments (`#`, `;`) and empty lines
- Validates keys: `[A-Za-z_][A-Za-z0-9_]*`
- Partial error recovery (invalid lines logged, valid ones applied)
**Tests**: 20+ unit tests in `src/config/envar.rs`

### AppConfig Structured Parsing
**Fix**: Replaced `config.contains("key=true")` with `AppConfig::load()`
**Impact**: No false positives from comments or substrings
**Settings mapped from config.ini**:
- `wayland_driver` → `PROTON_ENABLE_WAYLAND`
- `enable_hdr` → `ENABLE_HDR`
- `enable_wow64` → `PROTON_USE_WOW64`
- `discrete_gpu` → `__GLX_VENDOR_LIBRARY_NAME`
- `enable_logging` → `WINEDEBUG`, `WINE_MONO_TRACE`

### GameMode
**Before**: `LD_PRELOAD=gamemoderun` (incorrect)
**After**: `gamemoderun umu-run <args>` (correct)
**Fallback**: Launches without GameMode if `gamemoderun` not found (warning logged)

### Lossless Scaling
**Before**: `LD_PRELOAD=Lossless.dll` (Windows DLL, wrong)
**After**: Linux-native `LSFG_*` env vars:
- `LSFG_LEGACY=1` (Wine/Proton compatibility)
- `LSFG_MULTIPLIER=<value>`
- `LSFG_PERFORMANCE_MODE=1/0`
- `LSFG_HDR_MODE=1/0`
- `LSFG_FLOW_SCALE=1.0` (only when enabled)

### Config Caching
**Before**: 5+ reads of `config.ini` per launch
**After**: Single read via `AppConfig::load()`, cached for all settings

## Key Files Added/Modified

### New Files
- `src/config/envar.rs` — envar.txt parsing with validation
- `src/bin/faugus-run.rs` — CLI binary with unit-tested arg parsing
- `src/lib.rs` — Library exports for external binaries

### Modified Files
- `src/launcher/game_launcher.rs` — AppConfig structured parsing, envar.txt integration, GameMode/Lossless fixes
- `src/config/paths.rs` — Enabled `envar_txt()` for use
- `src/config/mod.rs` — Exported envar module
- `src/launcher/mod.rs` — Made game_launcher public
- `Cargo.toml` — Added `[[bin]] faugus-run`, `[lib]`, `default-run = "faugus-launcher-rs"`

## Configuration Files (Preserved)

| File | Format | Compatibility | Usage |
|------|--------|----------------|-------|
| `games.json` | JSON | Python→Rust via legacy_compat | Game configurations |
| `config.ini` | INI | Unchanged | Global settings (wayland, HDR, etc.) |
| `envar.txt` | KEY=value | Python-compatible | Global environment variables |
| XDG paths | Standard | Unchanged | `~/.config/faugus-launcher/`, `~/.local/share/faugus-launcher/` |

## Manual Testing

```bash
# Build
cargo build --bin faugus-run

# Test error cases
./target/debug/faugus-run                    # Should show usage (exit 1)
./target/debug/faugus-run --game nonexistent  # Should error "not found" (exit 1)

# Test with real game
cat ~/.config/faugus-launcher/games.json | jq -r '.[].gameid'  # List IDs
./target/debug/faugus-run --game <actual-id>  # Should launch (exit 0)
```

## Deployment

```bash
# Build release
cargo build --release

# Install CLI launcher
sudo install -m 755 target/release/faugus-run /usr/local/bin/

# Install GUI launcher
sudo install -m 755 target/release/faugus-launcher-rs /usr/local/bin/

# Update .desktop files to reference:
Exec=/usr/local/bin/faugus-run --game %u
```

## Packaging Verification (Iteration 3)

### Overview
Automated script to verify that both binaries (`faugus-launcher-rs` and `faugus-run`) are properly included in package distributions before release.

### Verification Script

**Location**: `scripts/verify-package.sh`

**Purpose**:
- Checks both binaries exist in `target/release/`
- Simulates package staging workflow (creates `dist/root/usr/bin/`)
- Validates binaries are included in mock archive
- Shows commands for inspecting actual package files

**Usage**:
```bash
# Build release binaries first
cargo build --release --bins

# Run verification script
bash scripts/verify-package.sh
```

**Script Output**:
- Step 1: Checks binaries in build directory
- Step 2: Simulates package staging (copies binaries to `dist/root/usr/bin/`)
- Step 3: Creates mock archive for inspection
- Step 4: Shows package inspection commands
- Step 5: Validates both binaries are packaged
- Step 6: Summary and next steps

**Exit Codes**:
- `0`: Verification passed (both binaries packaged)
- `1`: Verification failed (missing binary or error)

### Manual Testing Checklist

After running verification script, perform manual end-to-end testing:

1. **Build packages locally**:
   ```bash
   cargo build --release --bins
   # Note: Actual package building handled by CI workflow
   ```

2. **Run verification script**:
   ```bash
   bash scripts/verify-package.sh
   # Expected: ✓ All checks pass
   ```

3. **Install package on test system**:
   ```bash
   # DEB (Debian/Ubuntu)
   sudo dpkg -i faugus-launcher_*.deb

   # RPM (Fedora/openSUSE)
   sudo rpm -i faugus-launcher-*.rpm

   # Arch (pacman)
   sudo pacman -U faugus-launcher-*.pkg.tar.zst
   ```

4. **Verify binaries installed**:
   ```bash
   ls -lh /usr/bin/faugus-launcher-rs /usr/bin/faugus-run
   # Expected: Both binaries present and executable
   ```

5. **Create Steam shortcut via Rust launcher UI**:
   - Launch `faugus-launcher-rs`
   - Add or edit a game
   - Enable "Create Steam shortcut" option
   - Save game configuration

6. **Verify Steam shortcut Exec line**:
   ```bash
   # View generated .desktop file
   cat ~/.local/share/applications/faugus-launcher-*.desktop

   # Check Exec line points to faugus-run:
   Exec=/usr/bin/faugus-run --game <gameid>
   # OR: Exec=/usr/local/bin/faugus-run --game <gameid>
   ```

7. **Launch game from Steam**:
   - Open Steam client
   - Find game in Library
   - Click "Play"
   - **Expected**: Game launches via UMU-Launcher without errors

8. **Verify logs**:
   ```bash
   # Check launcher logs
   cat ~/.local/share/faugus-launcher/logs/<gameid>.log

   # Verify UMU-Launcher was called
   # Verify no Python errors (faugus_run.py should NOT be invoked)
   ```

## Clippy Compliance

- ✅ All clippy warnings addressed
- ✅ No `#[allow(...)]` suppressions
- ✅ Zero `unwrap()`/`panic()` in production paths
- ✅ Uses `anyhow::Result` for error propagation

## Contact

- Project: [Faugus Launcher](https://github.com/0FL01/Faugus-Launcher)
- Issues: Tag with `faugus-run`
