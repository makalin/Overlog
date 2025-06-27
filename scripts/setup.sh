#!/bin/bash

# Overlog Development Setup Script

set -e

echo "🚀 Setting up Overlog development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed: $(rustc --version)"

# Check if FFmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "⚠️  FFmpeg is not installed. Please install FFmpeg:"
    echo "   macOS: brew install ffmpeg"
    echo "   Ubuntu: sudo apt install ffmpeg"
    echo "   Windows: https://ffmpeg.org/download.html"
    echo ""
    echo "Continuing without FFmpeg (video features will not work)..."
else
    echo "✅ FFmpeg is installed: $(ffmpeg -version | head -n1)"
fi

# Update Rust toolchain
echo "🔄 Updating Rust toolchain..."
rustup update

# Install development tools
echo "📦 Installing development tools..."
cargo install cargo-audit
cargo install cargo-tarpaulin
cargo install cargo-watch

# Build the project
echo "🔨 Building Overlog..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

# Check code quality
echo "🔍 Running code quality checks..."
cargo fmt -- --check
cargo clippy -- -D warnings

# Create assets directory if it doesn't exist
mkdir -p assets/fonts

# Download Roboto font if not present
if [ ! -f "assets/fonts/Roboto-Regular.ttf" ]; then
    echo "📥 Downloading Roboto font..."
    curl -L -o assets/fonts/Roboto-Regular.ttf \
        "https://github.com/google/fonts/raw/main/apache/roboto/Roboto-Regular.ttf"
fi

echo ""
echo "🎉 Setup complete! You can now:"
echo ""
echo "  Build the project:     cargo build"
echo "  Run tests:            cargo test"
echo "  Run the CLI:          cargo run -- --help"
echo "  Parse telemetry:      cargo run -- parse examples/sample_telemetry.json"
echo "  Render overlay:       cargo run -- render examples/sample_telemetry.json output.webm"
echo ""
echo "📚 Check the README.md for more information and examples." 