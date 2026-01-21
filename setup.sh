#!/bin/bash
# Setup script cho cross-compilation Windows target

set -e

echo "🔧 Roblox Booster - Setup Script"
echo "================================"
echo ""

# Kiểm tra OS
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "⚠️  Script này dành cho Linux. Trên Windows, dùng native build."
    exit 1
fi

# Install MinGW
echo "📦 Installing MinGW-w64..."
if command -v apt-get &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y mingw-w64 build-essential
elif command -v yum &> /dev/null; then
    sudo yum install -y mingw64-gcc mingw64-gcc-c++
else
    echo "❌ Package manager không được hỗ trợ. Cài MinGW manually."
    exit 1
fi

# Setup Rust target
echo ""
echo "🦀 Setting up Rust target..."
rustup target add x86_64-pc-windows-gnu

# Verify
echo ""
echo "✅ Verification:"
echo "- MinGW: $(x86_64-w64-mingw32-gcc --version | head -n1)"
echo "- Rust target: $(rustup target list | grep x86_64-pc-windows-gnu)"

echo ""
echo "✅ Setup hoàn tất!"
echo ""
echo "📝 Next steps:"
echo "  1. cargo build --release"
echo "  2. Binary tại: target/x86_64-pc-windows-gnu/release/roblox_booster.exe"
echo ""