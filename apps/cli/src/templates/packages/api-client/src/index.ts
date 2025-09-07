/**
 * API Client package demonstrating complex inter-package dependencies
 * Depends on both utils and data packages
 */
export { default as ApiClient } from './client';
export { default as HttpClient } from './http-client';
export { default as AuthManager } from './auth-manager';
export { default as RequestBuilder } from './request-builder';
export { default as ResponseParser } from './response-parser';

// Export types
export type { 
  ApiClientConfig, 
  RequestConfig, 
  AuthConfig,
  RetryConfig 
} from './types';

export type { 
  HttpMethod, 
  RequestOptions, 
  ResponseType 
} from './http-client';

// Re-export commonly used types from dependencies for convenience
export type { 
  ApiResponse, 
  PaginatedResponse, 
  ValidationError 
} from 'utils';

export type { 
  User, 
  Post, 
  QueryOptions 
} from 'data';