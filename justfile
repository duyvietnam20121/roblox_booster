# Roblox Booster - Justfile

# Default recipe - hiển thị help
default:
    @just --list

# Build release version
build:
    cargo build --release
    @echo "✅ Binary: target/release/roblox_booster.exe"

# Run trong debug mode
run:
    cargo run

# Run release version
run-release:
    cargo run --release

# Format code
fmt:
    cargo fmt

# Lint với clippy
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Check code (fmt + clippy + test + build)
check: fmt lint test build

# Install dependencies (first time setup)
setup:
    rustup update stable
    rustup target add x86_64-pc-windows-gnu
    rustup component add clippy rustfmt
    @echo "Checking MinGW..."
    @command -v x86_64-w64-mingw32-gcc || echo "⚠️  Cần cài MinGW: sudo apt install mingw-w64"

# Show binary info
info:
    @ls -lh target/x86_64-pc-windows-gnu/release/roblox_booster.exe 2>/dev/null || echo "Chưa build. Chạy: just build"

# Build info (dependencies, features)
build-info:
    @echo "Target: x86_64-pc-windows-gnu"
    @echo "Rust version:"
    @rustc --version
    @echo ""
    @echo "Toolchain:"
    @rustup show active-toolchain
    @echo ""
    @echo "MinGW:"
    @x86_64-w64-mingw32-gcc --version | head -n1