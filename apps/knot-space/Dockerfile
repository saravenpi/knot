# Build stage
FROM rust:1.82-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy source file to cache dependencies
ENV SQLX_OFFLINE=true
RUN mkdir src &&     echo "fn main() {}" > src/main.rs &&     cargo build --release &&     rm -rf src

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Copy source code
COPY src/ src/
COPY migrations/ migrations/
COPY sqlx-data.json sqlx-data.json

# Build application
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app knot

# Set working directory
WORKDIR /app

# Copy binary from build stage
COPY --from=builder /app/target/release/knot-space /usr/local/bin/knot-space

# Create storage directory
RUN mkdir -p /app/storage/packages && \
    chown -R knot:knot /app

# Switch to non-root user
USER knot

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run application
CMD ["knot-space"]
