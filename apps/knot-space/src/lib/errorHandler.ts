import { Context } from 'hono';
import { logger } from './logger';

export interface AppError extends Error {
  statusCode: number;
  code?: string;
  isOperational?: boolean;
}

export class ValidationError extends Error implements AppError {
  statusCode = 400;
  code = 'VALIDATION_ERROR';
  isOperational = true;

  constructor(message: string) {
    super(message);
    this.name = 'ValidationError';
  }
}

export class AuthenticationError extends Error implements AppError {
  statusCode = 401;
  code = 'AUTHENTICATION_ERROR';
  isOperational = true;

  constructor(message: string = 'Authentication required') {
    super(message);
    this.name = 'AuthenticationError';
  }
}

export class AuthorizationError extends Error implements AppError {
  statusCode = 403;
  code = 'AUTHORIZATION_ERROR';
  isOperational = true;

  constructor(message: string = 'Insufficient permissions') {
    super(message);
    this.name = 'AuthorizationError';
  }
}

export class NotFoundError extends Error implements AppError {
  statusCode = 404;
  code = 'NOT_FOUND';
  isOperational = true;

  constructor(message: string = 'Resource not found') {
    super(message);
    this.name = 'NotFoundError';
  }
}

export class ConflictError extends Error implements AppError {
  statusCode = 409;
  code = 'CONFLICT';
  isOperational = true;

  constructor(message: string) {
    super(message);
    this.name = 'ConflictError';
  }
}

export class RateLimitError extends Error implements AppError {
  statusCode = 429;
  code = 'RATE_LIMIT_EXCEEDED';
  isOperational = true;

  constructor(message: string = 'Too many requests') {
    super(message);
    this.name = 'RateLimitError';
  }
}

export function errorHandler(error: Error, c: Context) {
  const requestId = c.req.header('x-request-id') || 'unknown';
  
  // Log error details
  logger.error('Request error', {
    requestId,
    error: error.message,
    stack: error.stack,
    url: c.req.url,
    method: c.req.method,
    userAgent: c.req.header('user-agent'),
    ip: c.req.header('x-forwarded-for') || c.req.header('x-real-ip'),
  });

  // Handle known application errors
  if (isAppError(error)) {
    return c.json({
      success: false,
      error: error.message,
      code: error.code,
      requestId,
    }, error.statusCode);
  }

  // Handle Prisma errors
  if (error.name === 'PrismaClientKnownRequestError') {
    const prismaError = error as any;
    if (prismaError.code === 'P2002') {
      return c.json({
        success: false,
        error: 'Resource already exists',
        code: 'DUPLICATE_ERROR',
        requestId,
      }, 409);
    }
  }

  // Handle validation errors from Zod
  if (error.name === 'ZodError') {
    return c.json({
      success: false,
      error: 'Validation failed',
      code: 'VALIDATION_ERROR',
      details: (error as any).errors,
      requestId,
    }, 400);
  }

  // Handle unexpected errors
  const isDevelopment = process.env.NODE_ENV === 'development';
  
  return c.json({
    success: false,
    error: isDevelopment ? error.message : 'Internal server error',
    code: 'INTERNAL_ERROR',
    requestId,
    ...(isDevelopment && { stack: error.stack }),
  }, 500);
}

function isAppError(error: Error): error is AppError {
  return 'statusCode' in error && 'isOperational' in error;
}