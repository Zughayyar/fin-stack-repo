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

# Check if certificate already exists
if sudo test -f "/etc/letsencrypt/live/${DOMAIN}/fullchain.pem"; then
    echo "📜 Certificate already exists. Checking if renewal is needed..."
    
    # Check certificate expiry
    EXPIRY_DATE=$(sudo openssl x509 -enddate -noout -in /etc/letsencrypt/live/${DOMAIN}/fullchain.pem | cut -d= -f2)
    EXPIRY_EPOCH=$(date -d "$EXPIRY_DATE" +%s)
    CURRENT_EPOCH=$(date +%s)
    DAYS_UNTIL_EXPIRY=$(( (EXPIRY_EPOCH - CURRENT_EPOCH) / 86400 ))
    
    echo "📅 Certificate expires in $DAYS_UNTIL_EXPIRY days"
    
    if [ $DAYS_UNTIL_EXPIRY -gt 30 ]; then
        echo "✅ Certificate is still valid (expires in $DAYS_UNTIL_EXPIRY days)"
        echo "📂 Copying existing certificates..."
        sudo mkdir -p nginx/ssl
        sudo cp /etc/letsencrypt/live/${DOMAIN}/fullchain.pem nginx/ssl/server.crt
        sudo cp /etc/letsencrypt/live/${DOMAIN}/privkey.pem nginx/ssl/server.key
        sudo chown $(whoami):$(whoami) nginx/ssl/server.*
        echo "✅ Certificates updated! No need to generate new ones."
        exit 0
    else
        echo "🔄 Certificate expires soon. Renewing..."
    fi
fi

# Generate or renew certificate
echo "📜 Generating/renewing Let's Encrypt certificate..."
sudo certbot certonly --standalone \
    --preferred-challenges http \
    --email admin@${DOMAIN} \
    --agree-tos \
    --no-eff-email \
    --force-renewal \
    -d ${DOMAIN}

# Copy certificates to nginx directory
echo "📂 Copying certificates..."
sudo mkdir -p nginx/ssl
sudo cp /etc/letsencrypt/live/${DOMAIN}/fullchain.pem nginx/ssl/server.crt
sudo cp /etc/letsencrypt/live/${DOMAIN}/privkey.pem nginx/ssl/server.key
sudo chown $(whoami):$(whoami) nginx/ssl/server.*

# Update nginx config with real domain (only if not already updated)
if grep -q "server_name localhost;" nginx/nginx.conf; then
    echo "📝 Updating nginx config with domain: $DOMAIN"
    sed -i "s/server_name localhost;/server_name ${DOMAIN};/g" nginx/nginx.conf
else
    echo "📝 Nginx config already updated with domain"
fi

echo "✅ Let's Encrypt certificate installed!"
echo "🔄 Starting nginx..."
docker compose start nginx

echo ""
echo "🌐 Your site is now available at: https://${DOMAIN}"
echo "🔄 Certificate will auto-renew via cron job (set up separately)" 