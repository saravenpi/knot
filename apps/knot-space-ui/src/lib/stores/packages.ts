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

    async fetchByName(name: string, version?: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        let pkg: Package;
        
        if (version) {
          // Get specific version
          pkg = await packagesApi.getByNameAndVersion(decodeURIComponent(name), version);
        } else {
          // Get latest version by getting all versions and picking the first one
          const versions = await packagesApi.getVersions(decodeURIComponent(name));
          if (versions.length === 0) {
            throw new Error('Package not found');
          }
          // Sort versions by publish date (newest first) and pick the first one
          versions.sort((a, b) => new Date(b.publishedAt || b.createdAt).getTime() - new Date(a.publishedAt || a.createdAt).getTime());
          pkg = versions[0];
        }
        
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

    async delete(name: string, version: string) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await packagesApi.delete(name, version);
        
        update(state => ({
          ...state,
          packages: state.packages.filter(pkg => pkg.name !== name || pkg.version !== version),
          selectedPackage: (state.selectedPackage?.name === name && state.selectedPackage?.version === version) ? null : state.selectedPackage,
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