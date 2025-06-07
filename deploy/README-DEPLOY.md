# Fin-Stack Deployment Guide

## 🚀 Quick Deploy on AWS

This folder contains everything needed to deploy the full stack application.

### Prerequisites on AWS Server

- Docker & Docker Compose installed
- Git installed

### Deployment Steps

1. **Clone/Pull the repository:**

   ```bash
   git clone <your-repo-url>
   cd fin-stack-repo/deploy
   ```

2. **Deploy everything:**

   ```bash
   docker compose up -d
   ```

3. **Check status:**

   ```bash
   docker compose ps
   docker compose logs
   ```

## 📊 Access Points

- **Frontend**: <http://your-server-ip:3000> (direct)
- **API**: <http://your-server-ip:8080> (direct)  
- **Nginx Proxy**: <http://your-server-ip> (routes everything)
- **PgAdmin**: <http://your-server-ip:8081> (<admin@example.com> / admin123)
- **Health Check**: <http://your-server-ip/health>

## 🔧 Environment Variables (Optional)

Create a `.env` file to customize:

```bash
POSTGRES_USER=your_user
POSTGRES_PASSWORD=your_password
POSTGRES_DB=finstack
RUST_LOG=info
```

## 📋 What's Included

- ✅ Pre-built Rust server binary (20MB)
- ✅ Pre-built Angular frontend
- ✅ PostgreSQL database with initialization
- ✅ Nginx reverse proxy with CORS
- ✅ PgAdmin for database management
- ✅ All configurations and dependencies

## 🛠️ Build Process (Local)

To rebuild everything locally:

```bash
./build.sh    # Builds both Rust server and Angular frontend
```

## 📝 Notes

- All containers are connected via `finstack-network`
- Database data persists in Docker volumes
- Logs are available via `docker compose logs`
- Health checks ensure services are running properly
