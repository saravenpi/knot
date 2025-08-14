import { Hono } from 'hono';
import { UsersController } from './controller';

const users = new Hono();

users.get('/:username', UsersController.getUserProfile);
users.get('/:username/packages', UsersController.getUserPackages);
users.get('/:username/stats', UsersController.getUserStats);

export default users;
export const prefix = '/api/users';