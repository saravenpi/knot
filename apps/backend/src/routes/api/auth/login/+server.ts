import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { LoginCredentials, ApiResponse, User } from '#/types';

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

// Simple JWT-like token generation (for demo purposes)
function generateToken(username: string): string {
  const payload = {
    username,
    iat: Date.now(),
    exp: Date.now() + (7 * 24 * 60 * 60 * 1000) // 7 days
  };
  return btoa(JSON.stringify(payload));
}

export const POST: RequestHandler = async ({ request }) => {
  try {
    const credentials = await request.json() as LoginCredentials;
    
    if (!credentials.username || !credentials.password) {
      return json({ 
        success: false, 
        message: 'Username and password are required' 
      }, { status: 400 });
    }

    const user = mockUsers.get(credentials.username.toLowerCase());
    
    if (!user || user.password !== credentials.password) {
      return json({ 
        success: false, 
        message: 'Invalid username or password' 
      }, { status: 401 });
    }

    // Generate token
    const token = generateToken(user.username);

    // Return user data without password
    const userData: User = {
      id: user.id,
      username: user.username,
      email: user.email,
      createdAt: user.createdAt
    };

    const response: ApiResponse<User> = {
      success: true,
      data: {
        ...userData,
        token
      },
      token,
      message: 'Login successful'
    };

    return json(response);

  } catch (error) {
    console.error('Login error:', error);
    return json({ 
      success: false, 
      message: 'Internal server error' 
    }, { status: 500 });
  }
};