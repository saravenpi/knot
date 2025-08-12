import { Context } from 'hono';
import { packagesService } from './service';
import { PublishPackageRequest } from '@/types';

export class PackagesController {
  static async publishPackage(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      const body = await c.req.json() as PublishPackageRequest;
      const result = await packagesService.publishPackage(body, user.id);
      
      return c.json({
        success: true,
        data: result,
        message: 'Package published successfully'
      }, 201);
    } catch (error) {
      console.error('Publish package error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to publish package'
      }, 400);
    }
  }

  static async listPackages(c: Context) {
    try {
      const query = c.req.query();
      const filters = {
        search: query.search,
        owner: query.owner,
        team: query.team,
        tags: query.tags?.split(',').filter(Boolean),
        limit: query.limit ? parseInt(query.limit) : 20,
        offset: query.offset ? parseInt(query.offset) : 0,
      };

      const packages = await packagesService.listPackages(filters);
      
      return c.json({
        success: true,
        data: packages
      });
    } catch (error) {
      console.error('List packages error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to list packages'
      }, 500);
    }
  }

  static async getPackageVersions(c: Context) {
    try {
      const name = c.req.param('name');
      if (!name) {
        return c.json({ success: false, error: 'Package name is required' }, 400);
      }

      const versions = await packagesService.getPackageVersions(name);
      
      return c.json({
        success: true,
        data: versions
      });
    } catch (error) {
      console.error('Get package versions error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Package not found'
      }, 404);
    }
  }

  static async getPackage(c: Context) {
    try {
      const name = c.req.param('name');
      const version = c.req.param('version');
      
      if (!name || !version) {
        return c.json({ 
          success: false, 
          error: 'Package name and version are required' 
        }, 400);
      }

      const pkg = await packagesService.getPackage(name, version);
      
      return c.json({
        success: true,
        data: pkg
      });
    } catch (error) {
      console.error('Get package error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Package not found'
      }, 404);
    }
  }

  static async downloadPackage(c: Context) {
    try {
      const name = c.req.param('name');
      const version = c.req.param('version');
      
      if (!name || !version) {
        return c.json({ 
          success: false, 
          error: 'Package name and version are required' 
        }, 400);
      }

      const downloadInfo = await packagesService.downloadPackage(name, version);
      
      // Increment download count
      await packagesService.incrementDownloadCount(name, version);
      
      return c.redirect(downloadInfo.downloadUrl);
    } catch (error) {
      console.error('Download package error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Package not found'
      }, 404);
    }
  }

  static async deletePackage(c: Context) {
    try {
      const name = c.req.param('name');
      const version = c.req.param('version');
      
      if (!name || !version) {
        return c.json({ 
          success: false, 
          error: 'Package name and version are required' 
        }, 400);
      }

      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      await packagesService.deletePackage(name, version, user.id);
      
      return c.json({
        success: true,
        message: 'Package deleted successfully'
      });
    } catch (error) {
      console.error('Delete package error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete package'
      }, 400);
    }
  }

  static async uploadPackageFile(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      // This would handle multipart file upload
      // For now, return a placeholder response
      return c.json({
        success: false,
        error: 'File upload not implemented yet'
      }, 501);
    } catch (error) {
      console.error('Upload package file error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to upload package file'
      }, 500);
    }
  }
}