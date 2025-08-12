import { z } from 'zod';

const envSchema = z.object({
  NODE_ENV: z.enum(['development', 'production', 'test']).default('development'),
  PORT: z.string().transform(val => parseInt(val)).pipe(z.number().min(1).max(65535)).default('3001'),
  
  // Database
  DATABASE_URL: z.string().url('DATABASE_URL must be a valid PostgreSQL connection string'),
  
  // Auth
  AUTH_SECRET: z.string().min(32, 'AUTH_SECRET must be at least 32 characters'),
  API_URL: z.string().url('API_URL must be a valid URL').default('http://localhost:3001'),
  
  // File Upload
  UPLOAD_DIR: z.string().default('./uploads'),
  MAX_FILE_SIZE: z.string().transform(val => parseInt(val)).pipe(z.number().min(1)).default('104857600'), // 100MB
  
  // Logging
  LOG_LEVEL: z.enum(['ERROR', 'WARN', 'INFO', 'DEBUG']).default('INFO'),
  
  // Rate Limiting
  RATE_LIMIT_WINDOW_MS: z.string().transform(val => parseInt(val)).pipe(z.number().min(1000)).default('900000'), // 15 minutes
  RATE_LIMIT_MAX_REQUESTS: z.string().transform(val => parseInt(val)).pipe(z.number().min(1)).default('100'),
  
  // CORS
  CORS_ORIGINS: z.string().default('http://localhost:3000,http://localhost:5173').transform(val => val.split(',')),
});

export type Env = z.infer<typeof envSchema>;

let env: Env;

try {
  env = envSchema.parse(process.env);
} catch (error) {
  console.error('❌ Environment validation failed:');
  if (error instanceof z.ZodError) {
    error.errors.forEach(err => {
      console.error(`  - ${err.path.join('.')}: ${err.message}`);
    });
  }
  process.exit(1);
}

export { env };

// Helper to check if required environment variables are set
export function validateRequiredEnv() {
  const required = [
    'DATABASE_URL',
    'AUTH_SECRET',
  ];
  
  const missing = required.filter(key => !process.env[key]);
  
  if (missing.length > 0) {
    console.error('❌ Missing required environment variables:');
    missing.forEach(key => console.error(`  - ${key}`));
    console.error('\nPlease set these variables before starting the application.');
    process.exit(1);
  }
}