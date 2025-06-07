#!/bin/bash

# Script to set up Let's Encrypt SSL certificates for production
# Usage: ./setup-letsencrypt.sh your-domain.com

DOMAIN=${1:-""}

if [ -z "$DOMAIN" ]; then
    echo "❌ Error: Please provide a domain name"
    echo "Usage: ./setup-letsencrypt.sh your-domain.com"
    exit 1
fi

echo "🔐 Setting up Let's Encrypt for domain: $DOMAIN"

# Install certbot if not available
if ! command -v certbot &> /dev/null; then
    echo "📦 Installing certbot..."
    sudo apt update
    sudo apt install -y certbot python3-certbot-nginx
fi

# Stop nginx temporarily
echo "⏸️  Stopping nginx..."
docker compose stop nginx

# Generate certificate
echo "📜 Generating Let's Encrypt certificate..."
sudo certbot certonly --standalone \
    --preferred-challenges http \
    --email admin@${DOMAIN} \
    --agree-tos \
    --no-eff-email \
    -d ${DOMAIN}

# Copy certificates to nginx directory
echo "📂 Copying certificates..."
sudo mkdir -p nginx/ssl
sudo cp /etc/letsencrypt/live/${DOMAIN}/fullchain.pem nginx/ssl/server.crt
sudo cp /etc/letsencrypt/live/${DOMAIN}/privkey.pem nginx/ssl/server.key
sudo chown $(whoami):$(whoami) nginx/ssl/server.*

# Update nginx config with real domain
sed -i "s/server_name localhost;/server_name ${DOMAIN};/g" nginx/nginx.conf

echo "✅ Let's Encrypt certificate installed!"
echo "🔄 Starting nginx..."
docker compose start nginx

echo ""
echo "🌐 Your site is now available at: https://${DOMAIN}"
echo "🔄 Certificate will auto-renew via cron job (set up separately)" 