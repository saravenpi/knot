# Multi-stage build for production optimization
FROM oven/bun:1 AS base
WORKDIR /app

# Knot CLI stage - build the knot binary
FROM rust:1.82-slim AS knot-builder
WORKDIR /usr/src/knot

# Install build dependencies for Knot CLI
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Knot CLI source from the project root
COPY ../../knot-cli/Cargo.toml ../../knot-cli/Cargo.lock ./
COPY ../../knot-cli/src ./src

# Build Knot CLI
RUN cargo build --release && strip target/release/knot

# Install system dependencies
FROM base AS deps
RUN apt-get update -y && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install dependencies
COPY package.json bun.lock ./
RUN bun install --production --frozen-lockfile

# Generate Prisma client
COPY prisma ./prisma/
RUN bunx prisma generate

# Build stage
FROM base AS builder
RUN apt-get update -y && apt-get install -y openssl ca-certificates

# Copy Knot CLI binary
COPY --from=knot-builder /usr/src/knot/target/release/knot /usr/local/bin/knot
RUN chmod +x /usr/local/bin/knot

# Copy dependencies from deps stage
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/prisma ./prisma

# Copy project configuration and packages
COPY ../../knot.yml ../knot.yml
COPY ../../packages ../packages

# Copy source code
COPY . .

# knot_packages are now included directly in the build context

# Build the application
RUN bun run build

# Production stage
FROM oven/bun:1-slim AS production
WORKDIR /app

# Install runtime dependencies
RUN apt-get update -y && apt-get install -y \
    openssl \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy built application with correct ownership
COPY --from=builder /app/server ./server
COPY --from=builder /app/prisma ./prisma
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/start.sh ./start.sh
COPY --from=builder /app/src ./src
COPY --from=builder /app/tsconfig.json ./tsconfig.json
COPY --from=builder /app/knot_packages ./knot_packages

# Make scripts executable
RUN chmod +x ./server ./start.sh

# Expose the port your server uses
EXPOSE 3001

# Start as root so we can fix permissions, then switch to appuser inside start.sh
CMD ["./start.sh"]
