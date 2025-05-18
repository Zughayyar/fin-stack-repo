# Stage 1: Build the app using Rust official image
FROM rust:1.82 as builder

# Install dependencies needed for build
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files (only Cargo.toml, we'll generate a fresh Cargo.lock)
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Copy migrations and diesel configuration
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Build the application (this will generate a fresh Cargo.lock compatible with this Rust version)
RUN cargo build --release

# Stage 2: Create a smaller production image
FROM debian:bookworm-slim

# Install only what's needed to run the binary
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    libpq5 \
    ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/server /usr/local/bin/app

# Copy migrations directory for database setup
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml ./diesel.toml

# Create directory for logs if needed
RUN mkdir -p logs

# Expose the application port
EXPOSE 8080

# Start the application
CMD ["app"]