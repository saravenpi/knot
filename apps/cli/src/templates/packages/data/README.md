# Data Package

A data management package demonstrating **heavy external dependencies** combined with **single internal dependency**. This package provides database connections, caching, and data schemas while leveraging the utils package for common operations.

## Overview

The `data` package handles all data-related operations including database connections, caching, schema validation, and data types. It demonstrates how to build a package that combines external database libraries with internal utility functions.

## Dependency Chain Position

```
utils (base package)
 ↑
data (depends on utils + heavy external deps)
 ↑
└── Other packages can use data types and operations
```

## Installation & Usage

```bash
# The package.yml automatically handles all dependencies
knot add data
```

```typescript
import { Database, Cache, Repository } from 'data';
import type { User, Post, QueryOptions } from 'data';
```

## External Dependencies

This package includes several heavy database-related dependencies:

- **@prisma/client** - Modern database ORM
- **zod** - Schema validation
- **ioredis** - Redis client for caching
- **mysql2** - MySQL database driver
- **pg** - PostgreSQL driver
- **mongodb** - MongoDB driver

## Core Features

### 1. Database Management
Multi-database support with connection pooling and retry logic.

```typescript
import { Database } from 'data';

const db = new Database({
  provider: 'postgresql',
  url: process.env.DATABASE_URL,
  poolSize: 10,
  retryAttempts: 3
});

await db.connect();

// Health check using utils package
const isHealthy = await db.healthCheck();
```

**Utils Package Integration:**
- Uses `retry()` for robust connection handling
- Uses `generateId()` for connection tracking
- Uses connection timeout logic from helpers

### 2. Caching Layer
Redis and in-memory caching with intelligent key management.

```typescript
import { Cache } from 'data';

const cache = new Cache({
  provider: 'redis',
  url: process.env.REDIS_URL,
  ttl: 3600,
  keyPrefix: 'app:'
});

await cache.set('user:123', userData, 7200);
const user = await cache.get<User>('user:123');
```

**Utils Package Integration:**
- Uses `generateId()` for cache instance tracking
- Uses `helpers.cloneDeep()` for memory cache safety
- Uses `retry()` for connection resilience

### 3. Schema Validation
Zod schemas with utils package validation integration.

```typescript
import { UserSchema, PostSchema } from 'data';
import { ValidationError } from 'utils';

try {
  const validUser = UserSchema.parse(userData);
} catch (error) {
  // Transform to utils ValidationError format
  const errors: ValidationError[] = error.errors.map(err => ({
    field: err.path.join('.'),
    message: err.message,
    value: err.received
  }));
}
```

### 4. Advanced Types
Rich type definitions extending utils package types.

```typescript
import { User, Post, QueryOptions } from 'data';
import { BaseEntity, ApiResponse } from 'utils';

// User extends BaseEntity from utils
interface UserWithMetadata extends User {
  metadata: Record<string, any>;
}

// Uses utils types for consistency
const queryOptions: QueryOptions = {
  page: 1,
  limit: 20,
  sort: [{ field: 'createdAt', direction: 'desc' }] // Uses SortConfig from utils
};
```

## Usage Examples

### Database Operations with Error Handling
```typescript
import { Database } from 'data';
import { retry } from 'utils';

const database = new Database(config);

// Robust connection with utils retry
await retry(async () => {
  await database.connect();
}, 3, 1000);

// Health monitoring
setInterval(async () => {
  const isHealthy = await database.healthCheck();
  if (!isHealthy) {
    console.warn('Database connection unhealthy');
  }
}, 30000);
```

### Cache Operations with Utils Integration
```typescript
import { Cache } from 'data';
import { helpers, generateId } from 'utils';

const cache = new Cache(cacheConfig);

// Store complex objects safely
const userData = { id: generateId('user'), name: 'John Doe' };
await cache.set('user:profile', userData);

// Retrieve with deep cloning for safety
const profile = await cache.get<User>('user:profile');
const modifiedProfile = helpers.deepMerge(profile, { lastActive: new Date() });
```

### Schema Validation with Error Transformation
```typescript
import { UserSchema, PostSchema } from 'data';
import { ValidationError } from 'utils';

function validateUserData(data: unknown): { user?: User; errors: ValidationError[] } {
  try {
    const user = UserSchema.parse(data);
    return { user, errors: [] };
  } catch (error) {
    const errors: ValidationError[] = error.errors.map(err => ({
      field: err.path.join('.'),
      message: err.message,
      value: err.received
    }));
    return { errors };
  }
}
```

## Configuration

### Database Configuration
```typescript
interface DatabaseConfig {
  provider: 'postgresql' | 'mysql' | 'mongodb' | 'sqlite';
  url: string;
  poolSize?: number;
  timeout?: number;
  retryAttempts?: number;
  ssl?: boolean;
}
```

### Cache Configuration
```typescript
interface CacheConfig {
  provider: 'redis' | 'memory';
  url?: string;
  ttl?: number;
  maxSize?: number;
  keyPrefix?: string;
}
```

## Development & Testing

### Running Tests
```bash
knot test data
```

### Database Migrations
```bash
knot run migrate    # Run Prisma migrations
knot run generate   # Generate Prisma client
knot run seed       # Seed database with test data
```

## Best Practices

### ✅ Do
- Use utils package functions instead of reimplementing common logic
- Leverage utils types for consistency across the project
- Use connection pooling for production databases
- Implement proper error handling with utils ValidationError format

### ❌ Don't
- Bypass utils package validation in favor of custom solutions
- Create database connections without retry logic
- Store sensitive connection strings in code
- Ignore connection health monitoring

## Performance Considerations

### Database Connections
- Uses connection pooling to manage resources
- Implements retry logic with exponential backoff
- Monitors connection health automatically

### Caching Strategy
- Configurable TTL for different data types
- Memory cache with LRU eviction for development
- Redis cluster support for production scaling

### Schema Validation
- Zod schemas are compiled once for performance
- Validation errors use utils format for consistency
- Support for partial validation for updates

## Monitoring and Observability

```typescript
// Database metrics
const stats = await database.getStats();
console.log(`Active connections: ${stats.activeConnections}`);

// Cache metrics
const cacheStats = await cache.getStats();
console.log(`Cache hit ratio: ${cacheStats.hitRatio}%`);
```

This package demonstrates how to build a robust data layer that combines the power of modern database tools with the consistency and reliability provided by your utils package foundation.