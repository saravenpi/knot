import { readdirSync } from 'fs';
import { join, resolve, normalize } from 'path';
import { Hono } from 'hono';
import { logger } from './logger';

export interface Module {
  router: Hono;
  name: string;
  prefix: string;
}

// Allowed module names (security: whitelist only expected modules)
const ALLOWED_MODULES = ['auth', 'teams', 'packages', 'users'];

// Security: Validate module name
function isValidModuleName(name: string): boolean {
  // Only allow alphanumeric names, no special characters to prevent path traversal
  return /^[a-z0-9]+$/.test(name) && ALLOWED_MODULES.includes(name);
}

export async function loadModules(): Promise<Module[]> {
  const modulesPath = resolve(join(__dirname, '../modules'));
  const modules: Module[] = [];

  try {
    logger.info('Loading modules from', { path: modulesPath });

    const moduleDirectories = readdirSync(modulesPath, { withFileTypes: true })
      .filter((dirent) => dirent.isDirectory())
      .map((dirent) => dirent.name)
      .filter((name) => isValidModuleName(name)); // Security: filter allowed modules

    logger.info('Found module directories', { modules: moduleDirectories });

    for (const moduleName of moduleDirectories) {
      try {
        // Security: normalize and validate path
        const routerFileName = 'router.ts';
        const modulePath = resolve(join(modulesPath, moduleName, routerFileName));

        // Security: ensure the resolved path is still within modules directory
        if (!modulePath.startsWith(modulesPath)) {
          logger.error('Security violation: Path traversal attempt', { moduleName, modulePath });
          continue;
        }

        logger.debug('Attempting to load module', { moduleName, modulePath });

        const moduleExports = await import(modulePath);

        // Security: validate module exports
        if (!moduleExports.default || typeof moduleExports.default !== 'object') {
          logger.warn('Invalid module export: missing or invalid router', { moduleName });
          continue;
        }

        // Security: validate router is actually a Hono instance
        const router = moduleExports.default;
        if (!router.fetch || typeof router.fetch !== 'function') {
          logger.warn('Invalid module export: router is not a Hono instance', { moduleName });
          continue;
        }

        // Security: validate prefix
        const prefix = moduleExports.prefix;
        if (!prefix || typeof prefix !== 'string' || !prefix.startsWith('/')) {
          logger.warn('Invalid module prefix', { moduleName, prefix });
          continue;
        }

        // Security: sanitize prefix to prevent injection
        const sanitizedPrefix = normalize(prefix).replace(/\.\./g, '');
        if (sanitizedPrefix !== prefix) {
          logger.error('Security violation: Invalid prefix detected', {
            moduleName,
            originalPrefix: prefix,
            sanitizedPrefix,
          });
          continue;
        }

        modules.push({
          router,
          name: moduleName,
          prefix: sanitizedPrefix,
        });

        logger.info('Module loaded successfully', {
          moduleName,
          prefix: sanitizedPrefix,
          routesCount: router.routes?.length || 'unknown',
        });
      } catch (error) {
        logger.error('Failed to load module', {
          moduleName,
          error: error instanceof Error ? error.message : 'Unknown error',
          stack: error instanceof Error ? error.stack : undefined,
        });
      }
    }
  } catch (error) {
    logger.error('Failed to read modules directory', {
      error: error instanceof Error ? error.message : 'Unknown error',
      modulesPath,
    });
    throw error;
  }

  logger.info('Module loading completed', {
    totalModules: modules.length,
    loadedModules: modules.map((m) => ({ name: m.name, prefix: m.prefix })),
  });

  return modules;
}
