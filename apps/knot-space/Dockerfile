# Multi-stage build for production optimization
FROM oven/bun:1 AS base
WORKDIR /app

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

# Copy dependencies from deps stage
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/prisma ./prisma

# Copy source code
COPY . .

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

# Create uploads folder and give ownership to appuser
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app/uploads

# Copy built application with correct ownership
COPY --from=builder --chown=appuser:appuser /app/server ./server
COPY --from=builder --chown=appuser:appuser /app/prisma ./prisma
COPY --from=builder --chown=appuser:appuser /app/node_modules ./node_modules
COPY --from=builder --chown=appuser:appuser /app/start.sh ./start.sh
COPY --from=builder --chown=appuser:appuser /app/src ./src
COPY --from=builder --chown=appuser:appuser /app/tsconfig.json ./tsconfig.json

# Make scripts executable
RUN chmod +x ./server ./start.sh

# Switch to non-root user
USER appuser

# Expose the port your server uses
EXPOSE 3001

# Start the application
CMD ["./start.sh"]

