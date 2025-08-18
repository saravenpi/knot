import { prisma } from '../../lib/prisma';
import { PublishPackageRequest } from '../../types';
import * as crypto from 'crypto';

export interface PackageFilters {
  search?: string;
  owner?: string;
  team?: string;
  tags?: string[];
  limit: number;
  offset: number;
}

class PackagesService {
  async publishPackage(data: PublishPackageRequest, ownerId: string) {
    return await prisma.$transaction(async (tx) => {
      // If team name is provided, find the team and check permissions
      let teamId: string | null = null;
      if (data.teamName) {
        const team = await tx.team.findUnique({
          where: { name: data.teamName },
          include: {
            members: {
              where: { userId: ownerId }
            }
          }
        });

        if (!team) {
          throw new Error('Team not found');
        }

        const member = team.members[0];
        if (!member || (member.role !== 'owner' && member.role !== 'admin')) {
          throw new Error('Insufficient permissions to publish to this team');
        }

        teamId = team.id;
      }

      // Check if package version already exists
      const existingPackage = await tx.package.findUnique({
        where: {
          name_version: {
            name: data.name,
            version: data.version,
          }
        }
      });

      if (existingPackage) {
        throw new Error('Package version already exists');
      }

      // For now, we'll create a placeholder package without file handling
      const packageData = {
        name: data.name,
        version: data.version,
        description: data.description,
        ownerId,
        teamId,
        downloadUrl: `https://example.com/packages/${data.name}/${data.version}`, // Placeholder
        filePath: `/uploads/${data.name}-${data.version}.tgz`, // Placeholder
        fileSize: BigInt(0), // Placeholder - should be set when file is uploaded
        checksumSha256: '0000000000000000000000000000000000000000000000000000000000000000', // Placeholder
      };

      const pkg = await tx.package.create({
        data: packageData,
        include: {
          owner: {
            select: {
              id: true,
              username: true,
              email: true,
              createdAt: true,
            }
          },
          team: true,
          tags: true,
        }
      });

      // Add tags if provided
      if (data.tags && data.tags.length > 0) {
        await tx.packageTag.createMany({
          data: data.tags.map(tag => ({
            packageId: pkg.id,
            tag: tag.toLowerCase(),
          }))
        });
      }

      // Convert BigInt fields to strings for JSON serialization
      return {
        ...pkg,
        fileSize: pkg.fileSize.toString(),
        downloadsCount: pkg.downloadsCount.toString(),
      };
    });
  }

  async listPackages(filters: PackageFilters) {
    const whereClause: any = {};

    if (filters.search) {
      whereClause.OR = [
        { name: { contains: filters.search, mode: 'insensitive' } },
        { description: { contains: filters.search, mode: 'insensitive' } },
      ];
    }

    if (filters.owner) {
      whereClause.owner = {
        username: filters.owner
      };
    }

    if (filters.team) {
      whereClause.team = {
        name: filters.team
      };
    }

    if (filters.tags && filters.tags.length > 0) {
      whereClause.tags = {
        some: {
          tag: {
            in: filters.tags
          }
        }
      };
    }

    // First, get all packages with their names
    const allPackages = await prisma.package.findMany({
      where: whereClause,
      include: {
        owner: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        },
        team: true,
        tags: true,
        _count: {
          select: {
            tags: true,
          }
        }
      },
      orderBy: [
        { publishedAt: 'desc' }
      ],
    });

    // Group by package name and get only the latest version of each, but calculate total downloads
    const latestPackagesMap = new Map<string, any>();
    const packageDownloadsMap = new Map<string, bigint>();
    
    // First pass: calculate total downloads per package name
    for (const pkg of allPackages) {
      const currentTotal = packageDownloadsMap.get(pkg.name) || BigInt(0);
      packageDownloadsMap.set(pkg.name, currentTotal + pkg.downloadsCount);
    }
    
    // Second pass: get latest version and set total downloads
    for (const pkg of allPackages) {
      const existing = latestPackagesMap.get(pkg.name);
      if (!existing || new Date(pkg.publishedAt) > new Date(existing.publishedAt)) {
        latestPackagesMap.set(pkg.name, {
          ...pkg,
          totalDownloadsCount: packageDownloadsMap.get(pkg.name) || BigInt(0)
        });
      }
    }

    // Convert map to array, sort by latest published date, and apply pagination
    const latestPackages = Array.from(latestPackagesMap.values())
      .sort((a, b) => new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime());
    const packages = latestPackages.slice(filters.offset, filters.offset + filters.limit);

    const total = latestPackages.length; // Total unique packages, not total versions

    // Convert BigInt fields to strings and transform tags for JSON serialization
    const serializedPackages = packages.map(pkg => ({
      ...pkg,
      fileSize: pkg.fileSize.toString(),
      downloadsCount: pkg.downloadsCount.toString(),
      totalDownloadsCount: pkg.totalDownloadsCount.toString(),
      tags: pkg.tags.map(tag => tag.tag), // Transform from {tag: string}[] to string[]
    }));

    return {
      packages: serializedPackages,
      pagination: {
        total,
        limit: filters.limit,
        offset: filters.offset,
        hasMore: filters.offset + filters.limit < total,
      }
    };
  }

  async getPackageVersions(name: string) {
    const versions = await prisma.package.findMany({
      where: { name },
      select: {
        id: true,
        version: true,
        description: true,
        publishedAt: true,
        downloadsCount: true,
      },
      orderBy: { publishedAt: 'desc' }
    });

    if (versions.length === 0) {
      throw new Error('Package not found');
    }

    // Convert BigInt fields to strings for JSON serialization
    return versions.map(version => ({
      ...version,
      downloadsCount: version.downloadsCount.toString(),
    }));
  }

  async getPackage(name: string, version: string) {
    const pkg = await prisma.package.findUnique({
      where: {
        name_version: { name, version }
      },
      include: {
        owner: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        },
        team: true,
        tags: true,
      }
    });

    if (!pkg) {
      throw new Error('Package not found');
    }

    // Calculate total downloads across all versions of this package
    const allVersions = await prisma.package.findMany({
      where: { name },
      select: { downloadsCount: true }
    });

    const totalDownloadsCount = allVersions.reduce((sum, version) => sum + version.downloadsCount, BigInt(0));

    // Convert BigInt fields to strings and transform tags for JSON serialization
    return {
      ...pkg,
      fileSize: pkg.fileSize.toString(),
      downloadsCount: pkg.downloadsCount.toString(),
      totalDownloadsCount: totalDownloadsCount.toString(),
      tags: pkg.tags.map(tag => tag.tag), // Transform from {tag: string}[] to string[]
    };
  }

  async downloadPackage(name: string, version: string) {
    const pkg = await prisma.package.findUnique({
      where: {
        name_version: { name, version }
      },
      select: {
        downloadUrl: true,
        filePath: true,
      }
    });

    if (!pkg) {
      throw new Error('Package not found');
    }

    return pkg;
  }

  async incrementDownloadCount(name: string, version: string, ipAddress?: string, userAgent?: string) {
    try {
      // Find the package to get its ID
      const pkg = await prisma.package.findUnique({
        where: {
          name_version: { name, version }
        },
        select: {
          id: true,
          name: true,
          version: true
        }
      });

      if (!pkg) {
        throw new Error('Package not found');
      }

      // Hash the IP address for privacy (if provided)
      const ipHash = ipAddress ? crypto.createHash('sha256').update(ipAddress).digest('hex') : null;

      // Use a transaction to ensure consistency
      const result = await prisma.$transaction([
        // Increment the total download count
        prisma.package.update({
          where: {
            name_version: { name, version }
          },
          data: {
            downloadsCount: {
              increment: 1
            }
          }
        }),
        // Record the individual download event for analytics
        prisma.packageDownload.create({
          data: {
            packageId: pkg.id,
            packageName: pkg.name,
            version: pkg.version,
            ipHash,
            userAgent: userAgent ? userAgent.substring(0, 500) : null, // Truncate to fit DB constraint
          }
        })
      ]);

      console.log(`Download count incremented for ${name}@${version}. New count: ${result[0].downloadsCount}`);
      return result;
    } catch (error) {
      console.error('Error incrementing download count:', error);
      throw error;
    }
  }

  async getDownloadStats(packageName: string, version: string, days: number = 7) {
    const startDate = new Date();
    startDate.setDate(startDate.getDate() - days);
    
    // Get downloads for the package in the date range
    const downloads = await prisma.packageDownload.findMany({
      where: {
        packageName,
        version,
        downloadAt: {
          gte: startDate
        }
      },
      select: {
        downloadAt: true
      },
      orderBy: {
        downloadAt: 'asc'
      }
    });

    // Create a complete date range and fill in missing days with 0
    const dateRange = [];
    for (let i = days - 1; i >= 0; i--) {
      const date = new Date();
      date.setDate(date.getDate() - i);
      date.setHours(0, 0, 0, 0); // Normalize to start of day
      dateRange.push(date);
    }

    // Map downloads to dates, filling missing days with 0
    const statsMap = new Map();
    downloads.forEach(download => {
      const date = new Date(download.downloadAt);
      date.setHours(0, 0, 0, 0); // Normalize to start of day
      const dateKey = date.toISOString().split('T')[0];
      const currentCount = statsMap.get(dateKey) || 0;
      statsMap.set(dateKey, currentCount + 1);
    });

    const dailyStats = dateRange.map(date => {
      const dateKey = date.toISOString().split('T')[0];
      return {
        date: dateKey,
        downloads: statsMap.get(dateKey) || 0
      };
    });

    return {
      packageName,
      version,
      totalDays: days,
      dailyStats,
      totalDownloads: dailyStats.reduce((sum, day) => sum + day.downloads, 0)
    };
  }

  async deletePackage(name: string, version: string, userId: string) {
    const pkg = await prisma.package.findUnique({
      where: {
        name_version: { name, version }
      },
      include: {
        team: {
          include: {
            members: {
              where: { userId }
            }
          }
        }
      }
    });

    if (!pkg) {
      throw new Error('Package not found');
    }

    // Check permissions
    const isOwner = pkg.ownerId === userId;
    const isTeamAdmin = pkg.team && pkg.team.members[0] && 
      (pkg.team.members[0].role === 'owner' || pkg.team.members[0].role === 'admin');

    if (!isOwner && !isTeamAdmin) {
      throw new Error('Insufficient permissions to delete this package');
    }

    await prisma.package.delete({
      where: {
        name_version: { name, version }
      }
    });
  }

  async handleFileUpload(file: File, userId: string) {
    // Extract file information
    const arrayBuffer = await file.arrayBuffer();
    const fileContent = new Uint8Array(arrayBuffer);
    const fileSize = BigInt(fileContent.length);
    
    // Calculate SHA256 checksum
    const hashBuffer = await crypto.subtle.digest('SHA-256', fileContent);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const checksum = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

    // Create uploads directory if it doesn't exist
    const fs = require('fs');
    const path = require('path');
    const uploadsDir = path.join(process.cwd(), 'uploads');
    if (!fs.existsSync(uploadsDir)) {
      fs.mkdirSync(uploadsDir, { recursive: true });
    }

    // Generate unique filename using checksum
    const filename = `${checksum}.tar.gz`;
    const filePath = path.join(uploadsDir, filename);

    // Save file to disk
    fs.writeFileSync(filePath, fileContent);

    console.log(`File uploaded and saved: ${filePath}`);

    return {
      fileSize: fileSize.toString(),
      checksum,
      originalName: file.name,
      filePath: `/uploads/${filename}`,
      downloadUrl: `/uploads/${filename}`,
      message: 'File uploaded and stored successfully'
    };
  }

  async updatePackageFileInfo(packageName: string, version: string, userId: string, fileInfo: any) {
    // Find the package
    const pkg = await prisma.package.findUnique({
      where: {
        name_version: { name: packageName, version }
      },
      include: {
        owner: true
      }
    });

    if (!pkg) {
      throw new Error('Package not found');
    }

    // Check if user owns the package
    if (pkg.ownerId !== userId) {
      throw new Error('Unauthorized: You can only update your own packages');
    }

    // Update package with real file information
    const baseUrl = process.env.KNOT_SPACE_URL || 'https://knot-space-production.up.railway.app';
    await prisma.package.update({
      where: {
        name_version: { name: packageName, version }
      },
      data: {
        fileSize: BigInt(parseInt(fileInfo.fileSize)),
        checksumSha256: fileInfo.checksum,
        filePath: fileInfo.filePath,
        downloadUrl: `${baseUrl}${fileInfo.downloadUrl}`,
      }
    });

    console.log(`Updated package ${packageName}@${version} with real file information`);
  }

  async getPackageByChecksum(checksum: string) {
    const pkg = await prisma.package.findFirst({
      where: {
        checksumSha256: checksum
      },
      select: {
        name: true,
        version: true
      }
    });

    return pkg;
  }

  async getGlobalStats() {
    // Get total unique packages count (distinct by name)
    const uniquePackagesResult = await prisma.package.groupBy({
      by: ['name'],
      _count: {
        name: true
      }
    });
    const totalPackages = uniquePackagesResult.length;

    // Get total downloads count
    const downloadCountResult = await prisma.package.aggregate({
      _sum: {
        downloadsCount: true
      }
    });

    // Get total users count
    const totalUsers = await prisma.user.count();

    // Get total teams count
    const totalTeams = await prisma.team.count();

    return {
      totalPackages,
      totalDownloads: Number(downloadCountResult._sum.downloadsCount || 0),
      totalUsers,
      totalTeams
    };
  }
}

export const packagesService = new PackagesService();