#!/bin/bash
set -e

echo "🚀 JxPoolMiner Installer Builder"
echo "================================"
echo ""

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first."
    exit 1
fi

# Install cargo-bundle if not present
if ! cargo install --list | grep -q "cargo-bundle"; then
    echo "📦 Installing cargo-bundle..."
    cargo install cargo-bundle
fi

# Detect platform
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM="linux";;
    Darwin*)    PLATFORM="macos";;
    *)          PLATFORM="unknown";;
esac

echo "🖥️  Platform: ${PLATFORM}"
echo ""

# Build release binary first
echo "🔨 Building release binary..."
cargo build --release
echo "✅ Binary built successfully"
echo ""

# Create installers based on platform
case "${PLATFORM}" in
    linux)
        echo "🐧 Creating Linux packages..."
        
        # Create .deb package
        if command -v dpkg-deb &> /dev/null; then
            echo "  📦 Building .deb package..."
            
            DEB_DIR="target/debian"
            mkdir -p "${DEB_DIR}/DEBIAN"
            mkdir -p "${DEB_DIR}/usr/local/bin"
            mkdir -p "${DEB_DIR}/usr/share/applications"
            mkdir -p "${DEB_DIR}/etc/jxpoolminer"
            
            # Copy binary
            cp target/release/jxpoolminer "${DEB_DIR}/usr/local/bin/"
            
            # Create control file
            cat > "${DEB_DIR}/DEBIAN/control" << EOF
Package: jxpoolminer
Version: 1.0.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: JxPoolMiner Team <support@jxpoolminer.com>
Description: Professional Cross-Platform Mining Software
 JxPoolMiner is a modern, high-performance cryptocurrency mining
 application with automatic device detection, multi-algorithm support,
 and a beautiful GUI.
EOF
            
            # Create desktop entry
            cat > "${DEB_DIR}/usr/share/applications/jxpoolminer.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=JxPoolMiner
Comment=Professional Cross-Platform Mining Software
Exec=/usr/local/bin/jxpoolminer
Icon=utilities-terminal
Terminal=false
Categories=Utility;System;
EOF
            
            # Build .deb
            dpkg-deb --build "${DEB_DIR}" "target/jxpoolminer_1.0.0_amd64.deb"
            echo "  ✅ .deb package created: target/jxpoolminer_1.0.0_amd64.deb"
        fi
        
        # Create AppImage
        if command -v appimagetool &> /dev/null; then
            echo "  📦 Building AppImage..."
            
            APPDIR="target/JxPoolMiner.AppDir"
            mkdir -p "${APPDIR}/usr/bin"
            mkdir -p "${APPDIR}/usr/share/applications"
            mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"
            
            # Copy binary
            cp target/release/jxpoolminer "${APPDIR}/usr/bin/"
            
            # Create AppRun
            cat > "${APPDIR}/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/jxpoolminer" "$@"
EOF
            chmod +x "${APPDIR}/AppRun"
            
            # Create desktop entry
            cat > "${APPDIR}/jxpoolminer.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=JxPoolMiner
Comment=Professional Cross-Platform Mining Software
Exec=jxpoolminer
Icon=jxpoolminer
Terminal=false
Categories=Utility;System;
EOF
            
            # Build AppImage
            appimagetool "${APPDIR}" "target/JxPoolMiner-1.0.0-x86_64.AppImage"
            echo "  ✅ AppImage created: target/JxPoolMiner-1.0.0-x86_64.AppImage"
        fi
        
        # Create tarball (always available)
        echo "  📦 Building tarball..."
        mkdir -p target/jxpoolminer-1.0.0-linux
        cp target/release/jxpoolminer target/jxpoolminer-1.0.0-linux/
        cp README.md target/jxpoolminer-1.0.0-linux/
        
        cat > target/jxpoolminer-1.0.0-linux/install.sh << 'EOF'
#!/bin/bash
set -e
echo "📦 Installing JxPoolMiner..."
sudo cp jxpoolminer /usr/local/bin/
sudo chmod +x /usr/local/bin/jxpoolminer
echo "✅ Installation complete! Run 'jxpoolminer' to start."
EOF
        chmod +x target/jxpoolminer-1.0.0-linux/install.sh
        
        cd target
        tar -czf jxpoolminer-1.0.0-linux-x86_64.tar.gz jxpoolminer-1.0.0-linux
        cd ..
        echo "  ✅ Tarball created: target/jxpoolminer-1.0.0-linux-x86_64.tar.gz"
        ;;
        
    macos)
        echo "🍎 Creating macOS packages..."
        
        # Create .app bundle
        echo "  📦 Building .app bundle..."
        
        APP_NAME="JxPoolMiner"
        APP_BUNDLE="target/${APP_NAME}.app"
        
        mkdir -p "${APP_BUNDLE}/Contents/MacOS"
        mkdir -p "${APP_BUNDLE}/Contents/Resources"
        
        # Copy binary
        cp target/release/jxpoolminer "${APP_BUNDLE}/Contents/MacOS/"
        
        # Create Info.plist
        cat > "${APP_BUNDLE}/Contents/Info.plist" << EOF
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
    <string>${APP_NAME}</string>
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
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF
        
        echo "  ✅ .app bundle created: ${APP_BUNDLE}"
        
        # Create DMG
        if command -v hdiutil &> /dev/null; then
            echo "  📦 Building DMG..."
            hdiutil create -volname "${APP_NAME}" \
                -srcfolder "${APP_BUNDLE}" \
                -ov -format UDZO \
                "target/${APP_NAME}-1.0.0-macos.dmg"
            echo "  ✅ DMG created: target/${APP_NAME}-1.0.0-macos.dmg"
        fi
        ;;
        
    *)
        echo "❌ Unsupported platform: ${PLATFORM}"
        exit 1
        ;;
esac

echo ""
echo "🎉 Build complete!"
echo ""
echo "📦 Installers created in target/ directory"
echo ""
echo "Installation instructions:"
case "${PLATFORM}" in
    linux)
        echo "  .deb:      sudo dpkg -i target/jxpoolminer_1.0.0_amd64.deb"
        echo "  AppImage:  chmod +x target/JxPoolMiner-1.0.0-x86_64.AppImage && ./target/JxPoolMiner-1.0.0-x86_64.AppImage"
        echo "  Tarball:   tar -xzf target/jxpoolminer-1.0.0-linux-x86_64.tar.gz && cd jxpoolminer-1.0.0-linux && ./install.sh"
        ;;
    macos)
        echo "  DMG:       Open target/JxPoolMiner-1.0.0-macos.dmg and drag to Applications"
        echo "  .app:      Copy target/JxPoolMiner.app to /Applications"
        ;;
esac
