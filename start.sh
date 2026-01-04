#!/usr/bin/env bash
# =============================================================================
# GeoTruth Narrative Engine - Startup Script
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# =============================================================================
# Check Prerequisites
# =============================================================================
check_prerequisites() {
    log_step "Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker Desktop."
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        log_error "Docker is not running. Please start Docker Desktop."
        exit 1
    fi
    
    log_info "Docker is ready"
}

# =============================================================================
# Setup Environment
# =============================================================================
setup_env() {
    log_step "Setting up environment..."
    
    if [ ! -f "backend/.env" ]; then
        if [ -f "backend/.env.example" ]; then
            cp backend/.env.example backend/.env
            # Set default password for development
            sed -i '' 's/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=devpassword/' backend/.env 2>/dev/null || \
            sed -i 's/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=devpassword/' backend/.env
            log_info "Created backend/.env with default settings"
        else
            log_warn "No .env.example found, creating minimal .env"
            echo "POSTGRES_PASSWORD=devpassword" > backend/.env
        fi
    else
        log_info "Using existing backend/.env"
    fi
}

# =============================================================================
# Start Backend Services
# =============================================================================
start_backend() {
    log_step "Starting backend services..."
    
    cd backend
    docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
    cd ..
    
    log_info "Waiting for services to be healthy..."
    sleep 5
    
    # Wait for API to be ready
    local max_attempts=30
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        if curl -s http://localhost:8000/v1/health > /dev/null 2>&1; then
            log_info "API is ready!"
            break
        fi
        attempt=$((attempt + 1))
        sleep 1
    done
    
    if [ $attempt -eq $max_attempts ]; then
        log_warn "API not responding yet, but services may still be starting"
    fi
}

# =============================================================================
# Start Desktop App (Optional)
# =============================================================================
start_desktop() {
    if command -v pnpm &> /dev/null && command -v cargo &> /dev/null; then
        log_step "Starting desktop app..."
        cd desktop
        pnpm install --frozen-lockfile 2>/dev/null || pnpm install
        pnpm tauri dev &
        cd ..
        log_info "Desktop app starting..."
    else
        log_warn "Skipping desktop app (requires pnpm and Rust)"
        log_info "To install: brew install pnpm && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
}

# =============================================================================
# Show Status
# =============================================================================
show_status() {
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  GeoTruth Narrative Engine is running!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${BLUE}API:${NC}        http://localhost:8000"
    echo -e "  ${BLUE}API Docs:${NC}   http://localhost:8000/docs"
    echo -e "  ${BLUE}Health:${NC}     http://localhost:8000/v1/health"
    echo ""
    echo -e "  ${BLUE}Database:${NC}   postgres://geotruth@localhost:5432/geotruth"
    echo -e "  ${BLUE}Redis:${NC}      redis://localhost:6379"
    echo ""
    echo -e "  ${YELLOW}To stop:${NC}    ./stop.sh  or  make stop"
    echo ""
}

# =============================================================================
# Main
# =============================================================================
main() {
    echo ""
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         GeoTruth Narrative Engine - Startup                   ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    check_prerequisites
    setup_env
    start_backend
    
    # Use --with-desktop flag to also start desktop app
    if [[ "$1" == "--with-desktop" ]] || [[ "$1" == "-d" ]]; then
        start_desktop
    fi
    
    show_status
}

main "$@"
