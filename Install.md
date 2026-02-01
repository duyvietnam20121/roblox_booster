# 🛠️ HƯỚNG DẪN CÀI ĐẶT HOÀN CHỈNH

## 📋 Checklist Trước Khi Bắt Đầu

- [ ] Windows 10/11 (64-bit)
- [ ] Kết nối Internet ổn định
- [ ] ~2GB dung lượng trống
- [ ] Quyền Administrator

---

## BƯỚC 1: CÀI ĐẶT RUST

### Option A: Sử dụng Rustup (Khuyến nghị)

1. **Tải Rustup:**
   - Truy cập: https://rustup.rs/
   - Click **"Download rustup-init.exe (64-bit)"**

2. **Chạy Installer:**
   ```
   - Double-click rustup-init.exe
   - Chọn "1) Proceed with installation (default)"
   - Đợi cài đặt hoàn tất (~5 phút)
   ```

3. **Verify Installation:**
   ```bash
   # Mở Command Prompt mới
   rustc --version
   
   # Phải thấy output:
   # rustc 1.75.0 (hoặc cao hơn)
   ```

### Option B: Sử dụng Winget

```bash
# Mở PowerShell
winget install Rustlang.Rustup
```

---

## BƯỚC 2: CÀI ĐẶT MINGW-W64

### Option A: Sử dụng MSYS2 (Khuyến nghị)

1. **Tải MSYS2:**
   - Truy cập: https://www.msys2.org/
   - Download: `msys2-x86_64-latest.exe`

2. **Cài Đặt MSYS2:**
   ```
   - Double-click installer
   - Chọn folder: C:\msys64 (default)
   - Finish và tick "Run MSYS2 now"
   ```

3. **Update Package Database:**
   ```bash
   # Trong MSYS2 terminal (màu tím)
   pacman -Syu
   
   # Nếu terminal đóng, mở lại và chạy:
   pacman -Su
   ```

4. **Cài MinGW-w64 Toolchain:**
   ```bash
   # Cài compiler và tools
   pacman -S mingw-w64-x86_64-gcc
   pacman -S mingw-w64-x86_64-toolchain
   
   # Verify
   where gcc
   # Phải thấy: C:\msys64\mingw64\bin\gcc.exe
   ```

5. **Thêm vào PATH:**
   ```
   - Windows Key + R → sysdm.cpl → Enter
   - Tab "Advanced" → "Environment Variables"
   - Trong "System variables", chọn "Path" → Edit
   - New → Thêm: C:\msys64\mingw64\bin
   - OK → OK → OK
   ```

6. **Verify PATH:**
   ```bash
   # Mở Command Prompt MỚI
   x86_64-w64-mingw32-gcc --version
   
   # Phải thấy output:
   # gcc.exe (Rev10, Built by MSYS2 project) 13.x.x
   ```

### Option B: Sử dụng Winget

```bash
winget install MSYS2.MSYS2
# Sau đó follow steps 3-6 ở trên
```

### Option C: Download Trực Tiếp

1. Download từ: https://github.com/niXman/mingw-builds-binaries/releases
2. Tìm file: `x86_64-13.x.x-release-posix-seh-ucrt-rt_vxx-revx.7z`
3. Giải nén vào `C:\mingw64`
4. Thêm `C:\mingw64\bin` vào PATH

---

## BƯỚC 3: CẤU HÌNH RUST CHO GNU TOOLCHAIN

```bash
# Add target Windows GNU
rustup target add x86_64-pc-windows-gnu

# Verify
rustup target list | findstr installed

# Phải thấy:
# x86_64-pc-windows-gnu (installed)
```

---

## BƯỚC 4: TẢI VÀ GIẢI NÉN PROJECT

### Option A: Download ZIP

1. Tải project ZIP
2. Giải nén vào folder (VD: `C:\Users\YourName\roblox_booster`)
3. Verify structure:
   ```
   roblox_booster/
   ├── .gitignore
   ├── Cargo.toml
   ├── README.md
   ├── build.bat
   ├── run_admin.bat
   └── src/
       ├── main.rs
       ├── booster.rs
       ├── ui.rs
       └── config.rs
   ```

### Option B: Git Clone (nếu có Git)

```bash
git clone <repository_url>
cd roblox_booster
```

---

## BƯỚC 5: BUILD PROJECT

### Cách 1: Sử dụng Build Script (Dễ nhất)

```bash
# Mở Command Prompt trong thư mục project
cd C:\Users\YourName\roblox_booster

# Chạy build script
build.bat

# Script sẽ tự động:
# - Check dependencies
# - Clean old builds
# - Compile release version
```

### Cách 2: Manual Build

```bash
# Mở Command Prompt
cd C:\Users\YourName\roblox_booster

# Clean (optional)
cargo clean

# Build release
cargo build --release --target x86_64-pc-windows-gnu
```

### ⏱️ Thời Gian Build

- **Lần đầu**: 5-10 phút (download dependencies)
- **Lần sau**: 1-2 phút (chỉ compile changed code)

### ✅ Build Thành Công

Nếu thành công, bạn sẽ thấy:
```
   Compiling roblox_booster v0.1.0
    Finished release [optimized] target(s) in 3m 45s
```

Executable sẽ ở:
```
target\x86_64-pc-windows-gnu\release\roblox_booster.exe
```

---

## BƯỚC 6: CHẠY ỨNG DỤNG

### ⚠️ QUAN TRỌNG: Cần Quyền Administrator!

### Cách 1: Sử dụng Run Script (Khuyến nghị)

```bash
# Double-click file:
run_admin.bat

# Script sẽ tự động request admin rights
```

### Cách 2: Manual

1. Mở File Explorer
2. Navigate to: `target\x86_64-pc-windows-gnu\release\`
3. **Right-click** `roblox_booster.exe`
4. Chọn **"Run as administrator"**
5. UAC prompt → Yes

### Cách 3: PowerShell

```powershell
Start-Process "target\x86_64-pc-windows-gnu\release\roblox_booster.exe" -Verb RunAs
```

---

## BƯỚC 7: CẤU HÌNH LẦN ĐẦU

1. **Mở Settings:**
   - Click nút **"⚙️ SETTINGS"** trong app

2. **Chọn Features:**
   - ✅ **Timer Resolution** (Khuyến nghị: BẬT)
   - ✅ **GPU Priority** (Khuyến nghị: BẬT)
   - ✅ **Memory Cleanup** (BẬT nếu RAM < 16GB)
   - ⚠️ **CPU Affinity** (CHỈ bật nếu Intel 12th gen+)

3. **Save:**
   - Click **"💾 LƯU & ÁP DỤNG"**
   - Config được lưu vào `config.json`

4. **Bật Booster:**
   - Click **"BẬT AUTO BOOSTER"**
   - Console sẽ hiển thị:
     ```
     🚀 Auto Booster đã BẬT (interval: 60s)
     ✓ Timer Resolution set to 1ms (was 15.6ms)
     ```

5. **Mở Roblox:**
   - Launch bất kỳ game nào
   - Trong vòng 60 giây, app sẽ detect và boost:
     ```
     🎮 Found Roblox: RobloxPlayerBeta.exe (PID: 12345)
       ✓ GPU Priority boosted
       ✓ Memory cleanup thực hiện
     ```

---

## 🐛 TROUBLESHOOTING

### ❌ Problem: "rustc is not recognized"

**Nguyên nhân**: Rust chưa được cài hoặc chưa trong PATH

**Giải pháp**:
```bash
# Cài lại Rust
https://rustup.rs/

# Restart terminal sau khi cài
```

### ❌ Problem: "x86_64-w64-mingw32-gcc not found"

**Nguyên nhân**: MinGW chưa trong PATH

**Giải pháp**:
```bash
# Check PATH
echo %PATH%

# Phải có: C:\msys64\mingw64\bin

# Nếu không có, thêm vào:
# Control Panel → System → Advanced → Environment Variables
```

### ❌ Problem: "error: linker `x86_64-w64-mingw32-gcc` not found"

**Nguyên nhân**: Cargo không tìm thấy linker

**Giải pháp**:
```bash
# Option 1: Thêm vào PATH (xem trên)

# Option 2: Set explicitly
set RUSTFLAGS=-C linker=x86_64-w64-mingw32-gcc
cargo build --release --target x86_64-pc-windows-gnu
```

### ❌ Problem: Build bị lỗi "access denied"

**Nguyên nhân**: Antivirus block

**Giải pháp**:
```
1. Tạm tắt Antivirus
2. Hoặc add exception:
   - C:\Users\YourName\.cargo
   - C:\Users\YourName\roblox_booster\target
```

### ❌ Problem: App chạy nhưng không boost

**Check 1**: Có chạy với admin không?
```bash
# Phải thấy UAC prompt khi chạy
# Nếu không → Right-click → Run as administrator
```

**Check 2**: Roblox có đang chạy không?
```bash
# Mở Task Manager (Ctrl+Shift+Esc)
# Tab Processes → Tìm "Roblox"
```

**Check 3**: Console có error messages không?
```bash
# Xem console output
# Nếu thấy "Access Denied" → Cần admin rights
```

### ❌ Problem: Features không có tác dụng

**Nguyên nhân**: Có thể do hardware không support

**Giải pháp**:
```
Timer Resolution: Work trên mọi Windows
GPU Priority: Cần Windows 10+
Memory Cleanup: Work trên mọi Windows
CPU Affinity: Chỉ có ý nghĩa với hybrid CPUs
```

---

## 📊 VERIFY INSTALLATION

### Test 1: Check Executable

```bash
# Navigate to build folder
cd target\x86_64-pc-windows-gnu\release

# Check file exists
dir roblox_booster.exe

# Check file size (should be ~5-10 MB)
```

### Test 2: Run Console

```bash
# Chạy từ command line
roblox_booster.exe

# Phải thấy:
# ╔══════════════════════════════════════╗
# ║     ROBLOX BOOSTER v0.1.0            ║
# ╚══════════════════════════════════════╝
```

### Test 3: Config File

```bash
# Sau khi lưu settings, check:
dir config.json

# Mở file:
notepad config.json

# Phải thấy JSON valid
```

### Test 4: Timer Resolution

```bash
# Bật booster
# Console phải show:
✓ Timer Resolution set to 1ms (was 15.6ms)

# Có thể verify bằng tool: 
# https://vvvv.org/contribution/windows-system-timer-tool
```

---

## 🎯 QUICK START SUMMARY

```bash
# 1. Cài Rust
winget install Rustlang.Rustup

# 2. Cài MinGW
winget install MSYS2.MSYS2
pacman -S mingw-w64-x86_64-gcc

# 3. Add target
rustup target add x86_64-pc-windows-gnu

# 4. Build
cd roblox_booster
build.bat

# 5. Run
run_admin.bat

# 6. Enjoy!
```

---

## 📞 SUPPORT

Nếu gặp vấn đề:

1. **Check lại từng bước** trong guide này
2. **Read error messages** carefully trong console
3. **Google the error** - Most common issues có solutions online
4. **Create issue** on GitHub với:
   - Full error message
   - `rustc --version` output
   - `x86_64-w64-mingw32-gcc --version` output
   - Build log

---

## ✅ INSTALLATION COMPLETE!

Bạn giờ có:
- ✅ Rust toolchain với GNU target
- ✅ MinGW-w64 compiler
- ✅ Roblox Booster executable
- ✅ Config file ready

**Next steps:**
- Tùy chỉnh settings theo hardware
- Test với Roblox games
- Enjoy smooth gameplay! 🎮