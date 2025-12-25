# Changelog

All notable changes to JxPoolMiner will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive build system with cross-platform support
- Automated installer creation for Linux, macOS, and Windows
- `package.sh` script for creating platform-specific packages
- `build-installers.sh` script for advanced installer creation
- `install.sh` one-liner installation script
- Makefile for simplified build commands
- GitHub Actions workflows for CI/CD
- Comprehensive documentation:
  - `INSTALL.md` - Detailed installation guide
  - `BUILD.md` - Build and development guide
  - `CHANGELOG.md` - This file
- Desktop integration (Linux .desktop file, macOS .app bundle)
- Configuration file persistence with proper loading/saving
- Empty device detection with helpful error message

### Fixed
- **CRITICAL**: Configuration file now properly loads from disk instead of always using defaults
- **CRITICAL**: Fixed type mismatch between `jxpoolminer_config::PoolConfig` and `jxpoolminer_pool::PoolConfig`
- **CRITICAL**: Added `use_tls` field to config and properly pass it to pool client
- **HIGH**: Application now checks for empty device list and exits with helpful message
- Build warnings in `build.rs` (removed unused import)

### Changed
- Configuration now saves to platform-specific directories:
  - Linux: `~/.config/jxpoolminer/config.toml`
  - macOS: `~/Library/Application Support/jxpoolminer/config.toml`
  - Windows: `%APPDATA%\jxpoolminer\config.toml`
- Updated README with improved installation and build instructions
- Enhanced error messages for better user experience

### Security
- Configuration files now created with appropriate permissions
- Sensitive data (wallet addresses) stored in user-specific directories

## [1.0.0] - 2024-12-25

### Added
- Initial release
- Automatic device detection (ASIC, GPU, CPU)
- Multi-algorithm support (SHA-256, Ethash, GXHash)
- Modern GUI framework integration
- Pool connection with Stratum protocol support
- Real-time statistics collection
- Auto-update system
- Modular crate architecture:
  - `jxpoolminer-core` - Core types and traits
  - `jxpoolminer-devices` - Device detection
  - `jxpoolminer-mining` - Mining algorithms
  - `jxpoolminer-pool` - Pool connection
  - `jxpoolminer-gui` - GUI implementation
  - `jxpoolminer-config` - Configuration management
  - `jxpoolminer-stats` - Statistics collection
  - `jxpoolminer-updater` - Auto-updater

### Known Issues
- Mining engine stores jobs but doesn't spawn actual mining tasks (placeholder implementation)
- Pool client is simulated and doesn't make real network connections
- GUI is a placeholder and doesn't render actual interface
- GXHash algorithm implementation uses SHA-256 instead of actual GXHash
- Device status is never updated after initialization
- No cancellation mechanism for mining loops
- Stratum protocol module is empty

## Future Releases

### [1.1.0] - Planned
- Implement actual mining task execution
- Real Stratum V1 protocol implementation
- Functional GUI with egui/Iced
- Device status updates and monitoring
- Proper mining cancellation and job switching
- Temperature and power monitoring
- Performance optimizations

### [1.2.0] - Planned
- Stratum V2 protocol support
- GPU mining support (CUDA/OpenCL)
- ASIC miner support
- Advanced statistics and charts
- Export functionality (CSV/JSON)

### [2.0.0] - Future
- AI-based algorithm optimization
- Multi-pool mining
- Mobile app for monitoring
- Cloud sync
- Plugin system
- Custom algorithm support
- Distributed mining
- Advanced analytics

---

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit changes: `git commit -am "Release v1.x.x"`
4. Create tag: `git tag -a v1.x.x -m "Release v1.x.x"`
5. Push: `git push origin main --tags`
6. GitHub Actions will automatically build and create release

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to JxPoolMiner.

---

[Unreleased]: https://github.com/philani1H/JxPoolMiner/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/philani1H/JxPoolMiner/releases/tag/v1.0.0
