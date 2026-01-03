# 🚀 Roblox Booster

Ứng dụng tối ưu hóa hiệu suất cho Roblox với giao diện đơn giản, được viết bằng Rust với các tính năng hiện đại.

## ✨ Tính năng

- **Auto Boost**: Tự động tối ưu hiệu suất khi phát hiện Roblox đang chạy
- **Launch Roblox**: Khởi chạy Roblox trực tiếp từ app
- **Theo dõi trạng thái**: Hiển thị real-time Roblox có đang chạy hay không
- **Giao diện đơn giản**: Chỉ 2 nút chính, dễ sử dụng
- **Icon tích hợp**: Gradient xanh dương → xanh lá đẹp mắt
- **Windows Manifest**: App info đầy đủ, yêu cầu admin tự động

## 📋 Yêu cầu

- Rust 1.85.0+ ([Cài đặt Rust](https://rustup.rs/))
- Windows OS
- Roblox đã được cài đặt

## ⚡ Quick Start

### Build trong 3 bước

```bash
# 1. Tạo project
mkdir roblox_booster
cd roblox_booster
mkdir src

# 2. Copy files vào đúng vị trí
# - Cargo.toml
# - build.rs
# - src/main.rs
# - .gitignore

# 3. Build và chạy
cargo build --release
cargo run --release

# Hoặc chạy .exe trực tiếp
./target/release/roblox_booster.exe
```

### Cấu trúc thư mục

```
roblox_booster/
├── .gitignore
├── Cargo.toml
├── build.rs              # Build script tạo icon & manifest
├── README.md
├── LICENSE
└── src/
    └── main.rs
```

## 🎮 Hướng dẫn sử dụng

### 🔥 Auto Boost Button
- Click để BẬT/TẮT chế độ tự động tối ưu
- Khi BẬT: App tự động boost hiệu suất khi phát hiện Roblox
- Nút chuyển màu xanh lá khi đang bật

### 🎮 Khởi chạy Roblox Button
- Click để mở Roblox
- App tự động tìm và khởi chạy Roblox đã cài đặt

### 📊 Status Indicator
- **Chấm xanh**: Roblox đang chạy
- **Chấm xám**: Roblox chưa chạy

## 🛠️ Cách hoạt động

1. **Theo dõi processes**: Kiểm tra Roblox có đang chạy (mỗi 2 giây)
2. **Tối ưu priority**: Tăng độ ưu tiên CPU cho process Roblox
3. **Không xâm nhập**: Không can thiệp vào game, chỉ tối ưu hệ thống

## 🎨 Giao diện

```
┌─────────────────────────────┐
│   🚀 ROBLOX BOOSTER        │
│   Tối ưu hiệu suất Roblox  │
│                             │
│   ┌─────────────────────┐  │
│   │ ● Roblox đang chạy  │  │
│   └─────────────────────┘  │
│                             │
│   ┌─────────────────────┐  │
│   │ 🔥 AUTO BOOST: BẬT  │  │
│   └─────────────────────┘  │
│                             │
│   ┌─────────────────────┐  │
│   │ 🎮 KHỞI CHẠY ROBLOX │  │
│   └─────────────────────┘  │
│                             │
│   ✓ Status message...      │
└─────────────────────────────┘
```

## 🦀 Tính năng Rust hiện đại

- **Error Handling**: Sử dụng `Result` và `Option` đầy đủ
- **Pattern Matching**: Match expressions cho logic rõ ràng
- **Iterators**: Functional programming với `map`, `filter`
- **Arc + Mutex**: Thread-safe state management
- **Const Generics**: Compile-time optimizations
- **Type Safety**: Strong typing cho reliability
- **Zero-cost Abstractions**: Performance không trade-off

## 🔨 Build Commands

```bash
# Kiểm tra code
cargo check

# Format code
cargo fmt

# Run clippy (linter)
cargo clippy

# Build release
cargo build --release

# Build với logs
cargo build --release --verbose

# Clean build
cargo clean && cargo build --release
```

## 📦 Dependencies

- `eframe` & `egui`: Modern GUI framework
- `sysinfo`: System process monitoring
- `windows`: Windows API bindings
- `anyhow`: Error handling (build.rs)
- `embed-resource`: Embed manifest (build.rs)

## ⚡ Optimizations

App được tối ưu với:
- **LTO**: Link Time Optimization
- **Strip symbols**: Loại bỏ debug info
- **Codegen units = 1**: Maximum optimization
- **opt-level = 3**: Aggressive optimizations
- **Build script**: Generate icon & manifest at compile time
- **Const functions**: Compile-time computations

**Build size**: ~2-3 MB sau optimization

## 🚀 Performance

- **Startup time**: < 1 giây
- **Memory usage**: ~10-20 MB
- **CPU usage**: ~0% idle, < 1% active
- **Process detection**: Mỗi 2 giây
- **Auto-boost latency**: < 2 giây

## 🎨 Custom Icon

Icon gradient được generate bởi build script:
- 32x32 pixels
- Gradient: Xanh dương `#3498db` → Xanh lá `#2ecc71`
- Border trắng, background transparent

**Thay đổi màu** trong `build.rs`:
```rust
// Tìm dòng này và thay đổi RGB values
let r = (52.0 * (1.0 - t) + 46.0 * t) as u8;
let g = (152.0 * (1.0 - t) + 204.0 * t) as u8;
let b = (219.0 * (1.0 - t) + 113.0 * t) as u8;
```

**Gợi ý màu:**
- 🔴 Đỏ neon: `255, 0, 85` → `255, 85, 85`
- 💜 Tím galaxy: `102, 126, 234` → `118, 75, 162`
- 🟢 Matrix: `0, 255, 65` → `0, 183, 18`
- 🔵 Cyber: `0, 212, 255` → `0, 128, 255`

## ❓ Troubleshooting

**Lỗi compile:**
```bash
cargo update
cargo clean
cargo build --release
```

**Không launch được Roblox:**
- Kiểm tra Roblox đã cài từ Microsoft Store
- Thử mở Roblox thủ công trước

**Auto Boost không hoạt động:**
- Chạy app với quyền Administrator
- Kiểm tra Roblox đã khởi động chưa

**Build script errors:**
```bash
# Install build dependencies nếu cần
cargo install embed-resource
```

## 🔒 An toàn

- ✅ **Không cheat**: App không can thiệp vào game
- ✅ **Không ban**: Chỉ tối ưu system, tuân thủ ToS
- ✅ **Open source**: Code minh bạch, có thể review
- ✅ **No data collection**: Không thu thập dữ liệu

## 📝 License

MIT License - Tự do sử dụng và chỉnh sửa

## 🤝 Contributing

Mọi đóng góp đều được chào đón! Tạo issue hoặc pull request.

### Development Guidelines

```bash
# Format code trước khi commit
cargo fmt

# Chạy linter
cargo clippy -- -W clippy::all

# Test build
cargo build --release

# Check for common issues
cargo audit
```

## 💡 Tips & Tricks

- **First build lâu** do download dependencies (~2-5 phút)
- **Rebuild nhanh** khi chỉ đổi code (~5-10 giây)
- **Release build** nhanh và nhỏ hơn debug rất nhiều
- **Chạy với Admin** để boost hiệu quả nhất
- **Icon tự động** generate mỗi lần build

## 🔗 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [egui Documentation](https://docs.rs/egui/)
- [Windows Crate](https://docs.rs/windows/)

## 🌟 Features Roadmap

- [ ] Multi-language support
- [ ] Custom boost profiles
- [ ] System-wide optimizations
- [ ] GPU monitoring
- [ ] Network optimization
- [ ] Portable mode (no install)

---

**Made with ❤️ using Rust 🦀**

*Lưu ý: App này chỉ tối ưu hiệu suất hệ thống, không can thiệp vào gameplay hay vi phạm ToS của Roblox.*