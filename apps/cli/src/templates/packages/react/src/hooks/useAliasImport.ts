/**
 * Custom hook demonstrating how to work with dynamically imported packages
 * This shows patterns for working with the alias system at runtime
 */

import { useState, useEffect, useCallback } from 'react';
import type { AliasImportConfig, PackageMetadata } from '../types';

export interface UseAliasImportResult {
  packageData: PackageMetadata | null;
  loading: boolean;
  error: string | null;
  reload: () => void;
}

/**
 * Hook for working with alias-imported packages
 * This demonstrates patterns that work with Knot's alias system
 */
export const useAliasImport = (config: AliasImportConfig): UseAliasImportResult => {
  const [packageData, setPackageData] = useState<PackageMetadata | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadPackageData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      // In a real implementation, this would use dynamic imports
      // with the alias system to load package metadata
      // const module = await import(`${config.alias}/${config.packageName}`);

      // For this template, we'll simulate the data
      const metadata: PackageMetadata = {
        name: config.packageName,
        version: '1.0.0',
        description: `Package imported via ${config.alias} alias`,
        aliases: [config.alias],
        dependencies: [],
      };

      // Simulate loading time
      await new Promise(resolve => setTimeout(resolve, 100));

      setPackageData(metadata);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load package');
      setPackageData(null);
    } finally {
      setLoading(false);
    }
  }, [config]);

  useEffect(() => {
    loadPackageData();
  }, [loadPackageData]);

  const reload = useCallback(() => {
    loadPackageData();
  }, [loadPackageData]);

  return {
    packageData,
    loading,
    error,
    reload,
  };
};

/**
 * Hook for validating alias configuration
 */
export const useAliasValidation = (alias: string) => {
  const [isValid, setIsValid] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  useEffect(() => {
    // Basic alias validation
    const validateAlias = (aliasString: string): { valid: boolean; error?: string } => {
      if (!aliasString || aliasString.length === 0) {
        return { valid: false, error: 'Alias cannot be empty' };
      }

      if (aliasString.includes(' ')) {
        return { valid: false, error: 'Alias cannot contain spaces' };
      }

      if (aliasString.includes('/') || aliasString.includes('\\')) {
        return { valid: false, error: 'Alias cannot contain path separators' };
      }

      if (aliasString.length > 50) {
        return { valid: false, error: 'Alias is too long (max 50 characters)' };
      }

      return { valid: true };
    };

    const result = validateAlias(alias);
    setIsValid(result.valid);
    setValidationError(result.error || null);
  }, [alias]);

  return {
    isValid,
    validationError,
  };
};