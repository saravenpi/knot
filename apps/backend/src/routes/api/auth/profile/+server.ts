import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { ApiResponse, User } from '#/types';

// Mock user data - In a real implementation, this would be stored in a database
const mockUsers = new Map([
  ['admin', { 
    id: '1', 
    username: 'admin', 
    email: 'admin@knot.space', 
    password: 'admin123',
    createdAt: new Date('2024-01-01')
  }],
  ['demo', { 
    id: '2', 
    username: 'demo', 
    email: 'demo@knot.space', 
    password: 'demo123',
    createdAt: new Date('2024-01-15')
  }],
  ['test', { 
    id: '3', 
    username: 'test', 
    email: 'test@knot.space', 
    password: 'test123',
    createdAt: new Date('2024-02-01')
  }]
]);

// Simple JWT-like token validation (for demo purposes)
function validateToken(token: string): { isValid: boolean; username?: string } {
  try {
    const payload = JSON.parse(atob(token));
    
    // Check if token is expired
    if (payload.exp && payload.exp < Date.now()) {
      return { isValid: false };
    }

    return { isValid: true, username: payload.username };
  } catch {
    return { isValid: false };
  }
}

export const GET: RequestHandler = async ({ request }) => {
  try {
    const authHeader = request.headers.get('Authorization');
    
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      return json({ 
        success: false, 
        message: 'No authorization token provided' 
      }, { status: 401 });
    }

    const token = authHeader.substring(7); // Remove 'Bearer ' prefix
    const tokenValidation = validateToken(token);

    if (!tokenValidation.isValid || !tokenValidation.username) {
      return json({ 
        success: false, 
        message: 'Invalid or expired token' 
      }, { status: 401 });
    }

    const user = mockUsers.get(tokenValidation.username.toLowerCase());
    
    if (!user) {
      return json({ 
        success: false, 
        message: 'User not found' 
      }, { status: 404 });
    }

    // Return user data without password
    const userData: User = {
      id: user.id,
      username: user.username,
      email: user.email,
      createdAt: user.createdAt
    };

    const response: ApiResponse<User> = {
      success: true,
      data: userData,
      user: userData,
      message: 'Profile retrieved successfully'
    };

    return json(response);

  } catch (error) {
    console.error('Profile retrieval error:', error);
    return json({ 
      success: false, 
      message: 'Internal server error' 
    }, { status: 500 });
  }
};