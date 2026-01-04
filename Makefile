.PHONY: setup build clean run help install

TARGET = x86_64-pc-windows-gnu
RELEASE_DIR = target/$(TARGET)/release
EXE_NAME = roblox_booster.exe

help:
	@echo "Roblox Booster - Build Commands"
	@echo "================================"
	@echo "make setup   - Install dependencies and target"
	@echo "make build   - Build Windows executable"
	@echo "make clean   - Clean build artifacts"
	@echo "make install - Setup + Build in one command"
	@echo "make run     - Build and show exe info"

setup:
	@echo "Installing dependencies..."
	sudo apt update
	sudo apt install -y mingw-w64 build-essential
	@echo "Adding Windows GNU target..."
	rustup target add $(TARGET)
	@echo "✓ Setup complete!"

build:
	@echo "Building for Windows (GNU)..."
	cargo build --release --target $(TARGET)
	@echo "Copying executable..."
	cp $(RELEASE_DIR)/$(EXE_NAME) ./$(EXE_NAME)
	@echo ""
	@echo "✓ Build complete!"
	@echo "Executable: ./$(EXE_NAME)"
	@ls -lh $(EXE_NAME)

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f $(EXE_NAME)
	@echo "✓ Clean complete!"

install: setup build

run: build
	@echo ""
	@echo "Executable info:"
	@file $(EXE_NAME)
	@echo ""
	@echo "Size:"
	@du -h $(EXE_NAME)
	@echo ""
	@echo "Ready to transfer to Windows!"