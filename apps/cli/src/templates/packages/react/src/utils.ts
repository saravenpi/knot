/**
 * Utility functions for React components that demonstrate alias usage
 */

// Example: These functions could import from other local packages
// import { debounce } from 'utils';

export const createId = (): string => Math.random().toString(36).substr(2, 9);

export const formatMessage = (message: string, timestamp?: Date): string => {
  const time = timestamp || new Date();
  return `[${time.toISOString()}] ${message}`;
};

export const classNames = (...classes: (string | undefined | null | false)[]): string => {
  return classes.filter(Boolean).join(' ');
};

export const mergeProps = <T extends Record<string, any>>(
  defaultProps: Partial<T>,
  userProps: Partial<T>
): T => {
  return { ...defaultProps, ...userProps } as T;
};

export const getDisplayName = (Component: React.ComponentType<any>): string => {
  return Component.displayName || Component.name || 'Component';
};

// Utility for handling component variants based on theme
export const getVariantClasses = (variant: string, theme: 'light' | 'dark' = 'light'): string => {
  const variants = {
    light: {
      primary: 'bg-blue-600 text-white hover:bg-blue-700',
      secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300',
      success: 'bg-green-600 text-white hover:bg-green-700',
      warning: 'bg-yellow-600 text-white hover:bg-yellow-700',
      danger: 'bg-red-600 text-white hover:bg-red-700',
      ghost: 'text-blue-600 hover:bg-blue-50',
    },
    dark: {
      primary: 'bg-blue-500 text-white hover:bg-blue-600',
      secondary: 'bg-gray-700 text-gray-100 hover:bg-gray-600',
      success: 'bg-green-500 text-white hover:bg-green-600',
      warning: 'bg-yellow-500 text-white hover:bg-yellow-600',
      danger: 'bg-red-500 text-white hover:bg-red-600',
      ghost: 'text-blue-400 hover:bg-gray-800',
    },
  };

  return variants[theme][variant as keyof typeof variants[typeof theme]] || variants[theme].primary;
};

// Utility for handling component sizes
export const getSizeClasses = (size: string): string => {
  const sizes = {
    sm: 'h-8 px-3 text-sm',
    md: 'h-10 px-4 py-2',
    lg: 'h-12 px-6 text-lg',
    xl: 'h-14 px-8 text-xl',
  };

  return sizes[size as keyof typeof sizes] || sizes.md;
};