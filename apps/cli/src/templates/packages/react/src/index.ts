/**
 * React Package Template - Main exports
 * 
 * This template demonstrates how to create React component packages
 * that work with Knot's alias system.
 */

// Example: Import from other local packages using direct names
// In apps, these imports would be available via aliases
// import { formatMessage } from 'utils';
// import { theme } from 'design-tokens';

export { default as Button } from './Button';
export * from './Button';

export { default as Input } from './Input';
export * from './Input';

export { default as Card } from './Card';
export * from './Card';

// Re-export common utilities that React components might need
// These would be available to apps via aliases like @ui/react-package
export { createId, formatMessage } from './utils';

// Export hooks that use the alias system
export { usePackageTheme } from './hooks/usePackageTheme';
export { useAliasImport } from './hooks/useAliasImport';

// Export types that might be shared across packages
export type { ComponentVariant, ComponentSize, ThemeConfig } from './types';