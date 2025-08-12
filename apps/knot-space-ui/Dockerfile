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

# Build stage
FROM node:18-alpine as builder

# Set working directory
WORKDIR /app

# Install knot CLI
COPY --from=knot-builder /usr/src/knot/target/release/knot /usr/local/bin/knot
RUN chmod +x /usr/local/bin/knot

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy project configuration and packages
COPY ../../knot.yml ../knot.yml
COPY ../../packages ../packages

# Copy source code
COPY . .

# knot_packages are now included directly in the build context

# Build application
RUN npm run build

# Runtime stage
FROM node:18-alpine

# Create app user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S nextjs -u 1001

# Set working directory
WORKDIR /app

# Copy built application
COPY --from=builder --chown=nextjs:nodejs /app/build ./build
COPY --from=builder --chown=nextjs:nodejs /app/knot_packages ./knot_packages
COPY --from=builder /app/package.json ./package.json

# Switch to non-root user
USER nextjs

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000 || exit 1

# Start application
CMD ["node", "build/index.js"]