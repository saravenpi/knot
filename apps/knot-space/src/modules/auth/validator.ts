import { zValidator } from '@hono/zod-validator';
import { CreateUserSchema, LoginSchema } from '../../types';

export const validateCreateUser = zValidator('json', CreateUserSchema);
export const validateLogin = zValidator('json', LoginSchema);