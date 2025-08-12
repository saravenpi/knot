import { writable } from 'svelte/store';
import { authApi, handleApiError, type User } from '../api';

interface AuthState {
  user: User | null;
  loading: boolean;
  isAuthenticated: boolean;
}

const createAuthStore = () => {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    loading: false,
    isAuthenticated: false
  });

  return {
    subscribe,
    
    async login(username: string, password: string) {
      update(state => ({ ...state, loading: true }));
      
      try {
        const response = await authApi.login({ username, password });
        
        if (response.token) {
          localStorage.setItem('knot_token', response.token);
        }
        
        const user = response.data || response as any;
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true
        }));
        
        return response;
      } catch (error) {
        update(state => ({ ...state, loading: false }));
        throw new Error(handleApiError(error));
      }
    },

    async register(username: string, email: string, password: string) {
      update(state => ({ ...state, loading: true }));
      
      try {
        const response = await authApi.register({ username, email, password });
        
        if (response.token) {
          localStorage.setItem('knot_token', response.token);
        }
        
        const user = response.data || response as any;
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true
        }));
        
        return response;
      } catch (error) {
        update(state => ({ ...state, loading: false }));
        throw new Error(handleApiError(error));
      }
    },

    async getProfile() {
      const token = localStorage.getItem('knot_token');
      if (!token) {
        return;
      }

      update(state => ({ ...state, loading: true }));
      
      try {
        const response = await authApi.getProfile();
        const user = response.data || response as any;
        
        update(state => ({
          ...state,
          user,
          loading: false,
          isAuthenticated: true
        }));
        
        return user;
      } catch (error) {
        localStorage.removeItem('knot_token');
        update(state => ({
          user: null,
          loading: false,
          isAuthenticated: false
        }));
        throw new Error(handleApiError(error));
      }
    },

    async logout() {
      try {
        await authApi.logout();
      } catch (error) {
        console.error('Logout error:', error);
      } finally {
        localStorage.removeItem('knot_token');
        set({
          user: null,
          loading: false,
          isAuthenticated: false
        });
      }
    },

    initialize() {
      const token = localStorage.getItem('knot_token');
      if (token) {
        this.getProfile().catch(() => {
          // Silent fail on initialization
          localStorage.removeItem('knot_token');
        });
      }
    }
  };
};

export const authStore = createAuthStore();