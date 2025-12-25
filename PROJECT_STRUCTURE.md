# JxPoolMiner - Professional Project Structure

## Overview

JxPoolMiner is a professional, cross-platform mining software with a modern GUI, supporting multiple algorithms (SHA-256, Ethash, GXHash) and automatic device detection.

## Project Architecture

```
JxPoolMiner/
├── Cargo.toml                 # Workspace root
├── src/
│   └── main.rs               # Application entry point
├── crates/                   # Modular crate architecture
│   ├── core/                 # Core types and utilities
│   ├── devices/              # Device detection and management
│   ├── mining/               # Mining algorithms and engines
│   ├── pool/                 # Pool connection and Stratum protocol
│   ├── gui/                  # GUI implementation (egui/Iced)
│   ├── config/               # Configuration management
│   ├── stats/                # Statistics and monitoring
│   └── updater/              # Auto-update system
├── assets/                   # GUI assets
│   ├── icons/
│   ├── themes/
│   └── fonts/
├── config/                   # Configuration files
│   ├── default.toml
│   └── pools.toml
├── docs/                     # Documentation
│   ├── USER_GUIDE.md
│   ├── DEVELOPER_GUIDE.md
│   └── API.md
├── tests/                    # Integration tests
│   ├── device_tests.rs
│   ├── mining_tests.rs
│   └── pool_tests.rs
├── benches/                  # Performance benchmarks
│   └── mining_bench.rs
├── scripts/                  # Build and deployment scripts
│   ├── build.sh
│   ├── package.sh
│   └── release.sh
└── installers/               # Platform-specific installers
    ├── windows/
    ├── macos/
    └── linux/
```

## Crate Structure

### 1. `crates/core/` - Core Types and Utilities

```
core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types/
│   │   ├── mod.rs
│   │   ├── device.rs         # Device types
│   │   ├── algorithm.rs      # Algorithm types
│   │   ├── share.rs          # Share types
│   │   └── job.rs            # Mining job types
│   ├── error.rs              # Error types
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── hash.rs           # Hashing utilities
│   │   └── time.rs           # Time utilities
│   └── constants.rs          # Global constants
```

**Purpose**: Shared types, utilities, and constants used across all crates.

### 2. `crates/devices/` - Device Detection and Management

```
devices/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── detector.rs           # Auto-detect devices
│   ├── asic/
│   │   ├── mod.rs
│   │   ├── detector.rs       # ASIC detection
│   │   └── manager.rs        # ASIC management
│   ├── gpu/
│   │   ├── mod.rs
│   │   ├── detector.rs       # GPU detection (CUDA, OpenCL)
│   │   ├── nvidia.rs         # NVIDIA GPUs
│   │   ├── amd.rs            # AMD GPUs
│   │   └── manager.rs        # GPU management
│   ├── cpu/
│   │   ├── mod.rs
│   │   ├── detector.rs       # CPU detection
│   │   └── manager.rs        # CPU management
│   ├── capabilities.rs       # Device capabilities
│   └── monitor.rs            # Device monitoring (temp, power)
```

**Purpose**: Automatic device detection, capability assessment, and device management.

**Key Features**:
- Auto-detect ASIC, GPU, CPU devices
- Query device capabilities (hashrate, memory, cores)
- Monitor temperature, power consumption
- Start/stop mining per device

### 3. `crates/mining/` - Mining Algorithms and Engines

```
mining/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── engine.rs             # Mining engine coordinator
│   ├── algorithms/
│   │   ├── mod.rs
│   │   ├── sha256/
│   │   │   ├── mod.rs
│   │   │   ├── asic.rs       # ASIC SHA-256
│   │   │   └── cpu.rs        # CPU SHA-256 (fallback)
│   │   ├── ethash/
│   │   │   ├── mod.rs
│   │   │   ├── gpu.rs        # GPU Ethash
│   │   │   └── dag.rs        # DAG generation
│   │   └── gxhash/
│   │       ├── mod.rs
│   │       ├── cpu.rs        # CPU GXHash
│   │       └── randomizer.rs # Randomization logic
│   ├── worker.rs             # Mining worker threads
│   ├── scheduler.rs          # Job scheduling
│   └── optimizer.rs          # Performance optimization
```

**Purpose**: Multi-algorithm mining engines optimized for each device type.

**Key Features**:
- SHA-256 for ASIC miners
- Ethash for GPU miners
- GXHash for CPU miners
- Multi-threaded mining
- Automatic difficulty adjustment

### 4. `crates/pool/` - Pool Connection and Stratum Protocol

```
pool/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── client.rs             # Pool client
│   ├── stratum/
│   │   ├── mod.rs
│   │   ├── v1.rs             # Stratum V1 protocol
│   │   ├── v2.rs             # Stratum V2 protocol
│   │   ├── messages.rs       # Protocol messages
│   │   └── codec.rs          # Message encoding/decoding
│   ├── connection.rs         # Connection management
│   ├── reconnect.rs          # Auto-reconnect logic
│   ├── auth.rs               # Authentication
│   └── failover.rs           # Failover pool support
```

**Purpose**: Pool connection, Stratum protocol implementation, and connection management.

**Key Features**:
- Stratum V1 and V2 support
- Automatic reconnection
- Failover pool support
- TLS/SSL encryption
- Authentication

### 5. `crates/gui/` - GUI Implementation

```
gui/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── app.rs                # Main application
│   ├── views/
│   │   ├── mod.rs
│   │   ├── dashboard.rs      # Dashboard view
│   │   ├── devices.rs        # Devices view
│   │   ├── pool.rs           # Pool connection view
│   │   ├── statistics.rs     # Statistics view
│   │   ├── settings.rs       # Settings view
│   │   └── advanced.rs       # Advanced/Developer view
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── hashrate_chart.rs # Hashrate graph
│   │   ├── device_card.rs    # Device card widget
│   │   ├── stats_panel.rs    # Statistics panel
│   │   └── connection_status.rs # Connection status
│   ├── theme.rs              # Theme management
│   ├── icons.rs              # Icon management
│   └── notifications.rs      # Notification system
```

**Purpose**: Modern, responsive GUI using egui or Iced.

**Key Features**:
- Dashboard with real-time stats
- Device management interface
- Pool connection management
- Statistics and charts
- Settings and configuration
- Dark/Light themes
- Notifications and alerts

### 6. `crates/config/` - Configuration Management

```
config/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── manager.rs            # Configuration manager
│   ├── types.rs              # Configuration types
│   ├── loader.rs             # Load from file
│   ├── saver.rs              # Save to file
│   ├── validator.rs          # Validate configuration
│   └── defaults.rs           # Default configuration
```

**Purpose**: Configuration management with validation and persistence.

**Key Features**:
- TOML/JSON configuration files
- GUI-based configuration editor
- Import/Export settings
- Validation
- Default configurations

### 7. `crates/stats/` - Statistics and Monitoring

```
stats/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── collector.rs          # Statistics collector
│   ├── aggregator.rs         # Data aggregation
│   ├── metrics/
│   │   ├── mod.rs
│   │   ├── hashrate.rs       # Hashrate metrics
│   │   ├── shares.rs         # Share metrics
│   │   ├── devices.rs        # Device metrics
│   │   └── pool.rs           # Pool metrics
│   ├── history.rs            # Historical data
│   ├── export.rs             # Export metrics
│   └── charts.rs             # Chart data generation
```

**Purpose**: Real-time statistics collection, aggregation, and visualization.

**Key Features**:
- Real-time hashrate tracking
- Share acceptance/rejection tracking
- Device performance metrics
- Historical data storage
- Export to CSV/JSON

### 8. `crates/updater/` - Auto-Update System

```
updater/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── checker.rs            # Check for updates
│   ├── downloader.rs         # Download updates
│   ├── installer.rs          # Install updates
│   ├── verifier.rs           # Verify update integrity
│   └── rollback.rs           # Rollback on failure
```

**Purpose**: Automatic software updates with integrity verification.

**Key Features**:
- Check for updates
- Download and verify updates
- Install updates
- Rollback on failure
- Update channels (stable, beta)

## Assets Structure

```
assets/
├── icons/
│   ├── app.ico               # Application icon
│   ├── asic.svg              # ASIC device icon
│   ├── gpu.svg               # GPU device icon
│   ├── cpu.svg               # CPU device icon
│   ├── connected.svg         # Connected status
│   └── disconnected.svg      # Disconnected status
├── themes/
│   ├── dark.toml             # Dark theme
│   └── light.toml            # Light theme
└── fonts/
    └── Inter-Regular.ttf     # UI font
```

## Configuration Structure

```
config/
├── default.toml              # Default configuration
├── pools.toml                # Pool configurations
└── devices.toml              # Device configurations
```

**default.toml**:
```toml
[app]
theme = "dark"
language = "en"
auto_start = false

[mining]
auto_detect_devices = true
auto_assign_algorithms = true

[pool]
primary = "stratum+tcp://pool.jxminer.com:3333"
fallback = "stratum+tcp://backup.jxminer.com:3333"
worker_name = "worker1"

[updates]
auto_check = true
channel = "stable"
```

## Documentation Structure

```
docs/
├── USER_GUIDE.md             # User guide
├── DEVELOPER_GUIDE.md        # Developer guide
├── API.md                    # API documentation
├── BUILDING.md               # Build instructions
├── TROUBLESHOOTING.md        # Troubleshooting guide
└── CHANGELOG.md              # Version history
```

## Testing Structure

```
tests/
├── integration/
│   ├── device_detection.rs   # Device detection tests
│   ├── mining_engine.rs      # Mining engine tests
│   ├── pool_connection.rs    # Pool connection tests
│   └── gui_tests.rs          # GUI tests
├── fixtures/
│   ├── mock_devices.json     # Mock device data
│   └── mock_pool.json        # Mock pool data
└── common/
    └── mod.rs                # Common test utilities
```

## Build Scripts

```
scripts/
├── build.sh                  # Build all platforms
├── package.sh                # Create installers
├── release.sh                # Release workflow
├── test.sh                   # Run all tests
└── bench.sh                  # Run benchmarks
```

## Installer Structure

```
installers/
├── windows/
│   ├── installer.nsi         # NSIS installer script
│   └── icon.ico              # Windows icon
├── macos/
│   ├── Info.plist            # macOS app info
│   └── icon.icns             # macOS icon
└── linux/
    ├── jxpoolminer.desktop   # Desktop entry
    └── install.sh            # Installation script
```

## Design Principles

### 1. Modularity
- Each crate has a single, well-defined responsibility
- Crates communicate through well-defined interfaces
- Easy to test and maintain

### 2. Performance
- Optimized mining algorithms
- Efficient device management
- Minimal GUI overhead
- Multi-threaded architecture

### 3. User Experience
- Modern, intuitive GUI
- Real-time feedback
- Clear error messages
- Comprehensive documentation

### 4. Cross-Platform
- Works on Windows, macOS, Linux
- Platform-specific optimizations
- Consistent user experience

### 5. Security
- TLS/SSL for pool connections
- Secure configuration storage
- Update verification
- No hardcoded credentials

### 6. Maintainability
- Clean code architecture
- Comprehensive tests
- Clear documentation
- Version control

## Technology Stack

- **Language**: Rust (2021 edition)
- **GUI**: egui or Iced
- **Async Runtime**: Tokio
- **Serialization**: serde
- **Configuration**: TOML
- **Logging**: tracing
- **Testing**: cargo test
- **Benchmarking**: criterion

## Development Workflow

1. **Setup**: Clone repository, install Rust
2. **Build**: `cargo build`
3. **Test**: `cargo test`
4. **Run**: `cargo run`
5. **Benchmark**: `cargo bench`
6. **Package**: `./scripts/package.sh`
7. **Release**: `./scripts/release.sh`

## Next Steps

1. Implement core types and utilities
2. Build device detection system
3. Implement mining algorithms
4. Create pool connection module
5. Build GUI
6. Add statistics and monitoring
7. Implement auto-update system
8. Create installers
9. Write documentation
10. Release v1.0.0

---

**Status**: Architecture designed, ready for implementation
**Version**: 1.0.0
**License**: MIT
