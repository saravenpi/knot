/**
 * Data package demonstrating heavy external dependencies and utils package usage
 */
export { default as Database } from './database';
export { default as Repository } from './repository';
export { default as Cache } from './cache';
export { default as Schema } from './schema';
export { default as Migration } from './migration';

// Export types
export type { DatabaseConfig, ConnectionOptions } from './database';
export type { CacheConfig, CacheProvider } from './cache';
export type { RepositoryOptions, QueryOptions } from './repository';
export * from './types';