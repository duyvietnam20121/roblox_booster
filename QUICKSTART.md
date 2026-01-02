# 🚀 Quick Start Guide

Hướng dẫn nhanh để build và chạy Roblox Booster.

## 📋 Yêu cầu

- Rust 1.85.0+ ([Cài đặt Rust](https://rustup.rs/))
- Windows OS
- Git (tùy chọn)

## ⚡ Build trong 3 bước

### 1️⃣ Tạo project
```bash
# Tạo thư mục
mkdir roblox_booster
cd roblox_booster

# Tạo thư mục src
mkdir src
```

### 2️⃣ Copy files
Copy các files sau vào đúng vị trí:
- `.gitignore` → `roblox_booster/.gitignore`
- `Cargo.toml` → `roblox_booster/Cargo.toml`
- `README.md` → `roblox_booster/README.md`
- `src/main.rs` → `roblox_booster/src/main.rs`

### 3️⃣ Build & Run
```bash
# Build release (khuyến nghị)
cargo build --release

# Chạy app
cargo run --release

# Hoặc chạy file .exe trực tiếp
./target/release/roblox_booster.exe
```

## 🎯 Cấu trúc thư mục

```
roblox_booster/
├── .gitignore
├── Cargo.toml
├── LICENSE
├── README.md
├── QUICKSTART.md
├── ICON_CUSTOMIZATION.md
├── src/
│   └── main.rs
└── target/              (tự động tạo khi build)
    └── release/
        └── roblox_booster.exe
```

## ⚙️ Build Options

### Standard Release Build
```bash
cargo build --release
```

### Portable Build
```bash
# Static linking để chạy trên máy khác không cần cài Rust (Windows)
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-pc-windows-msvc
```

## 🐛 Nếu gặp lỗi

### Lỗi về dependencies
```bash
cargo update
cargo build --release
```

### Lỗi compile
```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

### Lỗi Rust version
```bash
# Update Rust
rustup update stable

# Kiểm tra version
rustc --version
```

## 📦 File .exe đã build

Sau khi build xong, file .exe nằm ở:
```
target/release/roblox_booster.exe
```

Bạn có thể copy file này ra desktop hoặc nơi khác để chạy độc lập!

## 🎮 Sử dụng

1. Double-click `roblox_booster.exe`
2. Click **AUTO BOOST: TẮT** để bật
3. Click **KHỞI CHẠY ROBLOX** để mở game
4. Enjoy! 🎉

## 💡 Tips

- **Build lần đầu sẽ lâu** (download dependencies) - lần sau nhanh hơn
- **Release build nhanh và nhỏ hơn** debug build rất nhiều
- **Chạy với Admin** để boost hiệu quả hơn (tùy chọn)
- **Icon tích hợp sẵn** - gradient xanh dương đến xanh lá đẹp mắt

## 🔗 Links hữu ích

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [egui Documentation](https://docs.rs/egui/)

---

**Happy Gaming! 🎮**