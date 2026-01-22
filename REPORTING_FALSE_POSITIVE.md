# 🛡️ Hướng dẫn Report False Positive

Nếu antivirus của bạn báo Roblox Booster là malware, đây là **false positive** (cảnh báo nhầm).

## 🤔 Tại sao bị false positive?

### Lý do kỹ thuật:

1. **OpenProcess() API**
   - Chúng tôi dùng để get handle của Roblox
   - Malware cũng dùng API này
   - → Antivirus cảnh báo

2. **Unsigned Binary**
   - Chưa có code signing certificate ($200/năm)
   - → Antivirus nghi ngờ

3. **Heuristic Detection**
   - App mới, chưa có reputation
   - → Antivirus cẩn thận

### Những gì chúng tôi làm:

```rust
// CHỈ làm 1 việc duy nhất
SetPriorityClass(handle, HIGH_PRIORITY_CLASS)
```

**Tương đương**: Bạn vào Task Manager → Right-click Roblox → Set Priority → High

---

## ✅ Verify an toàn

### 1. Build từ source (100% safe)

```bash
# Clone repository
git clone https://github.com/yourusername/roblox_booster
cd roblox_booster

# Build yourself
cargo build --release

# Binary: target/x86_64-pc-windows-gnu/release/roblox_booster.exe
```

**Lợi ích**: Bạn tự build, bạn biết code làm gì.

### 2. Scan với VirusTotal

Upload file lên: https://www.virustotal.com

**Expected**:
- 0-2 vendors có thể flag
- Đa số (60+) sẽ clean

### 3. Check file hash

```powershell
# Windows PowerShell
Get-FileHash roblox_booster.exe -Algorithm SHA256
```

So sánh với hash trong GitHub Release.

---

## 📝 Report False Positive

### Windows Defender

**Method 1: Qua Settings**

1. Windows Security → Virus & threat protection
2. Protection history
3. Tìm "Roblox Booster"
4. Actions → Allow on device

**Method 2: Exclude folder**

1. Windows Security → Virus & threat protection
2. Manage settings
3. Exclusions → Add or remove exclusions
4. Folder → Chọn folder chứa roblox_booster.exe

**Method 3: Report false positive**

https://www.microsoft.com/en-us/wdsi/filesubmission

- File: roblox_booster.exe
- Reason: "False positive - legitimate Windows priority setter"
- Description: "Safe tool, only uses SetPriorityClass API"

---

### Other Antiviruses

#### Avast / AVG

https://www.avast.com/false-positive-file-form.php

#### Kaspersky

https://opentip.kaspersky.com/

#### Norton

https://submit.norton.com/

#### Bitdefender

https://www.bitdefender.com/consumer/support/answer/29358/

#### McAfee

https://www.mcafee.com/enterprise/en-us/threat-center/submit-sample.html

---

## 🔍 Information to provide

Khi report, cung cấp:

**File Details:**
```
Name: roblox_booster.exe
Size: ~7-8 MB
SHA256: [get from powershell]
Publisher: Made by AI (unsigned)
```

**Description:**
```
This is a legitimate Windows performance tool for Roblox game.

What it does:
- ONLY uses SetPriorityClass() Windows API
- Changes CPU priority of Roblox process
- Same as "Set Priority" in Task Manager

What it does NOT do:
- No memory reading/writing
- No code injection
- No DLL injection
- No network activity
- No data collection

Source code: https://github.com/yourusername/roblox_booster
Security documentation: SECURITY.md
```

---

## 🛠️ Technical Evidence

### API Calls (for AV vendors)

Tool **ONLY** calls these Windows APIs:

```
OpenProcess(PROCESS_SET_INFORMATION, ...)  // Get handle
SetPriorityClass(handle, priority)         // Set priority
CloseHandle(handle)                         // Cleanup
```

**NO malicious APIs:**
- ❌ ReadProcessMemory
- ❌ WriteProcessMemory
- ❌ VirtualAllocEx
- ❌ CreateRemoteThread
- ❌ LoadLibrary (in remote process)

### Verify with Process Monitor

1. Download Sysinternals Process Monitor
2. Filter: "Process Name is roblox_booster.exe"
3. Observe operations:
   - ✅ OpenProcess
   - ✅ SetPriorityClass
   - ✅ CloseHandle
   - ❌ Nothing else

---

## 🎯 For Developers

### Add to allowlist

**Group Policy (Enterprise)**:

```
Computer Configuration
  → Windows Components
    → Microsoft Defender Antivirus
      → Exclusions
        → Path: C:\Path\To\roblox_booster.exe
```

**Registry (Advanced)**:

```powershell
# WARNING: Only if you trust the file
New-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths" `
  -Name "C:\Path\To\roblox_booster.exe" -Value 0 -PropertyType DWord
```

---

## 💡 Tips

### 1. Chờ reputation build

- Càng nhiều người dùng → AV tin tưởng hơn
- 1-2 tháng thường đủ

### 2. Download từ official source

- GitHub Releases (verified)
- Không download từ third-party sites

### 3. Check signatures

```powershell
# Verify file signature (nếu có)
Get-AuthenticodeSignature roblox_booster.exe
```

---

## 🚀 Future Plans

### Code Signing

Chúng tôi đang cân nhắc:
- EV Code Signing Certificate (~$200-400/năm)
- Instant trust từ Windows
- SmartScreen sẽ không cảnh báo

**Chi phí cao** → Chưa implement ngay

---

## ❓ FAQ

**Q: Có an toàn không?**
A: Có. Source code public, bạn có thể audit. Chỉ dùng SetPriorityClass.

**Q: Tại sao không sign?**
A: Code signing certificate đắt ($200-400/năm). Project hiện tại free.

**Q: Có thể bị ban không?**
A: Không. Tool không vi phạm Roblox ToS, chỉ thay đổi Windows system settings.

**Q: Build từ source khó không?**
A: Cần Rust toolchain, nhưng có hướng dẫn chi tiết trong README.

---

**Contact**: [GitHub Issues](https://github.com/yourusername/roblox_booster/issues)

**Last Updated**: 2026-01-22