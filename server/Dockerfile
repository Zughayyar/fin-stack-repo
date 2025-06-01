# Multi-stage build for optimized production image
FROM rust:latest AS builder

# Aggressive memory management for 1GB servers
ENV CARGO_BUILD_JOBS=1
ENV CARGO_NET_RETRY=10
ENV RUST_BACKTRACE=0
# Reduce LLVM memory usage
ENV RUSTFLAGS="-C opt-level=2 -C debuginfo=0"

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml ./

# Create dummy Cargo.lock if it doesn't exist
RUN touch Cargo.lock

# Create dummy source to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this layer will be cached)
RUN CARGO_INCREMENTAL=0 cargo build --release && rm -rf src target/release/deps/server*

# Copy source code
COPY src ./src
COPY diesel.toml ./

# Build the application
RUN CARGO_INCREMENTAL=0 cargo build --release

# Runtime stage - smaller base image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/server /app/server

# Copy any configuration files if needed
COPY --from=builder /app/diesel.toml /app/diesel.toml

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the binary
CMD ["./server"] 