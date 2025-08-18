# Knot CLI Project Overview

## What is Knot?

Knot is a modern monorepo package manager for TypeScript/JavaScript projects written in Rust. It simplifies dependency management, package linking, and build orchestration across multiple applications and packages within a single repository.

## Core Concepts

**Project Structure:**
- **Root**: Contains `knot.yml` with project configuration
- **Packages**: Reusable code libraries stored in `packages/` directory
- **Apps**: Applications stored in `apps/` directory that consume packages

**Package Types:**
- **Local packages**: Located in `packages/`, referenced by name (e.g., `types`, `utils`)
- **Online packages**: Downloaded from remote registries
  - Public: `@jwt`, `@axios`
  - Team namespaced: `@team/package-name`

## Key Features

### 🔗 Smart Package Linking
- **Copy-first approach**: Local packages are copied to `knot_packages/` in apps by default
- **Optional symlinks**: Use `--symlink` flag for traditional symlink behavior
- Supports both local and remote package dependencies
- No manual `node_modules` management needed
- Better Docker and deployment compatibility with copied packages

### ⚡ TypeScript Integration
- Per-app TypeScript alias configuration (`tsAlias`)
- Automatic `tsconfig.json` path mapping updates
- Supports custom aliases like `@/*`, `#/*`, `~/*`

### 🔨 Build Management
- Context-aware building (project-wide or single app)
- Configurable build commands per app in `app.yml`
- Parallel execution support

### ▶️ Script Runner
- Context-aware script resolution with priority order:
  1. App directory (`app.yml`) - highest priority
  2. Package directory (`package.yml`) - medium priority
  3. Project root (`knot.yml`) - lowest priority
- Supports complex command chaining with `&&` and `||`

### 📦 Package Publishing with .knotignore
- **Smart file filtering**: Use `.knotignore` file to exclude unwanted files from published packages
- **GitIgnore-style syntax**: Supports glob patterns like `*.log`, `temp_*`, `*cache*`
- **Sensible defaults**: Automatically excludes common files (node_modules, .git, *.tar.gz, etc.)
- **Flexible patterns**: Supports prefix matching (`temp*`), suffix matching (`*.log`), and contains matching (`*cache*`)
- **Comment support**: Add comments with `#` and skip empty lines

## CLI Commands

### Project Management
- `knot init <name>` - Initialize new project
- `knot status` - Show project status and dependencies
- `knot info` - Display help and version information

### Package & App Management
- `knot init:package <name> [--team <team>]` - Create new package
- `knot init:app <name>` - Create new application

### Linking & Building
- `knot link` - Copy packages to apps and setup TypeScript aliases (default)
- `knot link --symlink` - Use symlinks instead of copying for development
- `knot build` - Build apps using configured build commands
- `knot run <script>` - Execute scripts from configuration files

## Configuration Files

### Project Configuration (`knot.yml`)
```yaml
name: MyProject
description: Project description
scripts:
  setup: "npm install"
  test-all: "knot run test --all"
apps:
  frontend:
    tsAlias: "@"
    packages: [types, utils, "@jwt"]
```

### App Configuration (`app.yml`)
```yaml
name: frontend
description: React frontend app
build: "npm run build"
scripts:
  dev: "vite dev --port 3000"
  test: "vitest run"
packages: [types, utils]
```

### Package Configuration (`package.yml`)
```yaml
name: utils
team: myteam
version: 1.0.0
scripts:
  build: "tsc && rollup -c"
  test: "vitest run"
```

## Technical Implementation

**Language**: Rust (using Cargo)
**Key Dependencies**:
- `clap` - Command line argument parsing
- `serde_yaml` - YAML configuration parsing
- `tokio` - Async runtime for command execution
- `reqwest` - HTTP client for downloading packages
- `symlink` - Cross-platform symbolic linking

**Architecture**:
- `main.rs` - CLI command routing
- `commands.rs` - Command implementations
- `config.rs` - Configuration structs and parsing
- `project.rs` - Project discovery and loading
- `linker.rs` - Package linking logic
- `typescript.rs` - TypeScript alias management
- `downloader.rs` - Remote package downloading

## Development Workflow

1. **Initialize** project with `knot init`
2. **Create** packages and apps with `init:package` and `init:app`
3. **Configure** dependencies and tags in YAML files
4. **Link** packages with `knot link` (copies by default, use `--symlink` for development)
5. **Build** with `knot build`
6. **Run** development scripts with `knot run <script>`
7. **Configure** publishing with `.knotignore` file to exclude unwanted files
8. **Publish** packages with `knot publish` (requires authentication)

## .knotignore File Usage

The `.knotignore` file allows you to specify which files and directories should be excluded when publishing packages. It works similarly to `.gitignore` but specifically for package publishing.

### Pattern Syntax
- **Exact match**: `filename.txt` - matches exactly that file
- **Extension match**: `*.log` - matches all .log files  
- **Prefix match**: `temp*` - matches files starting with "temp"
- **Suffix match**: `*cache` - matches files ending with "cache"
- **Contains match**: `*cache*` - matches files containing "cache"
- **Directory match**: `node_modules/` - matches directories
- **Comments**: `# This is a comment` - lines starting with # are ignored

### Default Excluded Files
Even without a `.knotignore` file, Knot automatically excludes:
- `*.tar.gz`, `*.tgz` - Previous package archives
- `.git/` - Git repository data
- `node_modules/` - Node.js dependencies
- `target/` - Rust build directory
- `.DS_Store`, `Thumbs.db` - OS system files
- `*.tmp`, `*.temp` - Temporary files
- `.env`, `.env.local` - Environment files (may contain secrets)

### Example .knotignore
```
# Build artifacts
dist/
build/
*.tar.gz

# Development files
.vscode/
*.log
temp_*

# Test files (if you don't want to publish tests)
__tests__/
*.test.js
```

## New Features Added

### 🏷️ **Package Tags Support**
- Add `tags` array to `package.yml` for better discoverability
- Search packages by tags in the web interface
- Automatic tag validation and normalization

### 👤 **User Management**
- Account registration and login via CLI and web UI
- Account deletion with cascade cleanup
- JWT token-based authentication

### 👥 **Team Collaboration**
- Create and manage teams via CLI
- Role-based permissions (owner, admin, member)
- Team package namespacing (`@team/package`)

### 🐳 **Production Ready Deployment**
- Complete Docker containerization
- docker-compose for full-stack deployment
- Nginx reverse proxy with SSL support
- PostgreSQL database with migrations
- Health checks and monitoring

## Package Linking Behavior

**Default (Copy Mode)**:
- Packages are copied to `knot_packages/` directories
- Better for Docker builds and deployment
- No symlink compatibility issues
- Files can be committed to git
- Pre-commit hook automatically updates copied packages

**Development (Symlink Mode)**:
- Use `knot link --symlink` for development with live updates
- Changes to packages are immediately reflected in apps
- Not suitable for Docker or production deployments
- Symlinks may not work across all platforms/environments

## Testing Commands

Based on the project structure, likely test commands:
- `cargo test` - Run Rust unit tests
- `cargo build --release` - Build production binary
- `./install.sh` - Install script for distribution

## Linting/Type Checking

For Rust projects:
- `cargo check` - Fast syntax and type checking
- `cargo clippy` - Rust linting
- `cargo fmt` - Code formatting
