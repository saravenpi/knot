import { OpenAPIHono, createRoute } from '@hono/zod-openapi';
import { swaggerUI } from '@hono/swagger-ui';
import { CreateUserSchema, LoginSchema, CreateTeamSchema, AddTeamMemberSchema, PublishPackageSchema } from '@/types';
import { z } from 'zod';

// Common response schemas
const SuccessResponseSchema = z.object({
  success: z.boolean(),
  data: z.any().optional(),
  message: z.string().optional(),
});

const ErrorResponseSchema = z.object({
  success: z.boolean(),
  error: z.string(),
  details: z.string().optional(),
});

const UserProfileSchema = z.object({
  id: z.string(),
  username: z.string(),
  email: z.string(),
  createdAt: z.string(),
});

const TeamSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().nullable(),
  ownerId: z.string(),
  createdAt: z.string(),
  updatedAt: z.string(),
});

const PackageSchema = z.object({
  id: z.string(),
  name: z.string(),
  version: z.string(),
  description: z.string().nullable(),
  ownerId: z.string(),
  teamId: z.string().nullable(),
  downloadUrl: z.string(),
  filePath: z.string(),
  fileSize: z.string(),
  checksumSha256: z.string(),
  downloadsCount: z.string(),
  publishedAt: z.string(),
  updatedAt: z.string(),
});

// Create OpenAPI app
export const swagger = new OpenAPIHono();

// Auth routes
export const registerRoute = createRoute({
  method: 'post',
  path: '/api/auth/register',
  request: {
    body: {
      content: {
        'application/json': {
          schema: CreateUserSchema,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        'application/json': {
          schema: SuccessResponseSchema,
        },
      },
      description: 'User registered successfully',
    },
    400: {
      content: {
        'application/json': {
          schema: ErrorResponseSchema,
        },
      },
      description: 'Registration failed',
    },
  },
  tags: ['Authentication'],
});

export const loginRoute = createRoute({
  method: 'post',
  path: '/api/auth/login',
  request: {
    body: {
      content: {
        'application/json': {
          schema: LoginSchema,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        'application/json': {
          schema: SuccessResponseSchema,
        },
      },
      description: 'Login successful',
    },
    401: {
      content: {
        'application/json': {
          schema: ErrorResponseSchema,
        },
      },
      description: 'Login failed',
    },
  },
  tags: ['Authentication'],
});

// Team routes
export const createTeamRoute = createRoute({
  method: 'post',
  path: '/api/teams',
  request: {
    body: {
      content: {
        'application/json': {
          schema: CreateTeamSchema,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        'application/json': {
          schema: SuccessResponseSchema.extend({
            data: TeamSchema.optional(),
          }),
        },
      },
      description: 'Team created successfully',
    },
    400: {
      content: {
        'application/json': {
          schema: ErrorResponseSchema,
        },
      },
      description: 'Failed to create team',
    },
  },
  security: [{ Bearer: [] }],
  tags: ['Teams'],
});

// Package routes
export const publishPackageRoute = createRoute({
  method: 'post',
  path: '/api/packages',
  request: {
    body: {
      content: {
        'application/json': {
          schema: PublishPackageSchema,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        'application/json': {
          schema: SuccessResponseSchema.extend({
            data: PackageSchema.optional(),
          }),
        },
      },
      description: 'Package published successfully',
    },
    400: {
      content: {
        'application/json': {
          schema: ErrorResponseSchema,
        },
      },
      description: 'Failed to publish package',
    },
  },
  security: [{ Bearer: [] }],
  tags: ['Packages'],
});

export const listPackagesRoute = createRoute({
  method: 'get',
  path: '/api/packages',
  request: {
    query: z.object({
      search: z.string().optional(),
      owner: z.string().optional(),
      team: z.string().optional(),
      tags: z.string().optional(),
      limit: z.string().optional(),
      offset: z.string().optional(),
    }),
  },
  responses: {
    200: {
      content: {
        'application/json': {
          schema: SuccessResponseSchema.extend({
            data: z.object({
              packages: z.array(PackageSchema),
              pagination: z.object({
                total: z.number(),
                limit: z.number(),
                offset: z.number(),
                hasMore: z.boolean(),
              }),
            }).optional(),
          }),
        },
      },
      description: 'Packages retrieved successfully',
    },
  },
  tags: ['Packages'],
});

// Setup Swagger UI
swagger.doc('/doc', {
  openapi: '3.0.0',
  info: {
    title: 'Knot Space API',
    version: '0.1.0',
    description: 'Online Package Repository Server API',
  },
  servers: [
    {
      url: process.env.BETTER_AUTH_URL || 'http://localhost:3001',
      description: 'Development server',
    },
  ],
  components: {
    securitySchemes: {
      Bearer: {
        type: 'http',
        scheme: 'bearer',
        bearerFormat: 'JWT',
      },
    },
  },
});

swagger.get('/ui', swaggerUI({ url: '/doc' }));