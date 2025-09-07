// Common data types used throughout the application

export interface User {
  id: string;
  email: string;
  name: string;
  avatar?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface Project {
  id: string;
  name: string;
  description?: string;
  ownerId: string;
  teamId?: string;
  status: 'active' | 'archived' | 'draft';
  createdAt: Date;
  updatedAt: Date;
}

export interface Team {
  id: string;
  name: string;
  description?: string;
  members: TeamMember[];
  createdAt: Date;
  updatedAt: Date;
}

export interface TeamMember {
  userId: string;
  role: 'owner' | 'admin' | 'member' | 'viewer';
  joinedAt: Date;
}

export interface ApiResponse<T = any> {
  data: T;
  success: boolean;
  message?: string;
  errors?: string[];
}

export interface PaginatedResponse<T = any> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    pages: number;
  };
}

export interface CacheOptions {
  ttl?: number;
  maxSize?: number;
  keyPrefix?: string;
}

export interface ValidationRule<T = any> {
  field: keyof T;
  required?: boolean;
  type?: 'string' | 'number' | 'boolean' | 'email' | 'url';
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  custom?: (value: any) => boolean | string;
}

export interface ValidationResult {
  valid: boolean;
  errors: Record<string, string>;
}