import { Context, Next } from 'hono';

// Simple in-memory rate limiter (for production, use Redis)
const rateLimitStore = new Map<string, { count: number; resetTime: number }>();

export function rateLimit(windowMs: number, maxRequests: number) {
  return async (c: Context, next: Next) => {
    const ip = c.req.header('x-forwarded-for') || c.req.header('x-real-ip') || 'unknown';
    const key = `${ip}:${c.req.path}`;
    const now = Date.now();

    const current = rateLimitStore.get(key);

    if (!current || now > current.resetTime) {
      rateLimitStore.set(key, { count: 1, resetTime: now + windowMs });
      return await next();
    }

    if (current.count >= maxRequests) {
      return c.json(
        {
          success: false,
          error: 'Too many requests',
          retryAfter: Math.ceil((current.resetTime - now) / 1000),
        },
        429
      );
    }

    current.count++;
    return await next();
  };
}

export function securityHeaders() {
  return async (c: Context, next: Next) => {
    await next();

    // Security headers
    c.header('X-Content-Type-Options', 'nosniff');
    c.header('X-Frame-Options', 'DENY');
    c.header('X-XSS-Protection', '1; mode=block');
    c.header('Referrer-Policy', 'strict-origin-when-cross-origin');
    c.header('Permissions-Policy', 'camera=(), microphone=(), geolocation=()');

    // Remove server info
    c.header('Server', '');

    if (process.env.NODE_ENV === 'production') {
      c.header('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
    }
  };
}

export function requestLogger() {
  return async (c: Context, next: Next) => {
    const start = Date.now();
    const method = c.req.method;
    const path = c.req.path;
    const userAgent = c.req.header('user-agent') || 'unknown';
    const ip = c.req.header('x-forwarded-for') || c.req.header('x-real-ip') || 'unknown';

    await next();

    const duration = Date.now() - start;
    const status = c.res.status;

    console.log(
      `${new Date().toISOString()} ${method} ${path} ${status} ${duration}ms - ${ip} - ${userAgent}`
    );
  };
}
