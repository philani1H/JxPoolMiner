#!/bin/bash
set -e

VERSION="1.0.0"
APP_NAME="JxPoolMiner"
BINARY_NAME="jxpoolminer"

echo "🚀 Building ${APP_NAME} v${VERSION}..."

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    MINGW*|MSYS*|CYGWIN*) PLATFORM=windows;;
    *)          PLATFORM="unknown";;
esac

echo "📦 Platform detected: ${PLATFORM}"

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Create dist directory
DIST_DIR="dist/${PLATFORM}"
mkdir -p "${DIST_DIR}"

# Copy binary
if [ "${PLATFORM}" = "windows" ]; then
    cp "target/release/${BINARY_NAME}.exe" "${DIST_DIR}/"
    BINARY_PATH="${DIST_DIR}/${BINARY_NAME}.exe"
else
    cp "target/release/${BINARY_NAME}" "${DIST_DIR}/"
    BINARY_PATH="${DIST_DIR}/${BINARY_NAME}"
    chmod +x "${BINARY_PATH}"
fi

# Create config directory
mkdir -p "${DIST_DIR}/config"

# Copy default config
cat > "${DIST_DIR}/config/default.toml" << 'EOF'
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

# Copy README
cp README.md "${DIST_DIR}/"

# Create LICENSE file
cat > "${DIST_DIR}/LICENSE" << 'EOF'
MIT License

Copyright (c) 2024 JxPoolMiner Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

# Platform-specific packaging
case "${PLATFORM}" in
    linux)
        echo "🐧 Creating Linux package..."
        
        # Create install script
        cat > "${DIST_DIR}/install.sh" << 'INSTALL_EOF'
#!/bin/bash
set -e

echo "📦 Installing JxPoolMiner..."

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/jxpoolminer"
else
    INSTALL_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/jxpoolminer"
fi

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"

# Copy binary
cp jxpoolminer "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/jxpoolminer"

# Copy config
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    cp config/default.toml "$CONFIG_DIR/config.toml"
    echo "📝 Default config created at: $CONFIG_DIR/config.toml"
fi

# Create desktop entry
if [ "$EUID" -ne 0 ]; then
    DESKTOP_DIR="$HOME/.local/share/applications"
    mkdir -p "$DESKTOP_DIR"
    
    cat > "$DESKTOP_DIR/jxpoolminer.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=JxPoolMiner
Comment=Professional Cross-Platform Mining Software
Exec=$INSTALL_DIR/jxpoolminer
Icon=utilities-terminal
Terminal=false
Categories=Utility;
EOF
    
    echo "🖥️  Desktop entry created"
fi

echo "✅ Installation complete!"
echo ""
echo "To start mining, run: jxpoolminer"
echo "Config file: $CONFIG_DIR/config.toml"
INSTALL_EOF
        
        chmod +x "${DIST_DIR}/install.sh"
        
        # Create tarball
        cd dist
        tar -czf "${APP_NAME}-${VERSION}-linux-x86_64.tar.gz" "${PLATFORM}"
        cd ..
        
        echo "✅ Linux package created: dist/${APP_NAME}-${VERSION}-linux-x86_64.tar.gz"
        ;;
        
    macos)
        echo "🍎 Creating macOS package..."
        
        # Create app bundle structure
        APP_BUNDLE="${DIST_DIR}/${APP_NAME}.app"
        mkdir -p "${APP_BUNDLE}/Contents/MacOS"
        mkdir -p "${APP_BUNDLE}/Contents/Resources"
        
        # Move binary
        mv "${BINARY_PATH}" "${APP_BUNDLE}/Contents/MacOS/${BINARY_NAME}"
        
        # Create Info.plist
        cat > "${APP_BUNDLE}/Contents/Info.plist" << 'PLIST_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>jxpoolminer</string>
    <key>CFBundleIdentifier</key>
    <string>com.jxpoolminer.app</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>JxPoolMiner</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
PLIST_EOF
        
        # Create DMG
        cd dist
        hdiutil create -volname "${APP_NAME}" -srcfolder "${PLATFORM}/${APP_NAME}.app" -ov -format UDZO "${APP_NAME}-${VERSION}-macos.dmg"
        cd ..
        
        echo "✅ macOS package created: dist/${APP_NAME}-${VERSION}-macos.dmg"
        ;;
        
    windows)
        echo "🪟 Creating Windows package..."
        
        # Create installer script (NSIS)
        cat > "${DIST_DIR}/installer.nsi" << 'NSI_EOF'
!define APP_NAME "JxPoolMiner"
!define APP_VERSION "1.0.0"
!define APP_PUBLISHER "JxPoolMiner Team"
!define APP_EXE "jxpoolminer.exe"

Name "${APP_NAME}"
OutFile "JxPoolMiner-Setup.exe"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"
    SetOutPath "$INSTDIR"
    File "jxpoolminer.exe"
    File "README.md"
    File "LICENSE"
    
    SetOutPath "$INSTDIR\config"
    File "config\default.toml"
    
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
    CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
    
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\jxpoolminer.exe"
    Delete "$INSTDIR\README.md"
    Delete "$INSTDIR\LICENSE"
    Delete "$INSTDIR\config\default.toml"
    Delete "$INSTDIR\Uninstall.exe"
    
    RMDir "$INSTDIR\config"
    RMDir "$INSTDIR"
    
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
    Delete "$DESKTOP\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
SectionEnd
NSI_EOF
        
        # Create zip archive
        cd dist
        if command -v zip &> /dev/null; then
            zip -r "${APP_NAME}-${VERSION}-windows-x86_64.zip" "${PLATFORM}"
        else
            echo "⚠️  zip not found, skipping archive creation"
        fi
        cd ..
        
        echo "✅ Windows package created: dist/${PLATFORM}/"
        echo "ℹ️  To create installer, run: makensis dist/${PLATFORM}/installer.nsi"
        ;;
esac

echo ""
echo "🎉 Build complete!"
echo "📦 Package location: ${DIST_DIR}"
echo ""
echo "To install:"
case "${PLATFORM}" in
    linux)
        echo "  cd ${DIST_DIR} && ./install.sh"
        ;;
    macos)
        echo "  Open dist/${APP_NAME}-${VERSION}-macos.dmg"
        ;;
    windows)
        echo "  Run dist/${PLATFORM}/${BINARY_NAME}.exe"
        ;;
esac
