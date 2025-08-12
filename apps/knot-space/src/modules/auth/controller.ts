import { Context } from 'hono';
import { authService } from './service';
import { CreateUserRequest, LoginRequest } from '@/types';

export class AuthController {
  static async register(c: Context) {
    try {
      const body = await c.req.json() as CreateUserRequest;
      const result = await authService.register(body);
      
      return c.json({
        success: true,
        data: result,
        message: 'User registered successfully'
      }, 201);
    } catch (error) {
      console.error('Registration error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Registration failed'
      }, 400);
    }
  }

  static async login(c: Context) {
    try {
      const body = await c.req.json() as LoginRequest;
      const result = await authService.login(body);
      
      return c.json({
        success: true,
        data: result,
        message: 'Login successful'
      });
    } catch (error) {
      console.error('Login error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Login failed'
      }, 401);
    }
  }

  static async getProfile(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({
          success: false,
          error: 'User not authenticated'
        }, 401);
      }

      const profile = await authService.getProfile(user.id);
      
      return c.json({
        success: true,
        data: profile
      });
    } catch (error) {
      console.error('Get profile error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get profile'
      }, 500);
    }
  }

  static async deleteAccount(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({
          success: false,
          error: 'User not authenticated'
        }, 401);
      }

      await authService.deleteAccount(user.id);
      
      return c.json({
        success: true,
        message: 'Account deleted successfully'
      });
    } catch (error) {
      console.error('Delete account error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete account'
      }, 500);
    }
  }

  static async getUserByUsername(c: Context) {
    try {
      const username = c.req.param('username');
      if (!username) {
        return c.json({
          success: false,
          error: 'Username is required'
        }, 400);
      }

      const user = await authService.getUserByUsername(username);
      
      return c.json({
        success: true,
        data: user
      });
    } catch (error) {
      console.error('Get user by username error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'User not found'
      }, 404);
    }
  }
}