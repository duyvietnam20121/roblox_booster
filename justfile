# Roblox Booster - Modern build commands with just
# Install: cargo install just

# Default recipe (runs when you just type "just")
default:
    just --list

# Build (release)
build:
    cargo build --release

# Build and run
run:
    cargo run --release

# Run tests
test:
    cargo test

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean
    rm -f target/release/roblox_booster target/x86_64-pc-windows-gnu/release/roblox_booster

# Full CI check (format, lint, test, build)
ci: fmt-check lint test build

# Build optimized release and show info
dist: clean
    cargo build --release
    echo "✓ Build complete!"
    echo ""
    echo "=== Executable Info ==="
    ls -lh target/release/roblox_booster 2>/dev/null || ls -lh target/x86_64-pc-windows-gnu/release/roblox_booster
    echo ""
    echo "=== Optimization Details ==="
    echo "LTO: fat"
    echo "Opt-level: 3"
    echo "Stripped: yes"
    echo "Panic: abort"

# Check for warnings and errors
check:
    cargo check --all-targets
    cargo clippy --all-targets -- -D warnings

# Watch and rebuild on changes (requires cargo-watch)
watch:
    cargo watch -x "build --release"

# Show binary info
info:
    echo "=== Executable Info ==="
    file target/release/roblox_booster 2>/dev/null || file target/x86_64-pc-windows-gnu/release/roblox_booster
    echo ""
    echo "=== Size ==="
    du -h target/release/roblox_booster 2>/dev/null || du -h target/x86_64-pc-windows-gnu/release/roblox_booster
    echo ""
    echo "=== SHA256 ==="
    sha256sum target/release/roblox_booster 2>/dev/null || sha256sum target/x86_64-pc-windows-gnu/release/roblox_booster

# Install development dependencies
setup:
    echo "Installing development tools..."
    rustup component add rustfmt clippy
    rustup target add x86_64-pc-windows-gnu
    echo "✓ Setup complete!"