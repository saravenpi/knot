import { Context } from 'hono';

export class UploadsController {
  static async serveFile(c: Context) {
    try {
      const fs = await import('fs');
      const path = await import('path');
      
      // Extract filename from URL
      const requestPath = c.req.path;
      const filename = requestPath.replace('/uploads/', '');
      const filePath = path.join(process.cwd(), 'uploads', filename);
      
      // Check if file exists
      if (!fs.existsSync(filePath)) {
        return c.json({ success: false, error: 'File not found' }, 404);
      }
      
      // Read file
      const fileBuffer = fs.readFileSync(filePath);
      
      // Set appropriate headers
      const mimeType = filename.endsWith('.tar.gz') ? 'application/gzip' : 'application/octet-stream';
      
      // Track analytics for successful downloads
      try {
        if (filename.endsWith('.tar.gz')) {
          const { packagesService } = await import('../packages/service');
          const checksum = filename.replace('.tar.gz', '');
          
          const pkg = await packagesService.getPackageByChecksum(checksum);
          if (pkg) {
            const clientIP = c.req.header('x-forwarded-for') || c.req.header('x-real-ip') || 'unknown';
            const userAgent = c.req.header('user-agent');
            await packagesService.incrementDownloadCount(pkg.name, pkg.version, clientIP, userAgent);
          }
        }
      } catch (analyticsError) {
        console.error('Failed to track download analytics:', analyticsError);
        // Don't fail the file serving because of analytics error
      }
      
      return new Response(fileBuffer, {
        headers: {
          'Content-Type': mimeType,
          'Content-Length': fileBuffer.length.toString(),
          'Content-Disposition': `attachment; filename="${filename}"`,
          'Cache-Control': 'public, max-age=31536000' // 1 year cache
        }
      });
    } catch (error) {
      console.error('File serving error:', error);
      return c.json({ success: false, error: 'Internal server error' }, 500);
    }
  }
}