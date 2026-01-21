# 🛠️ Developer Notes - Roblox Booster

## 🎯 Target Platform

**Primary Target**: `x86_64-pc-windows-gnu`

### Tại sao GNU thay vì MSVC?

| Aspect | GNU (MinGW) | MSVC |
|--------|-------------|------|
| Cross-compile từ Linux | ✅ Dễ | ❌ Khó |
| Dependencies | MinGW-w64 | Visual Studio |
| Binary size | Nhỏ hơn | Lớn hơn |
| C Runtime | libgcc/libstdc++ | MSVCRT |
| Compatibility | Windows 7+ | Windows 10+ |

**Kết luận**: GNU phù hợp cho cross-compile và có binary nhỏ hơn.

## 🔧 Build Environment Setup

### Linux/Codespaces

```bash
# Method 1: Auto setup
./setup.sh

# Method 2: Manual
sudo apt install mingw-w64 build-essential
rustup target add x86_64-pc-windows-gnu
```

### Windows Native

```bash
# Không cần MinGW, build trực tiếp
cargo build --release
```

## 📁 Build Artifacts

```
target/
└── x86_64-pc-windows-gnu/
    ├── debug/
    │   └── roblox_booster.exe
    └── release/
        └── roblox_booster.exe  # ← Binary cuối cùng
```

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Test (cần Windows)

1. Copy `roblox_booster.exe` sang Windows
2. Run Roblox
3. Chạy booster
4. Verify optimization hoạt động

## ⚙️ Configuration Files

### .cargo/config.toml

Tự động chọn target `x86_64-pc-windows-gnu` và linker MinGW.

```toml
[build]
target = "x86_64-pc-windows-gnu"
```

### rust-toolchain.toml

Pin Rust version 1.85 với Edition 2024.

```toml
channel = "1.85"
targets = ["x86_64-pc-windows-gnu"]
```

## 🏗️ Build Process

```
cargo build --release
    ↓
rustc compiles Rust → LLVM IR
    ↓
LLVM generates Windows object files
    ↓
MinGW linker (x86_64-w64-mingw32-gcc)
    ↓
roblox_booster.exe (Windows PE32+)
```

## 📦 Dependencies

### Core
- **eframe/egui** - GUI framework
- **sysinfo** - Process detection
- **serde/serde_json** - Config serialization
- **anyhow** - Error handling
- **dirs** - Config path

### Windows-specific
- **windows** crate - Windows API bindings
  - Win32_System_Threading
  - Win32_System_ProcessStatus
  - Win32_System_Memory

## 🔒 Security Considerations

### Windows API Usage

```rust
// ✅ SAFE - Public Windows APIs
SetPriorityClass()       // Change CPU priority
K32EmptyWorkingSet()     // Trim memory
OpenProcess()            // Get process handle
CloseHandle()            // Cleanup handle

// ❌ FORBIDDEN - ToS violations
WriteProcessMemory()     // Code injection
ReadProcessMemory()      // Memory reading
VirtualAllocEx()         // Memory allocation in process
CreateRemoteThread()     // Execute in process
```

### Handle Management

**CRITICAL**: Always close handles!

```rust
unsafe {
    let handle = OpenProcess(...)?;
    
    // Do work...
    
    CloseHandle(handle).ok();  // ← MUST DO
}
```

Memory leak nếu không close → process sẽ chậm dần.

## 🎨 Code Style

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy -- -D warnings
```

Zero warnings policy! Fix tất cả warnings trước khi commit.

## 🚀 Release Process

1. **Update version** trong `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Run full CI**:
   ```bash
   just check  # fmt + lint + test + build
   ```
4. **Test trên Windows**
5. **Create GitHub release**
6. **Attach binary** (`roblox_booster.exe`)

## 📊 Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Binary size | < 10 MB | ~7-8 MB ✅ |
| Startup time | < 1s | ~500ms ✅ |
| Memory usage | < 50 MB | ~30 MB ✅ |
| Auto-detect scan | < 100ms | ~50ms ✅ |

## 🐛 Common Issues

### Issue: "linker not found"

```bash
# Fix
sudo apt install mingw-w64
```

### Issue: Windows API functions not found

```bash
# Check windows crate version
cargo tree | grep windows

# Re-add features nếu thiếu
```

### Issue: Binary quá lớn

```bash
# Check strip
cargo bloat --release

# Verify LTO
cat Cargo.toml | grep lto
```

## 📚 Resources

- [Rust Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [MinGW-w64](https://www.mingw-w64.org/)
- [Windows API Docs](https://docs.microsoft.com/en-us/windows/win32/)
- [egui Docs](https://docs.rs/egui/latest/egui/)

## 🤝 Contributing

1. Fork repo
2. Create feature branch
3. Make changes
4. Run `just check`
5. Submit PR

---

**Questions?** Open an issue on GitHub!