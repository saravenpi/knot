import { prisma } from '@/lib/prisma';
import { CreateUserRequest, LoginRequest, UserProfile, AuthResponse } from '@/types';
import * as bcrypt from 'bcryptjs';
import * as jwt from 'jsonwebtoken';

class AuthService {
  private readonly JWT_SECRET = process.env.BETTER_AUTH_SECRET!;
  private readonly JWT_EXPIRES_IN = '7d';

  async register(data: CreateUserRequest): Promise<AuthResponse> {
    const existingUser = await prisma.user.findFirst({
      where: {
        OR: [
          { username: data.username },
          { email: data.email }
        ]
      }
    });

    if (existingUser) {
      throw new Error('Username or email already exists');
    }

    const hashedPassword = await bcrypt.hash(data.password, 12);

    const user = await prisma.user.create({
      data: {
        username: data.username,
        email: data.email,
        passwordHash: hashedPassword,
      },
      select: {
        id: true,
        username: true,
        email: true,
        createdAt: true,
      }
    });

    const token = this.generateToken(user.id, user.username);

    return {
      token,
      user: user as UserProfile,
    };
  }

  async login(data: LoginRequest): Promise<AuthResponse> {
    // Add delay to prevent timing attacks
    const startTime = Date.now();
    
    const user = await prisma.user.findUnique({
      where: { username: data.username },
    });

    let isValidPassword = false;
    if (user) {
      isValidPassword = await bcrypt.compare(data.password, user.passwordHash);
    } else {
      // Perform dummy hash to prevent timing attacks
      await bcrypt.compare('dummy', '$2a$12$dummy.hash.to.prevent.timing.attacks');
    }

    // Ensure minimum processing time to prevent timing attacks
    const elapsed = Date.now() - startTime;
    if (elapsed < 100) {
      await new Promise(resolve => setTimeout(resolve, 100 - elapsed));
    }

    if (!user || !isValidPassword) {
      throw new Error('Invalid username or password');
    }

    const token = this.generateToken(user.id, user.username);

    return {
      token,
      user: {
        id: user.id,
        username: user.username,
        email: user.email,
        createdAt: user.createdAt,
      },
    };
  }

  async getProfile(userId: string): Promise<UserProfile> {
    const user = await prisma.user.findUnique({
      where: { id: userId },
      select: {
        id: true,
        username: true,
        email: true,
        createdAt: true,
      }
    });

    if (!user) {
      throw new Error('User not found');
    }

    return user;
  }

  async deleteAccount(userId: string): Promise<void> {
    await prisma.user.delete({
      where: { id: userId }
    });
  }

  async getUserByUsername(username: string): Promise<UserProfile> {
    const user = await prisma.user.findUnique({
      where: { username },
      select: {
        id: true,
        username: true,
        email: true,
        createdAt: true,
      }
    });

    if (!user) {
      throw new Error('User not found');
    }

    return user;
  }

  verifyToken(token: string): { userId: string; username: string } {
    try {
      const decoded = jwt.verify(token, this.JWT_SECRET, {
        algorithms: ['HS256'], // Specify allowed algorithms
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

  private generateToken(userId: string, username: string): string {
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
}

export const authService = new AuthService();