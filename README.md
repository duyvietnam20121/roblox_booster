# 🚀 Roblox Booster

Ứng dụng tối ưu hóa hiệu suất cho Roblox với giao diện đơn giản, được viết bằng Rust.

## ✨ Tính năng

- **Auto Boost**: Tự động tối ưu hiệu suất khi phát hiện Roblox đang chạy
- **Launch Roblox**: Khởi chạy Roblox trực tiếp từ app
- **Theo dõi trạng thái**: Hiển thị real-time Roblox có đang chạy hay không
- **Giao diện đơn giản**: Chỉ 2 nút chính, dễ sử dụng

## 📋 Yêu cầu

- Rust 1.85.0 hoặc mới hơn
- Windows OS
- Roblox đã được cài đặt

## 🔧 Cài đặt

1. **Tạo project:**
```bash
mkdir roblox_booster
cd roblox_booster
```

2. **Tạo cấu trúc thư mục:**
```
roblox_booster/
├── .gitignore
├── Cargo.toml
├── README.md
├── QUICKSTART.md
└── src/
    └── main.rs
```

3. **Build project:**
```bash
# Build release (khuyến nghị)
cargo build --release
```

4. **Chạy ứng dụng:**
```bash
# Chạy trực tiếp
cargo run --release

# Hoặc chạy file .exe
./target/release/roblox_booster.exe
```

## 🎮 Hướng dẫn sử dụng

### 🔥 Auto Boost Button
- **Click để BẬT/TẮT** chế độ tự động tối ưu
- Khi BẬT: App sẽ tự động boost hiệu suất khi phát hiện Roblox
- Nút sẽ chuyển màu xanh lá khi đang bật

### 🎮 Khởi chạy Roblox Button
- Click để mở Roblox
- App sẽ tự động tìm và khởi chạy Roblox đã cài đặt

### 📊 Status Indicator
- **Chấm xanh**: Roblox đang chạy
- **Chấm xám**: Roblox chưa chạy

## 🛠️ Cách hoạt động

App này hoạt động bằng cách:
1. **Theo dõi processes**: Kiểm tra Roblox có đang chạy không (mỗi 2 giây)
2. **Tối ưu priority**: Tăng độ ưu tiên CPU cho process Roblox khi Auto Boost bật
3. **Không xâm nhập**: Không can thiệp vào game hay cheat, chỉ tối ưu hệ thống

## 📦 Dependencies

- `eframe` & `egui`: GUI framework hiện đại
- `sysinfo`: Theo dõi system processes
- `windows`: Windows API cho launch và optimization

## 📁 Project Files

```
roblox_booster/
├── .gitignore                # Git ignore file
├── Cargo.toml                # Rust package config
├── LICENSE                   # MIT License
├── README.md                 # Tài liệu chính
├── QUICKSTART.md             # Hướng dẫn nhanh
├── ICON_CUSTOMIZATION.md     # Custom icon guide
└── src/
    └── main.rs               # Source code (bao gồm icon tích hợp)
```

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

**Icon:** App có icon gradient tích hợp sẵn (xanh dương → xanh lá). Muốn custom? Xem [ICON_CUSTOMIZATION.md](ICON_CUSTOMIZATION.md)

## 🔨 Build Commands

```bash
# Kiểm tra code
cargo check

# Format code
cargo fmt

# Build release
cargo build --release

# Build portable (standalone .exe)
cargo build --release --target x86_64-pc-windows-msvc
```

## ❓ Troubleshooting

**Lỗi compile:**
```bash
# Nếu gặp lỗi về dependencies, update cargo
cargo update

# Clean và rebuild
cargo clean
cargo build --release
```

**App không launch được Roblox:**
- Kiểm tra Roblox đã cài đặt từ Microsoft Store
- Thử mở Roblox thủ công trước, sau đó dùng Auto Boost

**Auto Boost không hoạt động:**
- Kiểm tra Roblox đã khởi động chưa
- Chờ vài giây để app phát hiện process

**App crash:**
- Rebuild: `cargo clean && cargo build --release`
- Cập nhật Rust: `rustup update`

## 🔒 An toàn

- **Không cheat**: App không can thiệp vào game
- **Không ban**: Chỉ tối ưu system, không vi phạm ToS
- **Open source**: Code rõ ràng, có thể review

## ⚡ Tối ưu hóa

App đã được tối ưu với:
- **LTO (Link Time Optimization)**: Giảm size và tăng tốc độ
- **Strip symbols**: Loại bỏ debug info để file nhỏ hơn
- **Codegen units = 1**: Build chậm hơn nhưng code nhanh hơn
- **opt-level = 3**: Tối ưu hóa tối đa
- **Process refresh mỗi 2 giây**: Tiết kiệm CPU
- **Icon tích hợp sẵn**: Không cần file ngoài, gradient xanh dương-xanh lá

Build size sau tối ưu: ~2-3 MB

## 🚀 Performance

- **Startup time**: < 1 giây
- **Memory usage**: ~10-20 MB
- **CPU usage**: ~0% khi idle, < 1% khi active
- **Process detection**: Mỗi 2 giây
- **Auto-boost latency**: < 2 giây sau khi phát hiện Roblox

## 📝 License

MIT License - Tự do sử dụng và chỉnh sửa

## 🤝 Contributing

Mọi đóng góp đều được chào đón! Tạo issue hoặc pull request.

---

**Made with ❤️ using Rust 🦀**

*Lưu ý: App này chỉ tối ưu hiệu suất hệ thống, không can thiệp vào gameplay*