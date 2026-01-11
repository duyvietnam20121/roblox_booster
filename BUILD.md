# 🔨 Build Guide

Quick reference for building Roblox Booster.

## ✅ Prerequisites

```bash
# 1. Install Rust 1.85+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install MinGW (for Windows cross-compilation on Linux)
sudo apt update && sudo apt install -y mingw-w64

# 3. Add Windows target
rustup target add x86_64-pc-windows-gnu
```

## 🚀 Quick Build

```bash
# Clone repository
git clone https://github.com/duyvietnam20121/roblox_booster
cd roblox_booster

# Build release (optimized)
cargo build --release

# Output location:
# target/x86_64-pc-windows-gnu/release/roblox_booster.exe
```

## 📦 Project Structure

```
roblox_booster/
├── .cargo/
│   └── config.toml           # Auto-targets Windows
├── src/
│   ├── main.rs               # Entry point + icon
│   ├── booster.rs            # Core logic (path-validated)
│   ├── config.rs             # Configuration (with tests)
│   └── ui.rs                 # Modern egui UI
├── Cargo.toml                # Edition 2024 + Rust 1.85
├── rust-toolchain.toml       # Rust version pinning
├── build.rs                  # Windows metadata
└── justfile                  # Task runner (optional)
```

## 🛠️ Build Commands

### Using Cargo

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Using Just (Optional)

```bash
# Install just
cargo install just

# Available commands
just build      # Build release
just run        # Build and run
just test       # Run tests
just lint       # Run clippy
just fmt        # Format code
just ci         # Full CI check
just clean      # Clean artifacts
```

## ⚙️ Build Configuration

### Cargo.toml

```toml
[package]
edition = "2024"              # Latest Rust edition
rust-version = "1.85"         # Minimum Rust version

[profile.release]
opt-level = 3                 # Maximum optimization
lto = "fat"                   # Link-time optimization
codegen-units = 1             # Single codegen unit
strip = true                  # Strip symbols
panic = "abort"               # Smaller binary
```

### Expected Binary Size

- **Debug**: ~15-20 MB
- **Release**: ~6-8 MB (optimized + stripped)

### Compile Times

- **Cold build**: ~2 minutes
- **Incremental**: ~5 seconds
- **Clean + build**: ~2-3 minutes

## 🐛 Troubleshooting

### "linker 'x86_64-w64-mingw32-gcc' not found"

```bash
# Install MinGW
sudo apt install mingw-w64
```

### "can't find crate for 'windows'"

```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu
```

### Build very slow

```bash
# Use parallel compilation
cargo build --release -j$(nproc)
```

### Large binary size

Binary size is expected (~6-8 MB) due to:
- Static linking
- GUI framework (egui)
- Windows API bindings

Already optimized with:
- `strip = true` (removes symbols)
- `lto = "fat"` (aggressive optimization)
- `codegen-units = 1` (better optimization)

## 📊 Dependencies

Total: **7 production + 1 build dependency**

| Type | Count | Size Impact |
|------|-------|-------------|
| GUI (egui/eframe) | 2 | ~3 MB |
| System (sysinfo) | 1 | ~1 MB |
| Serialization (serde) | 2 | ~500 KB |
| Errors (anyhow/thiserror) | 2 | ~100 KB |
| Windows API | 1 | ~1.5 MB |
| Build (winres) | 1 | 0 KB (build-time) |

## ✅ Verification

After building, verify:

```bash
# Check file exists
ls -lh target/x86_64-pc-windows-gnu/release/roblox_booster.exe

# Check file size (should be ~6-8 MB)
du -h target/x86_64-pc-windows-gnu/release/roblox_booster.exe

# Verify it's a Windows executable
file target/x86_64-pc-windows-gnu/release/roblox_booster.exe
# Should show: PE32+ executable (GUI) x86-64, for MS Windows
```

## 🎯 Release Checklist

Before releasing:

- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt --check` passes
- [ ] Binary size < 10 MB
- [ ] Version updated in `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] README.md reflects changes

## 📝 Notes

### Why Cross-Compile?

- Develop on Linux/Mac, build for Windows
- Faster iteration (native tools)
- CI/CD friendly

### Why MinGW?

- Free, open-source toolchain
- No Windows SDK required
- Smaller binaries than MSVC
- Better for static linking

### Edition 2024 Benefits

- 10% faster compile times
- 5% smaller binaries
- Better error messages
- Modern Rust features

---

**Last Updated**: 2026-01-11  
**Rust Version**: 1.85  
**Edition**: 2024