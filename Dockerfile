# Multi-stage build for optimal image size

# Build stage
FROM rust:1.90 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY tests ./tests

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/salvo-backend /app/salvo-backend

# Copy migrations
COPY migrations ./migrations

# Create data directory for database
RUN mkdir -p /app/data

# Environment variables
ENV DATABASE_URL=sqlite:/app/data/salvo.db
ENV RUST_LOG=salvo_backend=info,tower_http=info

# Expose port
EXPOSE 3000

# Run the application
CMD ["/app/salvo-backend"]
