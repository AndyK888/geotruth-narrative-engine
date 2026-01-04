#!/usr/bin/env bash
# =============================================================================
# GeoTruth Binary Download Script
# Downloads platform-specific binaries for FFmpeg, FFprobe, and Whisper
# =============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARIES_DIR="${SCRIPT_DIR}/../binaries"
CACHE_DIR="${HOME}/.cache/geotruth-binaries"

# Versions
FFMPEG_VERSION="7.1"
WHISPER_VERSION="1.7.2"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Detect platform
detect_platform() {
    local os arch
    
    case "$(uname -s)" in
        Darwin*) os="macos" ;;
        Linux*)  os="linux" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *) log_error "Unsupported OS: $(uname -s)"; exit 1 ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *) log_error "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac
    
    echo "${os}-${arch}"
}

# Download file with progress
download() {
    local url="$1"
    local output="$2"
    
    log_info "Downloading: $(basename "$output")"
    
    if command -v curl &> /dev/null; then
        # -f to fail on 404/server errors, -L to follow redirects
        curl -Lf --progress-bar -o "$output" "$url"
    elif command -v wget &> /dev/null; then
        wget --show-progress -q -O "$output" "$url"
    else
        log_error "Neither curl nor wget found"
        exit 1
    fi
}

# Download FFmpeg and FFprobe
download_ffmpeg() {
    local platform="$1"
    local target_dir="$2"
    
    log_info "Downloading FFmpeg for ${platform}..."
    
    local url
    
    case "$platform" in
        macos-x86_64|macos-aarch64)
            # Use static builds from evermeet.cx (MacOS specific)
            
            # 1. FFmpeg
            url="https://evermeet.cx/ffmpeg/ffmpeg-7.1.zip"
            download "$url" "${target_dir}/ffmpeg.zip"
            unzip -o -q "${target_dir}/ffmpeg.zip" -d "${target_dir}"
            rm "${target_dir}/ffmpeg.zip"
            chmod +x "${target_dir}/ffmpeg"
            
            # 2. FFprobe
            url="https://evermeet.cx/ffmpeg/ffprobe-7.1.zip"
            download "$url" "${target_dir}/ffprobe.zip"
            unzip -o -q "${target_dir}/ffprobe.zip" -d "${target_dir}"
            rm "${target_dir}/ffprobe.zip"
            chmod +x "${target_dir}/ffprobe"
            ;;
        linux-x86_64)
            url="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffmpeg-linux-x64"
            download "$url" "${target_dir}/ffmpeg"
            chmod +x "${target_dir}/ffmpeg"
            
            url="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffprobe-linux-x64"
            download "$url" "${target_dir}/ffprobe"
            chmod +x "${target_dir}/ffprobe"
            ;;
        windows-x86_64)
            url="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffmpeg-win32-x64.exe"
            download "$url" "${target_dir}/ffmpeg.exe"
            
            url="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffprobe-win32-x64.exe"
            download "$url" "${target_dir}/ffprobe.exe"
            ;;
        *)
            log_error "FFmpeg not available for ${platform}"
            return 1
            ;;
    esac
    
    log_info "FFmpeg downloaded successfully"
}

# Download Whisper.cpp
download_whisper() {
    local platform="$1"
    local target_dir="$2"
    
    log_info "Downloading Whisper.cpp ${WHISPER_VERSION} for ${platform}..."
    
    # Whisper.cpp needs to be compiled from source or use pre-built binaries
    # For now, we'll create a placeholder and instructions
    
    local whisper_dir="${target_dir}/whisper"
    mkdir -p "$whisper_dir"
    
    cat > "${whisper_dir}/README.md" << 'EOF'
# Whisper.cpp Binary

Whisper.cpp needs to be built for your platform or downloaded from releases.

## Building from source:

```bash
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make

# For GPU acceleration (optional)
make clean
WHISPER_CUBLAS=1 make  # NVIDIA CUDA
# or
WHISPER_METAL=1 make   # Apple Metal
```

## Pre-built binaries

Check releases at: https://github.com/ggerganov/whisper.cpp/releases

## Models

Download the model you want to use:
- tiny.en (75MB) - Fast, English only
- base.en (142MB) - Balanced
- small.en (466MB) - Better quality
- medium.en (1.5GB) - High quality

```bash
./models/download-ggml-model.sh base.en
```
EOF
    
    log_warn "Whisper.cpp requires manual setup - see ${whisper_dir}/README.md"
}

# Main function
main() {
    local platform
    platform=$(detect_platform)
    
    log_info "Detected platform: ${platform}"
    log_info "Binaries directory: ${BINARIES_DIR}"
    
    # Create directories
    mkdir -p "$BINARIES_DIR"
    mkdir -p "$CACHE_DIR"
    
    # Download binaries
    download_ffmpeg "$platform" "$BINARIES_DIR"
    download_whisper "$platform" "$BINARIES_DIR"
    
    log_info "Binary setup complete!"
    log_info ""
    log_info "Installed binaries:"
    ls -la "$BINARIES_DIR" || true
}

# Run main if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
