import BACKEND_URL from "./apiUrl";
import type {
  ApiResponse,
  LoginCredentials,
  RegisterData,
  User,
  Package,
  Team,
} from "#/types";

// Re-export shared types for convenience
export type {
  ApiResponse,
  LoginCredentials,
  RegisterData,
  User,
  Package,
  Team,
} from "#/types";

export const requestApi = async <T>(
  method: string,
  endpoint: string,
  body?: any,
): Promise<T> => {
  const token =
    typeof localStorage !== "undefined"
      ? localStorage.getItem("knot_token")
      : null;

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
    if (typeof localStorage !== "undefined") {
      localStorage.removeItem("knot_token");
    }
    // Don't call authStore.logout() here to avoid circular imports
    // The auth store will handle this when it detects an invalid token
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
    return requestApi<ApiResponse<User>>(
      "POST",
      "/api/auth/login",
      credentials,
    );
  },

  async register(data: RegisterData): Promise<ApiResponse<User>> {
    return requestApi<ApiResponse<User>>("POST", "/api/auth/register", data);
  },

  async getProfile(): Promise<ApiResponse<User>> {
    return requestApi<ApiResponse<User>>("GET", "/api/auth/profile");
  },

  async logout(): Promise<void> {
    // Since there's no backend logout endpoint, just clear the token
    if (typeof localStorage !== "undefined") {
      localStorage.removeItem("knot_token");
    }
  },
};

// Packages API
export const packagesApi = {
  async getAll(): Promise<Package[]> {
    const response = await requestApi<{
      success: boolean;
      data: { packages: Package[]; pagination: any };
    }>("GET", "/api/packages");
    return response.data.packages;
  },

  async getById(id: string): Promise<Package> {
    // Note: Backend doesn't have this endpoint, this method should not be used
    throw new Error("getById not supported - use getByName instead");
  },

  async getByName(name: string): Promise<Package> {
    // Backend expects name and version, but we only have name
    // This will need to be handled differently - get latest version or specific version
    throw new Error(
      "getByName requires version - use getByNameAndVersion instead",
    );
  },

  async getByNameAndVersion(name: string, version: string): Promise<Package> {
    const response = await requestApi<{ success: boolean; data: Package }>(
      "GET",
      `/api/packages/${encodeURIComponent(name)}/${encodeURIComponent(version)}`,
    );
    return response.data;
  },

  async getVersions(name: string): Promise<Package[]> {
    const response = await requestApi<{ success: boolean; data: Package[] }>(
      "GET",
      `/api/packages/${encodeURIComponent(name)}/versions`,
    );
    return response.data;
  },

  async create(packageData: Partial<Package>): Promise<Package> {
    return requestApi<Package>("POST", "/api/packages", packageData);
  },

  async update(id: string, packageData: Partial<Package>): Promise<Package> {
    // Backend doesn't have this endpoint
    throw new Error("Package updates not supported via API");
  },

  async delete(name: string, version: string): Promise<void> {
    return requestApi(
      "DELETE",
      `/api/packages/${encodeURIComponent(name)}/${encodeURIComponent(version)}`,
    );
  },

  async search(query: string): Promise<Package[]> {
    // Backend doesn't have search endpoint, use list with search parameter
    const response = await requestApi<{
      success: boolean;
      data: { packages: Package[] };
    }>("GET", `/api/packages?search=${encodeURIComponent(query)}`);
    return response.data.packages;
  },

  async getGlobalStats(): Promise<{
    totalPackages: number;
    totalDownloads: number;
    totalUsers: number;
    totalTeams: number;
  }> {
    const response = await requestApi<{
      success: boolean;
      data: {
        totalPackages: number;
        totalDownloads: number;
        totalUsers: number;
        totalTeams: number;
      };
    }>("GET", "/api/packages/stats");
    return response.data;
  },
};

// Teams API
export const teamsApi = {
  async getAll(): Promise<Team[]> {
    const response = await requestApi<ApiResponse<Team[]>>("GET", "/api/teams");
    return response.data || [];
  },

  async getById(id: string): Promise<Team> {
    const response = await requestApi<ApiResponse<Team>>(
      "GET",
      `/api/teams/${id}`,
    );
    return response.data!;
  },

  async create(teamData: Partial<Team>): Promise<Team> {
    const response = await requestApi<ApiResponse<Team>>(
      "POST",
      "/api/teams",
      teamData,
    );
    return response.data!;
  },

  async update(id: string, teamData: Partial<Team>): Promise<Team> {
    const response = await requestApi<ApiResponse<Team>>(
      "PUT",
      `/api/teams/${id}`,
      teamData,
    );
    return response.data!;
  },

  async delete(id: string): Promise<void> {
    return requestApi("DELETE", `/api/teams/${id}`);
  },

  async addMember(
    id: string,
    username: string,
    role: string = "member",
  ): Promise<void> {
    return requestApi("POST", `/api/teams/${id}/members`, { username, role });
  },

  async removeMember(id: string, userId: string): Promise<void> {
    return requestApi("DELETE", `/api/teams/${id}/members/${userId}`);
  },

  async updateMemberRole(
    id: string,
    userId: string,
    role: string,
  ): Promise<void> {
    return requestApi("PUT", `/api/teams/${id}/members/${userId}`, { role });
  },
};

// Users API
export const usersApi = {
  async getAllUsers(): Promise<User[]> {
    const response = await requestApi<{ success: boolean; data: User[] }>(
      "GET",
      "/api/users",
    );
    return response.data;
  },

  async getUserProfile(username: string): Promise<User> {
    const response = await requestApi<{ success: boolean; data: User }>(
      "GET",
      `/api/users/${encodeURIComponent(username)}`,
    );
    return response.data;
  },

  async getUserPackages(
    username: string,
    offset: number = 0,
    limit: number = 20,
  ): Promise<{ packages: Package[]; pagination: any }> {
    const response = await requestApi<{
      success: boolean;
      data: { packages: Package[]; pagination: any };
    }>(
      "GET",
      `/api/users/${encodeURIComponent(username)}/packages?offset=${offset}&limit=${limit}`,
    );
    return response.data;
  },

  async getUserStats(username: string): Promise<{
    totalPackages: number;
    totalDownloads: number;
    totalTeams: number;
  }> {
    const response = await requestApi<{
      success: boolean;
      data: {
        totalPackages: number;
        totalDownloads: number;
        totalTeams: number;
      };
    }>("GET", `/api/users/${encodeURIComponent(username)}/stats`);
    return response.data;
  },
};

// Helper function to handle errors consistently
export const handleApiError = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  return "An unexpected error occurred";
};
