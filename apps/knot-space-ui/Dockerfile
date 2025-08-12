# Knot CLI stage - install knot using install script
FROM debian:bookworm-slim AS knot-installer

# Install dependencies for knot install script
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust for the install script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install knot using the install script from GitHub
RUN curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash

# Build stage
FROM node:18-alpine as builder

# Set working directory
WORKDIR /app

# Install knot CLI
COPY --from=knot-installer /usr/local/bin/knot /usr/local/bin/knot
RUN chmod +x /usr/local/bin/knot

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy project configuration and packages
COPY knot.yml ./knot.yml
COPY packages ./packages

# Copy app source code
COPY apps/knot-space-ui ./

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