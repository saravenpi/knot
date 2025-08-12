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

# Build the project
build_project() {
    print_step "Building Knot..."
    
    cd "$TEMP_DIR/knot/apps/knot-cli"
    
    # Build in release mode for optimal performance
    if ! cargo build --release; then
        print_error "Failed to build Knot"
        exit 1
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
    
    BINARY_PATH="$TEMP_DIR/knot/apps/knot-cli/target/release/knot"
    
    if [[ ! -f "$BINARY_PATH" ]]; then
        print_error "Binary not found at $BINARY_PATH"
        exit 1
    fi
    
    # Copy binary to install directory
    if ! cp "$BINARY_PATH" "$INSTALL_DIR/knot"; then
        print_error "Failed to copy binary to $INSTALL_DIR"
        print_error "Make sure you have write permissions to $INSTALL_DIR"
        exit 1
    fi
    
    # Make it executable
    chmod +x "$INSTALL_DIR/knot"
    
    print_success "Knot installed to $INSTALL_DIR/knot"
}

# Verify installation
verify_installation() {
    print_step "Verifying installation..."
    
    if command_exists knot; then
        VERSION=$(knot info 2>/dev/null | grep "Version:" | head -1 || echo "Version: 0.1.0")
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
    echo "  knot info                     # Show help information"
    echo ""
    print_step "Documentation: https://github.com/saravenpi/knot"
}

# Main installation function
main() {
    print_header
    
    detect_system
    check_prerequisites
    create_temp_dir
    clone_repository
    build_project
    determine_install_dir
    install_binary
    verify_installation
    show_usage
    
    print_success "Installation completed successfully! 🎉"
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