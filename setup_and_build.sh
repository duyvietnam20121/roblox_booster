#!/bin/bash

set -e

echo "=========================================="
echo "Roblox Booster - Setup & Build Script"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo -e "${RED}Error: This script is for Linux only${NC}"
    echo "For Windows, run: cargo build --release"
    exit 1
fi

echo -e "${YELLOW}[1/5] Checking Rust installation...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust not found!${NC}"
    echo "Install from: https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✓ Rust $(rustc --version)${NC}"
echo ""

echo -e "${YELLOW}[2/5] Installing MinGW cross-compiler...${NC}"
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "MinGW not found, installing..."
    sudo apt update
    sudo apt install -y mingw-w64
    echo -e "${GREEN}✓ MinGW installed${NC}"
else
    echo -e "${GREEN}✓ MinGW already installed${NC}"
fi
echo ""

echo -e "${YELLOW}[3/5] Adding Windows target to Rust...${NC}"
rustup target add x86_64-pc-windows-gnu
echo -e "${GREEN}✓ Target added${NC}"
echo ""

echo -e "${YELLOW}[4/5] Cleaning old build artifacts...${NC}"
cargo clean
echo -e "${GREEN}✓ Clean complete${NC}"
echo ""

echo -e "${YELLOW}[5/5] Building Windows executable...${NC}"
echo "This may take a few minutes on first build..."
echo ""

if cargo build --release --target x86_64-pc-windows-gnu; then
    echo ""
    echo -e "${GREEN}=========================================="
    echo "BUILD SUCCESSFUL! 🚀"
    echo "==========================================${NC}"
    echo ""
    
    EXE_PATH="target/x86_64-pc-windows-gnu/release/roblox_booster.exe"
    if [ -f "$EXE_PATH" ]; then
        FILE_SIZE=$(du -h "$EXE_PATH" | cut -f1)
        echo -e "${GREEN}Executable Details:${NC}"
        echo "  Location: $EXE_PATH"
        echo "  Size: $FILE_SIZE"
        echo ""
        echo -e "${YELLOW}To copy to current directory:${NC}"
        echo "  cp $EXE_PATH ./roblox_booster.exe"
        echo ""
        
        # Auto copy to current directory
        cp "$EXE_PATH" ./roblox_booster.exe
        echo -e "${GREEN}✓ Copied to: ./roblox_booster.exe${NC}"
    else
        echo -e "${RED}Warning: .exe file not found!${NC}"
    fi
else
    echo ""
    echo -e "${RED}=========================================="
    echo "BUILD FAILED!"
    echo "==========================================${NC}"
    echo ""
    echo "Common issues:"
    echo "1. Missing dependencies - run: sudo apt install build-essential"
    echo "2. Network issues - check internet connection"
    echo "3. Disk space - check available space"
    exit 1
fi

echo ""
echo -e "${GREEN}=========================================="
echo "Done!"
echo "==========================================${NC}"