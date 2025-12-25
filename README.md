# JxPoolMiner

**Professional Cross-Platform Mining Software with Modern GUI**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue.svg)](https://github.com/philani1H/JxPoolMiner)

## Features

### 🎯 Core Features

- **Automatic Device Detection** - Detects ASIC, GPU, and CPU devices automatically
- **Multi-Algorithm Support** - SHA-256 (ASIC), Ethash (GPU), GXHash (CPU)
- **Modern GUI** - Beautiful, responsive interface with real-time statistics
- **Pool Connection** - Stratum V1/V2 protocol support with auto-reconnect
- **Real-Time Monitoring** - Live hashrate, shares, and device statistics
- **Auto-Update** - Automatic software updates with integrity verification

### 💎 Advanced Features

- **Multi-Device Management** - Control multiple devices independently
- **Failover Pools** - Automatic failover to backup pools
- **Performance Optimization** - AI-based algorithm assignment and tuning
- **Temperature Monitoring** - Real-time device temperature and power tracking
- **Dark/Light Themes** - Customizable UI themes
- **Export Statistics** - Export performance data to CSV/JSON

## Screenshots

### Dashboard
![Dashboard](docs/screenshots/dashboard.png)

### Device Management
![Devices](docs/screenshots/devices.png)

### Statistics
![Statistics](docs/screenshots/statistics.png)

## Installation

### Windows

1. Download `JxPoolMiner-Setup.exe` from [Releases](https://github.com/philani1H/JxPoolMiner/releases)
2. Run the installer
3. Launch JxPoolMiner from Start Menu

### macOS

1. Download `JxPoolMiner.dmg` from [Releases](https://github.com/philani1H/JxPoolMiner/releases)
2. Open the DMG and drag JxPoolMiner to Applications
3. Launch from Applications folder

### Linux

```bash
# Download and install
wget https://github.com/philani1H/JxPoolMiner/releases/latest/download/jxpoolminer-linux.tar.gz
tar -xzf jxpoolminer-linux.tar.gz
cd jxpoolminer
./install.sh

# Run
jxpoolminer
```

## Quick Start

### 1. Configure Pool

```toml
# config/default.toml
[pool]
primary = "stratum+tcp://pool.jxminer.com:3333"
fallback = "stratum+tcp://backup.jxminer.com:3333"
wallet_address = "your_wallet_address"
worker_name = "worker1"
```

### 2. Launch Application

```bash
jxpoolminer
```

### 3. Start Mining

1. Open JxPoolMiner
2. Go to **Devices** tab
3. Click **Start Mining** on desired devices
4. Monitor performance in **Dashboard**

## Building from Source

### Prerequisites

- Rust 1.70 or higher
- CMake (for RocksDB)
- OpenSSL development libraries

### Build Steps

```bash
# Clone repository
git clone https://github.com/philani1H/JxPoolMiner.git
cd JxPoolMiner

# Build
cargo build --release

# Run
cargo run --release
```

### Platform-Specific Dependencies

**Windows:**
```powershell
# Install Visual Studio Build Tools
# Install CMake
```

**macOS:**
```bash
brew install cmake openssl
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install cmake libssl-dev pkg-config

# Fedora
sudo dnf install cmake openssl-devel
```

## Configuration

### Configuration File

Location: `config/default.toml`

```toml
[app]
theme = "dark"              # "dark" or "light"
language = "en"             # Language code
auto_start = false          # Start mining on launch

[mining]
auto_detect_devices = true  # Auto-detect devices
auto_assign_algorithms = true  # Auto-assign algorithms

[pool]
primary = "stratum+tcp://pool.jxminer.com:3333"
fallback = "stratum+tcp://backup.jxminer.com:3333"
wallet_address = "your_wallet_address"
worker_name = "worker1"
use_tls = true              # Use TLS/SSL

[devices]
# Device-specific settings
[devices.gpu]
threads = 0                 # 0 = auto
intensity = "auto"          # "low", "medium", "high", "auto"

[devices.cpu]
threads = 0                 # 0 = auto (all cores)
affinity = []               # CPU core affinity

[updates]
auto_check = true           # Check for updates
channel = "stable"          # "stable" or "beta"
```

### Pool Configuration

Location: `config/pools.toml`

```toml
[[pools]]
name = "JxMiner Pool"
url = "stratum+tcp://pool.jxminer.com:3333"
algorithm = "auto"          # "sha256", "ethash", "gxhash", "auto"
priority = 1

[[pools]]
name = "Backup Pool"
url = "stratum+tcp://backup.jxminer.com:3333"
algorithm = "auto"
priority = 2
```

## GUI Overview

### Dashboard Tab
- Real-time hashrate graph
- Pending and confirmed rewards
- Active algorithms per device
- Pool connection status

### Devices Tab
- List of detected devices
- Device type, capabilities, algorithm
- Start/stop mining per device
- Temperature and power monitoring

### Pool Connection Tab
- Test pool connection
- Server latency and status
- Miner address and worker name
- Connection logs

### Statistics Tab
- Accepted/rejected shares
- Per-device contribution
- Historical performance charts
- Export data

### Settings Tab
- Mining algorithm selection
- Thread/core allocation
- Update channel
- Theme and language

### Advanced Tab
- Raw job submission logs
- Pool response logs
- Debug information
- Performance metrics

## Supported Algorithms

| Algorithm | Hardware | Description |
|-----------|----------|-------------|
| **SHA-256** | ASIC | Bitcoin-style double SHA-256 hashing |
| **Ethash** | GPU | Memory-hard algorithm for Ethereum-style mining |
| **GXHash** | CPU | Randomized, ASIC-resistant algorithm |

## Device Support

### ASIC Miners
- Antminer series
- Whatsminer series
- Avalon series
- Custom ASIC devices

### GPU Miners
- NVIDIA GPUs (CUDA)
- AMD GPUs (OpenCL)
- Multi-GPU setups

### CPU Miners
- Intel CPUs
- AMD CPUs
- ARM CPUs (experimental)

## Performance

### Benchmarks

| Device | Algorithm | Hashrate | Power |
|--------|-----------|----------|-------|
| Antminer S19 | SHA-256 | 110 TH/s | 3250W |
| RTX 3090 | Ethash | 120 MH/s | 350W |
| Ryzen 9 5950X | GXHash | 15 MH/s | 105W |

### Optimization

- **Multi-threading**: Automatic thread allocation per device
- **Memory optimization**: Efficient memory usage for DAG generation
- **CPU affinity**: Pin threads to specific cores for better performance
- **Dynamic difficulty**: Automatic difficulty adjustment per device

## Troubleshooting

### Device Not Detected

1. Check device drivers are installed
2. Restart JxPoolMiner
3. Check device is not in use by another application
4. Enable debug logging in Settings

### Pool Connection Failed

1. Check internet connection
2. Verify pool URL is correct
3. Check firewall settings
4. Try failover pool

### Low Hashrate

1. Check device temperature
2. Reduce intensity in Settings
3. Update device drivers
4. Check for background processes

### GUI Not Responding

1. Check system resources (CPU, RAM)
2. Restart application
3. Reset configuration to defaults
4. Report issue on GitHub

## Development

### Project Structure

```
JxPoolMiner/
├── crates/           # Modular crates
│   ├── core/         # Core types
│   ├── devices/      # Device detection
│   ├── mining/       # Mining algorithms
│   ├── pool/         # Pool connection
│   ├── gui/          # GUI implementation
│   ├── config/       # Configuration
│   ├── stats/        # Statistics
│   └── updater/      # Auto-update
├── assets/           # GUI assets
├── config/           # Configuration files
├── docs/             # Documentation
└── tests/            # Tests
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Benchmarks
cargo bench
```

## Roadmap

### v1.0.0 (Current)
- [x] Device auto-detection
- [x] Multi-algorithm support
- [x] Modern GUI
- [x] Pool connection
- [x] Real-time statistics

### v1.1.0 (Planned)
- [ ] Stratum V2 support
- [ ] Advanced AI optimization
- [ ] Mobile app (monitoring)
- [ ] Cloud sync
- [ ] Multi-pool mining

### v2.0.0 (Future)
- [ ] Plugin system
- [ ] Custom algorithms
- [ ] Distributed mining
- [ ] Advanced analytics
- [ ] API for third-party integrations

## Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/philani1H/JxPoolMiner/issues)
- **Discord**: [Join our Discord](https://discord.gg/jxpoolminer)
- **Email**: support@jxpoolminer.com

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by SlashMinerPool
- Built with Rust and egui/Iced
- Community contributors

## Disclaimer

Mining cryptocurrency requires significant computational resources and electricity. Always ensure you have proper cooling and power supply. JxPoolMiner is provided "as is" without warranty of any kind.

---

**Made with ❤️ by the JxPoolMiner Team**
