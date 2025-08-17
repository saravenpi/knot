import { Context } from 'hono';
import { packagesService } from './service';
import { PublishPackageRequest } from '../../types';

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
      
      // Check if the package has a valid download URL (not a placeholder)
      if (!downloadInfo.downloadUrl || downloadInfo.downloadUrl.includes('example.com')) {
        return c.json({
          success: false,
          error: 'Package file not available. The package may not have been uploaded yet.'
        }, 404);
      }
      
      // Increment download count with analytics
      try {
        const clientIP = c.req.header('x-forwarded-for') || c.req.header('x-real-ip') || 'unknown';
        const userAgent = c.req.header('user-agent');
        await packagesService.incrementDownloadCount(name, version, clientIP, userAgent);
      } catch (countError) {
        console.error('Failed to increment download count:', countError);
        // Don't fail the download because of analytics error
      }
      
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

  static async getDownloadStats(c: Context) {
    try {
      const name = c.req.param('name');
      const version = c.req.param('version');
      const days = parseInt(c.req.query('days') || '7');
      
      if (!name || !version) {
        return c.json({ 
          success: false, 
          error: 'Package name and version are required' 
        }, 400);
      }

      if (days < 1 || days > 90) {
        return c.json({ 
          success: false, 
          error: 'Days must be between 1 and 90' 
        }, 400);
      }

      const stats = await packagesService.getDownloadStats(name, version, days);
      
      return c.json({
        success: true,
        data: stats
      });
    } catch (error) {
      console.error('Get download stats error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get download statistics'
      }, 404);
    }
  }

  static async uploadPackageFile(c: Context) {
    try {
      const user = c.get('user');
      if (!user) {
        return c.json({ success: false, error: 'Authentication required' }, 401);
      }

      const body = await c.req.parseBody();
      const file = body['file'] as File;
      const packageName = body['packageName'] as string;
      const version = body['version'] as string;
      
      if (!file) {
        return c.json({ success: false, error: 'No file provided' }, 400);
      }

      if (!packageName || !version) {
        return c.json({ success: false, error: 'Package name and version are required' }, 400);
      }

      // Handle file upload and get file information
      const fileInfo = await packagesService.handleFileUpload(file, user.id);
      
      // Update the package record with real file information
      await packagesService.updatePackageFileInfo(packageName, version, user.id, fileInfo);
      
      return c.json({
        success: true,
        data: fileInfo,
        message: 'File uploaded and package updated successfully'
      });
    } catch (error) {
      console.error('Upload package file error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to upload package file'
      }, 500);
    }
  }

  static async getGlobalStats(c: Context) {
    try {
      const stats = await packagesService.getGlobalStats();
      
      return c.json({
        success: true,
        data: stats
      });
    } catch (error) {
      console.error('Get global stats error:', error);
      return c.json({
        success: false,
        error: error instanceof Error ? error.message : 'Failed to get global statistics'
      }, 500);
    }
  }
}