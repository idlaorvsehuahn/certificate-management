# Builder Stage
FROM rust:latest-slim AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy the workspace configuration and source files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/ services/
COPY tools/ tools/

# Build the inventory-service binary in release mode
RUN cargo build --release --bin inventory-service

# Runner Stage
FROM debian:bookworm-slim

# Install runtime dependencies (CA certificates for secure TLS and libssl)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/inventory-service /usr/app/inventory-service

# Expose default port
EXPOSE 8081

# Run the binary
CMD ["/usr/app/inventory-service"]
