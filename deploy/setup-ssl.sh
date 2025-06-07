#!/bin/bash

# Script to generate self-signed SSL certificates for development/testing

set -e

echo "🔐 Setting up SSL certificates..."

# Create SSL directory if it doesn't exist
mkdir -p nginx/ssl

# Generate private key
echo "📝 Generating private key..."
openssl genrsa -out nginx/ssl/server.key 2048

# Generate certificate signing request
echo "📝 Generating certificate signing request..."
openssl req -new -key nginx/ssl/server.key -out nginx/ssl/server.csr -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Generate self-signed certificate
echo "📝 Generating self-signed certificate..."
openssl x509 -req -days 365 -in nginx/ssl/server.csr -signkey nginx/ssl/server.key -out nginx/ssl/server.crt

# Set proper permissions
chmod 600 nginx/ssl/server.key
chmod 644 nginx/ssl/server.crt

# Clean up CSR file
rm nginx/ssl/server.csr

echo "✅ SSL certificates generated!"
echo "📂 Certificates location:"
echo "   - Private key: nginx/ssl/server.key"
echo "   - Certificate: nginx/ssl/server.crt"
echo ""
echo "⚠️  Note: This is a self-signed certificate for testing only."
echo "🌐 For production, use Let's Encrypt or a real CA certificate."
echo ""
echo "🚀 Ready to restart with HTTPS!" 