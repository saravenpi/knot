import { uuid } from 'utils'; // Using alias system
import type { CacheOptions } from './types';

export class MemoryCache<T = any> {
  private cache = new Map<string, { value: T; expires: number; size: number }>();
  private keyPrefix: string;
  private defaultTtl: number;
  private maxSize: number;
  private currentSize = 0;

  constructor(options: CacheOptions = {}) {
    this.keyPrefix = options.keyPrefix || 'cache_';
    this.defaultTtl = options.ttl || 300000; // 5 minutes
    this.maxSize = options.maxSize || 100 * 1024 * 1024; // 100MB
  }

  private getKey(key: string): string {
    return `${this.keyPrefix}${key}`;
  }

  private getSize(value: T): number {
    // Simple size estimation
    return JSON.stringify(value).length * 2; // Rough estimate for UTF-16
  }

  private cleanup(): void {
    const now = Date.now();
    const expiredKeys: string[] = [];

    for (const [key, entry] of this.cache.entries()) {
      if (entry.expires < now) {
        expiredKeys.push(key);
      }
    }

    for (const key of expiredKeys) {
      this.delete(key.replace(this.keyPrefix, ''));
    }
  }

  private evictLRU(): void {
    // Simple LRU eviction - remove oldest entries
    const entries = Array.from(this.cache.entries());
    entries.sort((a, b) => a[1].expires - b[1].expires);
    
    const toRemove = Math.ceil(entries.length * 0.2); // Remove 20% of entries
    for (let i = 0; i < toRemove && this.currentSize > this.maxSize * 0.8; i++) {
      const [key] = entries[i];
      this.delete(key.replace(this.keyPrefix, ''));
    }
  }

  set(key: string, value: T, ttl?: number): void {
    const fullKey = this.getKey(key);
    const size = this.getSize(value);
    const expires = Date.now() + (ttl || this.defaultTtl);

    // Remove existing entry if it exists
    if (this.cache.has(fullKey)) {
      this.currentSize -= this.cache.get(fullKey)!.size;
    }

    // Check if we need to make space
    if (this.currentSize + size > this.maxSize) {
      this.cleanup();
      if (this.currentSize + size > this.maxSize) {
        this.evictLRU();
      }
    }

    this.cache.set(fullKey, { value, expires, size });
    this.currentSize += size;
  }

  get(key: string): T | null {
    const fullKey = this.getKey(key);
    const entry = this.cache.get(fullKey);

    if (!entry) {
      return null;
    }

    if (entry.expires < Date.now()) {
      this.delete(key);
      return null;
    }

    return entry.value;
  }

  delete(key: string): boolean {
    const fullKey = this.getKey(key);
    const entry = this.cache.get(fullKey);
    
    if (entry) {
      this.currentSize -= entry.size;
      return this.cache.delete(fullKey);
    }
    
    return false;
  }

  clear(): void {
    this.cache.clear();
    this.currentSize = 0;
  }

  has(key: string): boolean {
    const fullKey = this.getKey(key);
    const entry = this.cache.get(fullKey);
    
    if (!entry) {
      return false;
    }

    if (entry.expires < Date.now()) {
      this.delete(key);
      return false;
    }

    return true;
  }

  size(): number {
    this.cleanup();
    return this.cache.size;
  }

  getStats(): { size: number; currentSize: number; maxSize: number; hitRatio: number } {
    this.cleanup();
    return {
      size: this.cache.size,
      currentSize: this.currentSize,
      maxSize: this.maxSize,
      hitRatio: 0, // Would need to track hits/misses for real implementation
    };
  }
}

// Global cache instance
export const globalCache = new MemoryCache({
  ttl: 600000, // 10 minutes
  maxSize: 50 * 1024 * 1024, // 50MB
  keyPrefix: 'global_',
});

// Cache key generator using utils
export const generateCacheKey = (prefix: string, ...parts: (string | number)[]): string => {
  return `${prefix}_${parts.join('_')}_${uuid().slice(0, 8)}`;
};