# Knot Docker Image
# Using install script from GitHub

FROM debian:bookworm-slim

# Install runtime dependencies including Rust toolchain
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/bash knot

# Install Rust for the install script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install knot using the install script from GitHub
RUN curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash

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
