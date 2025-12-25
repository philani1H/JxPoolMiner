#!/bin/bash
# JxPoolMiner Quick Installer
set -e

VERSION="1.0.0"
REPO="philani1H/JxPoolMiner"
APP_NAME="jxpoolminer"

echo "╔════════════════════════════════════════╗"
echo "║   JxPoolMiner Installer v${VERSION}      ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Linux*)     PLATFORM="linux";;
    Darwin*)    PLATFORM="macos";;
    *)          
        echo "❌ Unsupported operating system: ${OS}"
        exit 1
        ;;
esac

case "${ARCH}" in
    x86_64|amd64)   ARCH="x86_64";;
    aarch64|arm64)  ARCH="aarch64";;
    *)
        echo "❌ Unsupported architecture: ${ARCH}"
        exit 1
        ;;
esac

echo "🖥️  Platform: ${PLATFORM}-${ARCH}"
echo ""

# Check for required tools
if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
    echo "❌ Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Determine installation directory
if [ "$EUID" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/${APP_NAME}"
else
    INSTALL_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/${APP_NAME}"
fi

echo "📁 Installation directory: ${INSTALL_DIR}"
echo "📁 Configuration directory: ${CONFIG_DIR}"
echo ""

# Create directories
mkdir -p "${INSTALL_DIR}"
mkdir -p "${CONFIG_DIR}"

# Download binary
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${APP_NAME}-${VERSION}-${PLATFORM}-${ARCH}.tar.gz"
TEMP_FILE="/tmp/${APP_NAME}.tar.gz"

echo "⬇️  Downloading JxPoolMiner..."
if command -v curl &> /dev/null; then
    curl -L -o "${TEMP_FILE}" "${DOWNLOAD_URL}" || {
        echo "❌ Download failed. Building from source instead..."
        BUILD_FROM_SOURCE=1
    }
elif command -v wget &> /dev/null; then
    wget -O "${TEMP_FILE}" "${DOWNLOAD_URL}" || {
        echo "❌ Download failed. Building from source instead..."
        BUILD_FROM_SOURCE=1
    }
fi

if [ "${BUILD_FROM_SOURCE}" = "1" ]; then
    echo ""
    echo "🔨 Building from source..."
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        echo "📦 Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Clone and build
    TEMP_DIR="/tmp/jxpoolminer-build"
    rm -rf "${TEMP_DIR}"
    
    echo "📥 Cloning repository..."
    git clone "https://github.com/${REPO}.git" "${TEMP_DIR}"
    cd "${TEMP_DIR}"
    
    echo "🔨 Building release binary..."
    cargo build --release
    
    # Copy binary
    cp "target/release/${APP_NAME}" "${INSTALL_DIR}/"
    
    # Cleanup
    cd -
    rm -rf "${TEMP_DIR}"
else
    # Extract downloaded archive
    echo "📦 Extracting..."
    tar -xzf "${TEMP_FILE}" -C /tmp/
    
    # Copy binary
    cp "/tmp/${APP_NAME}" "${INSTALL_DIR}/"
    
    # Cleanup
    rm -f "${TEMP_FILE}"
    rm -f "/tmp/${APP_NAME}"
fi

# Make executable
chmod +x "${INSTALL_DIR}/${APP_NAME}"

# Create default config if it doesn't exist
if [ ! -f "${CONFIG_DIR}/config.toml" ]; then
    echo "📝 Creating default configuration..."
    cat > "${CONFIG_DIR}/config.toml" << 'EOF'
[app]
theme = "dark"
language = "en"

[mining]
auto_detect_devices = true
auto_assign_algorithms = true

[pool]
primary = "stratum+tcp://pool.jxminer.com:3333"
fallback = "stratum+tcp://backup.jxminer.com:3333"
wallet_address = "your_wallet_address_here"
worker_name = "worker1"
use_tls = false
EOF
fi

# Add to PATH if not already there
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo ""
    echo "⚠️  ${INSTALL_DIR} is not in your PATH"
    echo ""
    echo "Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    echo ""
fi

# Create desktop entry (Linux only)
if [ "${PLATFORM}" = "linux" ] && [ "$EUID" -ne 0 ]; then
    DESKTOP_DIR="$HOME/.local/share/applications"
    mkdir -p "${DESKTOP_DIR}"
    
    cat > "${DESKTOP_DIR}/${APP_NAME}.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=JxPoolMiner
Comment=Professional Cross-Platform Mining Software
Exec=${INSTALL_DIR}/${APP_NAME}
Icon=utilities-terminal
Terminal=false
Categories=Utility;System;
EOF
    
    echo "🖥️  Desktop entry created"
fi

echo ""
echo "╔════════════════════════════════════════╗"
echo "║   ✅ Installation Complete!            ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "📍 Binary installed to: ${INSTALL_DIR}/${APP_NAME}"
echo "📍 Config file: ${CONFIG_DIR}/config.toml"
echo ""
echo "⚠️  IMPORTANT: Edit the config file and set your wallet address!"
echo ""
echo "To start mining:"
echo "  ${APP_NAME}"
echo ""
echo "To edit config:"
if command -v nano &> /dev/null; then
    echo "  nano ${CONFIG_DIR}/config.toml"
elif command -v vim &> /dev/null; then
    echo "  vim ${CONFIG_DIR}/config.toml"
else
    echo "  \$EDITOR ${CONFIG_DIR}/config.toml"
fi
echo ""
echo "For help and documentation:"
echo "  ${APP_NAME} --help"
echo "  https://github.com/${REPO}"
echo ""
