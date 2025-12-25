# 🏠 Local Stratum Server Setup for JxPoolMiner

## Overview

JxPoolMiner is **pre-configured to work with local stratum server on `localhost:3333`** by default.

This guide explains how to set up and use JxPoolMiner with a local stratum server.

---

## 🎯 Default Configuration

### Out of the Box
JxPoolMiner connects to:
- **Primary**: `stratum+tcp://localhost:3333`
- **Fallback**: `stratum+tcp://127.0.0.1:3333`

**No configuration needed** if your stratum server runs on port 3333!

---

## 🚀 Quick Start with Local Stratum

### Step 1: Start Your Stratum Server
```bash
# Example: Start your GXC stratum server
./stratum-server --port 3333
```

### Step 2: Install JxPoolMiner
```bash
curl -sSL https://raw.githubusercontent.com/philani1H/JxPoolMiner/feature/installable-build-and-bugfixes/install.sh | bash
```

### Step 3: Configure Wallet (Optional)
Edit `~/.config/jxpoolminer/config.toml`:
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
wallet_address = "YOUR_GXC_WALLET_ADDRESS"
worker_name = "worker1"
```

### Step 4: Run JxPoolMiner
```bash
jxpoolminer
```

**That's it!** JxPoolMiner will automatically connect to your local stratum server.

---

## 🔧 Configuration Options

### Default Config Location
- **Linux**: `~/.config/jxpoolminer/config.toml`
- **macOS**: `~/Library/Application Support/jxpoolminer/config.toml`
- **Windows**: `%APPDATA%\jxpoolminer\config.toml`

### Full Configuration Example
```toml
[app]
theme = "dark"
language = "en"
auto_start = false

[mining]
auto_detect_devices = true
auto_assign_algorithms = true

[pool]
# Local stratum server (default)
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://127.0.0.1:3333"

# Your GXC wallet address
wallet_address = "GXC_YOUR_WALLET_ADDRESS_HERE"

# Worker identification
worker_name = "worker1"

# TLS/SSL (usually false for local)
use_tls = false
```

---

## 🌐 Using Different Ports

### Custom Port Configuration

If your stratum server runs on a different port:

```toml
[pool]
# Custom port (e.g., 3334)
primary = "stratum+tcp://localhost:3334"
fallback = "stratum+tcp://127.0.0.1:3334"
```

### Multiple Local Servers

```toml
[pool]
# Primary local server
primary = "stratum+tcp://localhost:3333"

# Backup local server on different port
fallback = "stratum+tcp://localhost:3334"
```

---

## 🔌 Connection Scenarios

### Scenario 1: Local Stratum Only
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://127.0.0.1:3333"
```
**Use Case**: Testing, development, private mining

### Scenario 2: Local + Remote Backup
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://pool.gxc.com:3333"
```
**Use Case**: Local primary, remote backup

### Scenario 3: Remote Pool
```toml
[pool]
primary = "stratum+tcp://pool.gxc.com:3333"
fallback = "stratum+tcp://backup.gxc.com:3333"
```
**Use Case**: Public pool mining

### Scenario 4: LAN Stratum Server
```toml
[pool]
primary = "stratum+tcp://192.168.1.100:3333"
fallback = "stratum+tcp://localhost:3333"
```
**Use Case**: Mining farm with local network stratum

---

## 🧪 Testing Connection

### Method 1: GUI Test Button
1. Open JxPoolMiner
2. Go to **Pool** tab
3. Click **Test Connection**
4. Check status indicator

### Method 2: Command Line
```bash
# Test if stratum server is listening
telnet localhost 3333

# Or using netcat
nc -zv localhost 3333

# Or using curl
curl telnet://localhost:3333
```

### Method 3: Check Logs
```bash
# Run JxPoolMiner with debug logging
RUST_LOG=debug jxpoolminer
```

Look for:
```
[INFO] Connecting to pool: stratum+tcp://localhost:3333
[INFO] Connecting to Stratum server: localhost:3333
```

---

## 🐛 Troubleshooting

### Problem: "Connection refused"

**Cause**: Stratum server not running

**Solution**:
```bash
# Check if stratum server is running
ps aux | grep stratum

# Check if port 3333 is listening
netstat -tuln | grep 3333
# Or
ss -tuln | grep 3333

# Start your stratum server
./stratum-server --port 3333
```

### Problem: "Connection timeout"

**Cause**: Firewall blocking connection

**Solution**:
```bash
# Linux - Allow port 3333
sudo ufw allow 3333

# Check firewall status
sudo ufw status
```

### Problem: "Authentication failed"

**Cause**: Invalid wallet address or worker name

**Solution**:
```toml
# Update config with correct credentials
[pool]
wallet_address = "YOUR_VALID_GXC_ADDRESS"
worker_name = "worker1"
```

### Problem: "Wrong protocol version"

**Cause**: Stratum version mismatch

**Solution**:
- JxPoolMiner uses Stratum V1
- Ensure your stratum server supports Stratum V1
- Check server logs for protocol errors

---

## 📊 Monitoring Local Stratum

### Check Connection Status

In JxPoolMiner GUI:
1. **Pool Tab** → Shows connection status
2. **Advanced Tab** → Shows connection details
3. **Dashboard** → Shows if shares are being submitted

### Server-Side Monitoring

Check your stratum server logs:
```bash
# Example log locations
tail -f /var/log/stratum/server.log
tail -f ~/stratum-server/logs/connections.log
```

Look for:
- Worker connections
- Share submissions
- Authentication events

---

## 🔐 Security Considerations

### Local Network Only

If running stratum on local network:
```toml
[pool]
# Bind to specific interface
primary = "stratum+tcp://192.168.1.100:3333"
```

### TLS/SSL for Remote

If connecting to remote stratum:
```toml
[pool]
primary = "stratum+ssl://secure-pool.gxc.com:3334"
use_tls = true
```

### Firewall Rules

```bash
# Allow only local connections
sudo iptables -A INPUT -p tcp --dport 3333 -s 127.0.0.1 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 3333 -j DROP

# Allow LAN connections
sudo iptables -A INPUT -p tcp --dport 3333 -s 192.168.1.0/24 -j ACCEPT
```

---

## 🎮 Example Workflows

### Workflow 1: Development Setup

```bash
# Terminal 1: Start local stratum
cd ~/stratum-server
./start.sh --port 3333 --debug

# Terminal 2: Run JxPoolMiner
jxpoolminer

# Terminal 3: Monitor logs
tail -f ~/.local/share/jxpoolminer/logs/jxpoolminer.log
```

### Workflow 2: Production Mining

```bash
# Start stratum as service
sudo systemctl start stratum-server

# Verify it's running
sudo systemctl status stratum-server

# Start JxPoolMiner
jxpoolminer

# Check mining status
# (Use GUI Dashboard tab)
```

### Workflow 3: Testing New Pool

```bash
# Edit config
nano ~/.config/jxpoolminer/config.toml

# Change pool URL
[pool]
primary = "stratum+tcp://localhost:3333"

# Save and restart JxPoolMiner
jxpoolminer
```

---

## 📝 Configuration Templates

### Template 1: Local Development
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://127.0.0.1:3333"
wallet_address = "GXC_DEV_WALLET"
worker_name = "dev-worker"
use_tls = false
```

### Template 2: Local Production
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://192.168.1.100:3333"
wallet_address = "GXC_PRODUCTION_WALLET"
worker_name = "prod-worker-01"
use_tls = false
```

### Template 3: Hybrid (Local + Remote)
```toml
[pool]
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://pool.gxc.com:3333"
wallet_address = "GXC_YOUR_WALLET"
worker_name = "hybrid-worker"
use_tls = false
```

---

## 🚀 Performance Tips

### 1. Use Local Stratum for Lower Latency
- Local: ~1ms latency
- Remote: ~50-200ms latency
- **Result**: Faster share submission, less stale shares

### 2. Run Stratum on Same Machine
```toml
primary = "stratum+tcp://localhost:3333"
```
**Benefit**: Lowest possible latency

### 3. Use Fallback Pool
```toml
primary = "stratum+tcp://localhost:3333"
fallback = "stratum+tcp://backup-pool.com:3333"
```
**Benefit**: Automatic failover if local server goes down

### 4. Monitor Connection Quality
- Check **Pool** tab for latency
- Watch for connection drops
- Monitor share acceptance rate

---

## 📞 Support

### Common Questions

**Q: Do I need to change anything to use local stratum?**  
A: No! JxPoolMiner is pre-configured for `localhost:3333`

**Q: Can I use a different port?**  
A: Yes, just edit the config file and change `:3333` to your port

**Q: Can I connect to multiple pools?**  
A: Yes, use primary and fallback pool settings

**Q: Does it work with Stratum V2?**  
A: Currently supports Stratum V1. V2 support planned for v1.1.0

**Q: Can I mine to multiple wallets?**  
A: One wallet per worker. Run multiple instances for multiple wallets

---

## ✅ Checklist

Before starting mining with local stratum:

- [ ] Stratum server is running on port 3333
- [ ] Port 3333 is accessible (check with `telnet localhost 3333`)
- [ ] JxPoolMiner is installed
- [ ] Config file has correct wallet address
- [ ] Worker name is set
- [ ] Firewall allows connections (if needed)
- [ ] Devices are detected (check Devices tab)

---

## 🎉 Ready to Mine!

With local stratum configured, you can now:
1. ✅ Mine with lowest latency
2. ✅ Test mining locally
3. ✅ Run private mining operations
4. ✅ Develop and debug easily
5. ✅ Have full control over your mining

**Happy Mining with Local Stratum!** ⛏️

---

*JxPoolMiner - Optimized for Local and Remote Stratum*
