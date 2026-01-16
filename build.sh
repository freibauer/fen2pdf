#!/bin/bash

echo "Building fen2pdf..."

# PNG assets are now embedded in the binary

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the application
echo "Compiling Rust application..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful! Executable created: target/release/fen2pdf"
    # Create symlink for easier access
    ln -sf target/release/fen2pdf fen2pdf
    echo "Symlink created: fen2pdf -> target/release/fen2pdf"
else
    echo "Build failed!"
    exit 1
fi