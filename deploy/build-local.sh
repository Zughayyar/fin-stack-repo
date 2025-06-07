#!/bin/bash

# Script to build Rust binary locally using Docker and extract it

set -e

echo "🔨 Building Rust binary in Linux container..."

# Build the image with the build Dockerfile from parent directory
# This allows access to both server/ and deploy/ directories
cd ..
docker build -f deploy/Dockerfile.build -t rust-builder .
cd deploy

# Create a temporary container from the image
CONTAINER_ID=$(docker create rust-builder)

# Copy the built binary from the container to the deploy directory
echo "📦 Extracting binary..."
docker cp $CONTAINER_ID:/app/target/release/server ./server

# Clean up the temporary container
docker rm $CONTAINER_ID

echo "✅ Binary extracted to ./server"
echo "📂 Binary size: $(ls -lh server | awk '{print $5}')"

echo ""
echo "🚀 Ready to deploy! Run:"
echo "   docker build -t your-app-name ." 