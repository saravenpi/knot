# Knot 🪢

A modern monorepo package manager for any programming language that simplifies dependency management and builds across multiple apps and packages.

## ✨ Features

- **🔗 Smart Package Linking** - Automatic symlinking of local packages
- **☁️ Online Package Support** - Download packages from knot space (`@package`, `@team/package`)
- **⚡ Language Integration** - Per-app configuration with special TypeScript support
- **🔨 Build Management** - Execute build commands across apps or individually
- **▶️ Script Runner** - Run scripts from any config file with `knot run`
- **📦 Flexible Configuration** - YAML-based configuration with multiple syntax options
- **🌍 Multi-Language Support** - Works with any programming language (Rust, Go, Python, Java, etc.)

## 🚀 Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash
```

### Prerequisites
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning repositories

## 📋 Commands

### Project Management
```bash
knot init <name> [--description <desc>]    # Initialize a new project
knot status                                # Show project status
```

### Package & App Management
```bash
knot init:package <name> [--team <team>]   # Create a new package
knot init:app <name> [--description <desc>] # Create a new app
```

### Linking & Building
```bash
knot link                                  # Link packages to apps
knot build                                 # Build apps (context-aware)
knot run <script>                          # Run scripts from config files
```

### Updates & Maintenance
```bash
knot update                                # Update to latest version
knot update --force                        # Force reinstall current version
```

## 🎯 Quick Start

```bash
# 1. Initialize project
knot init my-monorepo --description "My awesome monorepo"
cd my-monorepo

# 2. Create shared packages
cd packages
knot init:package utils --team myteam
knot init:package types --team myteam
cd ..

# 3. Create applications
cd apps
knot init:app frontend --description "Web frontend"
knot init:app backend --description "API server"
cd ..

# 4. Configure dependencies and scripts in knot.yml
# 5. Link packages and build
knot link
knot build

# 6. Run development scripts
knot run dev    # Run dev script
knot run test   # Run test script
```

## ⚙️ Configuration

### Project Configuration (`knot.yml`)

```yaml
name: MyProject
description: My awesome monorepo

# Global project scripts
scripts:
  setup: "npm install && echo 'Setup complete'"
  test-all: "knot run test --all"
  clean: "rm -rf */dist */build"
  deploy: "knot build && deploy.sh"

apps:
  frontend:
    tsAlias: "@"
    packages:
      - types
      - utils
      - "@jwt"
  backend:
    tsAlias: "#"
    packages:
      - types
      - "@klysium/logger"
```

### App Configuration (`app.yml`)

```yaml
name: frontend
description: Web frontend app
tsAlias: "~"                    # Optional: Override project tsAlias (TypeScript projects)
build: "npm run build"          # Build command

# App-specific scripts
scripts:
  dev: "npm run dev"
  test: "npm run test"
  lint: "npm run lint"
  serve: "npm run serve"

packages:                       # Optional: Additional packages
  - types
  - utils
  - "@jwt"
```

### Package Configuration (`package.yml`)

```yaml
name: utils
team: myteam      # Optional: Team namespace
version: 1.0.0

# Package scripts
scripts:
  build: "npm run build"
  test: "npm run test"
  docs: "npm run docs"
  benchmark: "npm run benchmark"
```

## 🔨 Build Commands

Knot supports flexible build management with the `knot build` command:

### Context-Aware Building

- **From project root**: Builds all apps that have build commands configured
- **From app directory**: Builds only the current app

### Configuration

Add build commands to your `app.yml` files:

```yaml
name: frontend
description: Web frontend
build: "npm run build"          # Simple command
# build: "pnpm build --prod"    # Alternative build tools  
# build: "./build.sh"           # Custom scripts
# build: "cargo build --release" # Rust projects
# build: "go build -o bin/app"  # Go projects
```

### Examples

```bash
# Build all apps from project root
cd my-monorepo
knot build
# Output:
# 🔨 Building all apps for project 'MyProject'...
# 📋 Found 2 app(s) with build commands
# 🔨 Building app 'frontend'...
# 🔨 Building app 'backend'...
# 🎉 All apps built successfully!

# Build single app from app directory
cd apps/frontend
knot build
# Output:
# 🔨 Building app 'frontend'...
# 📝 Running: npm run build
# ✅ Successfully built app 'frontend'
```

## ▶️ Script Runner

Knot provides a powerful script runner similar to npm scripts, with context-aware execution:

### Context-Aware Script Resolution

Scripts are resolved in priority order:
1. **App directory** (`app.yml`) - Highest priority
2. **Package directory** (`package.yml`) - Medium priority
3. **Project root** (`knot.yml`) - Lowest priority

### Usage Examples

```bash
# Run scripts from different contexts
cd apps/frontend && knot run dev     # Runs frontend dev script
cd packages/utils && knot run test   # Runs utils test script
cd project-root && knot run setup    # Runs project setup script

# Script not found shows available options
knot run invalid-script
# Output:
# ❌ Script 'invalid-script' not found
# 💡 Available scripts:
#   📱 App scripts (frontend)
#     • dev - npm run dev
#     • test - npm run test
#     • lint - npm run lint
#   🏗️ Project scripts (MyProject)
#     • setup - npm install && echo 'Setup complete'
#     • clean - rm -rf */dist */build
```

### Script Features

- **Environment variables** - Access to full shell environment
- **Working directory** - Scripts run in their config file's directory
- **Command chaining** - Use `&&` and `||` for complex workflows
- **Cross-platform** - Works on Windows, macOS, and Linux

### Common Script Patterns

```yaml
# Development workflow
scripts:
  dev: "concurrently 'knot run dev-server' 'knot run dev-client'"
  dev-server: "npm run start:dev"
  dev-client: "npm run dev"

# Testing pipeline
scripts:
  test: "npm run test"
  test-watch: "npm run test:watch"
  test-e2e: "npm run test:e2e"
  test-all: "knot run test && knot run test-e2e"

# Build pipeline
scripts:
  prebuild: "knot run lint && knot run test"
  build: "npm run build"
  postbuild: "cp README.md dist/"
```

## 📦 Package Types

### Local Packages
- Located in `packages/` directory
- Referenced by name: `types`, `utils`
- Symlinked to `knot_packages/` in apps

### Online Packages
- **Public packages**: `@jwt`, `@axios`
- **Team packages**: `@team/package-name`
- Downloaded to `knot_packages/` in apps

## 🎨 TypeScript Integration (Optional)

Knot works with any programming language, but provides special integration features for TypeScript projects.

### Per-App Aliases
Configure TypeScript path aliases per app:

```yaml
# knot.yml
apps:
  frontend:
    tsAlias: "@"      # Creates @/* alias
  backend:
    tsAlias: "#"      # Creates #/* alias
```

### Priority Order
1. `app.yml` `tsAlias` (highest priority)
2. `knot.yml` app-specific `tsAlias`
3. `knot.yml` global `tsAlias`

### Generated TypeScript Configuration
Knot automatically updates `tsconfig.json` files:

```json
{
  "compilerOptions": {
    "paths": {
      "@/*": ["./knot_packages/*"],
      "src/*": ["./src/*"]
    }
  }
}
```

## 📁 Project Structure

```
my-monorepo/
├── knot.yml                 # Project configuration
├── packages/               # Local packages
│   ├── types/
│   │   ├── package.yml
│   │   └── src/           # Source files (any language)
│   └── utils/
│       ├── package.yml
│       └── src/           # Source files (any language)
└── apps/                   # Applications
    ├── frontend/
    │   ├── app.yml         # App configuration
    │   ├── tsconfig.json   # Auto-updated (TypeScript projects only)
    │   ├── knot_packages/  # Linked packages
    │   │   ├── types -> ../../packages/types
    │   │   ├── utils -> ../../packages/utils
    │   │   └── @jwt/       # Downloaded package
    │   └── src/
    └── backend/
        ├── app.yml
        ├── tsconfig.json       # (TypeScript projects only)
        ├── knot_packages/
        └── src/
```

## 🛠️ Manual Installation

```bash
git clone https://github.com/saravenpi/knot.git
cd knot/apps/knot-cli
cargo build --release
sudo cp target/release/knot /usr/local/bin/
```

## 📖 Advanced Usage

### Multiple Package Sources

```yaml
# knot.yml
apps:
  frontend:
    packages:
      - types              # Local package
      - utils              # Local package
      - "@jwt"            # Public online package
      - "@myteam/logger"  # Team online package
```

### Custom Build Scripts

```yaml
# app.yml
name: frontend
build: "./scripts/build.sh"    # Custom build script
```

```bash
#!/bin/bash
# scripts/build.sh
echo "🔨 Building with custom script..."
npm run lint                    # Or: cargo clippy, go fmt, etc.
npm run test                    # Or: cargo test, go test, etc.  
npm run build:prod              # Or: cargo build --release, go build, etc.
echo "✅ Build complete!"
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Repository**: [https://github.com/saravenpi/knot](https://github.com/saravenpi/knot)
- **Issues**: [https://github.com/saravenpi/knot/issues](https://github.com/saravenpi/knot/issues)
- **Documentation**: [https://github.com/saravenpi/knot](https://github.com/saravenpi/knot)
