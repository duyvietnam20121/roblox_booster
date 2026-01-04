# 🚀 Roblox Booster

A system performance optimizer for Roblox, built with Rust and egui.

## ✨ Features

- **Auto Booster**: Toggle system optimizations on/off
- **Auto-detect**: Automatically detects and boosts Roblox processes
- **Launch Roblox**: Start Roblox directly from the app
- **Configurable Settings**: Persistent configuration with JSON
- **Priority Control**: Adjust process priority (Normal, Above Normal, High)
- **Memory Optimization**: Clear system cache for better performance
- **Live Monitoring**: Real-time Roblox process count

## 🛠️ Build Instructions

### Prerequisites

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install MinGW for cross-compilation (Linux/Mac)
sudo apt update
sudo apt install -y mingw-w64 build-essential
```

### Build on Linux/Mac (for Windows target)

```bash
# Add Windows GNU target
rustup target add x86_64-pc-windows-gnu

# Build using Makefile
make install

# Or manual build
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/roblox_booster.exe ./
```

### Build on Windows

```bash
# Simple build
cargo build --release

# Output: target\release\roblox_booster.exe
```

## 📦 Makefile Commands

```bash
make help       # Show all available commands
make setup      # Install dependencies and target
make build      # Build the executable
make clean      # Clean build artifacts
make install    # Setup + Build in one command
make run        # Build and show exe info
```

## 🎯 Usage

1. **Download/Build** the `roblox_booster.exe`
2. **Run** the executable on Windows
3. **Launch Roblox** using the app button
4. **Enable Booster** to optimize performance
5. **Configure** settings as needed

## ⚙️ Configuration

Config file is automatically created at:
- Windows: `%APPDATA%\roblox_booster\config.json`
- Linux/Mac: `~/.config/roblox_booster/config.json`

Example config:
```json
{
  "auto_start_booster": false,
  "auto_detect_roblox": true,
  "priority_level": 2,
  "clear_memory_cache": true
}
```

## 🔧 How It Works

- **Process Priority**: Boosts Roblox process priority to High/Above Normal
- **Memory Cache**: Clears Windows standby memory
- **Auto-detection**: Scans for Roblox processes every 2 seconds
- **Safe Operation**: Only modifies priority, no memory injection

## 🛡️ Safety

- ✅ No game file modification
- ✅ No memory injection
- ✅ No ToS violations
- ✅ System-level optimizations only

## 📋 System Requirements

- **OS**: Windows 10/11
- **RAM**: 4GB minimum
- **Roblox**: Installed from roblox.com or Microsoft Store

## 🛡️ Security Warning

**Windows may show a security warning** because the executable is not signed with a commercial certificate.

### How to bypass:
1. Click **"More info"**
2. Click **"Run anyway"**

### For developers - Self-signing:
```powershell
# Run PowerShell as Administrator on Windows
.\sign_exe.ps1
```

This creates a self-signed certificate. Users will still see a warning unless they install your certificate.

### For production - Get a certificate:
- Purchase a Code Signing Certificate from Sectigo, DigiCert, or SSL.com ($100-400/year)
- Sign the executable with `signtool`

## 🐛 Troubleshooting

**Executable not found after build?**
```bash
# Check target directory
ls target/x86_64-pc-windows-gnu/release/

# Copy to current directory
cp target/x86_64-pc-windows-gnu/release/roblox_booster.exe ./
```

**MinGW not installed?**
```bash
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

## 📝 License

This project is provided as-is for educational purposes.

## 🤝 Contributing

Feel free to open issues or submit pull requests!

---

Built with ❤️ using Rust, egui, and sysinfo