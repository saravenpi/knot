#!/bin/bash
set -e

echo "🚀 Starting OffOn Backend..."

# Fix permissions for uploads directory if it exists
if [ -d "./uploads" ]; then
    echo "🔧 Fixing uploads directory permissions..."
    chown -R appuser:appuser ./uploads || true
else
    echo "📂 Creating uploads directory..."
    mkdir -p ./uploads
    chown -R appuser:appuser ./uploads
fi

# Check if we're in production or development
if [ "$NODE_ENV" = "production" ]; then
    echo "📦 Running in PRODUCTION mode"

    # Wait for database to be available
    echo "⏳ Waiting for database connection..."
    while ! bunx prisma db push --accept-data-loss > /dev/null 2>&1; do
        echo "Database not ready, waiting 2 seconds..."
        sleep 2
    done

    echo "✅ Database is ready"
    echo "🏃 Starting production server..."

    # Switch to appuser to run the server
    exec su appuser -c "bun src/index.ts"

else
    echo "🔧 Running in DEVELOPMENT mode"

    # Generate Prisma client if needed
    echo "🔨 Generating Prisma client..."
    bunx prisma generate

    # Push database schema (development only)
    echo "🗃️ Syncing database schema..."
    bunx prisma db push --accept-data-loss

    echo "✅ Database setup complete"
    echo "🏃 Starting development server with hot reload..."
    exec su appuser -c "bun run --hot src/index.ts"
fi

