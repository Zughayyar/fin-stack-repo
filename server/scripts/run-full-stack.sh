#!/bin/bash

# Script to run the full FinStack application with all services
# This includes: PostgreSQL, Rust API server, Angular frontend, and Nginx reverse proxy

set -e

echo "🚀 Starting FinStack Full Application..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Load environment variables
if [ -f "env.prod" ]; then
    echo "📁 Loading production environment variables..."
    export $(cat env.prod | xargs)
elif [ -f "env.dev" ]; then
    echo "📁 Loading development environment variables..."
    export $(cat env.dev | xargs)
else
    echo "⚠️  No environment file found. Using default values."
fi

echo "🔨 Building and starting all services..."

# Start all services with the with-server profile (includes frontend)
docker-compose --profile with-server up --build -d

echo "⏳ Waiting for services to be healthy..."

# Wait for services to be healthy
echo "🔍 Checking PostgreSQL..."
docker-compose exec postgres pg_isready -U ${POSTGRES_USER:-user} -d ${POSTGRES_DB:-finstack}

echo "🔍 Checking API server..."
timeout 60 bash -c 'until curl -sf http://localhost:8080/health; do sleep 2; done'

echo "🔍 Checking frontend..."
timeout 60 bash -c 'until curl -sf http://localhost:3000/health; do sleep 2; done'

echo "🔍 Checking nginx..."
timeout 60 bash -c 'until curl -sf http://localhost:80/health; do sleep 2; done'

echo ""
echo "✅ FinStack application is running!"
echo ""
echo "🌐 Access points:"
echo "   • Frontend (via Nginx): http://localhost"
echo "   • API (via Nginx): http://localhost/api/"
echo "   • API (direct): http://localhost:8080"
echo "   • Frontend (direct): http://localhost:3000"
echo "   • Database: localhost:5432"
echo ""
echo "📊 To view logs:"
echo "   docker-compose logs -f [service_name]"
echo ""
echo "🛑 To stop all services:"
echo "   docker-compose --profile with-server down"
echo ""
echo "📋 Running containers:"
docker-compose ps 