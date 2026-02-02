#!/bin/bash

echo "Building fen2pdf for Linux and Windows..."

# PNG assets are now embedded in the binary

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Windows target is available
if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    echo "Adding Windows target..."
    rustup target add x86_64-pc-windows-gnu
fi

# Set up cross-compilation environment for Windows
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++

# Build for Linux
echo "Compiling for Linux (native)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Linux build successful! Executable created: target/release/fen2pdf"
    # Create symlink for easier access
    ln -sf target/release/fen2pdf fen2pdf
    echo "Symlink created: fen2pdf -> target/release/fen2pdf"
else
    echo "Linux build failed!"
    exit 1
fi

# Build for Windows
echo "Compiling for Windows (cross-compilation)..."
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "Windows build successful! Executable created: target/x86_64-pc-windows-gnu/release/fen2pdf.exe"
    # Copy to root for easier access
    cp target/x86_64-pc-windows-gnu/release/fen2pdf.exe fen2pdf.exe
    echo "Windows executable copied: fen2pdf.exe"
else
    echo "Windows build failed! Check mingw-w64 installation."
    exit 1
fi

echo ""
echo "Build complete!"
echo "Linux executable: fen2pdf"
echo "Windows executable: fen2pdf.exe"