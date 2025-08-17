import { Context } from 'hono';
import { usersService } from './service';

export class UsersController {
  static async getAllUsers(c: Context) {
    try {
      const users = await usersService.getAllUsers();
      
      return c.json({
        success: true,
        data: users
      });
    } catch (error) {
      console.error('Get all users error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get users'
      }, 500);
    }
  }

  static async getUserProfile(c: Context) {
    try {
      const username = c.req.param('username');
      if (!username) {
        return c.json({ success: false, error: 'Username is required' }, 400);
      }

      const userProfile = await usersService.getUserProfile(username);
      
      return c.json({
        success: true,
        data: userProfile
      });
    } catch (error) {
      console.error('Get user profile error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'User not found'
      }, 404);
    }
  }

  static async getUserPackages(c: Context) {
    try {
      const username = c.req.param('username');
      if (!username) {
        return c.json({ success: false, error: 'Username is required' }, 400);
      }

      const query = c.req.query();
      const filters = {
        limit: query.limit ? parseInt(query.limit) : 20,
        offset: query.offset ? parseInt(query.offset) : 0,
      };

      const packages = await usersService.getUserPackages(username, filters);
      
      return c.json({
        success: true,
        data: packages
      });
    } catch (error) {
      console.error('Get user packages error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get user packages'
      }, 500);
    }
  }

  static async getUserStats(c: Context) {
    try {
      const username = c.req.param('username');
      if (!username) {
        return c.json({ success: false, error: 'Username is required' }, 400);
      }

      const stats = await usersService.getUserStats(username);
      
      return c.json({
        success: true,
        data: stats
      });
    } catch (error) {
      console.error('Get user stats error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get user statistics'
      }, 500);
    }
  }
}