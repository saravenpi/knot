# Build stage
FROM oven/bun as builder

# Set working directory
WORKDIR /app

# Copy package files
COPY package.json bun.lock ./

# Install dependencies
RUN bun install --frozen-lockfile

# Copy app source code (knot_packages already included via pre-commit hook)
COPY . ./

# Set build-time environment variables for SvelteKit
ARG PUBLIC_BACKEND_URL
ENV PUBLIC_BACKEND_URL=${PUBLIC_BACKEND_URL}

# Build application
RUN bun run build

# Runtime stage
FROM oven/bun

# Create app user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S nextjs -u 1001

# Set working directory
WORKDIR /app

# Copy built application
COPY --from=builder --chown=nextjs:nodejs /app/build ./build
COPY --from=builder --chown=nextjs:nodejs /app/knot_packages ./knot_packages
COPY --from=builder --chown=nextjs:nodejs /app/node_modules ./node_modules
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
