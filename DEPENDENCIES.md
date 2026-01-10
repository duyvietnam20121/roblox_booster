# Dependencies Audit

## 📦 Production Dependencies

| Crate | Version | Purpose | Why Essential |
|-------|---------|---------|---------------|
| **eframe** | 0.29 | GUI framework runner | Core application framework |
| **egui** | 0.29 | Immediate mode GUI | User interface rendering |
| **sysinfo** | 0.32 | System information | Process monitoring & detection |
| **serde** | 1.0 | Serialization framework | Config serialization |
| **serde_json** | 1.0 | JSON support | Config file format |
| **anyhow** | 1.0 | Error handling | Ergonomic error propagation |
| **thiserror** | 1.0 | Error derive macro | Custom error types |

### Windows-specific
| Crate | Version | Purpose |
|-------|---------|---------|
| **windows** | 0.58 | Windows API bindings | Process priority & memory management |

## 🔨 Build Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| **winres** | 0.1 | Windows resource compiler | Executable metadata & manifest |

---

## ✅ Why These Dependencies Are Minimal

### Core GUI (eframe + egui)
- **egui**: Immediate mode GUI - lightweight, no framework overhead
- **eframe**: Minimal runner for egui - just what's needed
- **Total**: ~2 crates for entire GUI

### System Access (sysinfo)
- **sysinfo**: Cross-platform process info - one crate for all OS operations
- **Alternative would need**: Multiple OS-specific crates

### Config (serde + serde_json)
- **serde**: Industry standard serialization - zero runtime cost
- **serde_json**: JSON support - human-readable configs
- **Total**: 2 crates for full config management

### Error Handling (anyhow + thiserror)
- **anyhow**: Ergonomic error propagation - replaces Result<T, String>
- **thiserror**: Zero-cost error types - macro-based
- **Total**: 2 crates for professional error handling

### Windows API (windows)
- **windows**: Official Microsoft Rust bindings
- **Alternative**: Would need unsafe FFI everywhere

---

## 🚫 Removed/Not Used

### What We DON'T Use

❌ **tokio** - Not needed (no async/await requirements)
❌ **async-std** - Not needed (synchronous operations)
❌ **clap** - Not needed (GUI app, no CLI)
❌ **log/env_logger** - Not needed (simple eprintln! is sufficient)
❌ **tracing** - Not needed (no complex async tracing)
❌ **reqwest** - Not needed (no network requests)
❌ **directories** - Not needed (simple env::var is sufficient)

### Why We Don't Need These

**No Async Runtime**:
- All operations are fast and synchronous
- Process API calls are instant
- No network I/O
- No file I/O during runtime (only on startup/save)

**No CLI**:
- Pure GUI application
- No command-line arguments
- No parsing needed

**No Logging Framework**:
- Simple debug output with `eprintln!`
- No log levels needed
- No log file management

**No Network**:
- All operations are local
- No updates, no telemetry
- No external API calls

---

## 📊 Dependency Tree Size

```
Total dependencies (including transitive): ~30 crates
Direct dependencies: 7 crates
Build dependencies: 1 crate

Breakdown:
- GUI (egui + dependencies): ~15 crates
- Serialization (serde): ~5 crates
- Windows API: ~5 crates
- System info: ~3 crates
- Error handling: ~2 crates
```

---

## 🔄 Update Policy

### Semantic Versioning
- **Major updates**: Manual review required
- **Minor updates**: Safe to update
- **Patch updates**: Always safe

### Recommended Update Schedule
```bash
# Check for updates
cargo outdated

# Update within semver
cargo update

# Update to latest (breaking)
cargo upgrade
```

### Version Pinning Strategy
- **eframe/egui**: Pin to same minor version (0.29.x)
- **sysinfo**: Allow minor updates (0.32.x)
- **serde**: Allow patch updates (1.0.x)
- **windows**: Pin to major version (0.x)

---

## 🛡️ Security

### Audit Commands
```bash
# Install cargo-audit
cargo install cargo-audit

# Check for vulnerabilities
cargo audit

# Check dependencies
cargo tree
```

### Trusted Sources
All dependencies are from:
- ✅ crates.io (official Rust package registry)
- ✅ Well-maintained projects
- ✅ Microsoft (windows crate)

---

## 📈 Performance Impact

| Dependency | Compile Time | Runtime Cost | Binary Size |
|------------|--------------|--------------|-------------|
| egui | ~30s | Negligible | ~2MB |
| sysinfo | ~10s | <1ms/call | ~500KB |
| serde | ~5s | Zero-cost | ~200KB |
| windows | ~15s | Zero-cost | ~1MB |
| anyhow | ~2s | Negligible | ~50KB |
| thiserror | ~2s | Zero-cost | ~50KB |

**Total binary size**: ~6-8 MB (optimized)
**Cold compile**: ~2 minutes
**Incremental compile**: ~5 seconds

---

## 🎯 Optimization Techniques Used

### Cargo.toml Optimizations
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = "fat"            # Link-time optimization
codegen-units = 1      # Single codegen unit
strip = true           # Strip symbols
panic = "abort"        # Smaller binary

[profile.release.package."*"]
opt-level = 3          # Optimize all dependencies
strip = true           # Strip all
```

### Feature Flags
```toml
eframe = { 
    version = "0.29",
    default-features = false,  # Disable unused features
    features = ["glow", "default_fonts"]  # Only what we need
}
```

---

## 🔍 Alternatives Considered

| Need | Chosen | Considered | Why Not |
|------|--------|------------|---------|
| GUI | egui | gtk-rs, iced, druid | Too heavy / unstable |
| System Info | sysinfo | Windows-rs only | Not future-proof |
| Config | serde_json | toml, yaml | JSON is simplest |
| Errors | anyhow | eyre, color-eyre | Unnecessary features |

---

## 📝 Maintenance Notes

### Regular Tasks
- [ ] Monthly: Check for updates
- [ ] Quarterly: Run `cargo audit`
- [ ] Major version releases: Review breaking changes
- [ ] Annual: Dependency tree audit

### Breaking Change Checklist
When updating major versions:
1. Read CHANGELOG.md
2. Run tests
3. Check deprecation warnings
4. Update documentation
5. Test on Windows

---

**Last Updated**: 2026-01-10
**Rust Version**: 1.75
**Total Dependencies**: 7 direct, ~30 total