import { writable } from 'svelte/store';
import { authApi, handleApiError, type User } from '../api';
import { browser } from '$app/environment';

interface AuthState {
  user: User | null;
  loading: boolean;
  isAuthenticated: boolean;
  initialized: boolean;
}

// Helper to safely access localStorage
const getStoredToken = () => {
  if (!browser) return null;
  return localStorage.getItem('knot_token');
};

const setStoredToken = (token: string) => {
  if (!browser) return;
  localStorage.setItem('knot_token', token);
};

const removeStoredToken = () => {
  if (!browser) return;
  localStorage.removeItem('knot_token');
};

const createAuthStore = () => {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    loading: false,
    isAuthenticated: false,
    initialized: false
  });

  return {
    subscribe,
    
    async login(username: string, password: string) {
      update(state => ({ ...state, loading: true }));
      
      try {
        const response = await authApi.login({ username, password });
        
        // Handle both direct token and nested data structure
        const authData = response.data || response;
        const token = authData.token || response.token;
        const user = authData.user || response.user;
        
        if (token) {
          setStoredToken(token);
        }
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true,
          initialized: true
        }));
        
        return response;
      } catch (error) {
        update(state => ({ 
          ...state, 
          loading: false,
          isAuthenticated: false,
          user: null
        }));
        throw new Error(handleApiError(error));
      }
    },

    async register(username: string, email: string, password: string) {
      update(state => ({ ...state, loading: true }));
      
      try {
        const response = await authApi.register({ username, email, password });
        
        // Handle both direct token and nested data structure
        const authData = response.data || response;
        const token = authData.token || response.token;
        const user = authData.user || response.user;
        
        if (token) {
          setStoredToken(token);
        }
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true,
          initialized: true
        }));
        
        return response;
      } catch (error) {
        update(state => ({ 
          ...state, 
          loading: false,
          isAuthenticated: false,
          user: null
        }));
        throw new Error(handleApiError(error));
      }
    },

    async getProfile() {
      const token = getStoredToken();
      if (!token) {
        update(state => ({ 
          ...state, 
          initialized: true, 
          isAuthenticated: false,
          user: null,
          loading: false
        }));
        return;
      }

      update(state => ({ ...state, loading: true }));
      
      try {
        // Add timeout to prevent hanging
        const timeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error('Request timeout')), 3000); // 3 second timeout
        });
        
        const response = await Promise.race([
          authApi.getProfile(),
          timeoutPromise
        ]) as any;
        
        // Handle both direct token and nested data structure
        const authData = response.data || response;
        const newToken = authData.token || response.token;
        const user = authData.user || response.user;
        
        if (newToken) {
          setStoredToken(newToken);
        }
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true,
          initialized: true
        }));
        
        return user;
      } catch (error) {
        console.warn('Profile fetch failed:', error);
        removeStoredToken();
        update(state => ({
          user: null,
          loading: false,
          isAuthenticated: false,
          initialized: true
        }));
        // Don't throw error to prevent blocking initialization
        return null;
      }
    },

    async logout() {
      try {
        await authApi.logout();
      } catch (error) {
        console.error('Logout error:', error);
      } finally {
        removeStoredToken();
        set({
          user: null,
          loading: false,
          isAuthenticated: false,
          initialized: true
        });
      }
    },

    async initialize() {
      // Don't reinitialize if already done
      let currentState: AuthState;
      const unsubscribe = subscribe(state => { currentState = state; })();
      
      if (currentState!.initialized) {
        return;
      }
      
      const token = getStoredToken();
      if (token) {
        // getProfile now handles its own errors and won't throw
        await this.getProfile();
      } else {
        update(state => ({ 
          ...state, 
          initialized: true,
          isAuthenticated: false,
          user: null,
          loading: false
        }));
      }
    },

    // Method to manually refresh authentication state
    async refresh() {
      const token = getStoredToken();
      if (token) {
        try {
          await this.getProfile();
        } catch (error) {
          console.warn('Token refresh failed, user may need to re-login');
          this.logout();
        }
      }
    }
  };
};

export const authStore = createAuthStore();