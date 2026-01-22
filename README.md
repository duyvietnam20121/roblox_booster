# 🚀 Roblox Booster

Game booster tối ưu hóa hiệu suất Roblox một cách an toàn (không vi phạm ToS).

## ✨ Tính năng

- ✅ **Auto Boost**: Tự động tối ưu khi phát hiện Roblox
- ⚡ **Manual Boost**: Boost thủ công bất cứ lúc nào
- 🎚️ **3 CPU Priority Levels**:
  - **Low**: Normal Priority (mặc định Windows)
  - **Medium**: Above Normal Priority (khuyến nghị)
  - **High**: High Priority (maximum performance)
- 💾 **Lưu settings**: Config tự động save
- 🔒 **An toàn tuyệt đối**: CHỈ thay đổi CPU priority

## 🛠️ Cách tối ưu

App sử dụng Windows API để:
1. ✅ Tăng CPU priority của process Roblox (SetPriorityClass)
2. ❌ **KHÔNG** đọc/ghi memory
3. ❌ **KHÔNG** inject code
4. ❌ **KHÔNG** truy cập sâu vào game

**100% tuân thủ Roblox ToS** - chỉ dùng system-level APIs công khai.

## 📦 Build

### Requirements
- Rust 1.85+ (Edition 2024)
- Windows 10/11 (runtime)
- **Cross-compile từ Linux**: MinGW-w64

### Build trên Windows

```bash
# Debug build
cargo build

# Release build (tối ưu)
cargo build --release
```

### Cross-compile từ Linux (Codespaces/WSL)

```bash
# Setup (chỉ lần đầu)
chmod +x setup.sh
./setup.sh

# hoặc dùng just
just setup

# Build
cargo build --release

# Binary: target/x86_64-pc-windows-gnu/release/roblox_booster.exe
```

### Sử dụng Just (recommended)

```bash
# Install just: cargo install just

just setup          # Setup lần đầu
just build          # Build release
just check          # Format + Lint + Test + Build
just build-info     # Show build info
just info           # Show binary info
```

## 🎮 Cách dùng

1. Mở Roblox
2. Chạy Roblox Booster
3. Bật **Auto Boost** hoặc nhấn **Boost Ngay**
4. Vào **Settings** để chọn optimization level

## ⚙️ Settings

- **CPU Priority Level**: Chọn mức độ ưu tiên CPU (Low/Medium/High)
- **Auto-detect Roblox**: Tự động phát hiện process Roblox

## 🎯 Priority Levels Explained

| Level | Windows Priority | Khi nào dùng |
|-------|------------------|--------------|
| Low | Normal | Chơi game nhẹ, multitasking |
| Medium | Above Normal | Khuyến nghị - Cân bằng performance/ổn định |
| High | High | Maximum FPS, chơi game nặng |

**Lưu ý**: High priority có thể làm các app khác lag nếu CPU yếu.

## 📁 Project Structure

```
roblox_booster/
├── src/
│   ├── main.rs       # Entry point + icon
│   ├── booster.rs    # Core optimization logic
│   ├── config.rs     # Config management (JSON)
│   └── ui.rs         # GUI với egui
├── Cargo.toml
├── .gitignore
└── README.md
```

## 🔐 An toàn

- ✅ Chỉ dùng Windows API công khai
- ✅ Không vi phạm Roblox ToS
- ✅ Không thu thập dữ liệu
- ✅ Open source, audit được

## ⚠️ Lưu ý

- Chỉ hoạt động trên **Windows**
- Cần **quyền admin** để boost một số process
- **Không** tương tác với game logic

## 📝 License

MIT License - Free to use

## 🤝 Contribute

Issues và PRs welcome!

---

**Made by AI** 🤖