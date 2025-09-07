import { zValidator } from '@hono/zod-validator';
import { CreateUserSchema, LoginSchema, UpdateProfileSchema } from '../../types';

export const validateCreateUser = zValidator('json', CreateUserSchema);
export const validateLogin = zValidator('json', LoginSchema);
export const validateUpdateProfile = zValidator('json', UpdateProfileSchema);