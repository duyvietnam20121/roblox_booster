# Developer Notes - Roblox Booster

> **Lời nhắn cho developer tiếp theo**: Đây là những ghi chú quan trọng về project này. Đọc kỹ trước khi modify!

---

## 🎯 Tổng Quan Project

### **Mục đích:**
- Tối ưu hiệu suất Roblox bằng cách tăng CPU priority và optimize memory
- **KHÔNG** can thiệp vào game code, chỉ điều chỉnh system-level
- **AN TOÀN** và không vi phạm ToS của Roblox

### **Kiến trúc:**
```
roblox_booster/
├── src/
│   ├── main.rs       # Entry point + GUI setup
│   ├── booster.rs    # Core optimization logic ⚠️ CRITICAL
│   ├── config.rs     # Config management
│   └── ui.rs         # GUI rendering
├── build.rs          # Windows metadata (for AV trust)
└── Cargo.toml        # Dependencies
```

---

## ⚠️ CRITICAL - ĐỌC KỸ PHẦN NÀY

### **1. KHÔNG BAO GIỜ làm những điều sau:**

❌ **TUYỆT ĐỐI KHÔNG:**
- Inject code vào Roblox process
- Đọc/ghi memory của game
- Hook game functions
- Modify game files
- Bypass game anti-cheat

**Lý do:** Vi phạm ToS → Ban account → Legal issues

✅ **CHỈ ĐƯỢC:**
- Điều chỉnh process priority (SetPriorityClass)
- Trim working set (SetProcessWorkingSetSize)
- System-level optimizations
- Launch Roblox qua protocol handler

### **2. Windows API - Cẩn thận với UNSAFE**

```rust
// ✅ ĐÚNG - Always close handles
unsafe {
    let handle = OpenProcess(...)?;
    // ... do work ...
    CloseHandle(handle).ok(); // MUST cleanup
}

// ❌ SAI - Handle leak
unsafe {
    let handle = OpenProcess(...)?;
    // ... do work ...
    // Forgot to close! Memory leak!
}
```

**Rule:** Mọi `OpenProcess` PHẢI có `CloseHandle` tương ứng!

### **3. Error Handling Pattern**

```rust
// ✅ ĐÚNG - Use anyhow::Result
pub fn optimize_process(&self, pid: u32) -> Result<()> {
    // ... với proper error context
}

// ❌ SAI - String errors
pub fn optimize_process(&self, pid: u32) -> Result<String, String> {
    // Hard to debug, không có context
}
```

**Rule:** Luôn dùng `anyhow::Result` cho error propagation!

---

## 🔧 Code Guidelines

### **Naming Conventions:**

```rust
// Variables
let roblox_pids: HashSet<u32>     // snake_case
const BUTTON_SIZE: Vec2            // SCREAMING_SNAKE_CASE

// Functions
fn optimize_process()              // snake_case
fn is_roblox_process()            // snake_case

// Types
struct SystemBooster              // PascalCase
enum BoosterError                 // PascalCase
```

### **Performance Critical Sections:**

```rust
// ⚡ HOT PATH - Called every 2 seconds
pub fn auto_detect_and_boost(&mut self) -> Option<String> {
    self.system.refresh_processes(ProcessesToUpdate::All);
    // Keep this FAST - no heavy operations!
}

// ⚡ HOT PATH - UI rendering (60 FPS)
fn render_main_ui(&mut self, ui: &mut egui::Ui) {
    // No allocations if possible
    // Cache computed values
}
```

---

## 🐛 Common Pitfalls

### **Pitfall #1: Process Detection**

```rust
// ❌ SAI - Matches too many
if name.contains("roblox") { ... }
// Catches: RobloxBooster.exe, RobloxUninstaller.exe

// ✅ ĐÚNG - Filter carefully
fn is_roblox_process(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    (name_lower.contains("roblox") || name_lower.contains("rbx"))
        && !name_lower.contains("booster")
        && !name_lower.contains("uninstall")
}
```

### **Pitfall #2: Handle Cleanup**

```rust
// ❌ SAI - Early return without cleanup
SetPriorityClass(handle, priority)?; // Error here = handle leak!
CloseHandle(handle);

// ✅ ĐÚNG - Cleanup in all paths
SetPriorityClass(handle, priority).map_err(|e| {
    let _ = CloseHandle(handle); // Cleanup before error
    BoosterError::PrioritySet { ... }
})?;
CloseHandle(handle).ok();
```

### **Pitfall #3: Process ID Reuse**

```rust
// ⚠️ WARNING - PIDs can be reused by OS
// Always verify process is still alive!

self.roblox_pids.retain(|&pid| {
    self.system.process(Pid::from_u32(pid)).is_some()
});
```

---

## 🔍 Debugging Tips

### **Enable Logging:**

```rust
// Uncomment để debug
eprintln!("Optimizing PID {pid}: {name}");
eprintln!("Current priority: {current:?}");
```

### **Check Process Priority (Windows):**

```powershell
# PowerShell - Check if priority changed
Get-Process | Where-Object {$_.ProcessName -like "*Roblox*"} | Select-Object ProcessName, PriorityClass
```

### **Test Without Roblox:**

```rust
// Test với notepad.exe thay vì Roblox
fn is_roblox_process(name: &str) -> bool {
    name.to_lowercase().contains("notepad") // For testing
}
```

---

## 📊 Performance Targets

### **Compile Time:**
- Cold build: < 2m 30s
- Incremental: < 10s
- CI full check: < 5m

### **Runtime:**
- UI frame time: < 16ms (60 FPS)
- Auto-detect scan: < 100ms
- Process boost: < 50ms

### **Binary Size:**
- Release build: < 10 MB
- With LTO: < 8 MB

---

## 🚨 Security Considerations

### **Antivirus False Positives:**

**Why we get flagged:**
1. Uses `OpenProcess` (common in malware)
2. Uses `SetPriorityClass` (suspicious API)
3. Cross-compiled from Linux (unusual)
4. No code signing certificate

**Mitigations:**
- Comprehensive metadata in `build.rs`
- Clear documentation in `SECURITY.md`
- Open source code for auditing
- Report false positives to AV vendors

### **User Data Privacy:**

```rust
// ✅ Config lưu local only
// %APPDATA%\roblox_booster\config.json

// ❌ KHÔNG BAO GIỜ:
// - Send data to server
// - Collect telemetry
// - Track usage
```

---

## 🔄 Adding New Features

### **Checklist:**

- [ ] Code compiles without warnings
- [ ] Passes `cargo clippy -- -D warnings`
- [ ] Formatted with `cargo fmt`
- [ ] No unsafe code (unless absolutely necessary)
- [ ] Error handling with proper context
- [ ] Comments for complex logic
- [ ] Update CHANGELOG.md
- [ ] Test on Windows

### **Example - Adding New Optimization:**

```rust
// 1. Add to OptimizationStats
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    pub new_feature_enabled: bool, // Add here
}

// 2. Implement in SystemBooster
fn enable_new_feature(&self) -> Result<()> {
    // Implementation
    Ok(())
}

// 3. Call in enable()
if self.config.enable_new_feature {
    self.enable_new_feature()?;
    stats.new_feature_enabled = true;
}

// 4. Add config option
pub struct Config {
    pub enable_new_feature: bool,
}

// 5. Add UI toggle in settings
```

---

## 🧪 Testing

### **Manual Test Checklist:**

```bash
# 1. Build
cargo build --release

# 2. Run without Roblox
./target/release/roblox_booster.exe
# Should show "No Roblox processes found"

# 3. Launch Roblox
# Click "Launch Roblox" button

# 4. Enable booster
# Click "Booster: OFF" to enable

# 5. Check Task Manager
# Verify Roblox priority is "High"

# 6. Settings test
# Change priority, save, restart app
# Verify settings persisted

# 7. Auto-detect test
# Close and relaunch Roblox
# Should auto-boost if enabled
```

### **CI Tests:**

```bash
just ci  # Runs: fmt-check, lint, test, build
```

---

## 📚 Dependencies Management

### **Update Policy:**

```bash
# Check for updates
cargo outdated

# Update within semver
cargo update

# Major updates - REVIEW CAREFULLY
cargo upgrade
```

### **Critical Dependencies:**

| Crate | Why Critical | Notes |
|-------|--------------|-------|
| `windows` | Core Windows API | Pin to 0.58.x |
| `sysinfo` | Process detection | Test thoroughly |
| `eframe/egui` | GUI framework | Breaking UI changes |

---

## 🎯 Future Improvements

### **Easy Wins:**

1. **Add CPU affinity control**
```rust
// Pin Roblox to specific cores
SetProcessAffinityMask(handle, mask);
```

2. **GPU priority boost**
```rust
// Windows 10+ only
SetPriorityClass(handle, REALTIME_PRIORITY_CLASS);
// ⚠️ Be careful - can freeze system
```

3. **Network QoS optimization**
```rust
// Reduce network latency
// Requires admin rights
```

### **Complex Features:**

1. **Per-game profiles**
   - Save different settings for different games
   - Auto-detect game from window title

2. **Performance monitoring**
   - Track FPS improvements
   - Show before/after metrics

3. **Scheduled optimization**
   - Boost during gaming hours
   - Normal priority when idle

---

## ⚡ Performance Optimization Tips

### **1. Reduce Allocations:**

```rust
// ❌ Allocates every frame
let text = format!("FPS: {}", fps);

// ✅ Pre-allocated or static
const PREFIX: &str = "FPS: ";
// Or use a buffer
```

### **2. Cache System Queries:**

```rust
// ❌ Query every frame
if self.system.processes().count() > 0 { ... }

// ✅ Cache and update periodically
if self.last_check.elapsed() > Duration::from_secs(2) {
    self.cached_count = self.system.processes().count();
}
```

### **3. Minimize Refreshes:**

```rust
// ❌ Refresh all processes
self.system.refresh_all();

// ✅ Refresh only what's needed
self.system.refresh_processes(ProcessesToUpdate::All);
```

---

## 📞 Contact & Support

### **Bug Reports:**
1. Check existing issues on GitHub
2. Provide full error message
3. Include system info (Windows version, Rust version)
4. Steps to reproduce

### **Feature Requests:**
1. Explain the use case
2. Describe expected behavior
3. Consider performance impact
4. Check if conflicts with ToS

### **Security Issues:**
**DO NOT** open public issue!
- Email: security@example.com (replace with actual)
- Or create private security advisory on GitHub

---

## 🎓 Learning Resources

### **Windows API:**
- [Windows Process API](https://learn.microsoft.com/en-us/windows/win32/procthread/)
- [Process Priority Classes](https://learn.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities)

### **Rust:**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Edition 2024 Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)

### **GUI (egui):**
- [egui Documentation](https://docs.rs/egui/)
- [egui Examples](https://github.com/emilk/egui/tree/master/examples)

---

## 💡 Final Tips

### **DO:**
- ✅ Write clean, readable code
- ✅ Add comments for complex logic
- ✅ Handle errors properly
- ✅ Test on real Windows
- ✅ Keep it simple and safe

### **DON'T:**
- ❌ Add unnecessary dependencies
- ❌ Use unsafe without good reason
- ❌ Ignore clippy warnings
- ❌ Forget to close handles
- ❌ Violate Roblox ToS

---

## 🙏 Thank You!

Cảm ơn đã đọc đến đây! Project này được xây dựng với:
- ❤️ Passion for performance optimization
- 🦀 Love for Rust programming
- 🎮 Respect for gaming community
- 🔒 Commitment to safety and security

**Remember:** With great power comes great responsibility. Use these optimizations wisely!

---

**Last Updated:** 2026-01-10
**Maintainer:** AI Assistant
**Rust Version:** 1.85 (Edition 2024)
**Status:** ✅ Production Ready

**Questions?** Read the docs, check issues, or ask the community!

**Good luck coding! 🚀**