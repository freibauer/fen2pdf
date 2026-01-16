#!/bin/bash

echo "Cleaning build artifacts..."

# Remove executable symlink
if [ -f "fen2pdf" ]; then
    rm fen2pdf
    echo "Removed executable symlink: fen2pdf"
fi

# Remove Rust build directory
if [ -d "target" ]; then
    rm -rf target
    echo "Removed Rust build directory: target/"
fi

# Remove generated PDF
if [ -f "chess_positions.pdf" ]; then
    rm chess_positions.pdf
    echo "Removed generated PDF: chess_positions.pdf"
fi

# Remove temporary files
if [ -d "temp" ]; then
    rm -rf temp
    echo "Removed temporary directory: temp/"
fi

# Remove downloaded study file
if [ -f "lichess_study.pgn" ]; then
    rm lichess_study.pgn
    echo "Removed downloaded study: lichess_study.pgn"
fi


# Clean Cargo cache (optional)
if [ "$1" = "--all" ]; then
    echo "Cleaning Cargo cache..."
    cargo clean
fi

echo "Clean complete!"