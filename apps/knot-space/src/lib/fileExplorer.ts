import * as fs from 'fs/promises';
import * as path from 'path';
import * as tar from 'tar';
import * as zlib from 'zlib';
import { logger } from './logger';

export interface FileEntry {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  children?: FileEntry[];
}

export interface FileContent {
  content: string;
  encoding: 'utf-8' | 'base64';
  mimeType: string;
}

export class FileExplorerService {
  private packageStorePath: string;

  constructor(packageStorePath: string) {
    this.packageStorePath = packageStorePath;
  }

  /**
   * Extract and list files from a .tgz package
   */
  async listPackageFiles(packagePath: string): Promise<FileEntry[]> {
    try {
      const files: FileEntry[] = [];
      const fileMap = new Map<string, FileEntry>();

      // Extract tar.gz file and get file list
      await new Promise<void>((resolve, reject) => {
        const stream = fs.createReadStream(packagePath)
          .pipe(zlib.createGunzip())
          .pipe(tar.list());

        stream.on('entry', (entry) => {
          // Skip the root package directory if it exists
          let relativePath = entry.path;
          const pathParts = relativePath.split('/');
          
          // If first part looks like package name, remove it
          if (pathParts.length > 1 && pathParts[0].match(/^[^/]*$/)) {
            relativePath = pathParts.slice(1).join('/');
          }

          if (!relativePath) return;

          const isDirectory = entry.type === 'Directory';
          const fileEntry: FileEntry = {
            name: path.basename(relativePath),
            path: relativePath,
            type: isDirectory ? 'directory' : 'file',
            size: isDirectory ? undefined : entry.size,
          };

          fileMap.set(relativePath, fileEntry);

          // Build directory structure
          const parentPath = path.dirname(relativePath);
          if (parentPath !== '.' && parentPath !== relativePath) {
            // Ensure parent directory exists
            if (!fileMap.has(parentPath)) {
              this.ensureParentDirectories(parentPath, fileMap);
            }
            
            const parent = fileMap.get(parentPath);
            if (parent && parent.type === 'directory') {
              if (!parent.children) parent.children = [];
              parent.children.push(fileEntry);
            }
          } else {
            // Root level file
            files.push(fileEntry);
          }
        });

        stream.on('error', reject);
        stream.on('end', () => resolve());
      });

      // Sort files and directories
      this.sortFileEntries(files);
      fileMap.forEach(entry => {
        if (entry.children) {
          this.sortFileEntries(entry.children);
        }
      });

      return files;
    } catch (error) {
      logger.error('Failed to list package files', { packagePath, error });
      throw new Error('Failed to list package files');
    }
  }

  /**
   * Get content of a specific file from a .tgz package
   */
  async getFileContent(packagePath: string, filePath: string): Promise<FileContent> {
    try {
      let content: Buffer | null = null;

      await new Promise<void>((resolve, reject) => {
        const stream = fs.createReadStream(packagePath)
          .pipe(zlib.createGunzip())
          .pipe(tar.extract());

        stream.on('entry', (entry) => {
          // Skip the root package directory if it exists
          let relativePath = entry.path;
          const pathParts = relativePath.split('/');
          
          if (pathParts.length > 1 && pathParts[0].match(/^[^/]*$/)) {
            relativePath = pathParts.slice(1).join('/');
          }

          if (relativePath === filePath && entry.type === 'File') {
            const chunks: Buffer[] = [];
            entry.on('data', (chunk) => chunks.push(chunk));
            entry.on('end', () => {
              content = Buffer.concat(chunks);
            });
          }
        });

        stream.on('error', reject);
        stream.on('end', () => resolve());
      });

      if (!content) {
        throw new Error('File not found in package');
      }

      const mimeType = this.getMimeType(filePath);
      const encoding = this.isTextFile(mimeType) ? 'utf-8' : 'base64';

      return {
        content: encoding === 'utf-8' ? content.toString('utf-8') : content.toString('base64'),
        encoding,
        mimeType
      };
    } catch (error) {
      logger.error('Failed to get file content', { packagePath, filePath, error });
      throw new Error('Failed to get file content');
    }
  }

  private ensureParentDirectories(dirPath: string, fileMap: Map<string, FileEntry>) {
    const parts = dirPath.split('/');
    let currentPath = '';

    for (const part of parts) {
      if (currentPath) currentPath += '/';
      currentPath += part;

      if (!fileMap.has(currentPath)) {
        const dirEntry: FileEntry = {
          name: part,
          path: currentPath,
          type: 'directory',
          children: []
        };
        fileMap.set(currentPath, dirEntry);

        // Add to parent's children
        const parentPath = path.dirname(currentPath);
        if (parentPath !== '.' && parentPath !== currentPath) {
          const parent = fileMap.get(parentPath);
          if (parent && parent.children) {
            parent.children.push(dirEntry);
          }
        }
      }
    }
  }

  private sortFileEntries(entries: FileEntry[]) {
    entries.sort((a, b) => {
      // Directories first
      if (a.type !== b.type) {
        return a.type === 'directory' ? -1 : 1;
      }
      // Then alphabetically
      return a.name.localeCompare(b.name);
    });
  }

  private getMimeType(filePath: string): string {
    const ext = path.extname(filePath).toLowerCase();
    
    const mimeTypes: { [key: string]: string } = {
      '.js': 'application/javascript',
      '.ts': 'application/typescript',
      '.jsx': 'application/javascript',
      '.tsx': 'application/typescript',
      '.json': 'application/json',
      '.md': 'text/markdown',
      '.txt': 'text/plain',
      '.yml': 'application/x-yaml',
      '.yaml': 'application/x-yaml',
      '.xml': 'application/xml',
      '.html': 'text/html',
      '.css': 'text/css',
      '.scss': 'text/x-scss',
      '.sass': 'text/x-sass',
      '.less': 'text/x-less',
      '.py': 'text/x-python',
      '.java': 'text/x-java',
      '.c': 'text/x-csrc',
      '.cpp': 'text/x-c++src',
      '.h': 'text/x-chdr',
      '.hpp': 'text/x-c++hdr',
      '.go': 'text/x-go',
      '.rs': 'text/x-rustsrc',
      '.php': 'application/x-httpd-php',
      '.rb': 'text/x-ruby',
      '.sh': 'application/x-sh',
      '.bat': 'application/x-bat',
      '.dockerfile': 'text/x-dockerfile',
      '.gitignore': 'text/plain',
      '.env': 'text/plain',
      '.log': 'text/plain',
    };

    return mimeTypes[ext] || 'text/plain';
  }

  private isTextFile(mimeType: string): boolean {
    return mimeType.startsWith('text/') || 
           mimeType.startsWith('application/json') ||
           mimeType.startsWith('application/javascript') ||
           mimeType.startsWith('application/typescript') ||
           mimeType.startsWith('application/xml') ||
           mimeType.startsWith('application/x-yaml') ||
           mimeType.startsWith('application/x-sh');
  }
}

export const fileExplorerService = new FileExplorerService(process.env.UPLOAD_DIR || './uploads');