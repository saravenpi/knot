import { prisma } from '../../lib/prisma';

export interface UserPackageFilters {
  limit: number;
  offset: number;
}

class UsersService {
  async getAllUsers() {
    const users = await prisma.user.findMany({
      select: {
        id: true,
        username: true,
        email: false, // Don't expose email in public listings
        createdAt: true,
        ownedPackages: {
          select: {
            name: true
          },
          distinct: ['name']
        }
      },
      orderBy: [
        { createdAt: 'desc' }
      ]
    });

    // Transform the result to include unique package count
    return users.map(user => ({
      ...user,
      _count: {
        ownedPackages: user.ownedPackages.length
      },
      ownedPackages: undefined // Remove the packages array from the response
    }));
  }

  async getUserProfile(username: string) {
    const user = await prisma.user.findUnique({
      where: { username },
      select: {
        id: true,
        username: true,
        email: false, // Don't expose email in public profiles
        createdAt: true,
        ownedPackages: {
          select: {
            name: true
          },
          distinct: ['name']
        }
      }
    });

    if (!user) {
      throw new Error('User not found');
    }

    // Transform the result to include unique package count
    return {
      ...user,
      _count: {
        ownedPackages: user.ownedPackages.length
      },
      ownedPackages: undefined // Remove the packages array from the response
    };
  }

  async getUserPackages(username: string, filters: UserPackageFilters) {
    // First verify user exists
    const user = await prisma.user.findUnique({
      where: { username },
      select: { id: true }
    });

    if (!user) {
      throw new Error('User not found');
    }

    // Get all packages by this user first
    const allPackages = await prisma.package.findMany({
      where: { 
        owner: {
          username: username
        }
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

    // Group by package name and get only the latest version of each
    const latestPackagesMap = new Map<string, any>();
    
    for (const pkg of allPackages) {
      const existing = latestPackagesMap.get(pkg.name);
      if (!existing || new Date(pkg.publishedAt) > new Date(existing.publishedAt)) {
        latestPackagesMap.set(pkg.name, pkg);
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

  async getUserStats(username: string) {
    // First verify user exists
    const user = await prisma.user.findUnique({
      where: { username },
      select: { id: true }
    });

    if (!user) {
      throw new Error('User not found');
    }

    // Get total unique packages count (distinct by name)
    const uniquePackagesResult = await prisma.package.groupBy({
      by: ['name'],
      where: { 
        owner: {
          username: username
        }
      },
      _count: {
        name: true
      }
    });
    const totalPackages = uniquePackagesResult.length;

    // Get total downloads count
    const downloadCountResult = await prisma.package.aggregate({
      where: { 
        owner: {
          username: username
        }
      },
      _sum: {
        downloadsCount: true
      }
    });

    // Get total teams count
    const totalTeams = await prisma.teamMember.count({
      where: { 
        user: {
          username: username
        }
      }
    });

    return {
      totalPackages,
      totalDownloads: Number(downloadCountResult._sum.downloadsCount || 0),
      totalTeams
    };
  }
}

export const usersService = new UsersService();