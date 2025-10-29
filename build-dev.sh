#!/bin/bash

echo "Building Space Combat Game (DEV MODE - Fast Iteration)..."
echo "==============================
"
echo "Rust version:"
rustc --version
echo ""

echo "Building in dev mode with dynamic linking (fast builds)..."
cargo build

if [ $? -eq 0 ]; then
    echo ""
    echo "=============================="
    echo "Build complete!"
    echo "Run the game with: cargo run"
    echo "Or directly: ./target/debug/space-combat-game"
    echo ""
    echo "Note: Using dynamic linking for fast iteration."
    echo "For release builds, use: ./build.sh"
    echo "=============================="
else
    echo ""
    echo "Build failed!"
    exit 1
fi

