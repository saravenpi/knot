import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { prettyJSON } from 'hono/pretty-json';
import { loadModules } from './lib/loadModules';
import { securityHeaders, requestLogger, rateLimit } from './lib/security';
import { errorHandler } from './lib/errorHandler';
import { env } from './lib/env';

export async function createApp(): Promise<Hono> {
  const { logger } = await import('./lib/logger');
  const app = new Hono();

  // Global middleware - order matters!
  app.use('*', securityHeaders());
  app.use('*', requestLogger());

  // Rate limiting
  app.use('*', rateLimit(env.RATE_LIMIT_WINDOW_MS, env.RATE_LIMIT_MAX_REQUESTS));

  // CORS - Debug logging
  console.log('🔧 CORS Origins configured:', env.CORS_ORIGINS);

  app.use(
    '*',
    cors({
      origin: (origin) => {
        console.log('🌐 CORS check - Origin:', origin, 'Allowed:', env.CORS_ORIGINS);

        // Allow requests with no origin (e.g., mobile apps, Postman)
        if (!origin) return '*';

        // Check if origin is in allowed list
        const isAllowed = env.CORS_ORIGINS.includes('*') || env.CORS_ORIGINS.includes(origin);
        console.log('✅ CORS decision:', isAllowed ? 'ALLOWED' : 'BLOCKED');

        return isAllowed ? origin : null;
      },
      allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS', 'PATCH'],
      allowHeaders: ['Content-Type', 'Authorization', 'X-Request-ID'],
      credentials: true,
      maxAge: 86400, // 24 hours
    })
  );

  // Pretty JSON only in development
  if (env.NODE_ENV === 'development') {
    app.use('*', prettyJSON());
  }

  // Global error handler
  app.onError(errorHandler);

  // Load and mount all modules
  try {
    const modules = await loadModules();

    if (modules.length === 0) {
      throw new Error('No modules loaded - application cannot start');
    }

    for (const module of modules) {
      app.route(module.prefix, module.router);
      logger.info('Module mounted successfully', {
        name: module.name,
        prefix: module.prefix,
      });
    }

    logger.info('All modules loaded successfully', {
      totalModules: modules.length,
      modules: modules.map((m) => ({ name: m.name, prefix: m.prefix })),
    });
  } catch (error) {
    logger.error('Failed to load modules', { error });
    throw error;
  }

  return app;
}

