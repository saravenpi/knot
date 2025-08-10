# 🚀 Knot Space Backend Setup Guide

The backend uses **SQLX with compile-time query verification**, which requires a PostgreSQL database connection during compilation. Here's how to get it running:

## 🗄️ Database Setup

### Option 1: Local PostgreSQL (Recommended)

1. **Install PostgreSQL:**
   ```bash
   # macOS
   brew install postgresql
   brew services start postgresql
   
   # Ubuntu/Debian
   sudo apt update
   sudo apt install postgresql postgresql-contrib
   sudo systemctl start postgresql
   ```

2. **Run the setup script:**
   ```bash
   cd /Users/yannthevenin/code/Knot/apps/knot-space
   ./setup-db.sh
   ```

3. **Set environment variables:**
   ```bash
   export DATABASE_URL=postgresql://knot:password@localhost:5432/knot_space
   export JWT_SECRET=your-super-secure-jwt-secret-change-this-in-production
   ```

### Option 2: Docker PostgreSQL

1. **Start PostgreSQL container:**
   ```bash
   docker run --name knot-postgres \
     -e POSTGRES_DB=knot_space \
     -e POSTGRES_USER=knot \
     -e POSTGRES_PASSWORD=password \
     -p 5432:5432 -d postgres:16
   ```

2. **Set environment variables:**
   ```bash
   export DATABASE_URL=postgresql://knot:password@localhost:5432/knot_space
   export JWT_SECRET=your-super-secure-jwt-secret-change-this-in-production
   ```

## 🔄 Run Database Migrations

```bash
cd /Users/yannthevenin/code/Knot/apps/knot-space

# Run migrations to create tables
sqlx migrate run

# Verify migrations
sqlx migrate info
```

## 🏃 Run the Backend

```bash
# Make sure you're in the backend directory
cd /Users/yannthevenin/code/Knot/apps/knot-space

# Run in development mode
cargo run

# Or run with release optimizations
cargo run --release
```

The backend will start on **http://localhost:8000**

## 🧪 Test the API

```bash
# Health check
curl http://localhost:8000/health

# Register a user
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com", 
    "password": "password123"
  }'

# Login
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

## 📚 API Documentation

Once running, visit:
- **Swagger UI**: http://localhost:8000/swagger-ui/
- **OpenAPI Spec**: http://localhost:8000/api-docs/openapi.json

## 🚨 Troubleshooting

### "set DATABASE_URL to use query macros online"

This means SQLX cannot connect to the database during compilation. Fix:

1. Ensure PostgreSQL is running
2. Check DATABASE_URL is correct
3. Test connection: `psql $DATABASE_URL -c "SELECT 1;"`

### "Failed to connect to database"

1. Check if PostgreSQL is running: `pg_isready`
2. Verify credentials in DATABASE_URL
3. Check firewall/network access

### "Permission denied"

1. Grant proper permissions:
   ```sql
   GRANT ALL PRIVILEGES ON DATABASE knot_space TO knot;
   GRANT ALL ON SCHEMA public TO knot;
   ```

### "Migration failed"

1. Reset database:
   ```bash
   dropdb knot_space
   createdb knot_space
   sqlx migrate run
   ```

## 🎯 Quick Commands

```bash
# Full reset and start
dropdb knot_space && createdb knot_space
sqlx migrate run
cargo run

# Check compilation without running
cargo check

# Run tests
cargo test

# Format code
cargo fmt
```

## 🔧 Environment Variables

Create a `.env` file in the backend directory:

```bash
# Database
DATABASE_URL=postgresql://knot:password@localhost:5432/knot_space

# JWT Authentication
JWT_SECRET=your-super-secure-jwt-secret-change-this-in-production
ROCKET_SECRET_KEY=your-rocket-secret-key-for-cookies-and-sessions

# Optional: Custom ports
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0

# Optional: Log level
RUST_LOG=info
```

The backend is now ready to run! 🎉