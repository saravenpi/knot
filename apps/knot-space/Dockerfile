# Build stage
FROM rust:1.82-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Use a simpler approach - we'll handle migrations in the application code or use Railway's capabilities
# For now, let's focus on getting the build working without sqlx-cli installation

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy source file to cache dependencies
RUN mkdir src &&     echo "fn main() {}" > src/main.rs &&     cargo build --release &&     rm -rf src

# Copy source code and migrations
COPY src/ src/
COPY migrations/ migrations/

# Build application - Railway provides DATABASE_URL at build time
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Build the application (SQLx will use the database connection)
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    postgresql-client \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app knot

# Set working directory
WORKDIR /app

# Copy binary from build stage
COPY --from=builder /app/target/release/knot-space /usr/local/bin/knot-space

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Copy startup script
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

# Create storage directory
RUN mkdir -p /app/storage/packages && \
    chown -R knot:knot /app

# Switch to non-root user
USER knot

# Expose port
EXPOSE 8000


ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run application
CMD ["/app/start.sh"]
