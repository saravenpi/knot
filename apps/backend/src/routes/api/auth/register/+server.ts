import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { RegisterData, ApiResponse, User, CreateUserData } from '#/types';
import { VALIDATION_PATTERNS, LIMITS } from '#/types';

// Mock user storage - In a real implementation, this would be stored in a database
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

function validateRegistrationData(data: RegisterData): { isValid: boolean; error?: string } {
  if (!data.username || !data.email || !data.password) {
    return { isValid: false, error: 'All fields are required' };
  }

  if (data.username.length < LIMITS.USERNAME_MIN || data.username.length > LIMITS.USERNAME_MAX) {
    return { isValid: false, error: `Username must be between ${LIMITS.USERNAME_MIN} and ${LIMITS.USERNAME_MAX} characters` };
  }

  if (!VALIDATION_PATTERNS.USERNAME.test(data.username)) {
    return { isValid: false, error: 'Username must start with a letter or number and contain only letters, numbers, hyphens, and underscores' };
  }

  if (data.email.length > LIMITS.EMAIL_MAX) {
    return { isValid: false, error: `Email must be less than ${LIMITS.EMAIL_MAX} characters` };
  }

  if (!VALIDATION_PATTERNS.EMAIL.test(data.email)) {
    return { isValid: false, error: 'Invalid email format' };
  }

  if (data.password.length < LIMITS.PASSWORD_MIN || data.password.length > LIMITS.PASSWORD_MAX) {
    return { isValid: false, error: `Password must be between ${LIMITS.PASSWORD_MIN} and ${LIMITS.PASSWORD_MAX} characters` };
  }

  if (!VALIDATION_PATTERNS.PASSWORD.test(data.password)) {
    return { isValid: false, error: 'Password must contain at least one lowercase letter, one uppercase letter, and one number' };
  }

  return { isValid: true };
}

export const POST: RequestHandler = async ({ request }) => {
  try {
    const registrationData = await request.json() as RegisterData;
    
    // Validate input data
    const validation = validateRegistrationData(registrationData);
    if (!validation.isValid) {
      return json({ 
        success: false, 
        message: validation.error 
      }, { status: 400 });
    }

    // Check if username already exists
    if (mockUsers.has(registrationData.username.toLowerCase())) {
      return json({ 
        success: false, 
        message: 'Username already exists' 
      }, { status: 409 });
    }

    // Check if email already exists
    const emailExists = Array.from(mockUsers.values()).some(
      user => user.email.toLowerCase() === registrationData.email.toLowerCase()
    );

    if (emailExists) {
      return json({ 
        success: false, 
        message: 'Email already exists' 
      }, { status: 409 });
    }

    // Create new user
    const newUser = {
      id: String(Date.now()), // Simple ID generation for demo
      username: registrationData.username,
      email: registrationData.email,
      password: registrationData.password, // In production, this should be hashed
      createdAt: new Date()
    };

    // Add to mock storage
    mockUsers.set(newUser.username.toLowerCase(), newUser);

    // Generate token
    const token = generateToken(newUser.username);

    // Return user data without password
    const userData: User = {
      id: newUser.id,
      username: newUser.username,
      email: newUser.email,
      createdAt: newUser.createdAt
    };

    const response: ApiResponse<User> = {
      success: true,
      data: {
        ...userData,
        token
      },
      token,
      message: 'Registration successful'
    };

    return json(response);

  } catch (error) {
    console.error('Registration error:', error);
    return json({ 
      success: false, 
      message: 'Internal server error' 
    }, { status: 500 });
  }
};