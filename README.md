# 🚀 Roblox Booster

A modern system performance optimizer for Roblox, built with Rust and egui.

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
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Linux/Mac: Install MinGW for cross-compilation
sudo apt update && sudo apt install -y mingw-w64

# Add Windows target
rustup target add x86_64-pc-windows-gnu
```

### Quick Build

```bash
# Simple build (uses .cargo/config.toml automatically)
cargo build --release

# Or with just (recommended)
cargo install just
just build
```

### Using Just (Modern Task Runner)

```bash
just setup      # Install dev dependencies
just build      # Build release
just run        # Build and run
just test       # Run tests
just lint       # Run clippy
just fmt        # Format code
just ci         # Full CI check
just dist       # Build and show info
just clean      # Clean artifacts
```

## 🎯 Modern Rust Features Used

- **Rust 1.75** - Modern stable Rust
- **Edition 2021** - Latest Rust edition
- **eframe 0.29 / egui 0.29** - Latest GUI framework
- **sysinfo 0.32** - Modern system information API
- **anyhow & thiserror** - Modern error handling
- **Option::then** - Cleaner conditional returns
- **must_use** - Compiler warnings for unused returns
- **rust-toolchain.toml** - Automatic toolchain management
- **.cargo/config.toml** - Project-specific cargo configuration
- **just** - Modern alternative to Make

## 📦 Project Structure

```
roblox_booster/
├── .cargo/
│   └── config.toml          # Cargo configuration
├── src/
│   ├── main.rs              # Entry point
│   ├── booster.rs           # Core logic (anyhow error handling)
│   ├── config.rs            # Configuration management
│   └── ui.rs                # Modern egui UI
├── build.rs                 # Windows metadata
├── Cargo.toml               # Dependencies
├── rust-toolchain.toml      # Rust version pinning
├── justfile                 # Modern task runner
└── README.md
```

## ⚙️ Configuration

Config file location:
- Windows: `%APPDATA%\roblox_booster\config.json`
- Linux/Mac: `~/.config/roblox_booster/config.json`

Example:
```json
{
  "auto_start_booster": false,
  "auto_detect_roblox": true,
  "priority_level": 2,
  "clear_memory_cache": true
}
```

## 🔧 How It Works

### Advanced Optimization Pipeline

**Phase 1: Process Detection**
- Smart pattern matching (detects "roblox", "rbx" variants)
- Filters out installers and uninstallers
- Real-time process monitoring

**Phase 2: Process Optimization**
- **CPU Priority Boost**: Sets HIGH/ABOVE_NORMAL priority class
- **Working Set Trimming**: Optimizes memory allocation
- **Multi-stage verification**: Checks priority before/after

**Phase 3: System Optimization**
- Memory cache clearing
- System-wide performance tuning
- Cleanup on exit

### Technical Details
- Uses Windows API: `SetPriorityClass`, `SetProcessWorkingSetSize`
- Safe handle management with RAII
- Typed errors with `thiserror`
- Graceful degradation on failure

## ✨ New Features

- **📊 Optimization Stats**: Real-time metrics display
- **🎯 Smart Detection**: Better process filtering
- **💾 Working Set Optimization**: Memory trimming per process
- **⚡ Two-Phase Optimization**: Priority + Memory optimization
- **🛡️ Error Recovery**: Graceful handling of failures
- **📈 Performance Tracking**: Shows optimized process count

## 🔧 How It Works

## 🛡️ Security Warning

**Antivirus False Positives**

Some AVs may flag this due to Windows API usage (OpenProcess, SetPriorityClass).

**This is safe because:**
- ✅ Full source code available
- ✅ Modern Rust (memory-safe)
- ✅ No code injection
- ✅ Open source auditable

**Detections:**
- W64.AIDetectMalware (heuristic)
- ElasticMalicious (moderate confidence)

**Why?** Uses process management APIs + cross-compiled from Linux + no code signing.

**To bypass Windows SmartScreen:**
1. Click "More info"
2. Click "Run anyway"

See `SECURITY.md` and `report_false_positive.md` for details.

## 🛡️ Safety

- ✅ No game modification
- ✅ No memory injection
- ✅ No ToS violations
- ✅ System-level only
- ✅ Memory-safe Rust

## 📋 Requirements

- **OS**: Windows 10/11
- **RAM**: 4GB minimum
- **Rust**: 1.75+ (specified in rust-toolchain.toml)

## 🐛 Troubleshooting

**Build errors?**
```bash
# Setup everything automatically
just setup

# Or manual
rustup target add x86_64-pc-windows-gnu
```

**Can't find exe?**
```bash
# Check build location
just info

# Or find it manually
find target -name "roblox_booster.exe"
```

## 🚀 Development

```bash
# Format code
just fmt

# Run linter
just lint

# Run tests
just test

# Full CI check
just ci

# Watch mode (requires cargo-watch)
cargo install cargo-watch
just watch
```

## 📝 License

MIT OR Apache-2.0

## 🤝 Contributing

1. Fork the repo
2. Create feature branch
3. Make changes
4. Run `just ci`
5. Submit PR

---

Built with ❤️ using modern Rust, egui, and best practices