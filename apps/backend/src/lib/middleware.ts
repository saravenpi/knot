import { Context, Next } from 'hono';
import { authModuleService } from '../modules/auth/service';

export async function authMiddleware(c: Context, next: Next) {
  try {
    const authHeader = c.req.header('Authorization');

    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      return c.json(
        {
          success: false,
          error: 'Authentication required',
        },
        401
      );
    }

    const token = authHeader.substring(7);
    const decoded = authModuleService.verifyToken(token);

    // Just set the decoded user info instead of doing a database lookup
    c.set('user', { id: decoded.userId, username: decoded.username });
    await next();
  } catch (error) {
    console.error('Auth middleware error:', error);
    return c.json(
      {
        success: false,
        error: 'Invalid authentication token',
      },
      401
    );
  }
}

export async function optionalAuthMiddleware(c: Context, next: Next) {
  try {
    const authHeader = c.req.header('Authorization');

    if (authHeader && authHeader.startsWith('Bearer ')) {
      const token = authHeader.substring(7);
      const decoded = authModuleService.verifyToken(token);
      c.set('user', { id: decoded.userId, username: decoded.username });
    }

    await next();
  } catch {
    // Ignore authentication errors for optional auth
    await next();
  }
}
