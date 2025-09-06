import { Hono } from 'hono';
import { UploadsController } from './controller';

const uploads = new Hono();

uploads.get('/*', UploadsController.serveFile);

export default uploads;
export const prefix = '/uploads';