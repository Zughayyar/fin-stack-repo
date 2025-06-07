# Runtime-only Dockerfile - Uses pre-built binary
FROM debian:bookworm-slim

# Install runtime dependencies (for PostgreSQL and HTTPS)
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the pre-built Rust binary from deploy directory
COPY server /app/server

# Copy config files (if any)
COPY diesel.toml /app/diesel.toml

# Make binary executable
RUN chmod +x /app/server

# Expose the port your server listens on
EXPOSE 8080

# Health check to ensure app is running
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Start the binary
CMD ["./server"]