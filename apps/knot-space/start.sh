#!/bin/bash
set -e

echo "🚀 Starting Knot Space..."

if [ -z "$DATABASE_URL" ]; then
  echo "❌ ERROR: DATABASE_URL environment variable is not set"
  exit 1
fi

if [ -z "$JWT_SECRET" ]; then
  echo "❌ ERROR: JWT_SECRET environment variable is not set"
  exit 1
fi

npx prisma migrate deploy
node dist/server.js
