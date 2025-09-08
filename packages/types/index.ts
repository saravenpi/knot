// Shared types for Knot applications

// Basic entity interfaces
export interface User {
  id: string;
  username: string;
  email: string;
  createdAt?: string | Date;
}

export interface UserProfile extends User {
  createdAt: Date;
  token?: string;
}

export interface Package {
  id: string;
  name: string;
  version: string;
  description?: string;
  downloadsCount: number | string; // API returns as string due to BigInt serialization
  totalDownloadsCount?: number | string; // Total downloads across all versions
  owner: User;
  team?: { 
    id: string;
    name: string; 
  };
  tags?: string[];
  createdAt: string;
  updatedAt: string;
  publishedAt?: string;
  fileSize?: number | string; // API returns as string due to BigInt serialization
  files?: FileEntry[]; // File structure for browsing
}

export interface FileEntry {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  children?: FileEntry[];
}

export interface FileContent {
  content: string;
  encoding: 'utf-8' | 'base64';
  mimeType: string;
}

export interface Team {
  id: string;
  name: string;
  description?: string;
  createdAt?: string | Date;
  members: Array<{
    user: User;
    role: TeamRole;
  }>;
}

// Enums and literals
export type TeamRole = 'owner' | 'admin' | 'member';

// Request interfaces
export interface LoginCredentials {
  username: string;
  password: string;
}

export interface RegisterData {
  username: string;
  email: string;
  password: string;
}

export interface CreateTeamRequest {
  name: string;
  description?: string;
}

export interface AddTeamMemberRequest {
  username: string;
  role: TeamRole;
}

export interface PublishPackageRequest {
  name: string;
  version: string;
  description?: string;
  teamName?: string;
  tags?: string[];
}

// Response interfaces
export interface ApiResponse<T = unknown> {
  data?: T;
  message?: string;
  token?: string;
  success?: boolean;
}

export interface AuthResponse {
  token: string;
  user: UserProfile;
}

export interface ApiError {
  error: string;
  details?: string;
}

export interface SuccessResponse<T = unknown> {
  success: boolean;
  data?: T;
  message?: string;
}

// Search and pagination
export interface SearchParams {
  q?: string;
  page?: number;
  limit?: number;
  tags?: string[];
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  totalPages: number;
}

// File upload
export interface FileUploadRequest {
  file: File | ArrayBuffer;
  packageName: string;
  version: string;
}

// Package download
export interface PackageDownloadInfo {
  downloadUrl: string;
  checksum: string;
  size: number;
}

// Team member operations
export interface UpdateMemberRoleRequest {
  role: TeamRole;
}

// Package operations
export interface UpdatePackageRequest {
  description?: string;
  tags?: string[];
}

// Statistics
export interface PackageStats {
  totalPackages: number;
  totalDownloads: number;
  totalUsers: number;
  totalTeams: number;
}

export interface UserStats {
  packagesPublished: number;
  totalDownloads: number;
  teamsJoined: number;
}

export interface TeamStats {
  memberCount: number;
  packagesPublished: number;
  totalDownloads: number;
}

// Validation schemas (for reference - actual validation should use zod on backend)
export interface ValidationSchema {
  username: {
    minLength: 3;
    maxLength: 50;
    pattern: string; // /^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/
  };
  email: {
    maxLength: 255;
    pattern: string; // email regex
  };
  password: {
    minLength: 6;
    maxLength: 128;
    pattern: string; // /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/
  };
  teamName: {
    minLength: 3;
    maxLength: 50;
    pattern: string; // /^[a-z0-9][a-z0-9-]*$/
  };
  packageName: {
    minLength: 1;
    maxLength: 100;
    pattern: string; // /^(@[a-z0-9-]+\/)?[a-z0-9][a-z0-9-]*$/
  };
  version: {
    maxLength: 20;
    pattern: string; // semantic versioning regex
  };
  tag: {
    minLength: 1;
    maxLength: 30;
    pattern: string; // /^[a-z0-9][a-z0-9-]*$/
  };
}

// Constants
export const VALIDATION_PATTERNS = {
  USERNAME: /^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/,
  EMAIL: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  PASSWORD: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
  TEAM_NAME: /^[a-z0-9][a-z0-9-]*$/,
  PACKAGE_NAME: /^(@[a-z0-9-]+\/)?[a-z0-9][a-z0-9-]*$/,
  SEMANTIC_VERSION: /^[0-9]+\.[0-9]+\.[0-9]+(-[a-z0-9-]+)?(\+[a-z0-9-]+)?$/,
  TAG: /^[a-z0-9][a-z0-9-]*$/,
} as const;

export const LIMITS = {
  USERNAME_MIN: 3,
  USERNAME_MAX: 50,
  EMAIL_MAX: 255,
  PASSWORD_MIN: 6,
  PASSWORD_MAX: 128,
  TEAM_NAME_MIN: 3,
  TEAM_NAME_MAX: 50,
  PACKAGE_NAME_MAX: 100,
  VERSION_MAX: 20,
  DESCRIPTION_MAX: 1000,
  TAG_MIN: 1,
  TAG_MAX: 30,
  MAX_TAGS: 10,
} as const;

// Type guards
export function isUser(obj: unknown): obj is User {
  return typeof obj === 'object' && obj !== null && 
         'id' in obj && typeof (obj as User).id === 'string' && 
         'username' in obj && typeof (obj as User).username === 'string';
}

export function isPackage(obj: unknown): obj is Package {
  return typeof obj === 'object' && obj !== null && 
         'id' in obj && typeof (obj as Package).id === 'string' && 
         'name' in obj && typeof (obj as Package).name === 'string';
}

export function isTeam(obj: unknown): obj is Team {
  return typeof obj === 'object' && obj !== null && 
         'id' in obj && typeof (obj as Team).id === 'string' && 
         'name' in obj && typeof (obj as Team).name === 'string';
}

export function isApiError(obj: unknown): obj is ApiError {
  return typeof obj === 'object' && obj !== null && 
         'error' in obj && typeof (obj as ApiError).error === 'string';
}

// Utility types
export type CreatePackageData = Omit<Package, 'id' | 'created_at' | 'updated_at' | 'downloads_count' | 'owner'>;
export type UpdatePackageData = Partial<Pick<Package, 'description' | 'tags'>>;
export type CreateTeamData = Omit<Team, 'id' | 'createdAt' | 'members'>;
export type CreateUserData = Omit<User, 'id' | 'createdAt'> & { password: string };