import { writable } from 'svelte/store';
import { usersApi, handleApiError, type User, type Package } from '../api';

interface UsersState {
  selectedUser: User | null;
  userPackages: Package[];
  userStats: { totalPackages: number; totalDownloads: number; totalTeams: number; } | null;
  loading: boolean;
  error: string | null;
}

const createUsersStore = () => {
  const { subscribe, set, update } = writable<UsersState>({
    selectedUser: null,
    userPackages: [],
    userStats: null,
    loading: false,
    error: null
  });

  return {
    subscribe,

    async fetchUserProfile(username: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const user = await usersApi.getUserProfile(username);
        
        update(state => ({
          ...state,
          selectedUser: user,
          loading: false
        }));
        
        return user;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage
        }));
        throw new Error(errorMessage);
      }
    },

    async fetchUserPackages(username: string, offset: number = 0, limit: number = 20) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const result = await usersApi.getUserPackages(username, offset, limit);
        
        update(state => ({
          ...state,
          userPackages: offset === 0 ? result.packages : [...state.userPackages, ...result.packages],
          loading: false
        }));
        
        return result;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage
        }));
        throw new Error(errorMessage);
      }
    },

    async fetchUserStats(username: string) {
      try {
        const stats = await usersApi.getUserStats(username);
        
        update(state => ({
          ...state,
          userStats: stats
        }));
        
        return stats;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update(state => ({
          ...state,
          error: errorMessage
        }));
        throw new Error(errorMessage);
      }
    },

    clearSelected() {
      update(state => ({ 
        ...state, 
        selectedUser: null, 
        userPackages: [], 
        userStats: null 
      }));
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    }
  };
};

export const usersStore = createUsersStore();