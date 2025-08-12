import { PrismaClient } from '@prisma/client';
import { logger } from './logger';
import { env } from './env';

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
};

export const prisma =
  globalForPrisma.prisma ??
  new PrismaClient({
    log: env.NODE_ENV === 'development' 
      ? [
          { emit: 'event', level: 'query' },
          { emit: 'event', level: 'error' },
          { emit: 'event', level: 'info' },
          { emit: 'event', level: 'warn' },
        ]
      : [
          { emit: 'event', level: 'error' },
          { emit: 'event', level: 'warn' },
        ],
    datasources: {
      db: {
        url: env.DATABASE_URL,
      },
    },
    // Connection pool settings for production
    ...(env.NODE_ENV === 'production' && {
      datasourceUrl: env.DATABASE_URL,
    }),
  });

// Set up logging
if (env.NODE_ENV === 'development') {
  prisma.$on('query', (e) => {
    logger.debug('Database query', {
      query: e.query,
      params: e.params,
      duration: `${e.duration}ms`,
    });
  });
}

prisma.$on('error', (e) => {
  logger.error('Database error', {
    message: e.message,
    target: e.target,
  });
});

prisma.$on('warn', (e) => {
  logger.warn('Database warning', {
    message: e.message,
    target: e.target,
  });
});

prisma.$on('info', (e) => {
  logger.info('Database info', {
    message: e.message,
    target: e.target,
  });
});

// Connection health check
export async function checkDatabaseConnection() {
  try {
    await prisma.$queryRaw`SELECT 1`;
    logger.info('Database connection established successfully');
  } catch (error) {
    logger.error('Database connection failed', { error });
    throw error;
  }
}

// Graceful shutdown
export async function disconnectDatabase() {
  try {
    await prisma.$disconnect();
    logger.info('Database connection closed');
  } catch (error) {
    logger.error('Error closing database connection', { error });
    throw error;
  }
}

if (env.NODE_ENV !== 'production') {
  globalForPrisma.prisma = prisma;
}