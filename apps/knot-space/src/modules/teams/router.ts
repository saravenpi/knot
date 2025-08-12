import { Hono } from 'hono';
import { TeamsController } from './controller';
import { validateCreateTeam, validateAddTeamMember } from './validator';
import { authMiddleware, optionalAuthMiddleware } from '@/middleware';

const teams = new Hono();

teams.post('/', authMiddleware, validateCreateTeam, TeamsController.createTeam);
teams.get('/', optionalAuthMiddleware, TeamsController.listTeams);
teams.get('/:teamId', optionalAuthMiddleware, TeamsController.getTeam);
teams.get('/:teamId/members', optionalAuthMiddleware, TeamsController.getTeamMembers);
teams.post('/:teamId/members', authMiddleware, validateAddTeamMember, TeamsController.addTeamMember);
teams.delete('/:teamId/members/:userId', authMiddleware, TeamsController.removeTeamMember);
teams.delete('/:teamId', authMiddleware, TeamsController.deleteTeam);

export default teams;
export const prefix = '/api/teams';