import { Context } from 'hono';
import { teamsService } from './service';
import { CreateTeamRequest, AddTeamMemberRequest } from '../../types';

export class TeamsController {
  static async createTeam(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      const body = await c.req.json() as CreateTeamRequest;
      const result = await teamsService.createTeam(body, user.id);
      
      return c.json({
        success: true,
        data: result,
        message: 'Team created successfully'
      }, 201);
    } catch (error) {
      console.error('Create team error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to create team'
      }, 400);
    }
  }

  static async listTeams(c: Context) {
    try {
      const user = c.get('user');
      const teams = await teamsService.listTeams(user?.id);
      
      return c.json({
        success: true,
        data: teams
      });
    } catch (error) {
      console.error('List teams error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to list teams'
      }, 500);
    }
  }

  static async getTeam(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      if (!teamId) {
        return c.json({ success: false, error: 'Team ID is required' }, 400);
      }

      const user = c.get('user');
      const team = await teamsService.getTeam(teamId, user?.id);
      
      return c.json({
        success: true,
        data: team
      });
    } catch (error) {
      console.error('Get team error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Team not found'
      }, 404);
    }
  }

  static async getTeamMembers(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      if (!teamId) {
        return c.json({ success: false, error: 'Team ID is required' }, 400);
      }

      const user = c.get('user');
      const members = await teamsService.getTeamMembers(teamId);
      
      return c.json({
        success: true,
        data: members
      });
    } catch (error) {
      console.error('Get team members error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get team members'
      }, 500);
    }
  }

  static async addTeamMember(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      if (!teamId) {
        return c.json({ success: false, error: 'Team ID is required' }, 400);
      }

      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      const body = await c.req.json() as AddTeamMemberRequest;
      const result = await teamsService.addTeamMember(teamId, body, user.id);
      
      return c.json({
        success: true,
        data: result,
        message: 'Team member added successfully'
      });
    } catch (error) {
      console.error('Add team member error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to add team member'
      }, 400);
    }
  }

  static async removeTeamMember(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      const userId = c.req.param('userId');
      
      if (!teamId || !userId) {
        return c.json({ success: false, error: 'Team ID and User ID are required' }, 400);
      }

      const currentUser = c.get('user');
      if (!currentUser) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      await teamsService.removeTeamMember(teamId, userId, currentUser.id);
      
      return c.json({
        success: true,
        message: 'Team member removed successfully'
      });
    } catch (error) {
      console.error('Remove team member error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to remove team member'
      }, 400);
    }
  }

  static async updateMemberRole(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      const userId = c.req.param('userId');
      
      if (!teamId || !userId) {
        return c.json({ success: false, error: 'Team ID and User ID are required' }, 400);
      }

      const currentUser = c.get('user');
      if (!currentUser) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      const body = await c.req.json();
      const { role } = body;

      if (!role || !['member', 'admin'].includes(role)) {
        return c.json({ success: false, error: 'Valid role is required (member or admin)' }, 400);
      }

      await teamsService.updateMemberRole(teamId, userId, role, currentUser.id);
      
      return c.json({
        success: true,
        message: 'Team member role updated successfully'
      });
    } catch (error) {
      console.error('Update member role error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to update member role'
      }, 400);
    }
  }

  static async deleteTeam(c: Context) {
    try {
      const teamId = c.req.param('teamId');
      if (!teamId) {
        return c.json({ success: false, error: 'Team ID is required' }, 400);
      }

      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      await teamsService.deleteTeam(teamId, user.id);
      
      return c.json({
        success: true,
        message: 'Team deleted successfully'
      });
    } catch (error) {
      console.error('Delete team error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete team'
      }, 400);
    }
  }
}