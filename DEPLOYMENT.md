# Knot Space Deployment Guide

This guide covers deploying the Knot Space application with proper package management and shared types.

## 🏗️ Architecture Overview

The Knot Space application consists of:
- **Backend API** (`knot-space`): Bun-based API server with Prisma ORM
- **Frontend UI** (`knot-space-ui`): SvelteKit application
- **Shared Types** (`packages/types`): TypeScript type definitions shared between apps
- **PostgreSQL Database**: Data persistence
- **Redis**: Caching (optional)

## 📦 Knot Package Management

The application uses Knot's package management system for shared types:

- **Source Package**: `packages/types/index.ts` contains all shared TypeScript types
- **Linking**: `knot link` creates symlinks in `apps/*/knot_packages/types`
- **Imports**: Both apps import types using `#/types` alias
- **Build Process**: Docker builds run `knot link` to ensure types are available

## 🐳 Docker Deployment

### Prerequisites

1. Docker and Docker Compose installed
2. Environment variables configured
3. Database migrations ready

### Quick Start

```bash
# Clone the repository
git clone <repo-url>
cd knot

# Start the full stack
docker-compose up -d

# Check service health
docker-compose ps
```

### Individual Service Builds

```bash
# Build backend only
docker build -t knot-space-api -f ./apps/knot-space/Dockerfile .

# Build frontend only  
docker build -t knot-space-ui -f ./apps/knot-space-ui/Dockerfile .

# Test builds
./test-docker-build.sh
```

## 🔧 Build Process Details

### Backend Build Steps

1. **Knot CLI Build**: Compiles Rust-based Knot CLI from source
2. **Dependencies**: Installs Node.js/Bun dependencies
3. **Prisma**: Generates database client
4. **Package Linking**: Runs `knot link` to create type symlinks
5. **Application Build**: Compiles TypeScript/Bun application
6. **Production Image**: Copies built app and linked packages

### Frontend Build Steps

1. **Knot CLI Build**: Same Rust compilation as backend
2. **Dependencies**: Installs npm dependencies
3. **Package Linking**: Runs `knot link` for TypeScript aliases
4. **SvelteKit Build**: Compiles frontend application
5. **Production Image**: Creates optimized Node.js runtime

## 🌍 Environment Configuration

### Backend Environment Variables

```bash
NODE_ENV=production
DATABASE_URL=postgresql://user:password@host:5432/knot_space
JWT_SECRET=your-secret-key-change-in-production
REDIS_URL=redis://redis:6379
```

### Frontend Environment Variables

```bash
PUBLIC_BACKEND_URL=http://api.example.com
```

## 📊 Health Checks

Both services include health checks:

- **Backend**: `curl -f http://localhost:3001/health`
- **Frontend**: `wget --spider http://localhost:3000`
- **Database**: `pg_isready -U postgres`

## 🚀 Production Deployment

### Railway/Render/Vercel

1. Set build command: `docker build -f ./apps/knot-space/Dockerfile .`
2. Configure environment variables
3. Set up database connection
4. Deploy with health checks enabled

### Kubernetes

```yaml
# Example deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: knot-space-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: knot-space-api
  template:
    metadata:
      labels:
        app: knot-space-api
    spec:
      containers:
      - name: api
        image: knot-space-api:latest
        ports:
        - containerPort: 3001
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: knot-secrets
              key: database-url
```

## 🔍 Troubleshooting

### Common Issues

1. **Types not found**: Ensure `knot link` ran successfully during build
2. **Database connection**: Check DATABASE_URL and database is running
3. **CORS errors**: Verify PUBLIC_BACKEND_URL is correct
4. **Build failures**: Check Docker build context includes all necessary files

### Debugging Commands

```bash
# Check linked packages in container
docker run -it knot-space-api:latest ls -la /app/knot_packages

# Verify TypeScript compilation
docker run -it knot-space-api:latest npx tsc --noEmit

# Check knot configuration
docker run -it knot-space-api:latest knot status
```

### Logs and Monitoring

```bash
# View service logs
docker-compose logs -f knot-space-api
docker-compose logs -f knot-space-ui

# Monitor health checks
docker-compose ps
```

## 📝 Development vs Production

| Aspect | Development | Production |
|--------|-------------|------------|
| Package Linking | `knot link` on host | `knot link` in Docker |
| Types Location | Local `knot_packages/` | Container `knot_packages/` |
| Hot Reload | File watching | None |
| Database | Local/Docker | Managed service |
| Secrets | `.env` files | Environment variables |

## 🔒 Security Considerations

1. **Environment Variables**: Never commit secrets to version control
2. **Database**: Use connection pooling and SSL in production
3. **CORS**: Configure proper origins for API access
4. **User Permissions**: Run containers as non-root users
5. **Health Checks**: Monitor service availability

## 📈 Scaling

- **Horizontal**: Multiple container instances behind load balancer
- **Database**: Connection pooling, read replicas
- **CDN**: Serve static assets from CDN
- **Caching**: Redis for session and API response caching