# API Client Package

An HTTP client package demonstrating **multiple internal dependencies**. This package depends on both `utils` and `data` packages, showcasing how to build complex functionality by composing simpler packages.

## Overview

The `api-client` package provides a full-featured HTTP client with automatic retries, authentication, validation, and type safety. It demonstrates how packages can depend on multiple internal packages to avoid duplicating functionality.

## Dependency Chain Position

```
utils (base package)
 ↑
data (depends on utils)
 ↑
api-client (depends on utils + data)
 ↑
└── Applications and other packages can depend on this
```

## Installation & Usage

```bash
# The package.yml automatically handles all internal dependencies
knot add api-client
```

```typescript
import { ApiClient } from 'api-client';
import type { ApiClientConfig, RequestConfig } from 'api-client';
```

## Dependencies Usage

### From Utils Package
- **Validation** - Email, URL, and custom validation using `validators`
- **Retry Logic** - Network request retries using `retry()`
- **ID Generation** - Unique client IDs using `generateId()`
- **Error Types** - Consistent error handling with `ValidationError`
- **API Types** - Response structures using `ApiResponse`

### From Data Package  
- **Data Types** - User, Post, and other entity types
- **Schema Validation** - Zod schemas for request/response validation
- **Query Options** - Pagination, sorting, and filtering types

## Core Features

### 1. Configured API Client
Pre-configured client with authentication, retries, and validation.

```typescript
import { ApiClient } from 'api-client';

const client = new ApiClient({
  baseURL: 'https://api.example.com',
  timeout: 10000,
  authConfig: {
    type: 'bearer',
    token: process.env.API_TOKEN
  },
  retryConfig: {
    retries: 3,
    retryDelay: 1000
  }
});
```

**Utils Integration:**
- Uses `generateId()` for unique client identification
- Uses `retry()` for robust request handling
- Uses `ValidationError` for consistent error reporting

### 2. Type-Safe Operations
Fully typed operations using data package types.

```typescript
import { User, Post, QueryOptions } from 'data';

// Get users with type safety and validation
const users = await client.getUsers({
  page: 1,
  limit: 20,
  sort: [{ field: 'createdAt', direction: 'desc' }],
  filters: [{ field: 'isActive', operator: 'eq', value: true }]
});

// Create user with validation
const newUser = await client.createUser({
  name: 'John Doe',
  email: 'john@example.com'
});
```

**Data Integration:**
- Uses `User`, `Post` types for type safety
- Uses `QueryOptions` for consistent querying
- Uses `UserSchema`, `PostSchema` for validation

### 3. Built-in Validation
Automatic validation using both utils and data package validators.

```typescript
// Email validation from utils package
const client = new ApiClient(config);

// Automatically validates email format before API call
await client.createUser({
  name: 'John Doe',
  email: 'invalid-email' // Throws ValidationError before making request
});
```

**Validation Chain:**
1. Utils package validators (email, required fields, length)
2. Data package Zod schemas (comprehensive validation)
3. Combined error reporting in consistent format

### 4. Advanced Error Handling
Sophisticated error handling with automatic retries and token refresh.

```typescript
try {
  const data = await client.getUsers();
} catch (error) {
  if (error.validation) {
    // Handle validation errors from utils package
    error.validation.forEach(err => {
      console.log(`${err.field}: ${err.message}`);
    });
  } else if (error.status === 401) {
    // Automatic token refresh already attempted
    console.log('Authentication failed');
  }
}
```

## API Operations

### User Operations
```typescript
// Get all users with pagination
const users = await client.getUsers({
  page: 1,
  limit: 50,
  sort: [{ field: 'name', direction: 'asc' }]
});

// Get specific user
const user = await client.getUser('user-id-123');

// Create user with validation
const newUser = await client.createUser({
  name: 'Jane Doe',
  email: 'jane@example.com',
  isActive: true
});

// Update user
const updatedUser = await client.updateUser('user-id-123', {
  name: 'Jane Smith'
});

// Delete user
await client.deleteUser('user-id-123');
```

### Post Operations
```typescript
// Get posts with filtering
const posts = await client.getPosts({
  filters: [
    { field: 'published', operator: 'eq', value: true },
    { field: 'authorId', operator: 'eq', value: 'author-123' }
  ]
});

// Create post with validation
const newPost = await client.createPost({
  title: 'My Blog Post',
  content: 'This is the content...',
  authorId: 'author-123',
  tags: ['tech', 'programming']
});

// Publish post
const publishedPost = await client.publishPost('post-id-123');
```

## Authentication & Security

### Bearer Token Authentication
```typescript
const client = new ApiClient({
  baseURL: 'https://api.example.com',
  authConfig: {
    type: 'bearer',
    token: 'your-jwt-token',
    refreshToken: 'refresh-token',
    tokenEndpoint: '/auth/refresh'
  }
});

// Automatic token refresh on 401 responses
```

### API Key Authentication  
```typescript
const client = new ApiClient({
  baseURL: 'https://api.example.com',
  authConfig: {
    type: 'api-key',
    apiKey: 'your-api-key',
    apiKeyHeader: 'X-API-Key'
  }
});
```

## Request/Response Interceptors

### Request Interceptors
```typescript
client.addRequestInterceptor({
  onFulfilled: (config) => {
    // Add timestamp to all requests
    config.headers['X-Request-Time'] = new Date().toISOString();
    return config;
  },
  onRejected: (error) => {
    console.error('Request failed:', error);
    return Promise.reject(error);
  }
});
```

### Response Interceptors
```typescript
client.addResponseInterceptor({
  onFulfilled: (response) => {
    // Log successful responses
    console.log(`API call successful: ${response.config.url}`);
    return response;
  },
  onRejected: async (error) => {
    if (error.response?.status === 401) {
      // Automatic token refresh
      const refreshed = await client.refreshToken();
      if (refreshed) {
        return client.request(error.config);
      }
    }
    return Promise.reject(error);
  }
});
```

## Monitoring & Metrics

### Client Metrics
```typescript
// Get client performance metrics
const metrics = client.getMetrics();
console.log(`Requests made: ${metrics.requestCount}`);
console.log(`Error rate: ${metrics.errorCount / metrics.requestCount * 100}%`);
console.log(`Average response time: ${metrics.averageResponseTime}ms`);

// Rate limit information
if (metrics.rateLimitInfo) {
  console.log(`Rate limit: ${metrics.rateLimitInfo.remaining}/${metrics.rateLimitInfo.limit}`);
}
```

### Health Checks
```typescript
// Check API health
const isHealthy = await client.healthCheck();
if (!isHealthy) {
  console.warn('API is not responding');
}
```

## Testing

### Mocking for Tests
```typescript
// Mock utils and data packages
jest.mock('utils', () => ({
  generateId: jest.fn(() => 'test-id'),
  retry: jest.fn((fn) => fn()),
  validators: { validateEmail: jest.fn(() => null) }
}));

jest.mock('data', () => ({
  UserSchema: { parse: jest.fn() },
  PostSchema: { parse: jest.fn() }
}));
```

### Integration Testing
```typescript
// Use MSW for API mocking
import { rest } from 'msw';
import { setupServer } from 'msw/node';

const server = setupServer(
  rest.get('/api/users', (req, res, ctx) => {
    return res(ctx.json({ data: [], success: true }));
  })
);
```

## Best Practices

### ✅ Do
- Use data package types for all API operations
- Leverage utils validators before making requests
- Handle validation errors consistently
- Use retry logic for network resilience
- Monitor client metrics for performance

### ❌ Don't
- Duplicate validation logic that exists in utils/data packages
- Ignore authentication errors (401/403)
- Make requests without proper error handling
- Skip validation for performance (validate early)

## Configuration Examples

### Development Configuration
```typescript
const devClient = new ApiClient({
  baseURL: 'http://localhost:3000/api',
  timeout: 5000,
  retryConfig: { retries: 1, retryDelay: 500 }
});
```

### Production Configuration
```typescript
const prodClient = new ApiClient({
  baseURL: 'https://api.production.com',
  timeout: 10000,
  retryConfig: { 
    retries: 3, 
    retryDelay: 2000,
    shouldResetTimeout: true
  },
  authConfig: {
    type: 'bearer',
    token: process.env.API_TOKEN,
    refreshToken: process.env.REFRESH_TOKEN
  }
});
```

This package demonstrates how to build sophisticated functionality by composing multiple internal packages, creating a powerful API client that leverages shared validation, typing, and utility functions.