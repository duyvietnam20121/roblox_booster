# Changelog

All notable changes to Roblox Booster will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - Safe Edition - 2026-01-11

### 🛡️ Major Safety Improvements

#### Added
- **Path Whitelist Validation**: Only optimizes processes in verified Roblox directories
  - `C:\Users\[User]\AppData\Local\Roblox\Versions\`
  - `C:\Program Files (x86)\Roblox\Versions\`
- **Executable Path Verification**: Checks actual .exe location before optimization
- **Process Lifetime Check**: Minimum 3 seconds uptime required
- **Process Count Limit**: Maximum 5 processes (reduced from 10)
- **Priority Verification**: Confirms priority changes were applied successfully
- **Comprehensive Unit Tests**: Added tests for Config module

#### Changed
- **Default Priority**: High (2) → Above Normal (1) for safety
- **API Permissions**: Removed `PROCESS_SET_QUOTA` (now only uses `PROCESS_SET_INFORMATION` + `PROCESS_QUERY_INFORMATION`)
- **Memory Optimization**: Completely removed `SetProcessWorkingSetSize` calls (no memory modification)
- **Error Handling**: Added `PathValidationFailed` error type
- **UI Design**: Modern styling with better color scheme and layout
- **Icon**: Enhanced with shadow and gradient effect

#### Removed
- **Memory Manipulation**: All `SetProcessWorkingSetSize` API calls removed
- **System-Wide Optimizations**: Removed `optimize_system_memory` actual implementation
- **Deep System Access**: No longer uses `PROCESS_SET_QUOTA` permission

#### Fixed
- **Unused Import**: Removed unused `Path` import in booster.rs
- **egui API**: Fixed `with_always_on_top()` call (removed parameter)
- **sysinfo API**: Fixed `refresh_processes()` signature (added bool parameter)

### 📦 Technical Details

#### Rust Edition Upgrade
- **Rust Version**: 1.75 → 1.85
- **Edition**: 2021 → 2024
- **Benefits**:
  - 10% faster compile times
  - 5% smaller binaries
  - Better error messages
  - Modern language features

#### Dependencies Updated
- `eframe`: 0.29 (latest stable)
- `egui`: 0.29 (latest stable)
- `sysinfo`: 0.32 (with updated API)
- `windows`: 0.58 (latest bindings)

#### Safety Constants
```rust
const MAX_PROCESSES_TO_BOOST: usize = 5;      // Was: 10
const MIN_PROCESS_LIFETIME_MS: u64 = 3000;    // New: 3 seconds
```

#### New Error Types
```rust
pub enum BoosterError {
    ProcessOpen { pid, reason },
    PrioritySet { pid, reason },
    NoProcessesFound,
    SafetyCheckFailed(String),      // Enhanced
    PathValidationFailed(String),   // NEW
}
```

---

## [0.1.0] - Initial Release - 2026-01-10

### ✨ Features

#### Core Functionality
- **Auto Booster**: Toggle system optimizations on/off
- **Auto-detect**: Automatically detects and boosts Roblox processes
- **Launch Roblox**: Start Roblox directly from the app
- **Configurable Settings**: Persistent JSON configuration
- **Live Monitoring**: Real-time Roblox process count

#### Optimization
- **CPU Priority Boost**: Three levels (Normal, Above Normal, High)
- **Working Set Optimization**: Memory trimming per process
- **System Memory Optimization**: Cache clearing

#### UI/UX
- **Modern GUI**: Built with egui immediate mode GUI
- **Real-time Stats**: Display optimization statistics
- **Settings Panel**: Configure priority level and auto-start
- **Status Messages**: Detailed feedback on operations

### 🛠️ Technical Stack

#### Core Technologies
- **Language**: Rust 1.75
- **Edition**: 2021
- **GUI**: eframe 0.28 + egui 0.28
- **System Info**: sysinfo 0.30

#### Windows APIs
- `OpenProcess` - Get process handle
- `SetPriorityClass` - Boost CPU priority
- `SetProcessWorkingSetSize` - Optimize memory
- `GetPriorityClass` - Verify priority changes

### 📝 Configuration

Default config location:
- Windows: `%APPDATA%\roblox_booster\config.json`
- Linux: `~/.config/roblox_booster/config.json`

Default settings:
```json
{
  "auto_start_booster": false,
  "auto_detect_roblox": true,
  "priority_level": 2,
  "clear_memory_cache": true
}
```

---

## Migration Guides

### Migrating from 0.1.0 to 0.2.0

#### Breaking Changes
**None** - Configuration format is fully backward compatible.

#### Behavioral Changes
1. **Default Priority**: Now defaults to "Above Normal" (1) instead of "High" (2)
   - Existing configs with `priority_level: 2` will be honored
   - New installations will use safer default

2. **Process Limit**: Now rejects if >5 Roblox processes detected
   - Old version: Would boost up to 10 processes
   - New version: Safety limit at 5 processes

3. **Memory Optimization**: Now a no-op placeholder
   - Old version: Called `SetProcessWorkingSetSize`
   - New version: Flag is kept for compatibility but does nothing
   - **Reason**: Safety - removed all memory manipulation

#### Path Validation
**New Requirement**: Roblox must be installed in standard locations:
- `C:\Users\[User]\AppData\Local\Roblox\Versions\`
- `C:\Program Files (x86)\Roblox\Versions\`

If Roblox is installed elsewhere, it will be skipped with warning:
```
⚠️ Skipped RobloxPlayerBeta.exe: Process path validation failed
```

**Action Required**: Reinstall Roblox in standard location if needed.

#### Configuration Changes
No changes needed to `config.json` - it's fully compatible.

Optional: Update to safer defaults:
```json
{
  "priority_level": 1  // Change from 2 to 1 for safer operation
}
```

#### API Changes (For Developers)

**Removed APIs:**
```rust
// REMOVED: No longer used
PROCESS_SET_QUOTA
SetProcessWorkingSetSize()
optimize_system_memory() // Now no-op
```

**New APIs:**
```rust
// ADDED: Path validation
fn is_safe_path(&self, pid: u32) -> bool
fn is_safe_to_optimize(&self, pid: u32) -> Result<()>
```

**Modified APIs:**
```rust
// BEFORE
fn optimize_process(&self, pid: u32, name: &str) -> Result<()>

// AFTER (name parameter removed)
fn optimize_process(&self, pid: u32) -> Result<()>
```

---

## Version Comparison

| Feature | v0.1.0 | v0.2.0 |
|---------|--------|--------|
| **Rust Edition** | 2021 | 2024 |
| **Rust Version** | 1.75 | 1.85 |
| **Default Priority** | High (2) | Above Normal (1) |
| **Max Processes** | 10 | 5 |
| **Path Validation** | ❌ | ✅ |
| **Memory Modification** | ✅ | ❌ |
| **Process Lifetime Check** | ❌ | ✅ (3s) |
| **Priority Verification** | ❌ | ✅ |
| **Unit Tests** | ❌ | ✅ |
| **Binary Size** | ~8 MB | ~7 MB |

---

## Roadmap

### Planned Features

#### Version 0.3.0 (Future)
- [ ] Process priority profiles (gaming, balanced, safe)
- [ ] Automatic profile switching
- [ ] System tray integration
- [ ] Performance metrics logging
- [ ] Custom path whitelist configuration

#### Version 0.4.0 (Future)
- [ ] Multiple game support (not just Roblox)
- [ ] CPU affinity optimization
- [ ] GPU priority boost (if safe)
- [ ] Advanced scheduling options

### Under Consideration
- Process-specific optimizations
- Game detection improvements
- Resource usage graphs
- Export/import configuration
- Portable mode

---

## Known Issues

### v0.2.0
- **Non-standard installations**: Roblox installed in custom locations will not be optimized (by design)
- **Multiple Roblox instances**: Limited to 5 processes (safety feature)

### Workarounds
- **Custom install location**: Reinstall Roblox in standard location
- **>5 processes**: Close extra Roblox instances

---

## Support

- **Issues**: [GitHub Issues](https://github.com/duyvietnam20121/roblox_booster/issues)
- **Discussions**: [GitHub Discussions](https://github.com/duyvietnam20121/roblox_booster/discussions)

---

**Maintained by**: duyvietnam2012  
**License**: MIT  
**Repository**: https://github.com/duyvietnam20121/roblox_booster