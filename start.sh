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
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# =============================================================================
# Check & Install Prerequisites
# =============================================================================
check_docker() {
    log_step "Checking Docker..."
    
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed."
        echo -e "  Please install Docker Desktop from: ${CYAN}https://docker.com/products/docker-desktop${NC}"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        log_error "Docker is not running. Please start Docker Desktop."
        exit 1
    fi
    
    log_info "Docker is ready"
}

check_node() {
    log_step "Checking Node.js..."
    
    if ! command -v node &> /dev/null; then
        log_warn "Node.js not found. Installing via Homebrew..."
        if command -v brew &> /dev/null; then
            brew install node
        else
            log_error "Please install Node.js from: https://nodejs.org"
            exit 1
        fi
    fi
    
    log_info "Node.js $(node --version) is ready"
}

check_pnpm() {
    log_step "Checking pnpm..."
    
    if ! command -v pnpm &> /dev/null; then
        log_warn "pnpm not found. Installing..."
        npm install -g pnpm
    fi
    
    log_info "pnpm $(pnpm --version) is ready"
}

check_rust() {
    log_step "Checking Rust..."
    
    if ! command -v cargo &> /dev/null; then
        log_warn "Rust not found."
        echo ""
        echo -e "${YELLOW}Rust is required for the native desktop app.${NC}"
        echo -e "You can either:"
        echo -e "  1) Install Rust now (recommended, takes ~1 min)"
        echo -e "  2) Skip and run web-only mode"
        echo ""
        read -p "Install Rust? [Y/n] " -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[Nn]$ ]]; then
            return 1
        fi
        
        log_info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        
        if ! command -v cargo &> /dev/null; then
            log_error "Rust installation failed"
            return 1
        fi
    fi
    
    log_info "Rust $(cargo --version | cut -d' ' -f2) is ready"
    return 0
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
            if [[ "$OSTYPE" == "darwin"* ]]; then
                sed -i '' 's/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=devpassword/' backend/.env 2>/dev/null || true
            else
                sed -i 's/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=devpassword/' backend/.env 2>/dev/null || true
            fi
            log_info "Created backend/.env with default settings"
        else
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
    docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build
    cd ..
    
    log_info "Waiting for services to be healthy..."
    
    # Wait for API to be ready
    local max_attempts=30
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        if curl -s http://localhost:8000/v1/health > /dev/null 2>&1; then
            log_info "Backend API is ready!"
            return 0
        fi
        attempt=$((attempt + 1))
        sleep 1
        echo -n "."
    done
    echo ""
    
    log_warn "API not responding yet, but services may still be starting"
}

# =============================================================================
# Install Frontend Dependencies
# =============================================================================
install_frontend() {
    log_step "Installing frontend dependencies..."
    
    cd desktop
    pnpm install --frozen-lockfile 2>/dev/null || pnpm install
    cd ..
    
    log_info "Frontend dependencies installed"
}

# =============================================================================
# Start Desktop App
# =============================================================================
start_desktop_native() {
    log_step "Starting native desktop app..."
    
    cd desktop
    pnpm tauri dev &
    DESKTOP_PID=$!
    cd ..
    
    log_info "Desktop app starting (PID: $DESKTOP_PID)"
}

start_desktop_web() {
    log_step "Starting web-only mode..."
    
    cd desktop
    pnpm dev &
    DESKTOP_PID=$!
    cd ..
    
    log_info "Web app starting at http://localhost:5173"
}

# =============================================================================
# Show Status
# =============================================================================
show_status() {
    local mode=$1
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  GeoTruth Narrative Engine is running!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${BLUE}Backend API:${NC}    http://localhost:8000"
    echo -e "  ${BLUE}API Docs:${NC}       http://localhost:8000/docs"
    echo -e "  ${BLUE}Health Check:${NC}   http://localhost:8000/v1/health"
    echo ""
    
    if [ "$mode" == "native" ]; then
        echo -e "  ${BLUE}Desktop App:${NC}    Running as native Tauri app"
    elif [ "$mode" == "web" ]; then
        echo -e "  ${BLUE}Frontend:${NC}       http://localhost:5173"
    fi
    
    echo ""
    echo -e "  ${YELLOW}To stop:${NC}        ./stop.sh"
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
    
    # Parse arguments
    MODE="auto"
    while [[ $# -gt 0 ]]; do
        case $1 in
            --backend-only|-b)
                MODE="backend"
                shift
                ;;
            --web|-w)
                MODE="web"
                shift
                ;;
            --native|-n)
                MODE="native"
                shift
                ;;
            *)
                shift
                ;;
        esac
    done
    
    # Check prerequisites
    check_docker
    check_node
    check_pnpm
    setup_env
    
    # Start backend
    start_backend
    
    # Handle frontend based on mode
    if [ "$MODE" == "backend" ]; then
        show_status "backend"
        return
    fi
    
    install_frontend
    
    if [ "$MODE" == "web" ]; then
        start_desktop_web
        show_status "web"
    elif [ "$MODE" == "native" ]; then
        if check_rust; then
            start_desktop_native
            show_status "native"
        else
            log_warn "Falling back to web mode"
            start_desktop_web
            show_status "web"
        fi
    else
        # Auto mode - try native, fall back to web
        if check_rust; then
            start_desktop_native
            show_status "native"
        else
            start_desktop_web
            show_status "web"
        fi
    fi
}

main "$@"
