# Contributing to Roblox Booster

Thank you for your interest in contributing! 🎉

## 📋 Before You Start

### Read These First:
1. ✅ `README.md` - Project overview
2. ✅ `DEVELOPER_NOTES.md` - Critical development info
3. ✅ `SECURITY.md` - Security guidelines

## 🚀 Quick Start

### 1. Setup Development Environment

```bash
# Clone repository
git clone https://github.com/duyvietnam20121/roblox_booster.git
cd roblox_booster

# Install Rust 1.85+
rustup update

# Install development tools
rustup component add rustfmt clippy
cargo install just

# Setup target
rustup target add x86_64-pc-windows-gnu

# Build
just build
```

### 2. Development Workflow

```bash
# Format code
just fmt

# Run linter
just lint

# Run tests
just test

# Full CI check
just ci

# Build release
just build
```

## 🎯 Contribution Guidelines

### Code Style

**Follow Rust conventions:**
- Format with `cargo fmt`
- Pass `cargo clippy -- -D warnings`
- Use meaningful variable names
- Add comments for complex logic

**Example:**

```rust
// ✅ GOOD
fn optimize_process(&self, pid: u32, name: &str) -> Result<()> {
    // Open handle with required permissions
    let handle = OpenProcess(
        PROCESS_SET_INFORMATION | PROCESS_SET_QUOTA,
        false,
        pid,
    )?;
    
    // Set priority and cleanup
    let result = SetPriorityClass(handle, priority);
    CloseHandle(handle).ok();
    result?;
    Ok(())
}

// ❌ BAD
fn opt(p:u32)->Result<()>{let h=OpenProcess(0x1000,0,p)?;SetPriorityClass(h,0x80)?;Ok(())}
```

### Commit Messages

**Format:**
```
<type>: <subject>

<body (optional)>

<footer (optional)>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Tests
- `chore`: Maintenance

**Examples:**

```bash
# Good
feat: add CPU affinity control
fix: handle cleanup on error path
docs: update installation instructions

# Bad
updated stuff
fix bug
changes
```

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make changes**
4. **Test thoroughly**
   ```bash
   just ci
   ```
5. **Commit with meaningful messages**
6. **Push to your fork**
7. **Open Pull Request**

### PR Checklist:

- [ ] Code compiles without warnings
- [ ] Passes `just ci`
- [ ] Added/updated tests
- [ ] Updated documentation
- [ ] Added entry to CHANGELOG.md
- [ ] No breaking changes (or documented)

## 🐛 Bug Reports

### Before Reporting:
1. Search existing issues
2. Check if already fixed in latest version
3. Test on clean Windows install

### Report Template:

```markdown
**Description:**
Clear description of the bug

**Steps to Reproduce:**
1. Launch app
2. Click button X
3. See error

**Expected Behavior:**
What should happen

**Actual Behavior:**
What actually happens

**Environment:**
- OS: Windows 10/11
- Rust: 1.85
- App Version: 0.1.0

**Logs/Screenshots:**
(if applicable)
```

## 💡 Feature Requests

### Template:

```markdown
**Feature Description:**
What feature do you want?

**Use Case:**
Why is this useful?

**Proposed Implementation:**
(optional) How might this work?

**Alternatives:**
What other solutions exist?
```

## 🔒 Security

**For security issues:**
- ❌ DON'T open public issues
- ✅ DO email security contact
- ✅ DO wait for response before disclosure

## ❌ What We DON'T Accept

1. **Code that violates Roblox ToS**
   - No game hacking
   - No memory injection
   - No anti-cheat bypass

2. **Malicious code**
   - No data collection
   - No remote access
   - No malware

3. **Poorly tested changes**
   - Must pass CI
   - Must be tested on Windows

4. **Breaking changes without discussion**
   - Discuss in issue first
   - Provide migration path

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Windows API Docs](https://learn.microsoft.com/en-us/windows/win32/)
- [egui Documentation](https://docs.rs/egui/)

## 🙏 Recognition

Contributors will be:
- Listed in README.md
- Credited in release notes
- Appreciated by the community! ❤️

---

**Questions?** Open a discussion on GitHub!

**Thank you for contributing! 🚀**