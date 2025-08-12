// Register path mapping for TypeScript aliases
import 'tsconfig-paths/register';
import { serve } from '@hono/node-server';
import { setupRoutes } from './router';
import { validateRequiredEnv, env } from './lib/env';
import { logger } from './lib/logger';
import { checkDatabaseConnection, disconnectDatabase } from './lib/prisma';
import { fileUploadService } from './lib/fileUpload';

async function startServer() {
  try {
    logger.info('Starting Knot Space server...', {
      nodeEnv: env.NODE_ENV,
      port: env.PORT,
      logLevel: env.LOG_LEVEL,
    });
    
    // Validate environment variables
    validateRequiredEnv();
    
    // Initialize file upload service
    await fileUploadService.initialize();
    
    // Check database connection
    await checkDatabaseConnection();
    
    // Setup all routes
    const app = await setupRoutes();
    
    
    logger.info('Server configuration', {
      port: env.PORT,
      corsOrigins: env.CORS_ORIGINS,
      uploadDir: env.UPLOAD_DIR,
      maxFileSize: env.MAX_FILE_SIZE,
    });
    
    serve({
      fetch: app.fetch,
      port: env.PORT,
    });
    
    logger.info('Knot Space server started successfully', {
      port: env.PORT,
      nodeEnv: env.NODE_ENV,
      pid: process.pid,
    });
  } catch (error) {
    logger.error('Failed to start server', { 
      error: error instanceof Error ? error.message : 'Unknown error',
      stack: error instanceof Error ? error.stack : undefined,
    });
    process.exit(1);
  }
}

// Handle graceful shutdown
async function gracefulShutdown(signal: string) {
  logger.info('Received shutdown signal, shutting down gracefully...', { signal });
  
  try {
    // Close database connections
    await disconnectDatabase();
    
    logger.info('Graceful shutdown completed');
    process.exit(0);
  } catch (error) {
    logger.error('Error during graceful shutdown', { error });
    process.exit(1);
  }
}

process.on('SIGINT', () => gracefulShutdown('SIGINT'));
process.on('SIGTERM', () => gracefulShutdown('SIGTERM'));

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  logger.error('Uncaught exception', { 
    error: error.message, 
    stack: error.stack 
  });
  process.exit(1);
});

process.on('unhandledRejection', (reason) => {
  logger.error('Unhandled rejection', { 
    reason: reason instanceof Error ? reason.message : reason,
    stack: reason instanceof Error ? reason.stack : undefined,
  });
  process.exit(1);
});

startServer();