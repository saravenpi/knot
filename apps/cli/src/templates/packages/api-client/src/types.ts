/**
 * API Client types that extend and compose types from multiple packages
 */
// Import types from both utils and data packages
import { ValidationError, ApiResponse } from 'utils';
import { User, Post, QueryOptions } from 'data';

export interface ApiClientConfig {
  baseURL: string;
  timeout?: number;
  retryConfig?: RetryConfig;
  authConfig?: AuthConfig;
  defaultHeaders?: Record<string, string>;
  interceptors?: {
    request?: RequestInterceptor[];
    response?: ResponseInterceptor[];
  };
}

export interface RequestConfig {
  url: string;
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  data?: unknown;
  params?: Record<string, unknown>;
  headers?: Record<string, string>;
  timeout?: number;
  validateStatus?: (status: number) => boolean;
}

export interface AuthConfig {
  type: 'bearer' | 'basic' | 'api-key' | 'oauth2';
  token?: string;
  username?: string;
  password?: string;
  apiKey?: string;
  apiKeyHeader?: string;
  refreshToken?: string;
  tokenEndpoint?: string;
  clientId?: string;
  clientSecret?: string;
}

export interface RetryConfig {
  retries: number;
  retryDelay: number;
  retryCondition?: (error: unknown) => boolean;
  shouldResetTimeout?: boolean;
}

export interface RequestInterceptor {
  onFulfilled?: (config: RequestConfig) => RequestConfig | Promise<RequestConfig>;
  onRejected?: (error: unknown) => unknown;
}

export interface ResponseInterceptor {
  onFulfilled?: (response: unknown) => unknown;
  onRejected?: (error: unknown) => unknown;
}

// Extending data package types for API operations
export interface UserApiOperations {
  getUsers(options?: QueryOptions): Promise<ApiResponse<User[]>>;
  getUser(id: string): Promise<ApiResponse<User>>;
  createUser(data: Partial<User>): Promise<ApiResponse<User>>;
  updateUser(id: string, data: Partial<User>): Promise<ApiResponse<User>>;
  deleteUser(id: string): Promise<ApiResponse<void>>;
  validateUser(data: Partial<User>): ValidationError[];
}

export interface PostApiOperations {
  getPosts(options?: QueryOptions): Promise<ApiResponse<Post[]>>;
  getPost(id: string): Promise<ApiResponse<Post>>;
  createPost(data: Partial<Post>): Promise<ApiResponse<Post>>;
  updatePost(id: string, data: Partial<Post>): Promise<ApiResponse<Post>>;
  deletePost(id: string): Promise<ApiResponse<void>>;
  publishPost(id: string): Promise<ApiResponse<Post>>;
  validatePost(data: Partial<Post>): ValidationError[];
}

export interface ApiError extends Error {
  status?: number;
  code?: string;
  data?: unknown;
  validation?: ValidationError[];
}

export interface RateLimitInfo {
  limit: number;
  remaining: number;
  resetTime: Date;
}

export interface ApiMetrics {
  requestCount: number;
  errorCount: number;
  averageResponseTime: number;
  rateLimitInfo?: RateLimitInfo;
}