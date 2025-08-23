#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_header() {
    echo -e "${GREEN}"
    echo "🪢 Knot - Monorepo Package Manager"
    echo "=================================="
    echo -e "${NC}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if this is an update vs fresh install
check_existing_installation() {
    if command_exists knot; then
        CURRENT_VERSION=$(knot --version 2>/dev/null | sed 's/knot //' || echo "unknown")
        INSTALL_PATH=$(which knot)
        IS_UPDATE=true
        print_step "Found existing Knot installation"
        print_step "Current version: $CURRENT_VERSION"
        print_step "Location: $INSTALL_PATH"
    else
        IS_UPDATE=false
        print_step "No existing Knot installation found"
    fi
}

# Get latest version from GitHub
get_latest_version() {
    print_step "Checking latest version..."

    # Try to get latest release version
    LATEST_VERSION=$(curl -s https://api.github.com/repos/saravenpi/knot/releases/latest | grep '"tag_name"' | sed 's/.*"tag_name": "\([^"]*\)".*/\1/' | sed 's/^v//')

    if [ -z "$LATEST_VERSION" ] || [ "$LATEST_VERSION" = "null" ]; then
        LATEST_VERSION="development"
        print_step "Latest version: development (from main branch)"
    else
        print_step "Latest version: $LATEST_VERSION"
    fi
}

# Check if update is needed
should_update() {
    if [ "$IS_UPDATE" = false ]; then
        return 0  # Fresh install needed
    fi

    if [ "$LATEST_VERSION" = "development" ]; then
        print_step "Development version available, will update"
        return 0  # Update to latest development
    fi

    if [ "$CURRENT_VERSION" != "$LATEST_VERSION" ]; then
        print_step "Update available: $CURRENT_VERSION → $LATEST_VERSION"
        return 0  # Update needed
    fi

    print_success "Already on latest version ($CURRENT_VERSION)"
    print_step "Use --force to reinstall anyway"
    return 1  # No update needed
}

# Build from source
force_source_build() {
    print_step "Building from source"
    return 1  # Always build from source
}

# Use cached source if available for faster compilation
use_cached_source() {
    CACHE_DIR="$HOME/.knot-build-cache"
    SRC_DIR="$CACHE_DIR/src"

    if [ -d "$SRC_DIR" ]; then
        print_step "Found cached source, updating..."
        cd "$SRC_DIR"

        # Update the repository
        if git pull origin main >/dev/null 2>&1; then
            print_success "Updated cached source"
            return 0
        else
            print_warning "Failed to update cached source, will re-clone"
            rm -rf "$SRC_DIR"
            return 1
        fi
    else
        return 1
    fi
}

# Detect OS and architecture
detect_system() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case $ARCH in
        x86_64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ;;
        armv7l)
            ARCH="armv7"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac

    print_step "Detected system: $OS-$ARCH"
}

# Check prerequisites
check_prerequisites() {
    print_step "Checking prerequisites..."

    if ! command_exists git; then
        print_error "Git is required but not installed."
        print_error "Please install git and try again."
        exit 1
    fi

    if ! command_exists rustc; then
        print_error "Rust is required but not installed."
        print_error "Install Rust from https://rustup.rs/ and try again."
        print_error "Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    if ! command_exists cargo; then
        print_error "Cargo is required but not installed."
        print_error "Please install Rust/Cargo and try again."
        exit 1
    fi

    print_success "All prerequisites found"
}

# Create temporary directory
create_temp_dir() {
    TEMP_DIR=$(mktemp -d)
    print_step "Created temporary directory: $TEMP_DIR"

    # Cleanup on exit
    trap 'rm -rf "$TEMP_DIR"' EXIT
}

# Clone repository
clone_repository() {
    print_step "Cloning Knot repository..."

    REPO_URL="${KNOT_REPO_URL:-https://github.com/saravenpi/knot.git}"

    # If running locally for testing, check for monorepo structure
    if [[ -f "$(dirname "$0")/apps/knot-cli/Cargo.toml" ]]; then
        print_warning "Using local development version"
        cp -r "$(dirname "$0")" "$TEMP_DIR/knot"
    elif [[ -f "$(dirname "$0")/Cargo.toml" ]] && [[ -d "$(dirname "$0")/../.." ]]; then
        # Running from within the CLI directory
        print_warning "Using local development version"
        cp -r "$(dirname "$0")/../.." "$TEMP_DIR/knot"
    else
        if ! git clone "$REPO_URL" "$TEMP_DIR/knot"; then
            print_error "Failed to clone repository from $REPO_URL"
            print_error "Make sure the repository exists and is accessible"
            exit 1
        fi
    fi

    print_success "Repository cloned successfully"
}

# Build the project with caching
build_project() {
    print_step "Building Knot..."

    local src_dir
    local use_cache=false

    # Try to use cached source first
    if use_cached_source; then
        src_dir="$HOME/.knot-build-cache/src"
        use_cache=true
        print_step "Using cached source for faster build"
    else
        src_dir="$TEMP_DIR/knot"
        print_step "Using fresh source"

        # Setup cache for next time
        CACHE_DIR="$HOME/.knot-build-cache"
        mkdir -p "$CACHE_DIR"

        # Copy source to cache for future builds
        if [ -d "$src_dir" ]; then
            print_step "Caching source for future updates..."
            cp -r "$src_dir" "$CACHE_DIR/src" 2>/dev/null || true
        fi
    fi

    cd "$src_dir/apps/knot-cli"

    # Use cached build directory if available
    if [ "$use_cache" = true ]; then
        export CARGO_TARGET_DIR="$HOME/.knot-build-cache/target"
        mkdir -p "$CARGO_TARGET_DIR"
        print_step "Using cached build artifacts for faster compilation"
    fi

    # Build in release mode for optimal performance
    if ! cargo build --release; then
        print_error "Failed to build Knot"
        exit 1
    fi

    # Set binary path based on build location
    if [ "$use_cache" = true ]; then
        BINARY_PATH="$HOME/.knot-build-cache/target/release/knot"
    else
        BINARY_PATH="$src_dir/apps/knot-cli/target/release/knot"
    fi

    print_success "Build completed successfully"
}

# Determine install directory
determine_install_dir() {
    # Try to find a suitable directory in PATH
    if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]] && [[ -d "$HOME/.local/bin" ]]; then
        INSTALL_DIR="$HOME/.local/bin"
    elif [[ ":$PATH:" == *":$HOME/bin:"* ]] && [[ -d "$HOME/bin" ]]; then
        INSTALL_DIR="$HOME/bin"
    elif [[ -w "/usr/local/bin" ]]; then
        INSTALL_DIR="/usr/local/bin"
    else
        # Create ~/.local/bin if it doesn't exist
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"

        # Add to PATH if not already there
        if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
            print_warning "Adding $INSTALL_DIR to your PATH"

            # Detect shell and add to appropriate config file
            if [[ -n "$ZSH_VERSION" ]] || [[ "$SHELL" == *"zsh"* ]]; then
                SHELL_CONFIG="$HOME/.zshrc"
            elif [[ "$SHELL" == *"fish"* ]]; then
                SHELL_CONFIG="$HOME/.config/fish/config.fish"
                echo "set -gx PATH \$PATH $INSTALL_DIR" >> "$SHELL_CONFIG"
                print_warning "Added to $SHELL_CONFIG. Please restart your shell or run: source $SHELL_CONFIG"
                return
            else
                SHELL_CONFIG="$HOME/.bashrc"
            fi

            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_CONFIG"
            print_warning "Added to $SHELL_CONFIG. Please restart your shell or run: source $SHELL_CONFIG"
        fi
    fi

    print_step "Installing to: $INSTALL_DIR"
}

# Install binary
install_binary() {
    print_step "Installing Knot binary..."

    # BINARY_PATH should be set by either try_binary_download or build_project
    if [[ ! -f "$BINARY_PATH" ]]; then
        print_error "Binary not found at $BINARY_PATH"
        exit 1
    fi

    # If updating existing installation, preserve the location
    if [ "$IS_UPDATE" = true ] && [ -n "$INSTALL_PATH" ]; then
        TARGET_PATH="$INSTALL_PATH"
        INSTALL_DIR=$(dirname "$INSTALL_PATH")
        print_step "Updating existing installation at $TARGET_PATH"
    else
        TARGET_PATH="$INSTALL_DIR/knot"
        print_step "Installing to $TARGET_PATH"
    fi

    # Create backup for updates
    if [ "$IS_UPDATE" = true ] && [ -f "$TARGET_PATH" ]; then
        cp "$TARGET_PATH" "$TARGET_PATH.backup" 2>/dev/null || true
        print_step "Created backup: $TARGET_PATH.backup"
    fi

    # Copy binary to install directory
    if ! cp "$BINARY_PATH" "$TARGET_PATH"; then
        print_error "Failed to copy binary to $INSTALL_DIR"
        print_error "Make sure you have write permissions to $INSTALL_DIR"

        # Restore backup if update failed
        if [ "$IS_UPDATE" = true ] && [ -f "$TARGET_PATH.backup" ]; then
            mv "$TARGET_PATH.backup" "$TARGET_PATH" 2>/dev/null || true
            print_step "Restored backup due to installation failure"
        fi
        exit 1
    fi

    # Make it executable
    chmod +x "$TARGET_PATH"

    # Remove backup if successful
    if [ "$IS_UPDATE" = true ] && [ -f "$TARGET_PATH.backup" ]; then
        rm -f "$TARGET_PATH.backup"
    fi

    if [ "$IS_UPDATE" = true ]; then
        print_success "Knot updated successfully at $TARGET_PATH"
    else
        print_success "Knot installed to $TARGET_PATH"
    fi
}

# Verify installation
verify_installation() {
    print_step "Verifying installation..."

    if command_exists knot; then
        VERSION=$(knot --version 2>/dev/null || echo "knot 0.1.0")
        print_success "Knot is installed and working!"
        print_success "$VERSION"
        print_success "Location: $(which knot)"
    else
        print_warning "Knot binary installed but not found in PATH"
        print_warning "You may need to restart your shell or add $INSTALL_DIR to your PATH"
        print_warning "Try running: export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
}

# Show usage information
show_usage() {
    echo ""
    print_step "Getting started with Knot:"
    echo "  knot init my-project          # Initialize a new project"
    echo "  knot init:package my-package  # Create a new package"
    echo "  knot init:app my-app          # Create a new app"
    echo "  knot link                     # Link packages to apps"
    echo "  knot status                   # Show project status"
    echo "  knot --help                   # Show help information"
    echo ""
    print_step "Documentation: https://github.com/saravenpi/knot"
}

# Main installation function
main() {
    # Parse arguments
    FORCE_INSTALL=false
    for arg in "$@"; do
        case $arg in
            --force)
                FORCE_INSTALL=true
                shift
                ;;
        esac
    done

    print_header

    detect_system
    check_existing_installation
    get_latest_version

    # Check if update is needed (unless forced)
    if [ "$FORCE_INSTALL" = false ] && ! should_update; then
        return 0
    fi

    check_prerequisites
    create_temp_dir

    # Build from source
    print_step "Building from source..."
    clone_repository
    build_project

    # Only determine install dir for fresh installs
    if [ "$IS_UPDATE" = false ]; then
        determine_install_dir
    fi

    install_binary
    verify_installation

    if [ "$IS_UPDATE" = true ]; then
        print_success "Update completed successfully! 🎉"
        NEW_VERSION=$(knot --version 2>/dev/null | sed 's/knot //' || echo "unknown")
        print_success "Updated from $CURRENT_VERSION to $NEW_VERSION"
    else
        show_usage
        print_success "Installation completed successfully! 🎉"
    fi
}

# Handle errors
error_handler() {
    print_error "Installation failed!"
    print_error "Please check the error messages above and try again."
    exit 1
}

trap error_handler ERR

# Run main function
main "$@"
