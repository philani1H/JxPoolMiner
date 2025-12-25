# JxPoolMiner Installation Guide

This guide covers installation methods for all supported platforms.

## Table of Contents

- [Quick Install](#quick-install)
- [Platform-Specific Instructions](#platform-specific-instructions)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
- [Building from Source](#building-from-source)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

---

## Quick Install

### Linux (One-liner)

```bash
curl -sSL https://github.com/philani1H/JxPoolMiner/releases/latest/download/install.sh | bash
```

### macOS (Homebrew)

```bash
brew tap philani1H/jxpoolminer
brew install jxpoolminer
```

### Windows (Installer)

Download and run: [JxPoolMiner-Setup.exe](https://github.com/philani1H/JxPoolMiner/releases/latest/download/JxPoolMiner-Setup.exe)

---

## Platform-Specific Instructions

### Linux

#### Option 1: Debian/Ubuntu (.deb package)

```bash
# Download the .deb package
wget https://github.com/philani1H/JxPoolMiner/releases/latest/download/jxpoolminer_1.0.0_amd64.deb

# Install
sudo dpkg -i jxpoolminer_1.0.0_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f

# Run
jxpoolminer
```

#### Option 2: AppImage (Universal)

```bash
# Download AppImage
wget https://github.com/philani1H/JxPoolMiner/releases/latest/download/JxPoolMiner-1.0.0-x86_64.AppImage

# Make executable
chmod +x JxPoolMiner-1.0.0-x86_64.AppImage

# Run
./JxPoolMiner-1.0.0-x86_64.AppImage
```

#### Option 3: Tarball (Manual)

```bash
# Download and extract
wget https://github.com/philani1H/JxPoolMiner/releases/latest/download/jxpoolminer-1.0.0-linux-x86_64.tar.gz
tar -xzf jxpoolminer-1.0.0-linux-x86_64.tar.gz
cd jxpoolminer-1.0.0-linux

# Install
./install.sh

# Or install manually
sudo cp jxpoolminer /usr/local/bin/
sudo chmod +x /usr/local/bin/jxpoolminer

# Run
jxpoolminer
```

#### System Requirements (Linux)

- **OS**: Ubuntu 20.04+, Debian 11+, Fedora 35+, or equivalent
- **CPU**: x86_64 processor
- **RAM**: 2GB minimum, 4GB recommended
- **Disk**: 100MB free space
- **Dependencies**: 
  - `libssl` (usually pre-installed)
  - `libgcc` (usually pre-installed)

---

### macOS

#### Option 1: DMG Installer (Recommended)

```bash
# Download DMG
curl -LO https://github.com/philani1H/JxPoolMiner/releases/latest/download/JxPoolMiner-1.0.0-macos.dmg

# Open DMG
open JxPoolMiner-1.0.0-macos.dmg

# Drag JxPoolMiner.app to Applications folder
# Then launch from Applications
```

#### Option 2: Homebrew

```bash
# Add tap
brew tap philani1H/jxpoolminer

# Install
brew install jxpoolminer

# Run
jxpoolminer
```

#### Option 3: Manual Installation

```bash
# Download and extract
curl -LO https://github.com/philani1H/JxPoolMiner/releases/latest/download/jxpoolminer-1.0.0-macos.tar.gz
tar -xzf jxpoolminer-1.0.0-macos.tar.gz

# Install
sudo cp jxpoolminer /usr/local/bin/
sudo chmod +x /usr/local/bin/jxpoolminer

# Run
jxpoolminer
```

#### System Requirements (macOS)

- **OS**: macOS 10.13 (High Sierra) or later
- **CPU**: Intel or Apple Silicon (M1/M2)
- **RAM**: 2GB minimum, 4GB recommended
- **Disk**: 100MB free space

#### First Launch (Security)

On first launch, macOS may block the app. To allow:

1. Go to **System Preferences** → **Security & Privacy**
2. Click **Open Anyway** next to the JxPoolMiner message
3. Confirm by clicking **Open**

Or use Terminal:
```bash
xattr -d com.apple.quarantine /Applications/JxPoolMiner.app
```

---

### Windows

#### Option 1: Installer (Recommended)

1. Download [JxPoolMiner-Setup.exe](https://github.com/philani1H/JxPoolMiner/releases/latest/download/JxPoolMiner-Setup.exe)
2. Run the installer
3. Follow the installation wizard
4. Launch from Start Menu or Desktop shortcut

#### Option 2: Portable ZIP

```powershell
# Download ZIP
Invoke-WebRequest -Uri "https://github.com/philani1H/JxPoolMiner/releases/latest/download/jxpoolminer-1.0.0-windows-x86_64.zip" -OutFile "jxpoolminer.zip"

# Extract
Expand-Archive -Path jxpoolminer.zip -DestinationPath C:\JxPoolMiner

# Run
C:\JxPoolMiner\jxpoolminer.exe
```

#### System Requirements (Windows)

- **OS**: Windows 10 (64-bit) or later
- **CPU**: x86_64 processor
- **RAM**: 2GB minimum, 4GB recommended
- **Disk**: 100MB free space
- **Dependencies**: 
  - Visual C++ Redistributable 2015-2022 (usually pre-installed)

#### Windows Defender

Windows Defender may flag mining software. To allow:

1. Open **Windows Security**
2. Go to **Virus & threat protection**
3. Click **Manage settings**
4. Add exclusion for `C:\Program Files\JxPoolMiner`

---

## Building from Source

### Prerequisites

- **Rust**: 1.70 or higher
- **Git**: For cloning the repository
- **CMake**: For building dependencies
- **OpenSSL**: Development libraries

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Clone and Build

```bash
# Clone repository
git clone https://github.com/philani1H/JxPoolMiner.git
cd JxPoolMiner

# Build release binary
cargo build --release

# Run
./target/release/jxpoolminer
```

### Create Installers

```bash
# Create platform-specific installers
./build-installers.sh

# Or use the packaging script
./package.sh

# Or use Make
make package
```

### Platform-Specific Build Dependencies

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    libssl-dev \
    pkg-config \
    git
```

#### Linux (Fedora)

```bash
sudo dnf install -y \
    gcc \
    cmake \
    openssl-devel \
    pkg-config \
    git
```

#### macOS

```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install cmake openssl pkg-config
```

#### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Install [CMake](https://cmake.org/download/)
3. Install [Git for Windows](https://git-scm.com/download/win)

---

## Configuration

### Configuration File Location

- **Linux**: `~/.config/jxpoolminer/config.toml`
- **macOS**: `~/Library/Application Support/jxpoolminer/config.toml`
- **Windows**: `%APPDATA%\jxpoolminer\config.toml`

### Default Configuration

On first run, a default configuration file is created:

```toml
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
```

### Edit Configuration

**Important**: Update `wallet_address` with your actual wallet address before mining!

```bash
# Linux/macOS
nano ~/.config/jxpoolminer/config.toml

# Windows
notepad %APPDATA%\jxpoolminer\config.toml
```

---

## Troubleshooting

### Device Not Detected

**Problem**: No mining devices found

**Solutions**:
1. Check device drivers are installed
2. For GPUs, install CUDA (NVIDIA) or ROCm (AMD)
3. For ASICs, ensure USB drivers are installed
4. Run with elevated privileges (sudo/admin)

```bash
# Linux - Check devices
lspci | grep -i vga
lsusb

# macOS - Check devices
system_profiler SPDisplaysDataType
```

### Permission Denied

**Problem**: Cannot access devices or config

**Solutions**:

```bash
# Linux - Add user to required groups
sudo usermod -a -G video,render $USER
sudo usermod -a -G dialout $USER  # For ASIC USB devices

# Logout and login for changes to take effect
```

### Pool Connection Failed

**Problem**: Cannot connect to mining pool

**Solutions**:
1. Check internet connection
2. Verify pool URL in config
3. Check firewall settings
4. Try alternative pool (fallback)

```bash
# Test pool connectivity
telnet pool.jxminer.com 3333
# Or
nc -zv pool.jxminer.com 3333
```

### High CPU Usage

**Problem**: Application using too much CPU

**Solutions**:
1. Reduce thread count in config
2. Lower mining intensity
3. Check for background processes

### Application Won't Start

**Problem**: Application crashes on startup

**Solutions**:

```bash
# Run with debug logging
RUST_LOG=debug jxpoolminer

# Check logs
# Linux/macOS
tail -f ~/.local/share/jxpoolminer/logs/jxpoolminer.log

# Windows
type %LOCALAPPDATA%\jxpoolminer\logs\jxpoolminer.log
```

### Missing Dependencies (Linux)

**Problem**: Error about missing libraries

**Solutions**:

```bash
# Ubuntu/Debian
sudo apt-get install -y libssl3 libgcc-s1

# Fedora
sudo dnf install -y openssl-libs libgcc

# Check missing dependencies
ldd /usr/local/bin/jxpoolminer
```

---

## Uninstallation

### Linux

```bash
# If installed via .deb
sudo dpkg -r jxpoolminer

# If installed manually
sudo rm /usr/local/bin/jxpoolminer
rm -rf ~/.config/jxpoolminer
rm -rf ~/.local/share/jxpoolminer

# Remove desktop entry
rm ~/.local/share/applications/jxpoolminer.desktop
```

### macOS

```bash
# If installed via Homebrew
brew uninstall jxpoolminer

# If installed manually
sudo rm /usr/local/bin/jxpoolminer
rm -rf ~/Library/Application\ Support/jxpoolminer

# Remove app bundle
rm -rf /Applications/JxPoolMiner.app
```

### Windows

1. Go to **Settings** → **Apps** → **Apps & features**
2. Find **JxPoolMiner**
3. Click **Uninstall**

Or run the uninstaller:
```
C:\Program Files\JxPoolMiner\Uninstall.exe
```

---

## Getting Help

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/philani1H/JxPoolMiner/issues)
- **Discord**: [Join our Discord](https://discord.gg/jxpoolminer)
- **Email**: support@jxpoolminer.com

---

## Next Steps

After installation:

1. ✅ Edit configuration file with your wallet address
2. ✅ Launch JxPoolMiner
3. ✅ Check detected devices in the Devices tab
4. ✅ Start mining
5. ✅ Monitor performance in Dashboard

Happy mining! ⛏️
