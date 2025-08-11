# Build stage
FROM rust:1.82-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy source file to cache dependencies - build without SQLx queries
RUN mkdir src && \
    echo 'fn main() { println!("Hello, world!"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/knot-space

# Copy all source files
COPY src/ src/
COPY migrations/ migrations/
COPY sqlx-data.json .

# Build with either online or offline mode based on DATABASE_URL availability
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Try building with database connection, fall back to offline mode
RUN if [ -n "$DATABASE_URL" ]; then \
        echo "Building with database connection..." && \
        touch src/main.rs && \
        cargo build --release; \
    else \
        echo "No DATABASE_URL provided, using offline mode..." && \
        export SQLX_OFFLINE=true && \
        touch src/main.rs && \
        cargo build --release; \
    fi

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
