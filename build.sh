#!/bin/bash
# Build script for Space Combat Game

set -e

echo "Building Space Combat Game..."
echo "=============================="
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Rust version
echo "Rust version:"
rustc --version
echo ""

# Clean build
echo "Cleaning previous build..."
cargo clean

# Build in release mode
echo "Building in release mode (this may take a while)..."
cargo build --release

echo ""
echo "=============================="
echo "Build complete!"
echo "Run the game with: cargo run --release"
echo "Or directly: ./target/release/space-combat-game"
echo "=============================="

