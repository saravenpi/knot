import { Context, Next } from 'hono';
import { authModuleService } from '@/modules/auth/service';

export async function authMiddleware(c: Context, next: Next) {
  try {
    const authHeader = c.req.header('Authorization');
    
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      return c.json({
        success: false,
        error: 'Authentication required'
      }, 401);
    }

    const token = authHeader.substring(7);
    const sessionData = await authModuleService.verifySession(token);
    
    const user = {
      id: sessionData.userId,
      username: sessionData.username,
      email: '', // We'll get this from the session if needed
      createdAt: new Date(), // Placeholder
    };

    c.set('user', user);
    await next();
  } catch (error) {
    return c.json({
      success: false,
      error: 'Invalid authentication token'
    }, 401);
  }
}

export async function optionalAuthMiddleware(c: Context, next: Next) {
  try {
    const authHeader = c.req.header('Authorization');
    
    if (authHeader && authHeader.startsWith('Bearer ')) {
      const token = authHeader.substring(7);
      const sessionData = await authModuleService.verifySession(token);
      
      const user = {
        id: sessionData.userId,
        username: sessionData.username,
        email: '', // We'll get this from the session if needed
        createdAt: new Date(), // Placeholder
      };

      c.set('user', user);
    }
    
    await next();
  } catch {
    // Ignore authentication errors for optional auth
    await next();
  }
}