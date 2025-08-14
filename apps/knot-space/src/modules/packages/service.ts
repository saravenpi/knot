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

    const packages = await prisma.package.findMany({
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
      skip: filters.offset,
      take: filters.limit,
    });

    const total = await prisma.package.count({ where: whereClause });

    // Convert BigInt fields to strings for JSON serialization
    const serializedPackages = packages.map(pkg => ({
      ...pkg,
      fileSize: pkg.fileSize.toString(),
      downloadsCount: pkg.downloadsCount.toString(),
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

    // Convert BigInt fields to strings for JSON serialization
    return {
      ...pkg,
      fileSize: pkg.fileSize.toString(),
      downloadsCount: pkg.downloadsCount.toString(),
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

    // For now, just return the file metadata
    // In production, you would save the file to storage and return the path/URL
    return {
      fileSize: fileSize.toString(),
      checksum,
      originalName: file.name,
      // In production, you would update the package record with actual file info
      message: 'File processed successfully (not permanently stored in demo)'
    };
  }

  async getGlobalStats() {
    // Get total packages count
    const totalPackages = await prisma.package.count();

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