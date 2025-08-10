# Knot Space - Production Ready Makefile

.PHONY: help dev prod build clean test lint logs backup

# Default target
help: ## Show this help message
	@echo "🪢 Knot Space - Package Registry"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development
dev: ## Start development environment
	@echo "🚀 Starting development environment..."
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
	@echo "✅ Services started:"
	@echo "   Frontend: http://localhost:3000"
	@echo "   Backend:  http://localhost:8000"
	@echo "   Database: localhost:5432"

dev-logs: ## Show development logs
	docker compose -f docker-compose.yml -f docker-compose.dev.yml logs -f

# Production
prod: ## Start production environment
	@echo "🚀 Starting production environment..."
	@if [ ! -f .env ]; then echo "⚠️  .env file not found! Copy .env.example and configure."; exit 1; fi
	docker compose up -d
	@echo "✅ Production services started!"
	@echo "   Check status: make status"

prod-build: ## Build production images
	@echo "🔨 Building production images..."
	docker compose build --no-cache
	@echo "✅ Images built successfully!"

# Management
status: ## Show service status
	@echo "📊 Service Status:"
	docker compose ps

health: ## Check service health
	@echo "🏥 Health Checks:"
	@curl -s http://localhost/health | jq '.' || echo "❌ Health check failed"

logs: ## Show production logs
	docker compose logs -f --tail=100

stop: ## Stop all services
	@echo "⏹️  Stopping services..."
	docker compose down

restart: ## Restart all services
	@echo "🔄 Restarting services..."
	docker compose restart

# Development tools
build: ## Build all applications locally
	@echo "🔨 Building CLI..."
	cd apps/knot-cli && cargo build --release
	@echo "🔨 Building Backend..."
	cd apps/knot-space && cargo build --release
	@echo "🔨 Building Frontend..."
	cd apps/knot-space-ui && npm install && npm run build

test: ## Run all tests
	@echo "🧪 Running CLI tests..."
	cd apps/knot-cli && cargo test
	@echo "🧪 Running Backend tests..."
	cd apps/knot-space && cargo test

lint: ## Run linting
	@echo "🔍 Linting CLI..."
	cd apps/knot-cli && cargo clippy -- -D warnings
	@echo "🔍 Linting Backend..."
	cd apps/knot-space && cargo clippy -- -D warnings
	@echo "🔍 Linting Frontend..."
	cd apps/knot-space-ui && npm run lint

fmt: ## Format code
	@echo "✨ Formatting Rust code..."
	cd apps/knot-cli && cargo fmt
	cd apps/knot-space && cargo fmt
	@echo "✨ Formatting Frontend code..."
	cd apps/knot-space-ui && npm run format

# Database
db-migrate: ## Run database migrations
	docker compose exec knot-space sqlx migrate run

db-reset: ## Reset database (WARNING: Destroys all data)
	@echo "⚠️  This will destroy all data! Press Ctrl+C to cancel..."
	@sleep 5
	docker compose down -v
	docker compose up postgres -d
	@sleep 10
	$(MAKE) db-migrate

db-backup: ## Backup database
	@echo "💾 Creating database backup..."
	@mkdir -p backups
	docker compose exec postgres pg_dump -U knot knot_space > backups/backup_$$(date +%Y%m%d_%H%M%S).sql
	@echo "✅ Backup created in backups/ directory"

# Maintenance
clean: ## Clean up Docker resources
	@echo "🧹 Cleaning up..."
	docker compose down --remove-orphans
	docker system prune -f
	docker volume prune -f

clean-all: ## Clean up everything (WARNING: Removes all data)
	@echo "⚠️  This will remove all data! Press Ctrl+C to cancel..."
	@sleep 5
	docker compose down -v --remove-orphans
	docker system prune -af
	docker volume prune -af

update: ## Update to latest version
	@echo "🔄 Updating Knot Space..."
	git pull origin main
	docker compose build --no-cache
	docker compose down
	docker compose up -d
	@echo "✅ Update complete!"

# CLI Installation
install-cli: ## Install Knot CLI globally
	@echo "📦 Installing Knot CLI..."
	cd apps/knot-cli && cargo build --release
	sudo cp apps/knot-cli/target/release/knot /usr/local/bin/
	@echo "✅ CLI installed to /usr/local/bin/knot"

# Monitoring
monitor: ## Monitor system resources
	@echo "📊 System Resources:"
	docker stats --no-stream

tail-error: ## Show error logs only
	docker compose logs --tail=50 | grep -i error

# Security
security-scan: ## Run security scans
	@echo "🔒 Running security scans..."
	docker compose exec knot-space cargo audit
	cd apps/knot-space-ui && npm audit

ssl-check: ## Check SSL certificate
	@echo "🔒 Checking SSL certificate..."
	openssl s_client -connect localhost:443 -servername localhost < /dev/null 2>/dev/null | openssl x509 -text -noout

# Backup and restore
backup: ## Create full backup
	@echo "💾 Creating full backup..."
	@mkdir -p backups
	$(MAKE) db-backup
	@tar czf backups/storage_backup_$$(date +%Y%m%d_%H%M%S).tar.gz -C /var/lib/docker/volumes/knot-storage/_data . 2>/dev/null || true
	@echo "✅ Full backup created"

restore-db: ## Restore database from backup file (make restore-db FILE=backup.sql)
	@if [ -z "$(FILE)" ]; then echo "Usage: make restore-db FILE=backup.sql"; exit 1; fi
	@echo "📥 Restoring database from $(FILE)..."
	docker compose exec -T postgres psql -U knot -d knot_space < $(FILE)
	@echo "✅ Database restored"

# Quick setup
setup: ## Quick development setup
	@echo "🚀 Setting up Knot Space for development..."
	@if [ ! -f .env ]; then cp .env.example .env; echo "📝 Created .env file - please review and update!"; fi
	docker compose -f docker-compose.yml -f docker-compose.dev.yml pull
	docker compose -f docker-compose.yml -f docker-compose.dev.yml build
	$(MAKE) dev
	@echo ""
	@echo "✅ Development environment ready!"
	@echo "   Frontend: http://localhost:3000"
	@echo "   Backend:  http://localhost:8000"
	@echo "   API Docs: http://localhost:8000/swagger-ui/"

setup-prod: ## Quick production setup
	@echo "🚀 Setting up Knot Space for production..."
	@if [ ! -f .env ]; then cp .env.example .env; echo "⚠️  Please configure .env file with secure values!"; exit 1; fi
	docker compose pull
	docker compose build
	$(MAKE) prod
	@echo ""
	@echo "✅ Production environment ready!"
	@echo "   Web UI:   http://localhost"
	@echo "   API:      http://localhost/api"