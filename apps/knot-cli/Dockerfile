# Knot Docker Image
# Multi-stage build for optimal image size

# Build stage
FROM rust:1.82-slim as builder

WORKDIR /usr/src/knot

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source code
COPY src ./src

# Build the application
RUN touch src/main.rs && \
    cargo build --release && \
    strip target/release/knot

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false knot

# Copy the binary from builder stage
COPY --from=builder /usr/src/knot/target/release/knot /usr/local/bin/knot

# Set ownership and permissions
RUN chmod +x /usr/local/bin/knot

# Create working directory
WORKDIR /workspace
RUN chown knot:knot /workspace

# Switch to non-root user
USER knot

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD knot info || exit 1

# Default command
ENTRYPOINT ["knot"]
CMD ["info"]
