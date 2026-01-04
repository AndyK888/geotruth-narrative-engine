#!/usr/bin/env bash
# =============================================================================
# GeoTruth Monolith Startup Script
# Checks dependencies, sets up the environment, and launches the app.
# =============================================================================

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "$1 could not be found. Please install it."
        exit 1
    fi
}

# 1. Check Prerequisites & Install if Missing

install_rust() {
    log_warn "Rust not found. Installing via rustup..."
    if command -v curl &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # Source env to make rustc available in this script
        source "$HOME/.cargo/env"
    else
        log_error "curl is required to install Rust."
        exit 1
    fi
}

install_node() {
    log_warn "Node.js not found. Attempting to install..."
    if [[ "$OSTYPE" == "darwin"* ]] && command -v brew &> /dev/null; then
        log_info "Installing Node via Homebrew..."
        brew install node
    else
        log_warn "Cannot auto-install Node.js reliably (no Homebrew). Please install Node.js v20+ manually."
        exit 1
    fi
}

install_pnpm() {
    log_warn "pnpm not found. Installing..."
    if command -v npm &> /dev/null; then
        npm install -g pnpm
    else
        log_error "npm is required to install pnpm."
        exit 1
    fi
}

# Check Rust
if ! command -v rustc &> /dev/null; then
    install_rust
else
    log_info "Rust is installed: $(rustc --version)"
fi

# Check Node
if ! command -v node &> /dev/null; then
    install_node
else
    log_info "Node is installed: $(node --version)"
fi

# Check pnpm
if ! command -v pnpm &> /dev/null; then
    install_pnpm
else
    log_info "pnpm is installed: $(pnpm --version)"
fi

# 2. Setup Binaries
log_info "Setting up external binaries..."
chmod +x desktop/scripts/download-binaries.sh
./desktop/scripts/download-binaries.sh

# 3. Install Dependencies & Run
log_info "Installing frontend dependencies..."
cd desktop
pnpm install

log_info "Starting GeoTruth Desktop (Tauri Dev Mode)..."
log_info "Using GEMINI_API_KEY from environment if set."

# Run Tauri
pnpm tauri dev
