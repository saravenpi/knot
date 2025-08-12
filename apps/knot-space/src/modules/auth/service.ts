import { auth } from '@/lib/auth';
import { CreateUserRequest, LoginRequest, UserProfile, AuthResponse } from '@/types';

class AuthServiceModule {

  async register(data: CreateUserRequest): Promise<AuthResponse> {
    try {
      const result = await auth.api.signUpEmail({
        body: {
          email: data.email,
          password: data.password,
          name: data.username,
        },
      });

      if (!result.data?.user || !result.data?.session) {
        throw new Error('Registration failed');
      }

      return {
        token: result.data.session.token,
        user: {
          id: result.data.user.id,
          username: result.data.user.name,
          email: result.data.user.email,
          createdAt: result.data.user.createdAt,
        },
      };
    } catch (error) {
      if (error instanceof Error && error.message.includes('already exists')) {
        throw new Error('Username or email already exists');
      }
      throw new Error('Registration failed');
    }
  }

  async login(data: LoginRequest): Promise<AuthResponse> {
    try {
      const result = await auth.api.signInEmail({
        body: {
          email: data.username.includes('@') ? data.username : `${data.username}@example.com`, // Handle username or email
          password: data.password,
        },
      });

      if (!result.data?.user || !result.data?.session) {
        throw new Error('Invalid username or password');
      }

      return {
        token: result.data.session.token,
        user: {
          id: result.data.user.id,
          username: result.data.user.name,
          email: result.data.user.email,
          createdAt: result.data.user.createdAt,
        },
      };
    } catch (error) {
      throw new Error('Invalid username or password');
    }
  }

  async getProfile(userId: string): Promise<UserProfile> {
    try {
      const result = await auth.api.getSession({
        headers: {
          authorization: `Bearer ${userId}`, // This should be the session token
        },
      });

      if (!result.data?.user) {
        throw new Error('User not found');
      }

      return {
        id: result.data.user.id,
        username: result.data.user.name,
        email: result.data.user.email,
        createdAt: result.data.user.createdAt,
      };
    } catch (error) {
      throw new Error('User not found');
    }
  }

  async deleteAccount(userId: string): Promise<void> {
    // Better-auth handles user deletion through sessions
    // We'll implement this when we have proper session management
    throw new Error('Account deletion not implemented yet');
  }

  async getUserByUsername(username: string): Promise<UserProfile> {
    // Better-auth doesn't have a direct username search, we'll use our custom logic
    throw new Error('Username search not implemented yet');
  }

  async verifySession(sessionToken: string): Promise<{ userId: string; username: string }> {
    try {
      const result = await auth.api.getSession({
        headers: {
          authorization: `Bearer ${sessionToken}`,
        },
      });

      if (!result.data?.user) {
        throw new Error('Invalid session');
      }

      return {
        userId: result.data.user.id,
        username: result.data.user.name,
      };
    } catch (error) {
      throw new Error('Invalid session');
    }
  }

  // Legacy method for compatibility
  verifyToken(token: string): { userId: string; username: string } {
    // This will be async, but keeping sync for now for compatibility
    throw new Error('Use verifySession instead');
  }
}

export const authModuleService = new AuthServiceModule();