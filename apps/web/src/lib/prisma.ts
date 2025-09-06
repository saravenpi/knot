import { PrismaClient } from '@prisma/client';
import { logger } from './logger';
import { env } from './env';

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
};

export const prisma =
  globalForPrisma.prisma ??
  new PrismaClient({
    log:
      env.NODE_ENV === 'development'
        ? ['query', 'error', 'info', 'warn']
        : ['error', 'warn'],
    datasources: {
      db: {
        url: env.DATABASE_URL,
      },
    },
  });

// Set up logging - using simple console.log since event-based logging has type issues
if (env.NODE_ENV === 'development') {
  // Development logging will be handled through Prisma's built-in logging
}

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
