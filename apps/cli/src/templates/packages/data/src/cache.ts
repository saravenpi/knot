/**
 * Cache implementation using Redis and utils package helpers
 */
import Redis from 'ioredis';
// Import from utils package
import { generateId, retry, helpers } from 'utils';

export interface CacheConfig {
  provider: 'redis' | 'memory';
  url?: string;
  ttl?: number;
  maxSize?: number;
  keyPrefix?: string;
}

export type CacheProvider = 'redis' | 'memory';

class Cache {
  private redis?: Redis;
  private memoryCache: Map<string, { value: unknown; expiresAt: number }> = new Map();
  private config: CacheConfig;
  private cacheId: string;

  constructor(config: CacheConfig) {
    this.config = {
      ttl: 3600, // 1 hour default
      maxSize: 1000, // max items for memory cache
      keyPrefix: 'knot:',
      ...config,
    };
    
    // Use utils package to generate unique cache ID
    this.cacheId = generateId('cache');
  }

  async connect(): Promise<void> {
    if (this.config.provider === 'redis') {
      if (!this.config.url) {
        throw new Error('Redis URL is required for Redis cache provider');
      }

      // Use utils package retry helper for robust connection
      await retry(async () => {
        this.redis = new Redis(this.config.url!, {
          retryDelayOnFailover: 100,
          enableReadyCheck: true,
          maxRetriesPerRequest: 3,
        });

        await this.redis.ping();
      }, 3, 1000);

      console.log(`Redis cache connected (${this.cacheId})`);
    } else {
      console.log(`Memory cache initialized (${this.cacheId})`);
    }
  }

  async disconnect(): Promise<void> {
    if (this.redis) {
      await this.redis.quit();
      console.log(`Redis cache disconnected (${this.cacheId})`);
    } else {
      this.memoryCache.clear();
      console.log(`Memory cache cleared (${this.cacheId})`);
    }
  }

  private getKey(key: string): string {
    return `${this.config.keyPrefix}${key}`;
  }

  async get<T = unknown>(key: string): Promise<T | null> {
    const fullKey = this.getKey(key);

    if (this.redis) {
      const value = await this.redis.get(fullKey);
      if (value) {
        try {
          return JSON.parse(value);
        } catch {
          return value as unknown as T;
        }
      }
      return null;
    } else {
      // Memory cache implementation
      const item = this.memoryCache.get(fullKey);
      if (item) {
        if (Date.now() > item.expiresAt) {
          this.memoryCache.delete(fullKey);
          return null;
        }
        // Use utils package cloneDeep to avoid reference issues
        return helpers.cloneDeep(item.value);
      }
      return null;
    }
  }

  async set<T = unknown>(key: string, value: T, ttl?: number): Promise<void> {
    const fullKey = this.getKey(key);
    const effectiveTtl = ttl || this.config.ttl!;

    if (this.redis) {
      const serializedValue = typeof value === 'string' ? value : JSON.stringify(value);
      await this.redis.setex(fullKey, effectiveTtl, serializedValue);
    } else {
      // Memory cache implementation
      if (this.memoryCache.size >= this.config.maxSize!) {
        // Remove oldest entries (simple LRU)
        const firstKey = this.memoryCache.keys().next().value;
        if (firstKey) {
          this.memoryCache.delete(firstKey);
        }
      }

      this.memoryCache.set(fullKey, {
        value: helpers.cloneDeep(value),
        expiresAt: Date.now() + (effectiveTtl * 1000),
      });
    }
  }

  async del(key: string): Promise<void> {
    const fullKey = this.getKey(key);

    if (this.redis) {
      await this.redis.del(fullKey);
    } else {
      this.memoryCache.delete(fullKey);
    }
  }

  async exists(key: string): Promise<boolean> {
    const fullKey = this.getKey(key);

    if (this.redis) {
      const result = await this.redis.exists(fullKey);
      return result === 1;
    } else {
      const item = this.memoryCache.get(fullKey);
      if (item) {
        if (Date.now() > item.expiresAt) {
          this.memoryCache.delete(fullKey);
          return false;
        }
        return true;
      }
      return false;
    }
  }

  async keys(pattern: string): Promise<string[]> {
    const fullPattern = this.getKey(pattern);

    if (this.redis) {
      const keys = await this.redis.keys(fullPattern);
      return keys.map(key => key.replace(this.config.keyPrefix!, ''));
    } else {
      const regex = new RegExp(fullPattern.replace(/\*/g, '.*'));
      const matchingKeys: string[] = [];
      
      for (const key of this.memoryCache.keys()) {
        if (regex.test(key)) {
          matchingKeys.push(key.replace(this.config.keyPrefix!, ''));
        }
      }
      
      return matchingKeys;
    }
  }

  async clear(): Promise<void> {
    if (this.redis) {
      const keys = await this.redis.keys(`${this.config.keyPrefix}*`);
      if (keys.length > 0) {
        await this.redis.del(...keys);
      }
    } else {
      this.memoryCache.clear();
    }
  }

  async getStats(): Promise<{ provider: string; size: number; memory?: string }> {
    if (this.redis) {
      const info = await this.redis.info('memory');
      const memory = info.split('\r\n').find(line => line.startsWith('used_memory_human:'))?.split(':')[1];
      const dbsize = await this.redis.dbsize();
      
      return {
        provider: 'redis',
        size: dbsize,
        memory,
      };
    } else {
      return {
        provider: 'memory',
        size: this.memoryCache.size,
      };
    }
  }
}

export default Cache;