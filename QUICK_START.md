# 🚀 JxPoolMiner - Quick Start Guide

## What is JxPoolMiner?

**JxPoolMiner is a DESKTOP APPLICATION for mining GXC (GXChain) cryptocurrency** (like TeamViewer, Chrome, or any software you install on your computer)

- ✅ **Native GUI** - Opens a window on your desktop
- ✅ **No browser needed** - Runs directly on your computer
- ✅ **Easy to install** - Just like installing any software
- ✅ **Works offline** - Only needs internet to connect to mining pool

---

## 📥 Installation (Choose One Method)

### Method 1: One-Liner Install (Easiest!) 🎯

**Copy and paste this into your terminal:**

#### Linux:
```bash
curl -sSL https://raw.githubusercontent.com/philani1H/JxPoolMiner/feature/installable-build-and-bugfixes/install.sh | bash
```

#### macOS:
```bash
curl -sSL https://raw.githubusercontent.com/philani1H/JxPoolMiner/feature/installable-build-and-bugfixes/install.sh | bash
```

**That's it!** The software will:
1. Download automatically
2. Install to your system
3. Create desktop shortcut
4. Be ready to use

---

### Method 2: Download Package (Like Downloading Chrome)

#### For Linux:

**Step 1:** Download the package
```bash
wget https://github.com/philani1H/JxPoolMiner/releases/download/v1.0.0/JxPoolMiner-1.0.0-linux-x86_64.tar.gz
```

**Step 2:** Extract it
```bash
tar -xzf JxPoolMiner-1.0.0-linux-x86_64.tar.gz
cd jxpoolminer-1.0.0-linux
```

**Step 3:** Run the installer
```bash
./install.sh
```

**Done!** JxPoolMiner is now installed.

---

#### For Windows:

**Step 1:** Download `JxPoolMiner-Setup.exe` from:
```
https://github.com/philani1H/JxPoolMiner/releases/latest
```

**Step 2:** Double-click the installer

**Step 3:** Follow the installation wizard (Next → Next → Install)

**Done!** Find JxPoolMiner in your Start Menu.

---

#### For macOS:

**Step 1:** Download `JxPoolMiner.dmg` from:
```
https://github.com/philani1H/JxPoolMiner/releases/latest
```

**Step 2:** Open the DMG file

**Step 3:** Drag JxPoolMiner to Applications folder

**Done!** Launch from Applications.

---

## 🎮 How to Use (After Installation)

### Step 1: Launch the Application

**Linux:**
```bash
jxpoolminer
```

**Windows:**
- Click Start Menu → JxPoolMiner

**macOS:**
- Open Applications → JxPoolMiner

### Step 2: You'll See This Window

```
╔════════════════════════════════════════════════════════════╗
║ 🔷 JxPoolMiner                                             ║
║ 📊 Dashboard | 🖥️ Devices | 🌐 Pool | 📈 Statistics | ⚙️ Settings ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   ║
║  │ Total        │  │ Active       │  │ Accepted     │   ║
║  │ Hashrate     │  │ Devices      │  │ Shares       │   ║
║  │ 0.0 MH/s     │  │      0       │  │      0       │   ║
║  └──────────────┘  └──────────────┘  └──────────────┘   ║
║                                                            ║
║  [Start Mining]  [Stop Mining]  [Settings]                ║
╚════════════════════════════════════════════════════════════╝
```

### Step 3: Configure Your Pool

Click **⚙️ Settings** tab and enter:

```
Pool URL: stratum+tcp://gxc-pool.example.com:3333
Wallet Address: GXC_YOUR_WALLET_ADDRESS_HERE
Worker Name: worker1
```

**Or edit the config file:**
```bash
# Linux/macOS
nano ~/.config/jxpoolminer/config.toml

# Windows
notepad %APPDATA%\jxpoolminer\config.toml
```

### Step 4: Start Mining!

1. Go to **🖥️ Devices** tab
2. You'll see your detected hardware (CPU, GPU, ASIC)
3. Click **[Start Mining]** next to each device
4. Watch your hashrate in **📊 Dashboard**

---

## 🖥️ What You'll See (GUI Tabs)

### 📊 Dashboard Tab
- Real-time hashrate graph
- Total earnings
- Active devices count
- Share statistics

### 🖥️ Devices Tab
```
Device              Type    Algorithm   Hashrate    Status    Action
─────────────────────────────────────────────────────────────────────
CPU-0: Intel i7     CPU     GXHash      15 MH/s     Mining    [Stop]
GPU-0: RTX 3090     GPU     Ethash      120 MH/s    Idle      [Start]
ASIC-0: Antminer    ASIC    SHA-256     110 TH/s    Idle      [Start]
```

### 🌐 Pool Tab
- Connection status (Connected/Disconnected)
- Pool URL and worker info
- Latency monitoring
- Test connection button

### 📈 Statistics Tab
- Accepted shares: 1,234
- Rejected shares: 5
- Acceptance rate: 99.6%
- Per-device performance charts

### ⚙️ Settings Tab
- Pool configuration
- Theme (Dark/Light)
- Auto-start mining
- Update settings

### 🔧 Advanced Tab
- Debug logs
- Pool messages
- Performance metrics
- Export logs

---

## ❓ FAQ

### Q: Do I need to build anything?
**A:** NO! Just install like any software (Chrome, Zoom, etc.)

### Q: Is this a web application?
**A:** NO! It's a native desktop application with a GUI window.

### Q: Do I need Rust or Cargo installed?
**A:** NO! The installer includes everything you need.

### Q: Can I use it without a display?
**A:** The GUI requires a display. For headless servers, use the CLI mode (coming in v1.1.0).

### Q: Is it like a remote desktop?
**A:** Similar concept - it's a desktop application you install and run locally. But it's for mining, not remote control.

### Q: What hardware do I need?
**A:** Any of these:
- CPU (any modern processor)
- GPU (NVIDIA, AMD, Intel)
- ASIC miner (Antminer, Whatsminer, etc.)

### Q: Can I mine without a pool?
**A:** No, you need to connect to a mining pool. Configure it in Settings.

---

## 🎯 Quick Example

**Complete installation and start mining in 3 commands:**

```bash
# 1. Install
curl -sSL https://raw.githubusercontent.com/philani1H/JxPoolMiner/feature/installable-build-and-bugfixes/install.sh | bash

# 2. Configure (edit your wallet address)
nano ~/.config/jxpoolminer/config.toml

# 3. Run
jxpoolminer
```

**Then in the GUI:**
1. Click **🖥️ Devices** tab
2. Click **[Start Mining]** on your device
3. Watch earnings in **📊 Dashboard**

---

## 🆘 Troubleshooting

### "Command not found: jxpoolminer"
**Solution:** Add to PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### "No devices detected"
**Solution:** 
- For GPU: Install drivers (NVIDIA/AMD)
- For ASIC: Check USB connection
- CPU should always be detected

### "Cannot connect to pool"
**Solution:**
- Check pool URL is correct
- Check internet connection
- Try fallback pool

### "GUI doesn't open"
**Solution:**
- Make sure you have a display
- Check if X11/Wayland is running
- Try running from terminal to see errors

---

## 📞 Need Help?

- **Documentation**: See INSTALL.md for detailed guide
- **Issues**: https://github.com/philani1H/JxPoolMiner/issues
- **Repository**: https://github.com/philani1H/JxPoolMiner

---

## 🎉 You're Ready!

JxPoolMiner is now installed and ready to mine. Just:
1. ✅ Launch the application
2. ✅ Configure your pool
3. ✅ Start mining
4. ✅ Watch your earnings grow!

**Happy Mining! ⛏️💰**

---

*JxPoolMiner - Professional Mining Made Easy*
