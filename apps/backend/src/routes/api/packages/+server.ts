import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { Package } from '#/types';

// Mock packages data
const mockPackages: Package[] = [
  {
    id: '1',
    name: 'utils',
    version: '1.0.0',
    description: 'Common utility functions for development',
    downloadsCount: '1234',
    totalDownloadsCount: '5678',
    owner: {
      id: '1',
      username: 'admin',
      email: 'admin@knot.space',
      createdAt: '2024-01-01T00:00:00.000Z'
    },
    team: {
      id: '1',
      name: 'core'
    },
    tags: ['utility', 'helpers', 'common'],
    createdAt: '2024-01-15T10:30:00.000Z',
    updatedAt: '2024-01-15T10:30:00.000Z',
    publishedAt: '2024-01-15T10:30:00.000Z',
    fileSize: '45120'
  },
  {
    id: '2', 
    name: 'ui-components',
    version: '2.1.0',
    description: 'Reusable UI components library',
    downloadsCount: '2345',
    totalDownloadsCount: '8901',
    owner: {
      id: '2',
      username: 'demo',
      email: 'demo@knot.space',
      createdAt: '2024-01-15T00:00:00.000Z'
    },
    tags: ['ui', 'components', 'react'],
    createdAt: '2024-02-01T14:20:00.000Z',
    updatedAt: '2024-02-01T14:20:00.000Z',
    publishedAt: '2024-02-01T14:20:00.000Z',
    fileSize: '128560'
  },
  {
    id: '3',
    name: 'api-client',
    version: '0.5.2',
    description: 'HTTP client with built-in authentication and error handling',
    downloadsCount: '892',
    totalDownloadsCount: '1456',
    owner: {
      id: '3',
      username: 'test',
      email: 'test@knot.space',
      createdAt: '2024-02-01T00:00:00.000Z'
    },
    team: {
      id: '2',
      name: 'frontend'
    },
    tags: ['http', 'client', 'api'],
    createdAt: '2024-02-10T09:15:00.000Z',
    updatedAt: '2024-02-10T09:15:00.000Z',
    publishedAt: '2024-02-10T09:15:00.000Z',
    fileSize: '67890'
  }
];

export const GET: RequestHandler = async ({ url }) => {
  try {
    const searchParam = url.searchParams.get('search');
    const offsetParam = url.searchParams.get('offset');
    const limitParam = url.searchParams.get('limit');

    let filteredPackages = [...mockPackages];

    // Apply search filter
    if (searchParam) {
      const searchLower = searchParam.toLowerCase();
      filteredPackages = filteredPackages.filter(pkg => 
        pkg.name.toLowerCase().includes(searchLower) ||
        pkg.description?.toLowerCase().includes(searchLower) ||
        pkg.tags?.some(tag => tag.toLowerCase().includes(searchLower))
      );
    }

    // Apply pagination
    const offset = parseInt(offsetParam || '0', 10);
    const limit = parseInt(limitParam || '20', 10);
    
    const total = filteredPackages.length;
    const paginatedPackages = filteredPackages.slice(offset, offset + limit);

    const response = {
      success: true,
      data: {
        packages: paginatedPackages,
        pagination: {
          offset,
          limit,
          total,
          hasMore: offset + limit < total
        }
      }
    };

    return json(response);

  } catch (error) {
    console.error('Packages list error:', error);
    return json({ 
      success: false, 
      message: 'Internal server error' 
    }, { status: 500 });
  }
};