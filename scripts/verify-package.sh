#!/bin/bash
# verify-package.sh - Verify packaging includes both faugus-launcher and faugus-run
# Usage: bash scripts/verify-package.sh

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Binary names
BIN_GUI="faugus-launcher"
BIN_CLI="faugus-run"

# Paths
BUILD_DIR="target/release"
STAGING_DIR="dist/root/usr/bin"
ARTIFACT_DIR="dist"

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Step 1: Verify binaries exist in build
log_info "Step 1: Checking binaries in ${BUILD_DIR}"
if [ ! -f "${BUILD_DIR}/${BIN_GUI}" ]; then
    log_error "${BIN_GUI} not found. Run: cargo build --release --bins"
    exit 1
fi
log_info "✓ Found ${BIN_GUI} ($(stat -c%s "${BUILD_DIR}/${BIN_GUI}") bytes)"

if [ ! -f "${BUILD_DIR}/${BIN_CLI}" ]; then
    log_error "${BIN_CLI} not found. Run: cargo build --release --bins"
    exit 1
fi
log_info "✓ Found ${BIN_CLI} ($(stat -c%s "${BUILD_DIR}/${BIN_CLI}") bytes)"

# Step 2: Simulate package staging
log_info ""
log_info "Step 2: Simulating package staging (like CI workflow does)"
# Clean staging directory first to avoid stale binaries
rm -rf "${ARTIFACT_DIR}"
mkdir -p "${STAGING_DIR}"
cp "${BUILD_DIR}/${BIN_GUI}" "${STAGING_DIR}/"
cp "${BUILD_DIR}/${BIN_CLI}" "${STAGING_DIR}/"
log_info "✓ Staged binaries to ${STAGING_DIR}/"

# Verify staged binaries
log_info "Staged files:"
ls -lh "${STAGING_DIR}/"

# Step 3: Create mock archive for inspection
log_info ""
log_info "Step 3: Creating mock archive for inspection"

# Create tar archive (base for all packages)
cd "${ARTIFACT_DIR}" && tar -czf "faugus-launcher.tar.gz" -C root . && cd - > /dev/null
log_info "✓ Created mock archive: ${ARTIFACT_DIR}/faugus-launcher.tar.gz"

# Step 4: Inspect package contents
log_info ""
log_info "Step 4: Inspecting package contents"

# Show tar contents
log_info "Tar archive contents (simulated):"
if [ -f "${ARTIFACT_DIR}/faugus-launcher.tar.gz" ]; then
    tar -tzf "${ARTIFACT_DIR}/faugus-launcher.tar.gz" || true
fi

# Show commands for deb/rpm inspection (if packages exist)
log_info ""
log_info "Package inspection commands (for actual packages):"
echo "  # DEB package:"
echo "  dpkg-deb -c faugus-launcher_*.deb"
echo "  # RPM package:"
echo "  rpm -qlp faugus-launcher-*.rpm"
echo "  # Arch package:"
echo "  tar -tzf faugus-launcher-*.pkg.tar.zst | grep -E '(faugus-launcher|faugus-run)'"

# Step 5: Validate faugus-run is included
log_info ""
log_info "Step 5: Validating ${BIN_CLI} is packaged"

if [ -f "${ARTIFACT_DIR}/faugus-launcher.tar.gz" ]; then
    ARCHIVE_CONTENTS=$(tar -tzf "${ARTIFACT_DIR}/faugus-launcher.tar.gz")

    if echo "${ARCHIVE_CONTENTS}" | grep -q -F "usr/bin/${BIN_CLI}"; then
        log_info "✓ ${BIN_CLI} is included in package archive"
    else
        log_error "${BIN_CLI} is NOT in package archive!"
        exit 1
    fi

    if echo "${ARCHIVE_CONTENTS}" | grep -q -F "usr/bin/${BIN_GUI}"; then
        log_info "✓ ${BIN_GUI} is included in package archive"
    else
        log_error "${BIN_GUI} is NOT in package archive!"
        exit 1
    fi
fi

# Step 6: Summary
log_info ""
log_info "Step 6: Verification Summary"
log_info "================================"
log_info "Build Directory: ${BUILD_DIR}"
log_info "Staging Directory: ${STAGING_DIR}"
log_info "Mock Archive: ${ARTIFACT_DIR}/faugus-launcher.tar.gz"
log_info ""
log_info "Binaries verified:"
log_info "  ✓ ${BIN_GUI}"
log_info "  ✓ ${BIN_CLI}"
log_info ""
log_info "Next steps for manual testing:"
log_info "  1. Build packages: ./scripts/build-packages.sh"
log_info "  2. Install package: sudo dpkg -i *.deb (or rpm/pacman equivalent)"
log_info "  3. Verify install: ls -lh /usr/bin/faugus-launcher /usr/bin/faugus-run"
log_info "  4. Create Steam shortcut via launcher UI"
log_info "  5. Check Exec line points to faugus-run"
log_info "  6. Launch game from Steam and verify it works"
log_info ""
log_info "✓ Packaging verification complete!"
