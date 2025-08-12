import { Hono } from 'hono';
import { AuthController } from './controller';
import { validateCreateUser, validateLogin } from './validator';
import { authMiddleware } from '../../lib/middleware';

const auth = new Hono();

auth.post('/register', validateCreateUser, AuthController.register);
auth.post('/login', validateLogin, AuthController.login);
auth.get('/profile', authMiddleware, AuthController.getProfile);
auth.delete('/account', authMiddleware, AuthController.deleteAccount);
auth.get('/user/:username', AuthController.getUserByUsername);

export default auth;
export const prefix = '/api/auth';