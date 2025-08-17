import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { prettyJSON } from 'hono/pretty-json';
import { serveStatic } from 'hono/serve-static';
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

// CORS - Debug logging
console.log('🔧 CORS Origins configured:', env.CORS_ORIGINS);

app.use('*', cors({
  origin: (origin, c) => {
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

// Analytics middleware for tracking downloads
app.use('/uploads/*', async (c, next) => {
  await next();
  
  // Only track if file was served successfully
  if (c.res.status === 200) {
    try {
      // Extract package info from filename (format: checksum.tar.gz)
      const path = c.req.path;
      const filename = path.split('/').pop();
      
      if (filename && filename.endsWith('.tar.gz')) {
        // Find package by file checksum
        const { packagesService } = await import('./modules/packages/service');
        const checksum = filename.replace('.tar.gz', '');
        
        // Get package info by checksum
        const pkg = await packagesService.getPackageByChecksum(checksum);
        if (pkg) {
          const clientIP = c.req.header('x-forwarded-for') || c.req.header('x-real-ip') || 'unknown';
          const userAgent = c.req.header('user-agent');
          await packagesService.incrementDownloadCount(pkg.name, pkg.version, clientIP, userAgent);
        }
      }
    } catch (error) {
      console.error('Failed to track download analytics:', error);
      // Don't fail the file serving because of analytics error
    }
  }
});

// Serve static files from uploads directory
app.use('/uploads/*', serveStatic({ 
  root: './uploads',
  rewriteRequestPath: (path) => path.replace(/^\/uploads/, '')
}));

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