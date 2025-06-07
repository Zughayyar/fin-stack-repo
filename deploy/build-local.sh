#!/bin/bash

# Script to build Rust binary locally using Docker and extract it

set -e

echo "🔨 Building Rust binary in Linux container..."

# Build the image with explicit platform targeting using buildx
# This allows access to both server/ and deploy/ directories
cd ..
docker buildx build --platform linux/amd64 -f deploy/Dockerfile.build -t rust-builder --load .
cd deploy

# Create a temporary container from the image
CONTAINER_ID=$(docker create rust-builder)

# Copy the built binary from the container to the deploy directory
echo "📦 Extracting binary..."
docker cp $CONTAINER_ID:/app/target/release/server ./server

# Clean up the temporary container
docker rm $CONTAINER_ID

# Copy config files to deploy directory
cp ../server/diesel.toml ./diesel.toml
cp ../server/Cargo.toml ./Cargo.toml

echo "✅ Binary extracted to ./server"
echo "📂 Binary size: $(ls -lh server | awk '{print $5}')"

echo ""
echo "🚀 Ready to deploy! Run:"
echo "   docker build -t your-app-name ." 