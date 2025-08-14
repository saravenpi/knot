import { prisma } from '../../lib/prisma';

export interface UserPackageFilters {
  limit: number;
  offset: number;
}

class UsersService {
  async getUserProfile(username: string) {
    const user = await prisma.user.findUnique({
      where: { username },
      select: {
        id: true,
        username: true,
        email: false, // Don't expose email in public profiles
        createdAt: true,
        _count: {
          select: {
            ownedPackages: true,
          }
        }
      }
    });

    if (!user) {
      throw new Error('User not found');
    }

    return user;
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

    const packages = await prisma.package.findMany({
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
      skip: filters.offset,
      take: filters.limit,
    });

    const total = await prisma.package.count({ 
      where: { 
        owner: {
          username: username
        }
      }
    });

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

  async getUserStats(username: string) {
    // First verify user exists
    const user = await prisma.user.findUnique({
      where: { username },
      select: { id: true }
    });

    if (!user) {
      throw new Error('User not found');
    }

    // Get total packages count
    const totalPackages = await prisma.package.count({
      where: { 
        owner: {
          username: username
        }
      }
    });

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