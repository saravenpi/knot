# Knot Makefile
# A modern monorepo package manager

# Variables
BINARY_NAME := knot
CARGO := cargo
TARGET_DIR := target
INSTALL_DIR := /usr/local/bin
LOCAL_INSTALL_DIR := $(HOME)/.local/bin

# Default target
.PHONY: all
all: build

# Help target
.PHONY: help
help:
	@echo "🪢 Knot - Monorepo Package Manager"
	@echo "=================================="
	@echo ""
	@echo "Available targets:"
	@echo "  📦 build       - Build the project in debug mode"
	@echo "  🚀 release     - Build the project in release mode"
	@echo "  🧪 test        - Run all tests"
	@echo "  🔧 dev         - Build and run in development mode"
	@echo "  🧹 clean       - Clean build artifacts"
	@echo "  📋 check       - Check code without building"
	@echo "  🎨 fmt         - Format code with rustfmt"
	@echo "  📎 clippy      - Run clippy linter"
	@echo "  📊 bench       - Run benchmarks"
	@echo ""
	@echo "Installation targets:"
	@echo "  🌐 install     - Install to $(INSTALL_DIR) (requires sudo)"
	@echo "  🏠 install-user- Install to $(LOCAL_INSTALL_DIR)"
	@echo "  🗑️  uninstall   - Remove from $(INSTALL_DIR) (requires sudo)"
	@echo ""
	@echo "Development targets:"
	@echo "  🔄 watch       - Watch for changes and rebuild"
	@echo "  📝 docs        - Generate documentation"
	@echo "  🧪 test-all    - Run all tests including integration tests"
	@echo "  🚢 docker      - Build Docker image"
	@echo "  🐳 docker-run  - Run Docker container"
	@echo "  📊 docker-dev  - Start development environment with Docker Compose"
	@echo ""
	@echo "Utility targets:"
	@echo "  📏 size        - Show binary size information"
	@echo "  🔍 audit       - Run security audit"
	@echo "  🆙 update      - Update dependencies"

# Build targets
.PHONY: build
build:
	@echo "🔨 Building $(BINARY_NAME) in debug mode..."
	$(CARGO) build

.PHONY: release
release:
	@echo "🚀 Building $(BINARY_NAME) in release mode..."
	$(CARGO) build --release

.PHONY: dev
dev: build
	@echo "🔧 Running $(BINARY_NAME) in development mode..."
	$(TARGET_DIR)/debug/$(BINARY_NAME) info

# Test targets
.PHONY: test
test:
	@echo "🧪 Running tests..."
	$(CARGO) test

.PHONY: test-all
test-all:
	@echo "🧪 Running all tests (including integration tests)..."
	$(CARGO) test --all-features
	@echo "🧪 Running doc tests..."
	$(CARGO) test --doc

.PHONY: bench
bench:
	@echo "📊 Running benchmarks..."
	$(CARGO) bench

# Code quality targets
.PHONY: check
check:
	@echo "📋 Checking code..."
	$(CARGO) check --all-targets --all-features

.PHONY: fmt
fmt:
	@echo "🎨 Formatting code..."
	$(CARGO) fmt --all

.PHONY: clippy
clippy:
	@echo "📎 Running clippy..."
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: audit
audit:
	@echo "🔍 Running security audit..."
	$(CARGO) audit

# Clean targets
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf $(TARGET_DIR)

# Installation targets
.PHONY: install
install: release
	@echo "🌐 Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@if [ ! -w "$(INSTALL_DIR)" ]; then \
		echo "❌ Permission denied. Try: sudo make install"; \
		exit 1; \
	fi
	cp $(TARGET_DIR)/release/$(BINARY_NAME) $(INSTALL_DIR)/
	chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) installed to $(INSTALL_DIR)/$(BINARY_NAME)"
	@echo "💡 Run '$(BINARY_NAME) info' to verify installation"

.PHONY: install-user
install-user: release
	@echo "🏠 Installing $(BINARY_NAME) to $(LOCAL_INSTALL_DIR)..."
	@mkdir -p $(LOCAL_INSTALL_DIR)
	cp $(TARGET_DIR)/release/$(BINARY_NAME) $(LOCAL_INSTALL_DIR)/
	chmod +x $(LOCAL_INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) installed to $(LOCAL_INSTALL_DIR)/$(BINARY_NAME)"
	@echo "💡 Make sure $(LOCAL_INSTALL_DIR) is in your PATH"
	@echo "💡 Add this to your shell config: export PATH=\"$(LOCAL_INSTALL_DIR):\$$PATH\""

.PHONY: uninstall
uninstall:
	@echo "🗑️ Removing $(BINARY_NAME) from $(INSTALL_DIR)..."
	@if [ ! -w "$(INSTALL_DIR)" ]; then \
		echo "❌ Permission denied. Try: sudo make uninstall"; \
		exit 1; \
	fi
	rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "✅ $(BINARY_NAME) uninstalled"

# Development targets
.PHONY: watch
watch:
	@echo "🔄 Watching for changes..."
	@if command -v cargo-watch >/dev/null 2>&1; then \
		cargo watch -x 'build' -x 'test'; \
	else \
		echo "❌ cargo-watch not found. Install with: cargo install cargo-watch"; \
		exit 1; \
	fi

.PHONY: docs
docs:
	@echo "📝 Generating documentation..."
	$(CARGO) doc --no-deps --open

.PHONY: docs-build
docs-build:
	@echo "📝 Building documentation..."
	$(CARGO) doc --no-deps

# Docker targets
.PHONY: docker
docker:
	@echo "🚢 Building Docker image..."
	docker build -t $(BINARY_NAME):latest .

.PHONY: docker-run
docker-run:
	@echo "🚢 Running Docker container..."
	docker run --rm -it $(BINARY_NAME):latest

.PHONY: docker-dev
docker-dev:
	@echo "📊 Starting development environment..."
	docker-compose --profile dev up

.PHONY: docker-build-compose
docker-build-compose:
	@echo "🔨 Building with Docker Compose..."
	docker-compose --profile build up

.PHONY: docker-test-compose
docker-test-compose:
	@echo "🧪 Running tests with Docker Compose..."
	docker-compose --profile test up

.PHONY: docker-docs
docker-docs:
	@echo "📝 Starting documentation server..."
	docker-compose --profile docs up

# Utility targets
.PHONY: size
size: release
	@echo "📏 Binary size information:"
	@ls -lh $(TARGET_DIR)/release/$(BINARY_NAME) | awk '{print "  Size: " $$5}'
	@if command -v du >/dev/null 2>&1; then \
		echo "  Disk usage: $$(du -h $(TARGET_DIR)/release/$(BINARY_NAME) | cut -f1)"; \
	fi
	@if command -v file >/dev/null 2>&1; then \
		echo "  File type: $$(file $(TARGET_DIR)/release/$(BINARY_NAME) | cut -d: -f2)"; \
	fi

.PHONY: update
update:
	@echo "🆙 Updating dependencies..."
	$(CARGO) update
	@echo "📋 Checking for outdated dependencies..."
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		cargo outdated; \
	else \
		echo "💡 Install cargo-outdated for dependency updates: cargo install cargo-outdated"; \
	fi

# Comprehensive development workflow
.PHONY: pre-commit
pre-commit: fmt clippy test
	@echo "✅ Pre-commit checks passed!"

.PHONY: ci
ci: check clippy test test-all audit
	@echo "✅ CI pipeline completed successfully!"

# Release workflow
.PHONY: prepare-release
prepare-release: clean ci release size
	@echo "🎉 Release build ready!"
	@echo "📦 Binary location: $(TARGET_DIR)/release/$(BINARY_NAME)"

# Quick test with examples
.PHONY: test-examples
test-examples: release
	@echo "🧪 Testing with example project..."
	@mkdir -p /tmp/knot-test
	@cd /tmp/knot-test && \
		$(PWD)/$(TARGET_DIR)/release/$(BINARY_NAME) init test-project --description "Test project" && \
		cd test-project && \
		$(PWD)/$(TARGET_DIR)/release/$(BINARY_NAME) status && \
		echo "✅ Example test completed"
	@rm -rf /tmp/knot-test

# Development dependencies
.PHONY: dev-deps
dev-deps:
	@echo "📦 Installing development dependencies..."
	@echo "Installing cargo-watch..."
	cargo install cargo-watch
	@echo "Installing cargo-outdated..."
	cargo install cargo-outdated
	@echo "Installing cargo-audit..."
	cargo install cargo-audit
	@echo "✅ Development dependencies installed"

# Show current version
.PHONY: version
version:
	@echo "📦 Knot version information:"
	@grep "^version" Cargo.toml | head -1
	@if [ -f $(TARGET_DIR)/release/$(BINARY_NAME) ]; then \
		echo "Built binary version:"; \
		$(TARGET_DIR)/release/$(BINARY_NAME) info | grep Version || true; \
	fi

# Verify installation works
.PHONY: verify
verify:
	@echo "🔍 Verifying knot installation..."
	@if command -v knot >/dev/null 2>&1; then \
		echo "✅ knot found in PATH"; \
		knot info; \
	else \
		echo "❌ knot not found in PATH"; \
		echo "💡 Try: make install or make install-user"; \
		exit 1; \
	fi

# Build targets with timing
.PHONY: build-timed
build-timed:
	@echo "⏱️ Building with timing..."
	time $(CARGO) build

.PHONY: release-timed
release-timed:
	@echo "⏱️ Building release with timing..."
	time $(CARGO) build --release
