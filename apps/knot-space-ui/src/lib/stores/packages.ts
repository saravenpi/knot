import { writable } from 'svelte/store';
import { packagesApi, handleApiError, type Package } from '../api';

interface PackagesState {
  packages: Package[];
  loading: boolean;
  error: string | null;
  searchResults: Package[];
  selectedPackage: Package | null;
}

const createPackagesStore = () => {
  const { subscribe, set, update } = writable<PackagesState>({
    packages: [],
    loading: false,
    error: null,
    searchResults: [],
    selectedPackage: null
  });

  return {
    subscribe,

    async fetchAll() {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const packages = await packagesApi.getAll();
        
        update(state => ({
          ...state,
          packages,
          loading: false
        }));
        
        return packages;
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

    async fetchById(id: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const pkg = await packagesApi.getById(id);
        
        update(state => ({
          ...state,
          selectedPackage: pkg,
          loading: false
        }));
        
        return pkg;
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

    async fetchByName(name: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const pkg = await packagesApi.getByName(name);
        
        update(state => ({
          ...state,
          selectedPackage: pkg,
          loading: false
        }));
        
        return pkg;
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

    async search(query: string) {
      if (!query.trim()) {
        update(state => ({ ...state, searchResults: [] }));
        return [];
      }

      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const searchResults = await packagesApi.search(query);
        
        update(state => ({
          ...state,
          searchResults,
          loading: false
        }));
        
        return searchResults;
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

    async create(packageData: Partial<Package>) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const newPackage = await packagesApi.create(packageData);
        
        update(state => ({
          ...state,
          packages: [...state.packages, newPackage],
          loading: false
        }));
        
        return newPackage;
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

    async update(id: string, packageData: Partial<Package>) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const updatedPackage = await packagesApi.update(id, packageData);
        
        update(state => ({
          ...state,
          packages: state.packages.map(pkg => 
            pkg.id === id ? updatedPackage : pkg
          ),
          selectedPackage: state.selectedPackage?.id === id ? updatedPackage : state.selectedPackage,
          loading: false
        }));
        
        return updatedPackage;
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

    async delete(id: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await packagesApi.delete(id);
        
        update(state => ({
          ...state,
          packages: state.packages.filter(pkg => pkg.id !== id),
          selectedPackage: state.selectedPackage?.id === id ? null : state.selectedPackage,
          loading: false
        }));
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

    clearSearch() {
      update(state => ({ ...state, searchResults: [] }));
    },

    clearSelected() {
      update(state => ({ ...state, selectedPackage: null }));
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    }
  };
};

export const packagesStore = createPackagesStore();