/**
 * Data-specific types extending utils package types
 */
import { z } from 'zod';
// Import and extend types from utils package
import { BaseEntity, PaginationMeta, SortConfig, FilterConfig } from 'utils';

export interface User extends BaseEntity {
  email: string;
  name: string;
  avatar?: string;
  isActive: boolean;
  lastLoginAt?: Date;
}

export interface Post extends BaseEntity {
  title: string;
  content: string;
  authorId: string;
  published: boolean;
  publishedAt?: Date;
  tags: string[];
  metadata: Record<string, any>;
}

export interface DatabaseEntity extends BaseEntity {
  version: number;
  isDeleted: boolean;
  deletedAt?: Date;
}

export interface QueryResult<T = any> {
  data: T[];
  meta: PaginationMeta;
  total: number;
}

export interface Transaction {
  id: string;
  status: 'pending' | 'completed' | 'failed' | 'rolled_back';
  operations: TransactionOperation[];
  createdAt: Date;
  completedAt?: Date;
}

export interface TransactionOperation {
  table: string;
  operation: 'create' | 'update' | 'delete';
  data: any;
  where?: any;
}

// Zod schemas for validation
export const UserSchema = z.object({
  id: z.string(),
  email: z.string().email(),
  name: z.string().min(1).max(255),
  avatar: z.string().url().optional(),
  isActive: z.boolean().default(true),
  createdAt: z.date(),
  updatedAt: z.date(),
  lastLoginAt: z.date().optional(),
});

export const PostSchema = z.object({
  id: z.string(),
  title: z.string().min(1).max(255),
  content: z.string(),
  authorId: z.string(),
  published: z.boolean().default(false),
  publishedAt: z.date().optional(),
  tags: z.array(z.string()),
  metadata: z.record(z.any()),
  createdAt: z.date(),
  updatedAt: z.date(),
});

export const QueryOptionsSchema = z.object({
  page: z.number().min(1).default(1),
  limit: z.number().min(1).max(100).default(20),
  sort: z.array(z.object({
    field: z.string(),
    direction: z.enum(['asc', 'desc']),
  })).optional(),
  filters: z.array(z.object({
    field: z.string(),
    operator: z.enum(['eq', 'ne', 'gt', 'lt', 'gte', 'lte', 'in', 'nin', 'like']),
    value: z.any(),
  })).optional(),
});

export type UserInput = z.infer<typeof UserSchema>;
export type PostInput = z.infer<typeof PostSchema>;
export type QueryOptions = z.infer<typeof QueryOptionsSchema>;