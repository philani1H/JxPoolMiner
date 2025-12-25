# Building JxPoolMiner

This guide explains how to build JxPoolMiner from source and create installable packages.

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Building](#building)
- [Creating Installers](#creating-installers)
- [Cross-Compilation](#cross-compilation)
- [Development](#development)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

```bash
# Clone the repository
git clone https://github.com/philani1H/JxPoolMiner.git
cd JxPoolMiner

# Build release binary
make release

# Or use cargo directly
cargo build --release

# Run
./target/release/jxpoolminer
```

---

## Prerequisites

### Required Tools

- **Rust**: 1.70 or higher
- **Cargo**: Comes with Rust
- **Git**: For cloning the repository

### Platform-Specific Dependencies

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

#### Linux (Fedora/RHEL)

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    cmake \
    openssl-devel \
    pkg-config \
    git
```

#### Linux (Arch)

```bash
sudo pacman -S \
    base-devel \
    cmake \
    openssl \
    pkg-config \
    git
```

#### macOS

```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install cmake openssl pkg-config git
```

#### Windows

1. Install [Visual Studio Build Tools 2022](https://visualstudio.microsoft.com/downloads/)
   - Select "Desktop development with C++"
2. Install [CMake](https://cmake.org/download/)
3. Install [Git for Windows](https://git-scm.com/download/win)
4. Install [Rust](https://rustup.rs/)

### Installing Rust

If you don't have Rust installed:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows (PowerShell)
# Download and run: https://win.rustup.rs/
```

Verify installation:

```bash
rustc --version
cargo --version
```

---

## Building

### Debug Build

For development with debug symbols:

```bash
cargo build
# Or
make build
```

Binary location: `target/debug/jxpoolminer`

### Release Build

Optimized build for production:

```bash
cargo build --release
# Or
make release
```

Binary location: `target/release/jxpoolminer`

### Build Options

#### Optimize for Size

```bash
cargo build --release --config profile.release.opt-level='"z"'
```

#### Optimize for Speed

```bash
cargo build --release --config profile.release.opt-level=3
```

#### Enable LTO (Link-Time Optimization)

Already enabled in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

---

## Creating Installers

### Using the Package Script

The `package.sh` script creates platform-specific packages:

```bash
./package.sh
```

This creates:
- **Linux**: `.tar.gz` archive with install script
- **macOS**: `.app` bundle and `.dmg` installer
- **Windows**: `.zip` archive and NSIS installer script

### Using the Installer Builder

For more advanced installers:

```bash
./build-installers.sh
```

This creates:
- **Linux**: `.deb`, AppImage, and `.tar.gz`
- **macOS**: `.app` bundle and `.dmg`
- **Windows**: Portable `.zip`

### Using Make

```bash
make package
```

### Manual Packaging

#### Linux Tarball

```bash
cargo build --release
mkdir -p dist/jxpoolminer-1.0.0-linux
cp target/release/jxpoolminer dist/jxpoolminer-1.0.0-linux/
cp README.md LICENSE dist/jxpoolminer-1.0.0-linux/
cd dist
tar -czf jxpoolminer-1.0.0-linux-x86_64.tar.gz jxpoolminer-1.0.0-linux
```

#### macOS App Bundle

```bash
cargo build --release
mkdir -p "JxPoolMiner.app/Contents/MacOS"
cp target/release/jxpoolminer "JxPoolMiner.app/Contents/MacOS/"
# Create Info.plist (see package.sh for template)
```

#### Windows Zip

```bash
cargo build --release
mkdir dist
cp target/release/jxpoolminer.exe dist/
cd dist
zip -r jxpoolminer-windows.zip jxpoolminer.exe
```

---

## Cross-Compilation

### Linux to Windows

```bash
# Install cross-compilation tools
sudo apt-get install -y mingw-w64
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### macOS to Linux

```bash
# Install cross-compilation tools
brew install filosottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

### Using Cross

For easier cross-compilation:

```bash
# Install cross
cargo install cross

# Build for different targets
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target aarch64-unknown-linux-gnu
```

### Supported Targets

- `x86_64-unknown-linux-gnu` - Linux (glibc)
- `x86_64-unknown-linux-musl` - Linux (musl, static)
- `x86_64-apple-darwin` - macOS (Intel)
- `aarch64-apple-darwin` - macOS (Apple Silicon)
- `x86_64-pc-windows-msvc` - Windows (MSVC)
- `x86_64-pc-windows-gnu` - Windows (MinGW)

---

## Development

### Project Structure

```
JxPoolMiner/
├── src/                    # Main application
│   └── main.rs
├── crates/                 # Workspace crates
│   ├── core/              # Core types and traits
│   ├── devices/           # Device detection
│   ├── mining/            # Mining algorithms
│   ├── pool/              # Pool connection
│   ├── gui/               # GUI implementation
│   ├── config/            # Configuration
│   ├── stats/             # Statistics
│   └── updater/           # Auto-updater
├── tests/                 # Integration tests
├── Cargo.toml             # Workspace manifest
└── build.rs               # Build script
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p jxpoolminer-core

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint with Clippy
cargo clippy

# Clippy with strict warnings
cargo clippy -- -D warnings

# Check without building
cargo check
```

### Documentation

```bash
# Build documentation
cargo doc

# Build and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench --bench mining_benchmark
```

### Profiling

#### Linux (perf)

```bash
cargo build --release
perf record --call-graph=dwarf ./target/release/jxpoolminer
perf report
```

#### macOS (Instruments)

```bash
cargo build --release
instruments -t "Time Profiler" ./target/release/jxpoolminer
```

#### Flamegraph

```bash
cargo install flamegraph
cargo flamegraph
```

---

## Troubleshooting

### Build Errors

#### OpenSSL Not Found (Linux)

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev

# Fedora
sudo dnf install openssl-devel

# Or use vendored OpenSSL
cargo build --features vendored-openssl
```

#### CMake Not Found

```bash
# Ubuntu/Debian
sudo apt-get install cmake

# macOS
brew install cmake

# Windows
# Download from https://cmake.org/download/
```

#### Linker Errors (Windows)

Make sure Visual Studio Build Tools are installed with C++ support.

#### Out of Memory

```bash
# Reduce parallel jobs
cargo build --release -j 2

# Or set in config
export CARGO_BUILD_JOBS=2
```

### Performance Issues

#### Slow Compilation

```bash
# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache

# Use mold linker (Linux)
sudo apt-get install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
```

#### Large Binary Size

```bash
# Strip symbols
strip target/release/jxpoolminer

# Or enable in Cargo.toml
[profile.release]
strip = true

# Use UPX compression
upx --best --lzma target/release/jxpoolminer
```

### Runtime Issues

#### Missing Shared Libraries (Linux)

```bash
# Check dependencies
ldd target/release/jxpoolminer

# Build with static linking
cargo build --release --target x86_64-unknown-linux-musl
```

#### Permission Denied

```bash
# Make executable
chmod +x target/release/jxpoolminer

# Or run with sudo for device access
sudo ./target/release/jxpoolminer
```

---

## CI/CD

### GitHub Actions

The project includes GitHub Actions workflows:

- `.github/workflows/ci.yml` - Continuous Integration
- `.github/workflows/release.yml` - Release builds

### Creating a Release

```bash
# Tag the release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# GitHub Actions will automatically:
# 1. Build for all platforms
# 2. Run tests
# 3. Create release
# 4. Upload binaries
```

### Local Release Build

```bash
# Build for all targets
./build-installers.sh

# Or manually
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

---

## Advanced Topics

### Custom Features

Enable/disable features:

```bash
# Build with specific features
cargo build --release --features "gui,updater"

# Build without default features
cargo build --release --no-default-features
```

### Optimization Profiles

Create custom profiles in `Cargo.toml`:

```toml
[profile.production]
inherits = "release"
lto = "fat"
codegen-units = 1
opt-level = 3
strip = true
panic = "abort"
```

Build with custom profile:

```bash
cargo build --profile production
```

### Conditional Compilation

Use platform-specific code:

```rust
#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux-specific code
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows-specific code
}
```

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/philani1H/JxPoolMiner/issues)
- **Discussions**: [GitHub Discussions](https://github.com/philani1H/JxPoolMiner/discussions)
- **Discord**: [Join our Discord](https://discord.gg/jxpoolminer)

---

Happy building! 🔨
