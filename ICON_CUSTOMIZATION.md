# 🎨 Icon Customization Guide

App đã có icon tích hợp sẵn (gradient xanh dương -> xanh lá), nhưng bạn có thể custom icon theo ý muốn.

## 📝 Icon hiện tại

Icon mặc định là một vòng tròn gradient 32x32 pixels:
- Màu trên: Xanh dương `#3498db` (RGB: 52, 152, 219)
- Màu dưới: Xanh lá `#2ecc71` (RGB: 46, 204, 113)
- Border: Trắng
- Background: Transparent

## 🔧 Cách custom icon

### Option 1: Thay đổi màu gradient

Mở `src/main.rs` và tìm function `create_icon()`:

```rust
// Thay đổi màu ở đây
let r = (52.0 * (1.0 - t) + 46.0 * t) as u8;   // Red channel
let g = (152.0 * (1.0 - t) + 204.0 * t) as u8; // Green channel
let b = (219.0 * (1.0 - t) + 113.0 * t) as u8; // Blue channel
```

**Ví dụ - Gradient đỏ đến cam:**
```rust
let r = (231.0 * (1.0 - t) + 255.0 * t) as u8;  // #e74c3c -> #ff9800
let g = (76.0 * (1.0 - t) + 152.0 * t) as u8;
let b = (60.0 * (1.0 - t) + 0.0 * t) as u8;
```

**Ví dụ - Gradient tím đến hồng:**
```rust
let r = (155.0 * (1.0 - t) + 233.0 * t) as u8;  // #9b59b6 -> #e91e63
let g = (89.0 * (1.0 - t) + 30.0 * t) as u8;
let b = (182.0 * (1.0 - t) + 99.0 * t) as u8;
```

### Option 2: Thay đổi kích thước

```rust
fn create_icon() -> egui::IconData {
    let size = 64; // Thay đổi từ 32 -> 64 để icon lớn hơn
    // ... rest of code
}
```

### Option 3: Icon từ file PNG

Nếu bạn muốn dùng icon từ file PNG thay vì code:

```rust
fn main() -> Result<(), eframe::Error> {
    // Load icon từ file nếu có
    let icon = if let Ok(icon_bytes) = std::fs::read("icon.png") {
        eframe::icon_data::from_png_bytes(&icon_bytes).ok()
    } else {
        // Fallback to generated icon
        Some(create_icon())
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 550.0])
            .with_resizable(false)
            .with_icon(icon.unwrap()),
        ..Default::default()
    };
    
    // ... rest of code
}
```

Sau đó đặt file `icon.png` (32x32 hoặc 64x64) cùng thư mục với `.exe`.

### Option 4: Icon vuông thay vì tròn

```rust
fn create_icon() -> egui::IconData {
    let size = 32;
    let mut rgba = Vec::with_capacity(size * size * 4);
    
    for y in 0..size {
        for x in 0..size {
            // Gradient vuông đơn giản
            let t = y as f32 / size as f32;
            let r = (52.0 * (1.0 - t) + 46.0 * t) as u8;
            let g = (152.0 * (1.0 - t) + 204.0 * t) as u8;
            let b = (219.0 * (1.0 - t) + 113.0 * t) as u8;
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    }
    
    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}
```

## 🎨 Màu gợi ý

**Gaming themes:**
- 🔴 Đỏ neon: `#ff0055` → `#ff5555`
- 💜 Tím galaxy: `#667eea` → `#764ba2`
- 🟢 Matrix: `#00ff41` → `#00b712`
- 🔵 Cyber: `#00d4ff` → `#0080ff`

**Professional:**
- 🌑 Dark blue: `#141e30` → `#243b55`
- 🌅 Sunset: `#ff6b6b` → `#feca57`
- 🌊 Ocean: `#2e3192` → `#1bffff`

## 💡 Tips

- **32x32 là standard** cho Windows app icons
- **PNG với alpha channel** nếu muốn transparent background
- **High contrast** để dễ nhìn trên taskbar
- **Rebuild** sau khi thay đổi: `cargo build --release`

## 🔗 Tools hữu ích

- [Coolors.co](https://coolors.co/) - Chọn màu gradient
- [RGB to HEX](https://www.rgbtohex.net/) - Convert màu
- [Favicon Generator](https://favicon.io/) - Tạo icon nhanh

---

**Sau khi custom, nhớ rebuild:**
```bash
cargo build --release
```