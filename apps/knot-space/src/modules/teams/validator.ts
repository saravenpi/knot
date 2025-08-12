import { zValidator } from '@hono/zod-validator';
import { CreateTeamSchema, AddTeamMemberSchema } from '@/types';

export const validateCreateTeam = zValidator('json', CreateTeamSchema);
export const validateAddTeamMember = zValidator('json', AddTeamMemberSchema);