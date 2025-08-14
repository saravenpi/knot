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
    // If team name is provided, find the team and check permissions
    let teamId: string | null = null;
    if (data.teamName) {
      const team = await prisma.team.findUnique({
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
    const existingPackage = await prisma.package.findUnique({
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
    };

    const pkg = await prisma.package.create({
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
      await prisma.packageTag.createMany({
        data: data.tags.map(tag => ({
          packageId: pkg.id,
          tag: tag.toLowerCase(),
        }))
      });
    }

    return pkg;
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

    return {
      packages,
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

    return versions;
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

    return pkg;
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
    await prisma.$transaction([
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
  }

  async getDownloadStats(packageName: string, version: string, days: number = 7) {
    const startDate = new Date();
    startDate.setDate(startDate.getDate() - days);
    
    // Get downloads grouped by date for the past N days
    const downloads = await prisma.packageDownload.groupBy({
      by: ['downloadAt'],
      where: {
        packageName,
        version,
        downloadAt: {
          gte: startDate
        }
      },
      _count: {
        id: true
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
      statsMap.set(dateKey, download._count.id);
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
}

export const packagesService = new PackagesService();