import * as jwt from 'jsonwebtoken';
import { env } from './env';

export interface User {
  id: string;
  username: string;
  email: string;
  createdAt: Date;
}

export interface Session {
  userId: string;
  username: string;
  expiresAt: Date;
}

export class AuthService {
  private readonly JWT_SECRET = env.AUTH_SECRET;
  private readonly JWT_EXPIRES_IN = '7d';

  generateToken(userId: string, username: string): string {
    return jwt.sign(
      {
        sub: userId,
        username,
        iat: Math.floor(Date.now() / 1000),
      },
      this.JWT_SECRET,
      { expiresIn: this.JWT_EXPIRES_IN }
    );
  }

  verifyToken(token: string): { userId: string; username: string } {
    try {
      const decoded = jwt.verify(token, this.JWT_SECRET, {
        algorithms: ['HS256'],
        maxAge: this.JWT_EXPIRES_IN,
      }) as any;
      
      if (!decoded.sub || !decoded.username) {
        throw new Error('Invalid token payload');
      }
      
      return {
        userId: decoded.sub,
        username: decoded.username,
      };
    } catch (error) {
      if (error instanceof jwt.JsonWebTokenError) {
        throw new Error('Invalid token');
      }
      if (error instanceof jwt.TokenExpiredError) {
        throw new Error('Token expired');
      }
      throw new Error('Token verification failed');
    }
  }
}

export const authService = new AuthService();