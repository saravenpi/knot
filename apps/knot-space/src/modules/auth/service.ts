import { prisma } from '@/lib/prisma';
import { authService } from '@/lib/auth';
import { CreateUserRequest, LoginRequest, UserProfile, AuthResponse } from '@/types';
import * as bcrypt from 'bcryptjs';

class AuthServiceModule {

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

    const token = authService.generateToken(user.id, user.username);

    return {
      token,
      user: user as UserProfile,
    };
  }

  async login(data: LoginRequest): Promise<AuthResponse> {
    const user = await prisma.user.findUnique({
      where: { username: data.username },
    });

    if (!user) {
      throw new Error('Invalid username or password');
    }

    const isValidPassword = await bcrypt.compare(data.password, user.passwordHash);
    if (!isValidPassword) {
      throw new Error('Invalid username or password');
    }

    const token = authService.generateToken(user.id, user.username);

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
    return authService.verifyToken(token);
  }
}

export const authModuleService = new AuthServiceModule();