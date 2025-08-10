#!/bin/bash

# Knot Space Database Setup Script
# Run this script to set up a PostgreSQL database for development

echo "🗄️  Setting up Knot Space database..."

# Check if PostgreSQL is running
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL not found. Please install PostgreSQL first:"
    echo "   macOS: brew install postgresql"
    echo "   Ubuntu: sudo apt install postgresql postgresql-contrib"
    echo "   Or use Docker: docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres"
    exit 1
fi

# Try to connect to PostgreSQL
if ! psql -U postgres -c '\q' 2>/dev/null; then
    echo "❌ Cannot connect to PostgreSQL. Please ensure:"
    echo "   1. PostgreSQL is running"
    echo "   2. You can connect as user 'postgres'"
    echo "   3. Or set DATABASE_URL environment variable"
    echo ""
    echo "To start PostgreSQL:"
    echo "   macOS: brew services start postgresql"
    echo "   Ubuntu: sudo systemctl start postgresql"
    echo "   Docker: docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres"
    exit 1
fi

# Create database and user
echo "📝 Creating database and user..."

psql -U postgres <<EOF
-- Create database
DROP DATABASE IF EXISTS knot_space;
CREATE DATABASE knot_space;

-- Create user (skip if already exists)
DO \$\$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_roles 
      WHERE rolname = 'knot') THEN
      CREATE USER knot WITH PASSWORD 'password';
   END IF;
END
\$\$;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE knot_space TO knot;

\c knot_space

-- Grant schema privileges  
GRANT ALL ON SCHEMA public TO knot;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO knot;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO knot;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO knot;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO knot;
EOF

if [ $? -eq 0 ]; then
    echo "✅ Database setup completed!"
    echo ""
    echo "🔧 Set these environment variables:"
    echo "   export DATABASE_URL=postgresql://knot:password@localhost:5432/knot_space"
    echo "   export JWT_SECRET=your-super-secure-jwt-secret-change-this-in-production"
    echo ""
    echo "🚀 Now you can run:"
    echo "   cd apps/knot-space"
    echo "   sqlx migrate run"
    echo "   cargo run"
else
    echo "❌ Database setup failed. Please check PostgreSQL connection."
    exit 1
fi