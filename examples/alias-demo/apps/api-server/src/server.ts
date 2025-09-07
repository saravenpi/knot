// API Server using @core alias
import express from 'express';
import cors from 'cors';

// Using @core alias to import from knot_packages
import { isEmail, isEmpty } from '@core/utils/validation';
import { formatDate, addDays } from '@core/utils/dates';
import { uuid, debounce } from '@core/utils';
import { MemoryCache, globalCache } from '@core/data-layer/cache';
import type { User, ApiResponse } from '@core/data-layer';

const app = express();
const port = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Initialize cache for API responses
const apiCache = new MemoryCache({
  ttl: 300000, // 5 minutes
  keyPrefix: 'api_',
});

// Mock users data
const users: User[] = [
  {
    id: '1',
    email: 'admin@example.com',
    name: 'Admin User',
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: '2', 
    email: 'user@example.com',
    name: 'Regular User',
    createdAt: addDays(new Date(), -7),
    updatedAt: new Date(),
  },
];

// Helper function to create API responses
const createResponse = <T>(data: T, message?: string): ApiResponse<T> => ({
  data,
  success: true,
  message,
});

const createErrorResponse = (message: string, errors?: string[]): ApiResponse => ({
  data: null,
  success: false,
  message,
  errors,
});

// Validation middleware using utils from alias
const validateUser = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { name, email } = req.body;
  const errors: string[] = [];

  if (isEmpty(name)) {
    errors.push('Name is required');
  }

  if (isEmpty(email)) {
    errors.push('Email is required');
  } else if (!isEmail(email)) {
    errors.push('Email format is invalid');
  }

  if (errors.length > 0) {
    return res.status(400).json(createErrorResponse('Validation failed', errors));
  }

  next();
};

// Debounced cache cleanup
const cleanupCache = debounce(() => {
  console.log(`🧹 Cleaning up cache... Current size: ${apiCache.size()}`);
  // Cache cleanup happens automatically, but we could add custom logic here
}, 60000); // 1 minute

// Routes

// Health check
app.get('/health', (req, res) => {
  res.json(createResponse({
    status: 'healthy',
    timestamp: formatDate(new Date(), 'YYYY-MM-DD HH:mm:ss'),
    cache: apiCache.getStats(),
  }));
});

// Get all users
app.get('/api/users', (req, res) => {
  const cacheKey = 'users_all';
  const cached = apiCache.get(cacheKey);

  if (cached) {
    console.log('📦 Returning cached users');
    return res.json(cached);
  }

  const response = createResponse(
    users.map(user => ({
      ...user,
      createdAt: formatDate(user.createdAt),
      updatedAt: formatDate(user.updatedAt),
    })),
    'Users retrieved successfully'
  );

  apiCache.set(cacheKey, response);
  cleanupCache();
  
  res.json(response);
});

// Get user by ID
app.get('/api/users/:id', (req, res) => {
  const { id } = req.params;
  const cacheKey = `user_${id}`;
  
  const cached = apiCache.get(cacheKey);
  if (cached) {
    console.log(`📦 Returning cached user ${id}`);
    return res.json(cached);
  }

  const user = users.find(u => u.id === id);
  
  if (!user) {
    return res.status(404).json(
      createErrorResponse('User not found')
    );
  }

  const response = createResponse({
    ...user,
    createdAt: formatDate(user.createdAt),
    updatedAt: formatDate(user.updatedAt),
  });

  apiCache.set(cacheKey, response);
  cleanupCache();
  
  res.json(response);
});

// Create user
app.post('/api/users', validateUser, (req, res) => {
  const { name, email } = req.body;
  
  // Check if email already exists
  const existingUser = users.find(u => u.email === email);
  if (existingUser) {
    return res.status(409).json(
      createErrorResponse('Email already exists')
    );
  }

  const newUser: User = {
    id: uuid(),
    name,
    email,
    createdAt: new Date(),
    updatedAt: new Date(),
  };

  users.push(newUser);
  
  // Invalidate users cache
  apiCache.delete('users_all');
  
  const response = createResponse({
    ...newUser,
    createdAt: formatDate(newUser.createdAt),
    updatedAt: formatDate(newUser.updatedAt),
  }, 'User created successfully');

  res.status(201).json(response);
});

// Update user
app.put('/api/users/:id', validateUser, (req, res) => {
  const { id } = req.params;
  const { name, email } = req.body;
  
  const userIndex = users.findIndex(u => u.id === id);
  if (userIndex === -1) {
    return res.status(404).json(
      createErrorResponse('User not found')
    );
  }

  // Check if email already exists for different user
  const existingUser = users.find(u => u.email === email && u.id !== id);
  if (existingUser) {
    return res.status(409).json(
      createErrorResponse('Email already exists')
    );
  }

  // Update user
  users[userIndex] = {
    ...users[userIndex],
    name,
    email,
    updatedAt: new Date(),
  };

  // Invalidate cache
  apiCache.delete('users_all');
  apiCache.delete(`user_${id}`);

  const response = createResponse({
    ...users[userIndex],
    createdAt: formatDate(users[userIndex].createdAt),
    updatedAt: formatDate(users[userIndex].updatedAt),
  }, 'User updated successfully');

  res.json(response);
});

// Delete user
app.delete('/api/users/:id', (req, res) => {
  const { id } = req.params;
  
  const userIndex = users.findIndex(u => u.id === id);
  if (userIndex === -1) {
    return res.status(404).json(
      createErrorResponse('User not found')
    );
  }

  users.splice(userIndex, 1);
  
  // Invalidate cache
  apiCache.delete('users_all');
  apiCache.delete(`user_${id}`);

  res.json(createResponse(null, 'User deleted successfully'));
});

// Start server
app.listen(port, () => {
  console.log(`🚀 API Server running on port ${port}`);
  console.log(`📝 Using @core alias for package imports`);
  console.log(`🗓  Server started at ${formatDate(new Date(), 'YYYY-MM-DD HH:mm:ss')}`);
  console.log(`💾 Cache configured with ${apiCache.getStats().maxSize / (1024 * 1024)}MB limit`);
});