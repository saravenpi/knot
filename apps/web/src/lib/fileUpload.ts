import * as crypto from 'crypto';
import { promises as fs } from 'fs';
import * as path from 'path';
import { env } from './env';
import { logger } from './logger';

// Allowed MIME types for package files
const ALLOWED_MIME_TYPES = [
  'application/gzip',
  'application/x-gzip',
  'application/x-tar',
  'application/x-compressed',
  'application/octet-stream', // .tgz files
];

// Allowed file extensions
const ALLOWED_EXTENSIONS = ['.tgz', '.tar.gz', '.zip'];

// Magic bytes for file type validation
const FILE_SIGNATURES = {
  // Gzip
  gzip: [0x1f, 0x8b],
  // Zip
  zip: [0x50, 0x4b, 0x03, 0x04],
  // Tar
  tar: [0x75, 0x73, 0x74, 0x61, 0x72], // "ustar" at offset 257
};

export interface UploadResult {
  fileName: string;
  filePath: string;
  fileSize: number;
  checksumSha256: string;
  mimeType: string;
}

export class FileUploadService {
  private uploadDir: string;

  constructor() {
    this.uploadDir = env.UPLOAD_DIR;
  }

  async initialize() {
    try {
      await fs.mkdir(this.uploadDir, { recursive: true });
      // Set secure permissions (owner read/write only)
      await fs.chmod(this.uploadDir, 0o700);
    } catch (error) {
      logger.error('Failed to initialize upload directory', { error });
      throw error;
    }
  }

  async validateFile(buffer: Buffer, fileName: string, mimeType: string): Promise<void> {
    // Check file size
    if (buffer.length > env.MAX_FILE_SIZE) {
      throw new Error(`File size exceeds limit of ${env.MAX_FILE_SIZE} bytes`);
    }

    // Check file extension
    const ext = path.extname(fileName).toLowerCase();
    if (!ALLOWED_EXTENSIONS.includes(ext)) {
      throw new Error(`File extension ${ext} not allowed`);
    }

    // Check MIME type
    if (!ALLOWED_MIME_TYPES.includes(mimeType)) {
      throw new Error(`MIME type ${mimeType} not allowed`);
    }

    // Validate file signature (magic bytes)
    await this.validateFileSignature(buffer, ext);

    // Check for potential path traversal
    const normalizedName = path.normalize(fileName);
    if (normalizedName.includes('..') || path.isAbsolute(normalizedName)) {
      throw new Error('Invalid file name');
    }

    // Additional security checks
    if (fileName.length > 255) {
      throw new Error('File name too long');
    }

    if (!/^[a-zA-Z0-9\-_.]+$/.test(fileName)) {
      throw new Error('File name contains invalid characters');
    }
  }

  private async validateFileSignature(buffer: Buffer, extension: string): Promise<void> {
    if (buffer.length < 8) {
      throw new Error('File too small to validate');
    }

    const isGzipOrTar = extension === '.tgz' || extension === '.tar.gz';
    const isZip = extension === '.zip';

    if (isGzipOrTar) {
      // Check for gzip signature
      const gzipSig = FILE_SIGNATURES.gzip;
      const matches = gzipSig.every((byte, index) => buffer[index] === byte);
      if (!matches) {
        throw new Error('File content does not match expected format');
      }
    } else if (isZip) {
      // Check for zip signature
      const zipSig = FILE_SIGNATURES.zip;
      const matches = zipSig.every((byte, index) => buffer[index] === byte);
      if (!matches) {
        throw new Error('File content does not match expected format');
      }
    }
  }

  async saveFile(
    buffer: Buffer,
    originalName: string,
    mimeType: string,
    packageName: string,
    version: string
  ): Promise<UploadResult> {
    await this.validateFile(buffer, originalName, mimeType);

    // Generate secure filename
    const ext = path.extname(originalName);
    const timestamp = Date.now();
    const hash = crypto.randomBytes(8).toString('hex');
    const fileName = `${packageName}-${version}-${timestamp}-${hash}${ext}`;
    const filePath = path.join(this.uploadDir, fileName);

    // Ensure we're still within upload directory after path join
    const resolvedPath = path.resolve(filePath);
    const resolvedUploadDir = path.resolve(this.uploadDir);
    if (!resolvedPath.startsWith(resolvedUploadDir)) {
      throw new Error('Invalid file path');
    }

    try {
      // Write file with secure permissions
      await fs.writeFile(filePath, buffer, { mode: 0o600 });

      // Calculate SHA256 checksum
      const checksumSha256 = crypto.createHash('sha256').update(buffer).digest('hex');

      logger.info('File uploaded successfully', {
        fileName,
        fileSize: buffer.length,
        packageName,
        version,
        checksum: checksumSha256,
      });

      return {
        fileName,
        filePath: resolvedPath,
        fileSize: buffer.length,
        checksumSha256,
        mimeType,
      };
    } catch (error) {
      logger.error('Failed to save file', { error, fileName });
      throw new Error('Failed to save file');
    }
  }

  async deleteFile(filePath: string): Promise<void> {
    try {
      // Ensure file is within upload directory
      const resolvedPath = path.resolve(filePath);
      const resolvedUploadDir = path.resolve(this.uploadDir);
      if (!resolvedPath.startsWith(resolvedUploadDir)) {
        throw new Error('Invalid file path');
      }

      await fs.unlink(resolvedPath);
      logger.info('File deleted successfully', { filePath: resolvedPath });
    } catch (error) {
      logger.error('Failed to delete file', { error, filePath });
      throw error;
    }
  }

  async getFileStats(filePath: string) {
    try {
      const resolvedPath = path.resolve(filePath);
      const resolvedUploadDir = path.resolve(this.uploadDir);
      if (!resolvedPath.startsWith(resolvedUploadDir)) {
        throw new Error('Invalid file path');
      }

      return await fs.stat(resolvedPath);
    } catch (error) {
      logger.error('Failed to get file stats', { error, filePath });
      throw error;
    }
  }
}

export const fileUploadService = new FileUploadService();
