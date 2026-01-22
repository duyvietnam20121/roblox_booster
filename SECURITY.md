# 🔒 Security & Safety

## Cam kết an toàn

Roblox Booster được thiết kế với nguyên tắc **"tối thiểu quyền truy cập"** (Principle of Least Privilege).

## ✅ Những gì chúng tôi LÀM

### 1. CPU Priority Adjustment (SetPriorityClass)
```rust
SetPriorityClass(handle, ABOVE_NORMAL_PRIORITY_CLASS)
```

**Mô tả**: Thay đổi mức độ ưu tiên CPU của process Roblox.

**An toàn**: 
- ✅ Public Windows API
- ✅ Không truy cập memory
- ✅ Không thay đổi code
- ✅ Có thể revert bất cứ lúc nào

**Tương đương**: Click phải process trong Task Manager → Set Priority

---

## ❌ Những gì chúng tôi KHÔNG LÀM

### 1. Memory Reading
```rust
// ❌ KHÔNG BAO GIỜ làm
ReadProcessMemory(handle, address, buffer, size, ...)
```
**Lý do cấm**: Đọc memory = vi phạm Roblox ToS

### 2. Memory Writing
```rust
// ❌ KHÔNG BAO GIỜ làm
WriteProcessMemory(handle, address, data, size, ...)
```
**Lý do cấm**: Ghi memory = cheating

### 3. Code Injection
```rust
// ❌ KHÔNG BAO GIỜ làm
CreateRemoteThread(handle, ...)
VirtualAllocEx(handle, ...)
```
**Lý do cấm**: Inject code = hack

### 4. DLL Injection
```rust
// ❌ KHÔNG BAO GIỜ làm
LoadLibrary() trong context của Roblox
```
**Lý do cấm**: DLL injection = exploit

### 5. Hook/Detour
```rust
// ❌ KHÔNG BAO GIỜ làm
Detour/Hook Windows APIs hoặc game functions
```
**Lý do cấm**: Thay đổi behavior = cheat

---

## 🛡️ Process Verification

### Bảo vệ chống boost nhầm process

```rust
// Chỉ chấp nhận
name.contains("roblox")
    && !name.contains("studio")      // Không phải Studio
    && !name.contains("install")     // Không phải Installer
    && !name.contains("crash")       // Không phải Crash Reporter
```

**Mục đích**: Đảm bảo chỉ boost đúng process Roblox Player.

---

## 🔐 Permissions Required

### Windows API Permissions

```rust
OpenProcess(PROCESS_SET_INFORMATION, false, pid)
```

**Required Permission**: `PROCESS_SET_INFORMATION`

**Minimal Permission**: Chỉ đủ để set priority, không đủ để:
- Đọc memory
- Ghi memory
- Terminate process
- Create threads

### Admin Rights

**Không bắt buộc** cho hầu hết trường hợp.

**Khi cần admin**:
- Roblox chạy với elevated privileges
- User account control (UAC) enabled

---

## 🚨 False Positive Handling

### Tại sao antivirus có thể cảnh báo?

1. **Truy cập process khác** (OpenProcess)
   - Giải pháp: Windows metadata trong build.rs
   
2. **Unsigned binary**
   - Giải pháp: Code signing (optional, $200/năm)
   
3. **Heuristic detection**
   - Giải pháp: Build từ source, verify hash

### Mitigations

1. ✅ **Windows Resource Metadata** (build.rs)
   ```toml
   ProductName = "Roblox Booster"
   FileDescription = "Safe Roblox performance optimizer"
   CompanyName = "Made by AI"
   ```

2. ✅ **Open Source**
   - Full source code available
   - Community audit
   - Reproducible builds

3. ✅ **Clear Documentation**
   - Explain what we do
   - Explain what we DON'T do

---

## 📝 Roblox ToS Compliance

### Rule: "No unauthorized access"

**Chúng tôi tuân thủ**:
- ✅ SetPriorityClass là authorized (public Windows API)
- ✅ Không truy cập game memory
- ✅ Không modify game files
- ✅ Không bypass anti-cheat

### Comparable to:

| Action | Roblox Booster | Allowed? |
|--------|----------------|----------|
| Close Roblox từ Task Manager | Set priority từ code | ✅ YES |
| Install graphics driver | Install performance tool | ✅ YES |
| Overclock CPU | Boost CPU priority | ✅ YES |
| Use cheat engine | - | ❌ NO |
| Inject DLL | - | ❌ NO |

---

## 🔍 Code Audit

### Critical Section

File: `src/booster.rs`

```rust
unsafe {
    // Open with minimal permission
    let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;
    
    // ONLY set priority
    let result = SetPriorityClass(handle, priority);
    
    // ALWAYS cleanup
    CloseHandle(handle).ok();
    
    result?;
}
```

**Audit Points**:
1. ✅ Minimal permission (PROCESS_SET_INFORMATION)
2. ✅ Single operation (SetPriorityClass)
3. ✅ No memory access
4. ✅ Handle cleanup
5. ✅ Error propagation

---

## 🧪 Testing Safety

### Test trên VM/Sandbox

```bash
# 1. Build
cargo build --release

# 2. Copy to Windows VM

# 3. Run Process Monitor (Sysinternals)
# Verify ONLY these operations:
#   - OpenProcess
#   - SetPriorityClass
#   - CloseHandle

# 4. NO forbidden operations:
#   - ReadProcessMemory ❌
#   - WriteProcessMemory ❌
#   - VirtualAllocEx ❌
#   - CreateRemoteThread ❌
```

---

## 📞 Reporting Security Issues

Nếu bạn tìm thấy vấn đề bảo mật:

1. **KHÔNG** open public issue
2. Email: [your-security-email]
3. Describe issue chi tiết
4. We'll respond trong 48h

---

## 🎓 Educational Purpose

Roblox Booster là công cụ **educational** để:
- ✅ Học Rust systems programming
- ✅ Học Windows API
- ✅ Học best practices cho tool development

**Không phải**:
- ❌ Cheat tool
- ❌ Exploit framework
- ❌ Anti-cheat bypass

---

## ✅ Verification Checklist

Trước khi release, verify:

- [ ] Chỉ dùng SetPriorityClass
- [ ] Không import memory APIs
- [ ] Không inject code
- [ ] Process verification implemented
- [ ] Handle cleanup guaranteed
- [ ] Error handling proper
- [ ] Documentation updated
- [ ] Open source
- [ ] Windows metadata present

---

**Last Updated**: 2026-01-22
**Version**: 0.1.0