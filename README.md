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
├── src/
│   └── main.rs
└── assets/
    └── icon.png (optional)
```

3. **Build project:**
```bash
# Build release (khuyến nghị)
cargo build --release
```

4. **Chạy ứng dụng:**
```bash
cargo run --release
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

## 📝 License

MIT License - Tự do sử dụng và chỉnh sửa

## 🤝 Contributing

Mọi đóng góp đều được chào đón! Tạo issue hoặc pull request.

---

**Made with ❤️ using Rust 🦀**

*Lưu ý: App này chỉ tối ưu hiệu suất hệ thống, không can thiệp vào gameplay*