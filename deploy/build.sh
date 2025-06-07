#!/bin/bash

# Unified build script for Fin-Stack
# Builds both Rust server and Angular frontend locally using Docker

set -e

echo "🚀 Starting Fin-Stack build process..."
echo ""

# Function to print section headers
print_section() {
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Build Rust Server
print_section "🔨 Building Rust Server"

cd ..
echo "📦 Building Rust server in Linux container..."
docker buildx build --platform linux/amd64 -f deploy/Dockerfile.build -t rust-builder --load .
cd deploy

# Extract server binary
CONTAINER_ID=$(docker create rust-builder)
echo "📦 Extracting server binary..."
docker cp $CONTAINER_ID:/app/target/release/server ./server
docker rm $CONTAINER_ID

# Copy config files
cp ../server/diesel.toml ./diesel.toml
cp ../server/Cargo.toml ./Cargo.toml

echo "✅ Server build completed!"
echo "📂 Binary size: $(ls -lh server | awk '{print $5}')"
echo ""

# Build Angular Frontend
print_section "🌐 Building Angular Frontend"

cd ..
echo "📦 Building Angular frontend in Node container..."
docker build -f web/Dockerfile --target builder -t web-builder web/
cd deploy

# Extract frontend build
CONTAINER_ID=$(docker create web-builder)
echo "📦 Extracting frontend build..."
rm -rf ./dist
docker cp $CONTAINER_ID:/app/dist ./dist
docker rm $CONTAINER_ID

echo "✅ Frontend build completed!"
echo "📂 Build size: $(du -sh dist | awk '{print $1}')"
echo ""

# Final summary
print_section "🎉 Build Summary"
echo "✅ Rust Server Binary: $(ls -lh server | awk '{print $5}')"
echo "✅ Angular Frontend: $(du -sh dist | awk '{print $1}')"
echo "✅ Config Files: diesel.toml, Cargo.toml"
echo "✅ Nginx Config: nginx/nginx.conf"
echo "✅ Database Scripts: scripts/init-db.sql"
echo ""
echo "🚀 Ready to deploy! Run:"
echo "   docker compose up -d"
echo "" 