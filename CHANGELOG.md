# Changelog

All notable changes to Roblox Booster will be documented in this file.

## [0.1.0] - 2026-01-22

### 🎉 Initial Release

#### Added
- ✅ Auto Boost mode - Tự động tối ưu mỗi 5 giây
- ⚡ Manual Boost button
- 🎚️ 3 CPU Priority levels (Low/Medium/High)
- ⚙️ Settings dialog với persistent config
- 🔍 Auto-detect Roblox process (loại trừ Studio)
- 💾 JSON config persistence
- 📊 Real-time status display

#### Security
- 🔒 **Chỉ dùng SetPriorityClass** - Không đọc/ghi memory
- ✅ 100% tuân thủ Roblox ToS
- 🛡️ Không inject code vào game
- 📝 Windows metadata để giảm false positive

#### Technical
- 🦀 Rust 1.85 (Edition 2024)
- 🎨 egui/eframe GUI
- 🪟 Cross-compile x86_64-pc-windows-gnu
- 📦 Binary size ~7-8 MB (LTO + strip)
- ⚡ Zero-cost abstractions

### What We DON'T Do
- ❌ No memory reading/writing
- ❌ No code injection
- ❌ No game file modification
- ❌ No network calls
- ❌ No data collection

---

**Legend:**
- 🎉 Major feature
- ✅ Added
- 🔒 Security
- 🐛 Fixed
- ⚡ Performance
- 📝 Documentation