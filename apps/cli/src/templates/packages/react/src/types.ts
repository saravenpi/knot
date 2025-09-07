/**
 * Type definitions for React components with alias system support
 */

export type ComponentVariant = 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'ghost';
export type ComponentSize = 'sm' | 'md' | 'lg' | 'xl';

export interface BaseComponentProps {
  variant?: ComponentVariant;
  size?: ComponentSize;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

export interface ThemeConfig {
  colors: {
    primary: string;
    secondary: string;
    success: string;
    warning: string;
    danger: string;
    background: string;
    foreground: string;
  };
  spacing: {
    xs: string;
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
  typography: {
    fontFamily: string;
    fontSize: {
      sm: string;
      md: string;
      lg: string;
      xl: string;
    };
  };
}

export interface AliasImportConfig {
  packageName: string;
  alias: string;
  exports: string[];
}

export interface PackageMetadata {
  name: string;
  version: string;
  description: string;
  aliases?: string[];
  dependencies?: string[];
}