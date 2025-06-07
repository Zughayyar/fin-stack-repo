#!/bin/bash

# Script to build Rust binary for ARM64 (if your server is ARM64)

set -e

echo "🔨 Building ARM64 Rust binary in Linux container..."

# Build the image with explicit ARM64 platform targeting using buildx
cd ..
docker buildx build --platform linux/arm64 -f deploy/Dockerfile.build-arm64 -t rust-builder-arm64 --load .
cd deploy

# Create a temporary container from the image
CONTAINER_ID=$(docker create rust-builder-arm64)

# Copy the built binary from the container to the deploy directory
echo "📦 Extracting ARM64 binary..."
docker cp $CONTAINER_ID:/app/target/aarch64-unknown-linux-gnu/release/server ./server

# Clean up the temporary container
docker rm $CONTAINER_ID

echo "✅ ARM64 Binary extracted to ./server"
echo "📂 Binary size: $(ls -lh server | awk '{print $5}')"

echo ""
echo "🚀 Ready to deploy! Run:"
echo "   docker build -t your-app-name ." 