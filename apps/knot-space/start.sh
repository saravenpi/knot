#!/bin/bash
set -e

echo "🚀 Starting Knot Space..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ ERROR: DATABASE_URL environment variable is not set"
    exit 1
fi

echo "🔍 Testing database connection..."
# Test database connectivity
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if pg_isready -d "$DATABASE_URL" >/dev/null 2>&1; then
        echo "✅ Database is ready"
        break
    else
        RETRY_COUNT=$((RETRY_COUNT + 1))
        echo "⏳ Database connection attempt $RETRY_COUNT/$MAX_RETRIES failed, retrying in 2 seconds..."
        sleep 2
    fi
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo "❌ Failed to connect to database after $MAX_RETRIES attempts"
    exit 1
fi

echo "📊 Database migrations will be handled by the application..."
echo "🎯 Starting Knot Space server..."
exec knot-space