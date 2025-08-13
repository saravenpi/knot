import BACKEND_URL from "./apiUrl";
import type {
  ApiResponse,
  LoginCredentials,
  RegisterData,
  User,
  Package,
  Team
} from '#/types';

// Re-export shared types for convenience
export type {
  ApiResponse,
  LoginCredentials,
  RegisterData,
  User,
  Package,
  Team
} from '#/types';

export const requestApi = async <T>(
  method: string,
  endpoint: string,
  body?: any,
): Promise<T> => {
  const token = typeof localStorage !== 'undefined' ? localStorage.getItem('knot_token') : null;
  
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  const response = await fetch(`${BACKEND_URL}${endpoint}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
    credentials: "include",
  });

  if (response.status === 401) {
    // Clear token immediately
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem('knot_token');
    }
    // Import authStore dynamically to avoid circular imports
    import('./stores/auth').then(({ authStore }) => {
      authStore.logout();
    }).catch(console.error);
    throw new Error("Session expired. Please log in again.");
  }

  if (!response.ok) {
    const errorText = await response.text();
    let errorMessage = `Failed to fetch: ${response.status}`;
    try {
      const errorData = JSON.parse(errorText);
      errorMessage = errorData.message || errorMessage;
    } catch {
      errorMessage = errorText || errorMessage;
    }
    throw new Error(errorMessage);
  }

  const json = (await response.json()) as T;
  return json;
};

// Auth API
export const authApi = {
  async login(credentials: LoginCredentials): Promise<ApiResponse<User>> {
    return requestApi<ApiResponse<User>>('POST', '/api/auth/login', credentials);
  },

  async register(data: RegisterData): Promise<ApiResponse<User>> {
    return requestApi<ApiResponse<User>>('POST', '/api/auth/register', data);
  },

  async getProfile(): Promise<ApiResponse<User>> {
    return requestApi<ApiResponse<User>>('GET', '/api/auth/profile');
  },

  async logout(): Promise<void> {
    // Since there's no backend logout endpoint, just clear the token
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem('knot_token');
    }
  }
};

// Packages API
export const packagesApi = {
  async getAll(): Promise<Package[]> {
    return requestApi<Package[]>('GET', '/api/packages');
  },

  async getById(id: string): Promise<Package> {
    return requestApi<Package>('GET', `/api/packages/${id}`);
  },

  async getByName(name: string): Promise<Package> {
    return requestApi<Package>('GET', `/api/packages/name/${name}`);
  },

  async create(packageData: Partial<Package>): Promise<Package> {
    return requestApi<Package>('POST', '/api/packages', packageData);
  },

  async update(id: string, packageData: Partial<Package>): Promise<Package> {
    return requestApi<Package>('PUT', `/api/packages/${id}`, packageData);
  },

  async delete(id: string): Promise<void> {
    return requestApi('DELETE', `/api/packages/${id}`);
  },

  async search(query: string): Promise<Package[]> {
    return requestApi<Package[]>('GET', `/api/packages/search?q=${encodeURIComponent(query)}`);
  }
};

// Teams API
export const teamsApi = {
  async getAll(): Promise<Team[]> {
    return requestApi<Team[]>('GET', '/api/teams');
  },

  async getById(id: string): Promise<Team> {
    return requestApi<Team>('GET', `/api/teams/${id}`);
  },

  async create(teamData: Partial<Team>): Promise<Team> {
    return requestApi<Team>('POST', '/api/teams', teamData);
  },

  async update(id: string, teamData: Partial<Team>): Promise<Team> {
    return requestApi<Team>('PUT', `/api/teams/${id}`, teamData);
  },

  async delete(id: string): Promise<void> {
    return requestApi('DELETE', `/api/teams/${id}`);
  },

  async addMember(id: string, username: string, role: string = 'member'): Promise<void> {
    return requestApi('POST', `/api/teams/${id}/members`, { username, role });
  },

  async removeMember(id: string, userId: string): Promise<void> {
    return requestApi('DELETE', `/api/teams/${id}/members/${userId}`);
  },

  async updateMemberRole(id: string, userId: string, role: string): Promise<void> {
    return requestApi('PUT', `/api/teams/${id}/members/${userId}`, { role });
  }
};

// Helper function to handle errors consistently
export const handleApiError = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  return 'An unexpected error occurred';
};
