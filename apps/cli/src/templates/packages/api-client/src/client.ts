/**
 * Main API Client demonstrating complex usage of multiple package dependencies
 */
import axios, { AxiosInstance, AxiosResponse } from 'axios';
import axiosRetry from 'axios-retry';
import qs from 'qs';
// Import from utils package
import { retry, generateId, validators, formatters, ApiResponse, ValidationError } from 'utils';
// Import from data package
import { User, Post, QueryOptions, UserSchema, PostSchema } from 'data';
// Local imports
import { 
  ApiClientConfig, 
  RequestConfig, 
  ApiError, 
  UserApiOperations, 
  PostApiOperations,
  ApiMetrics 
} from './types';
import HttpClient from './http-client';
import AuthManager from './auth-manager';

class ApiClient implements UserApiOperations, PostApiOperations {
  private httpClient: HttpClient;
  private authManager: AuthManager;
  private metrics: ApiMetrics;
  private clientId: string;

  constructor(config: ApiClientConfig) {
    // Use utils package to generate unique client ID
    this.clientId = generateId('api_client');
    
    this.httpClient = new HttpClient(config);
    this.authManager = new AuthManager(config.authConfig);
    
    this.metrics = {
      requestCount: 0,
      errorCount: 0,
      averageResponseTime: 0,
    };

    this.setupInterceptors();
    this.setupRetry();
  }

  private setupInterceptors(): void {
    // Request interceptor for authentication
    this.httpClient.addRequestInterceptor({
      onFulfilled: async (config) => {
        this.metrics.requestCount++;
        
        const authHeaders = await this.authManager.getAuthHeaders();
        if (authHeaders) {
          config.headers = { ...config.headers, ...authHeaders };
        }
        
        return config;
      },
      onRejected: (error) => {
        this.metrics.errorCount++;
        return Promise.reject(error);
      }
    });

    // Response interceptor for metrics and error handling
    this.httpClient.addResponseInterceptor({
      onFulfilled: (response) => {
        this.updateMetrics(response);
        return response;
      },
      onRejected: async (error) => {
        this.metrics.errorCount++;
        
        if (error.response?.status === 401) {
          const refreshed = await this.authManager.refreshToken();
          if (refreshed) {
            // Retry the original request with new token
            return this.httpClient.request(error.config);
          }
        }
        
        return Promise.reject(this.transformError(error));
      }
    });
  }

  private setupRetry(): void {
    axiosRetry(this.httpClient.getAxiosInstance(), {
      retries: 3,
      retryDelay: axiosRetry.exponentialDelay,
      retryCondition: (error) => {
        return axiosRetry.isNetworkOrIdempotentRequestError(error) ||
               error.response?.status === 429; // Rate limit
      }
    });
  }

  private updateMetrics(response: AxiosResponse): void {
    // Use utils package formatters
    const responseTime = response.config.metadata?.endTime - response.config.metadata?.startTime;
    if (responseTime) {
      this.metrics.averageResponseTime = 
        (this.metrics.averageResponseTime + responseTime) / 2;
    }

    // Check rate limit headers
    if (response.headers['x-ratelimit-limit']) {
      this.metrics.rateLimitInfo = {
        limit: parseInt(response.headers['x-ratelimit-limit']),
        remaining: parseInt(response.headers['x-ratelimit-remaining']),
        resetTime: new Date(parseInt(response.headers['x-ratelimit-reset']) * 1000),
      };
    }
  }

  private transformError(error: any): ApiError {
    const apiError: ApiError = new Error(error.message);
    apiError.status = error.response?.status;
    apiError.code = error.code;
    apiError.data = error.response?.data;
    
    // Parse validation errors if present
    if (error.response?.data?.errors) {
      apiError.validation = error.response.data.errors;
    }
    
    return apiError;
  }

  // User operations implementing UserApiOperations
  async getUsers(options?: QueryOptions): Promise<ApiResponse<User[]>> {
    const params = options ? qs.stringify(options) : '';
    const response = await this.httpClient.get(`/users?${params}`);
    return response.data;
  }

  async getUser(id: string): Promise<ApiResponse<User>> {
    // Use utils package validators
    const validationError = validators.validateRequired(id, 'User ID');
    if (validationError) {
      throw new Error(validationError.message);
    }

    const response = await this.httpClient.get(`/users/${id}`);
    return response.data;
  }

  async createUser(data: Partial<User>): Promise<ApiResponse<User>> {
    const validationErrors = this.validateUser(data);
    if (validationErrors.length > 0) {
      const apiError: ApiError = new Error('Validation failed');
      apiError.validation = validationErrors;
      throw apiError;
    }

    const response = await this.httpClient.post('/users', data);
    return response.data;
  }

  async updateUser(id: string, data: Partial<User>): Promise<ApiResponse<User>> {
    const validationErrors = this.validateUser(data);
    if (validationErrors.length > 0) {
      const apiError: ApiError = new Error('Validation failed');
      apiError.validation = validationErrors;
      throw apiError;
    }

    const response = await this.httpClient.put(`/users/${id}`, data);
    return response.data;
  }

  async deleteUser(id: string): Promise<ApiResponse<void>> {
    const response = await this.httpClient.delete(`/users/${id}`);
    return response.data;
  }

  validateUser(data: Partial<User>): ValidationError[] {
    const errors: ValidationError[] = [];

    if (data.email) {
      const emailError = validators.validateEmail(data.email);
      if (emailError) errors.push(emailError);
    }

    if (data.name) {
      const nameError = validators.validateMinLength(data.name, 1, 'Name');
      if (nameError) errors.push(nameError);
    }

    // Use data package schema validation
    try {
      if (Object.keys(data).length > 0) {
        UserSchema.partial().parse(data);
      }
    } catch (error: any) {
      if (error.errors) {
        errors.push(...error.errors.map((err: any) => ({
          field: err.path.join('.'),
          message: err.message,
          value: err.received
        })));
      }
    }

    return errors;
  }

  // Post operations implementing PostApiOperations
  async getPosts(options?: QueryOptions): Promise<ApiResponse<Post[]>> {
    const params = options ? qs.stringify(options) : '';
    const response = await this.httpClient.get(`/posts?${params}`);
    return response.data;
  }

  async getPost(id: string): Promise<ApiResponse<Post>> {
    const response = await this.httpClient.get(`/posts/${id}`);
    return response.data;
  }

  async createPost(data: Partial<Post>): Promise<ApiResponse<Post>> {
    const validationErrors = this.validatePost(data);
    if (validationErrors.length > 0) {
      const apiError: ApiError = new Error('Validation failed');
      apiError.validation = validationErrors;
      throw apiError;
    }

    const response = await this.httpClient.post('/posts', data);
    return response.data;
  }

  async updatePost(id: string, data: Partial<Post>): Promise<ApiResponse<Post>> {
    const response = await this.httpClient.put(`/posts/${id}`, data);
    return response.data;
  }

  async deletePost(id: string): Promise<ApiResponse<void>> {
    const response = await this.httpClient.delete(`/posts/${id}`);
    return response.data;
  }

  async publishPost(id: string): Promise<ApiResponse<Post>> {
    const response = await this.httpClient.post(`/posts/${id}/publish`);
    return response.data;
  }

  validatePost(data: Partial<Post>): ValidationError[] {
    const errors: ValidationError[] = [];

    if (data.title) {
      const titleError = validators.validateMinLength(data.title, 1, 'Title');
      if (titleError) errors.push(titleError);
      
      const maxTitleError = validators.validateMaxLength(data.title, 255, 'Title');
      if (maxTitleError) errors.push(maxTitleError);
    }

    if (data.content) {
      const contentError = validators.validateMinLength(data.content, 1, 'Content');
      if (contentError) errors.push(contentError);
    }

    // Use data package schema validation
    try {
      if (Object.keys(data).length > 0) {
        PostSchema.partial().parse(data);
      }
    } catch (error: any) {
      if (error.errors) {
        errors.push(...error.errors.map((err: any) => ({
          field: err.path.join('.'),
          message: err.message,
          value: err.received
        })));
      }
    }

    return errors;
  }

  // Utility methods
  getMetrics(): ApiMetrics {
    return { ...this.metrics };
  }

  async healthCheck(): Promise<boolean> {
    try {
      await this.httpClient.get('/health');
      return true;
    } catch {
      return false;
    }
  }

  getClientId(): string {
    return this.clientId;
  }
}

export default ApiClient;