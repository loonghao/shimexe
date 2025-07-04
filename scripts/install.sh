#!/usr/bin/env bash
# shimexe installer script for Unix-like systems (macOS, Linux)
# Usage: curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh
# Usage with version: SHIMEXE_VERSION="0.3.0" curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh

set -euo pipefail

# Default values
SHIMEXE_VERSION="${SHIMEXE_VERSION:-latest}"
SHIMEXE_INSTALL_DIR="${SHIMEXE_INSTALL_DIR:-$HOME/.local/bin}"
SHIMEXE_REPO="loonghao/shimexe"
SHIMEXE_BASE_URL="https://github.com/${SHIMEXE_REPO}/releases"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Detect platform
detect_platform() {
    local os
    local arch

    case "$(uname -s)" in
        Linux*)     os="unknown-linux-gnu" ;;
        Darwin*)    os="apple-darwin" ;;
        *)          error "Unsupported operating system: $(uname -s)"; exit 1 ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        aarch64|arm64)  arch="aarch64" ;;
        *)              error "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac

    echo "${arch}-${os}"
}

# Map platform to release file naming convention
get_release_filename() {
    local platform="$1"

    case "$platform" in
        "x86_64-unknown-linux-gnu")    echo "shimexe-Linux-gnu-x86_64.tar.gz" ;;
        "aarch64-unknown-linux-gnu")   echo "shimexe-Linux-gnu-arm64.tar.gz" ;;
        "x86_64-unknown-linux-musl")   echo "shimexe-Linux-musl-x86_64.tar.gz" ;;
        "aarch64-unknown-linux-musl")  echo "shimexe-Linux-musl-arm64.tar.gz" ;;
        "x86_64-apple-darwin")         echo "shimexe-macOS-x86_64.tar.gz" ;;
        "aarch64-apple-darwin")        echo "shimexe-macOS-arm64.tar.gz" ;;
        "x86_64-unknown-freebsd")      echo "shimexe-FreeBSD-x86_64.tar.gz" ;;
        *)                             echo "shimexe-${platform}.tar.gz" ;;
    esac
}

# Get latest shimexe version from GitHub API with retry and fallback
get_latest_version() {
    local api_url="https://api.github.com/repos/${SHIMEXE_REPO}/releases"
    local max_retries=3
    local retry_delay=2
    local version=""

    # Try API with retries
    for i in $(seq 1 $max_retries); do
        info "Attempting to get latest shimexe version (attempt $i/$max_retries)..."

        if command -v curl >/dev/null 2>&1; then
            # Get all releases, filter for both v* and shimexe-v* patterns, and sort by version
            version=$(curl -s -H "User-Agent: shimexe-installer/1.0" -H "Accept: application/vnd.github.v3+json" --connect-timeout 10 "$api_url" 2>/dev/null | grep '"tag_name":' | grep -E '"(shimexe-)?v[0-9]+\.[0-9]+\.[0-9]+"' | sed -E 's/.*"(shimexe-)?v([^"]+)".*/\2/' | sort -V | tail -1 || true)
        elif command -v wget >/dev/null 2>&1; then
            # Get all releases, filter for both v* and shimexe-v* patterns, and sort by version
            version=$(wget -qO- --timeout=10 --header="User-Agent: shimexe-installer/1.0" --header="Accept: application/vnd.github.v3+json" "$api_url" 2>/dev/null | grep '"tag_name":' | grep -E '"(shimexe-)?v[0-9]+\.[0-9]+\.[0-9]+"' | sed -E 's/.*"(shimexe-)?v([^"]+)".*/\2/' | sort -V | tail -1 || true)
        else
            error "Neither curl nor wget is available"
            exit 1
        fi

        if [ -n "$version" ]; then
            info "Found latest shimexe version: v$version"
            echo "$version"
            return 0
        fi

        warn "Attempt $i failed"
        if [ $i -lt $max_retries ]; then
            info "Retrying in $retry_delay seconds..."
            sleep $retry_delay
            retry_delay=$((retry_delay * 2))  # Exponential backoff
        fi
    done

    # Fallback: try to get version from releases page HTML
    warn "API failed, trying fallback method..."
    local releases_url="https://github.com/${SHIMEXE_REPO}/releases"

    if command -v curl >/dev/null 2>&1; then
        # Look for both v* and shimexe-v* patterns and get the highest version
        version=$(curl -s -L --connect-timeout 10 "$releases_url" 2>/dev/null | grep -o 'releases/tag/\(shimexe-\)\?v[0-9]\+\.[0-9]\+\.[0-9]\+' | sed -E 's/.*\(shimexe-\)?v([0-9]+\.[0-9]+\.[0-9]+).*/\1/' | sort -V | tail -1 || true)
    elif command -v wget >/dev/null 2>&1; then
        # Look for both v* and shimexe-v* patterns and get the highest version
        version=$(wget -qO- --timeout=10 "$releases_url" 2>/dev/null | grep -o 'releases/tag/\(shimexe-\)\?v[0-9]\+\.[0-9]\+\.[0-9]\+' | sed -E 's/.*\(shimexe-\)?v([0-9]+\.[0-9]+\.[0-9]+).*/\1/' | sort -V | tail -1 || true)
    fi

    if [ -n "$version" ]; then
        info "Found shimexe version via fallback: v$version"
        echo "$version"
        return 0
    fi

    error "Failed to get latest version after all attempts. Please specify a version manually:"
    error "Example: SHIMEXE_VERSION='0.3.1' curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh"
    exit 1
}

# Download and install shimexe
install_shimexe() {
    local platform
    local version
    local download_url
    local temp_dir
    local archive_name
    
    platform=$(detect_platform)
    
    if [ "$SHIMEXE_VERSION" = "latest" ]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
        if [ -z "$version" ]; then
            error "Failed to get latest version"
            exit 1
        fi
    else
        version="$SHIMEXE_VERSION"
    fi
    
    info "Installing shimexe v${version} for ${platform}..."

    # Construct download URL using correct release file naming
    archive_name=$(get_release_filename "$platform")
    download_url="${SHIMEXE_BASE_URL}/download/v${version}/${archive_name}"
    
    # Create temporary directory
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    # Download
    info "Downloading from ${download_url}..."
    if command -v curl >/dev/null 2>&1; then
        curl -L --fail --progress-bar "$download_url" -o "$temp_dir/$archive_name"
    elif command -v wget >/dev/null 2>&1; then
        wget --progress=bar:force:noscroll "$download_url" -O "$temp_dir/$archive_name"
    else
        error "Neither curl nor wget is available"
        exit 1
    fi
    
    # Extract
    info "Extracting to ${SHIMEXE_INSTALL_DIR}..."
    mkdir -p "$SHIMEXE_INSTALL_DIR"
    tar -xzf "$temp_dir/$archive_name" -C "$temp_dir"
    
    # Find and copy the binary
    local binary_path
    binary_path=$(find "$temp_dir" -name "shimexe" -type f | head -1)
    
    if [ -z "$binary_path" ]; then
        error "shimexe binary not found in archive"
        exit 1
    fi
    
    cp "$binary_path" "$SHIMEXE_INSTALL_DIR/shimexe"
    chmod +x "$SHIMEXE_INSTALL_DIR/shimexe"
    
    success "shimexe v${version} installed to ${SHIMEXE_INSTALL_DIR}/shimexe"
    
    # Check if install directory is in PATH
    if ! echo "$PATH" | grep -q "$SHIMEXE_INSTALL_DIR"; then
        warn "Add ${SHIMEXE_INSTALL_DIR} to your PATH to use shimexe from anywhere:"
        echo "  export PATH=\"\$PATH:${SHIMEXE_INSTALL_DIR}\""
        echo ""
        echo "Or add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.)"
    fi
    
    # Verify installation
    if "$SHIMEXE_INSTALL_DIR/shimexe" --version >/dev/null 2>&1; then
        success "Installation verified successfully!"
        echo ""
        echo "Get started:"
        echo "  shimexe --help"
        echo "  shimexe init --examples"
        echo ""
        echo "Add your first shim:"
        echo "  # Option 1: Auto-configure PATH (recommended)"
        echo "  shimexe add my-tool --path /path/to/tool --add-system-path"
        echo "  my-tool --version  # Use directly"
        echo ""
        echo "  # Option 2: Use shimexe run (no PATH setup)"
        echo "  shimexe add my-tool --path /path/to/tool"
        echo "  shimexe run my-tool --version  # Use via shimexe"
        echo ""
        echo "Default shim directory: ~/.shimexe"
    else
        error "Installation verification failed"
        exit 1
    fi
}

# Main execution
main() {
    info "shimexe installer"
    echo ""
    
    # Check for required tools
    if ! command -v tar >/dev/null 2>&1; then
        error "tar is required but not installed"
        exit 1
    fi
    
    install_shimexe
}

# Run main function
main "$@"
