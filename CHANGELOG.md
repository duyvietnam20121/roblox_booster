# Changelog

All notable changes to Roblox Booster will be documented in this file.

## [0.1.0] - 2026-01-05

### ✨ Added
- **Smart Process Detection**: Enhanced pattern matching for Roblox processes
  - Detects "roblox" and "rbx" variants
  - Filters out installers and uninstallers
  - Better false positive prevention

- **Working Set Optimization**: Memory trimming per process
  - Uses `SetProcessWorkingSetSize` API
  - Reduces memory footprint
  - Improves cache locality

- **Optimization Statistics**: Real-time metrics tracking
  - Process count optimized
  - Priority level displayed
  - Memory optimization status

- **Enhanced Error Handling**: Typed errors with `thiserror`
  - `BoosterError` enum for specific error types
  - Better error messages with context
  - Graceful degradation on partial failures

- **Two-Phase Optimization**: 
  - Phase 1: Process priority boost
  - Phase 2: System memory optimization
  - Independent error handling per phase

- **Improved UI**: 
  - Status messages with emojis
  - Scrollable status area
  - Color-coded buttons
  - Detailed optimization stats display

### 🛠️ Changed
- Replaced `Result<String, String>` with `anyhow::Result`
- Improved process detection logic
- Enhanced cleanup on application exit
- Better error messages with full context

### 🐛 Fixed
- Fixed unused `BoosterError` enum warning
- Better handle permission errors
- Improved process handle cleanup
- Fixed race conditions in auto-detect

### 🔒 Security
- Safe handle management with RAII pattern
- Proper error propagation
- No unsafe code outside Windows API calls

### ⚡ Performance
- Optimized process iteration
- Reduced system calls
- Better memory allocation patterns
- Efficient HashSet usage

---

## Technical Details

### API Changes
```rust
// Old
pub fn enable(&mut self) -> Result<String, String>

// New
pub fn enable(&mut self) -> Result<String>  // anyhow::Result
```

### New Error Types
```rust
pub enum BoosterError {
    ProcessOpen { pid: u32, reason: String },
    PrioritySet { pid: u32, reason: String },
    MemoryOptimize { pid: u32, reason: String },
    PlatformUnsupported(String),
    NoProcessesFound,
}
```

### Optimization Stats
```rust
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    pub priority_level: u8,
}
```

---

## Migration Guide

No breaking changes for users. All improvements are backward compatible with existing config files.

For developers extending the code:
- Replace string-based errors with typed `BoosterError`
- Use `anyhow::Result` instead of `Result<T, String>`
- Access optimization stats via `booster.get_stats()`