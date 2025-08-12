# Use Bun's official image with specific version for reproducibility
FROM oven/bun:1.1.29 AS base
WORKDIR /usr/src/app

# Install system dependencies and security updates
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install dependencies into temp directory
# This will cache them and speed up future builds
FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json bun.lockb* /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

# Install with --production (exclude devDependencies)
RUN mkdir -p /temp/prod
COPY package.json bun.lockb* /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

# Copy node_modules from temp directory
# Then copy all (non-ignored) project files into the image
FROM base AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY . .

# Generate Prisma client
RUN bun prisma generate

# Build the project
RUN bun run build

# Copy production dependencies and source files into final image
FROM base AS release

# Copy production dependencies and built application
COPY --from=install /temp/prod/node_modules node_modules
COPY --from=prerelease /usr/src/app/dist dist
COPY --from=prerelease /usr/src/app/prisma prisma
COPY --from=prerelease /usr/src/app/package.json .

# Create uploads directory with secure permissions
RUN mkdir -p uploads && chmod 700 uploads

# Create non-root user with minimal privileges
RUN groupadd -g 1001 -r bun && \
    useradd -u 1001 -r -g bun -d /usr/src/app -s /bin/false bun && \
    chown -R bun:bun /usr/src/app

# Switch to non-root user
USER bun

# Expose port (non-privileged)
EXPOSE 3001/tcp

# Set secure environment variables
ENV NODE_ENV=production
ENV PORT=3001
ENV LOG_LEVEL=INFO

# Set security-related environment variables
ENV NODE_OPTIONS="--max-old-space-size=512 --enable-source-maps"

# Health check with more robust check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
  CMD curl -f http://localhost:3001/health || exit 1

# Use exec form for proper signal handling
ENTRYPOINT ["bun", "run", "start:prod"]

# Labels for container metadata
LABEL org.opencontainers.image.title="Knot Space"
LABEL org.opencontainers.image.description="Online Package Repository Server"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/saravenpi/knot"
LABEL maintainer="Knot Team"