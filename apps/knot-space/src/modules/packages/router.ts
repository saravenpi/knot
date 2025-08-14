import { Hono } from 'hono';
import { PackagesController } from './controller';
import { validatePublishPackage, validatePackageQuery } from './validator';
import { authMiddleware } from '../../lib/middleware';

const packages = new Hono();

packages.post('/', authMiddleware, validatePublishPackage, PackagesController.publishPackage);
packages.post('/upload', authMiddleware, PackagesController.uploadPackageFile);
packages.get('/stats', PackagesController.getGlobalStats);
packages.get('/', validatePackageQuery, PackagesController.listPackages);
packages.get('/:name/versions', PackagesController.getPackageVersions);
packages.get('/:name/:version', PackagesController.getPackage);
packages.get('/:name/:version/download', PackagesController.downloadPackage);
packages.get('/:name/:version/stats', PackagesController.getDownloadStats);
packages.delete('/:name/:version', authMiddleware, PackagesController.deletePackage);

export default packages;
export const prefix = '/api/packages';