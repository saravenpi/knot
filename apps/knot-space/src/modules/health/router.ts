import { Hono } from 'hono';
import { HealthController } from './controller';

const health = new Hono();

health.get('/', HealthController.getRoot);
health.get('/health', HealthController.getHealth);

export default health;
export const prefix = '/';