# 🚂 Railway Deployment Guide for Knot Space

This guide explains how to deploy the Knot Space backend to Railway.

## 🏗️ Dockerfile Changes Made

The Dockerfile has been updated to work properly with Railway's build environment:

### Key Updates:
1. **Smart build approach** - Tests database connectivity during build, uses optimal compilation mode
2. **Railway build compatibility** - Handles Railway's build-time database access limitations
3. **Application-level migrations** - Migrations handled directly in Rust code at startup
4. **No sqlx-cli dependency** - Avoids Rust edition 2024 compatibility issues
5. **Fallback sqlx-data.json** - Reliable offline compilation when database unreachable
6. **Production-ready configuration** - Includes health checks and proper error handling

### Build Process:
- **Database connectivity test**: Tests if DATABASE_URL is accessible during build
- **Online mode**: If database is reachable, SQLx queries verified against database
- **Offline mode fallback**: If no DATABASE_URL or database unreachable, uses `sqlx-data.json`
- **Runtime migrations**: Migrations run by application at startup using `sqlx::migrate!()`

## 🚀 Railway Deployment Steps

### 1. Connect Repository
```bash
# In Railway dashboard:
1. Click "New Project"
2. Select "Deploy from GitHub repo"
3. Choose your Knot monorepo
4. Set the root directory to: apps/knot-space
```

### 2. Add PostgreSQL Database
```bash
# In Railway project:
1. Click "New"
2. Select "Database" → "PostgreSQL" 
3. Railway automatically provides DATABASE_URL
```

### 3. Environment Variables
Railway automatically sets:
- `DATABASE_URL` (from PostgreSQL service)

You need to add:
```bash
JWT_SECRET=your-super-secure-jwt-secret-change-this-in-production
ROCKET_SECRET_KEY=your-rocket-secret-key-for-cookies-and-sessions
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
RUST_LOG=info
```

### 4. Build Configuration
Railway automatically detects the Dockerfile and builds the project.

**Build Context**: `apps/knot-space`
**Dockerfile**: `apps/knot-space/Dockerfile`

### 5. Deployment
```bash
# Railway handles:
1. Building the Docker container with DATABASE_URL
2. Running database migrations on startup
3. Starting the Knot Space API server
4. Health checks and automatic restarts
```

## 🔧 Local Testing with Railway CLI

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login to Railway
railway login

# Link to your project
railway link

# Run locally with Railway environment
railway run cargo run
```

## 🧪 Testing the Deployment

Once deployed, test your API:

```bash
# Replace YOUR_RAILWAY_URL with your actual Railway URL
export API_URL="https://your-app-name.railway.app"

# Health check
curl $API_URL/health

# Register a user
curl -X POST $API_URL/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'

# API documentation
open $API_URL/swagger-ui/
```

## 📊 Monitoring and Logs

### Railway Dashboard:
- **Metrics**: CPU, Memory, Network usage
- **Logs**: Real-time application logs
- **Deployments**: Build history and rollback options

### Health Check Endpoint:
```bash
GET /health
```

Returns:
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## 🚨 Troubleshooting

### Build Fails: "sqlx-data.json not found"
✅ **Fixed** - Dockerfile no longer requires this file

### Build Fails: "feature `edition2024` is required" (sqlx-cli)
✅ **Fixed** - Removed sqlx-cli dependency entirely, migrations now handled by application code

### Build Fails: "error communicating with database: Name or service not known"
✅ **Fixed** - Dockerfile tests database connectivity before compilation, falls back to offline mode if unreachable

**Note**: Railway provides DATABASE_URL during build but the database may not be accessible from the build container due to networking limitations. The Dockerfile now handles this gracefully.

### Build Fails: "failed to connect to database"
- Ensure PostgreSQL service is running in Railway
- Check DATABASE_URL is properly set as build arg
- This may be normal behavior - the build will automatically fall back to offline mode

### Runtime: "Migration failed"
- Check database permissions
- Verify migration files are copied to container
- Check startup logs for specific error messages

### Runtime: "Database connection failed"
- Verify PostgreSQL service is healthy
- Check DATABASE_URL format in environment variables
- Ensure network connectivity between services

### Performance Issues
- Monitor memory usage in Railway dashboard
- Check database connection pool settings
- Review query performance with PostgreSQL logs

## 🔐 Security Considerations

### Environment Variables
- Use Railway's secure environment variables
- Never commit secrets to git
- Rotate JWT_SECRET and ROCKET_SECRET_KEY regularly

### Database Security
- Railway PostgreSQL includes SSL by default
- Use connection pooling for better performance
- Monitor for unusual query patterns

### API Security
- Rate limiting is handled by Railway
- CORS is configured in the Rust application
- JWT tokens expire automatically

## 📈 Scaling

Railway automatically handles:
- **Horizontal scaling**: Multiple instances based on load
- **Vertical scaling**: CPU and memory adjustments
- **Database scaling**: Connection pooling and optimization

### Manual Scaling Options:
```bash
# In Railway dashboard:
1. Go to Settings → Environment
2. Adjust "Replicas" for horizontal scaling
3. Adjust "Resources" for vertical scaling
```

## 🔄 CI/CD Integration

Railway automatically deploys on git pushes to main branch.

### Custom Deploy Triggers:
```bash
# Railway CLI
railway up

# GitHub Actions (example)
- name: Deploy to Railway
  run: railway up --detach
```

## 📚 Additional Resources

- [Railway Documentation](https://docs.railway.app/)
- [PostgreSQL on Railway](https://docs.railway.app/databases/postgresql)
- [Environment Variables](https://docs.railway.app/develop/variables)
- [Custom Domains](https://docs.railway.app/deploy/custom-domains)