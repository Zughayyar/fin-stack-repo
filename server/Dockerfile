# Stage 1: Build the app using Rust Alpine image
FROM rust:1.70-alpine AS builder

# Install build dependencies
RUN apk add --no-cache build-base openssl-dev postgresql-dev

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Copy migrations and diesel configuration
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Build the application
RUN cargo build --release && \
    ls -la /app/target/release/

# Stage 2: Create a smaller production image
FROM alpine:3.18

# Install only what's needed to run the binary
RUN apk add --no-cache \
    openssl \
    postgresql-client \
    ca-certificates

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/server /usr/local/bin/server

# Verify binary exists and is executable
RUN ls -la /usr/local/bin/server && \
    chmod +x /usr/local/bin/server

# Copy migrations directory for database setup
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml ./diesel.toml

# Create directory for logs if needed
RUN mkdir -p logs

# Expose the application port
EXPOSE 8080

# Start the application
CMD ["/usr/local/bin/server"]