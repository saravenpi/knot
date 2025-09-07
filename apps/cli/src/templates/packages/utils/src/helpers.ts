/**
 * General helper utilities
 */
import { debounce, throttle, cloneDeep } from 'lodash';

export function generateId(prefix = ''): string {
  const timestamp = Date.now().toString(36);
  const random = Math.random().toString(36).substr(2, 5);
  return `${prefix}${prefix ? '_' : ''}${timestamp}_${random}`;
}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export function retry<T>(
  fn: () => Promise<T>,
  maxAttempts = 3,
  delay = 1000
): Promise<T> {
  return new Promise((resolve, reject) => {
    let attempts = 0;
    
    const attempt = async () => {
      try {
        attempts++;
        const result = await fn();
        resolve(result);
      } catch (error) {
        if (attempts >= maxAttempts) {
          reject(error);
        } else {
          setTimeout(attempt, delay * attempts);
        }
      }
    };
    
    attempt();
  });
}

export function chunk<T>(array: T[], size: number): T[][] {
  const chunks: T[][] = [];
  for (let i = 0; i < array.length; i += size) {
    chunks.push(array.slice(i, i + size));
  }
  return chunks;
}

export function unique<T>(array: T[]): T[] {
  return [...new Set(array)];
}

export function omit<T extends Record<string, any>, K extends keyof T>(
  obj: T,
  keys: K[]
): Omit<T, K> {
  const result = { ...obj };
  keys.forEach(key => delete result[key]);
  return result;
}

export function pick<T extends Record<string, any>, K extends keyof T>(
  obj: T,
  keys: K[]
): Pick<T, K> {
  const result = {} as Pick<T, K>;
  keys.forEach(key => {
    if (key in obj) {
      result[key] = obj[key];
    }
  });
  return result;
}

export function deepMerge<T extends Record<string, any>>(target: T, source: Partial<T>): T {
  const result = cloneDeep(target);
  
  for (const key in source) {
    if (source.hasOwnProperty(key)) {
      if (typeof source[key] === 'object' && source[key] !== null && !Array.isArray(source[key])) {
        result[key] = deepMerge(result[key] || {}, source[key]);
      } else {
        result[key] = source[key] as any;
      }
    }
  }
  
  return result;
}

const helpers = {
  generateId,
  sleep,
  retry,
  chunk,
  unique,
  omit,
  pick,
  deepMerge,
  debounce,
  throttle,
  cloneDeep,
};

export default helpers;