import { Context, Next } from 'hono';
import { authService } from '@/modules/auth/service';
import { prisma } from './prisma';

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
    const decoded = authService.verifyToken(token);
    
    const user = await prisma.user.findUnique({
      where: { id: decoded.userId },
      select: {
        id: true,
        username: true,
        email: true,
        createdAt: true,
      }
    });

    if (!user) {
      return c.json({
        success: false,
        error: 'User not found'
      }, 401);
    }

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
      const decoded = authService.verifyToken(token);
      
      const user = await prisma.user.findUnique({
        where: { id: decoded.userId },
        select: {
          id: true,
          username: true,
          email: true,
          createdAt: true,
        }
      });

      if (user) {
        c.set('user', user);
      }
    }
    
    await next();
  } catch {
    // Ignore authentication errors for optional auth
    await next();
  }
}