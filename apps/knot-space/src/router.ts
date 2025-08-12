import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { prettyJSON } from 'hono/pretty-json';
import { loadModules } from './lib/loadModules';
import { securityHeaders, requestLogger, rateLimit } from './lib/security';
import { errorHandler } from './lib/errorHandler';
import { env } from './lib/env';

const app = new Hono();

// Global middleware - order matters!
app.use('*', securityHeaders());
app.use('*', requestLogger());

// Rate limiting
app.use('*', rateLimit(env.RATE_LIMIT_WINDOW_MS, env.RATE_LIMIT_MAX_REQUESTS));

// CORS
app.use('*', cors({
  origin: env.CORS_ORIGINS,
  allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS', 'PATCH'],
  allowHeaders: ['Content-Type', 'Authorization', 'X-Request-ID'],
  credentials: true,
  maxAge: 86400, // 24 hours
}));

// Pretty JSON only in development
if (env.NODE_ENV === 'development') {
  app.use('*', prettyJSON());
}

// Global error handler
app.onError(errorHandler);

// Health check endpoint
app.get('/', (c) => {
  return c.json({
    success: true,
    message: 'Knot Space - Online Package Repository Server',
    version: '0.1.0',
    timestamp: new Date().toISOString(),
  });
});

app.get('/health', (c) => {
  return c.json({
    success: true,
    status: 'healthy',
    service: 'knot-space',
    version: '0.1.0',
    timestamp: new Date().toISOString(),
  });
});

// Load and mount all modules
export async function setupRoutes() {
  const { logger } = await import('./lib/logger');
  
  try {
    const modules = await loadModules();
    
    if (modules.length === 0) {
      throw new Error('No modules loaded - application cannot start');
    }
    
    for (const module of modules) {
      app.route(module.prefix, module.router);
      logger.info('Module mounted successfully', { 
        name: module.name, 
        prefix: module.prefix 
      });
    }
    
    logger.info('All modules loaded successfully', { 
      totalModules: modules.length,
      modules: modules.map(m => ({ name: m.name, prefix: m.prefix }))
    });
  } catch (error) {
    logger.error('Failed to load modules', { error });
    throw error;
  }
  
  return app;
}

export default app;