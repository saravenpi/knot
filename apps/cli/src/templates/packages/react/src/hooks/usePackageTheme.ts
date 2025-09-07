/**
 * Theme hook that demonstrates cross-package imports with aliases
 */

import { useState, useEffect, createContext, useContext } from 'react';
import type { ThemeConfig } from '../types';

// Example: This would import from a design-tokens package via alias
// import { defaultTheme } from 'design-tokens';

const defaultTheme: ThemeConfig = {
  colors: {
    primary: '#3B82F6',
    secondary: '#6B7280',
    success: '#10B981',
    warning: '#F59E0B',
    danger: '#EF4444',
    background: '#FFFFFF',
    foreground: '#1F2937',
  },
  spacing: {
    xs: '0.25rem',
    sm: '0.5rem',
    md: '1rem',
    lg: '1.5rem',
    xl: '2rem',
  },
  typography: {
    fontFamily: 'system-ui, sans-serif',
    fontSize: {
      sm: '0.875rem',
      md: '1rem',
      lg: '1.25rem',
      xl: '1.5rem',
    },
  },
};

// Theme context for providing theme across components
const ThemeContext = createContext<{
  theme: ThemeConfig;
  setTheme: (theme: ThemeConfig) => void;
  isDark: boolean;
  toggleDark: () => void;
}>({
  theme: defaultTheme,
  setTheme: () => {},
  isDark: false,
  toggleDark: () => {},
});

export const usePackageTheme = () => {
  const context = useContext(ThemeContext);
  
  if (!context) {
    // Fallback when used outside of ThemeProvider
    const [theme, setTheme] = useState<ThemeConfig>(defaultTheme);
    const [isDark, setIsDark] = useState(false);
    
    useEffect(() => {
      // Check for saved theme preference or system preference
      const savedTheme = localStorage.getItem('package-theme');
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      
      if (savedTheme) {
        try {
          const parsedTheme = JSON.parse(savedTheme);
          setTheme(parsedTheme);
        } catch (error) {
          console.warn('Failed to parse saved theme, using default');
        }
      }
      
      setIsDark(prefersDark);
    }, []);

    const toggleDark = () => {
      setIsDark(prev => {
        const newValue = !prev;
        
        // Update theme colors based on dark mode
        if (newValue) {
          setTheme(prev => ({
            ...prev,
            colors: {
              ...prev.colors,
              background: '#1F2937',
              foreground: '#F9FAFB',
              secondary: '#4B5563',
            },
          }));
        } else {
          setTheme(prev => ({
            ...prev,
            colors: {
              ...prev.colors,
              background: '#FFFFFF',
              foreground: '#1F2937',
              secondary: '#6B7280',
            },
          }));
        }
        
        return newValue;
      });
    };

    return {
      theme,
      setTheme: (newTheme: ThemeConfig) => {
        setTheme(newTheme);
        localStorage.setItem('package-theme', JSON.stringify(newTheme));
      },
      isDark,
      toggleDark,
    };
  }

  return context;
};

// Provider component for theme context
export const PackageThemeProvider: React.FC<{
  children: React.ReactNode;
  initialTheme?: ThemeConfig;
}> = ({ children, initialTheme = defaultTheme }) => {
  const [theme, setTheme] = useState<ThemeConfig>(initialTheme);
  const [isDark, setIsDark] = useState(false);

  useEffect(() => {
    const savedTheme = localStorage.getItem('package-theme');
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (savedTheme) {
      try {
        const parsedTheme = JSON.parse(savedTheme);
        setTheme(parsedTheme);
      } catch (error) {
        console.warn('Failed to parse saved theme, using default');
      }
    }
    
    setIsDark(prefersDark);
  }, []);

  const toggleDark = () => {
    setIsDark(prev => !prev);
  };

  const handleSetTheme = (newTheme: ThemeConfig) => {
    setTheme(newTheme);
    localStorage.setItem('package-theme', JSON.stringify(newTheme));
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme: handleSetTheme, isDark, toggleDark }}>
      {children}
    </ThemeContext.Provider>
  );
};

// Utility hook for getting CSS variables from theme
export const useThemeCSS = () => {
  const { theme, isDark } = usePackageTheme();

  const cssVariables = {
    '--color-primary': theme.colors.primary,
    '--color-secondary': theme.colors.secondary,
    '--color-success': theme.colors.success,
    '--color-warning': theme.colors.warning,
    '--color-danger': theme.colors.danger,
    '--color-background': theme.colors.background,
    '--color-foreground': theme.colors.foreground,
    '--spacing-xs': theme.spacing.xs,
    '--spacing-sm': theme.spacing.sm,
    '--spacing-md': theme.spacing.md,
    '--spacing-lg': theme.spacing.lg,
    '--spacing-xl': theme.spacing.xl,
    '--font-family': theme.typography.fontFamily,
    '--font-size-sm': theme.typography.fontSize.sm,
    '--font-size-md': theme.typography.fontSize.md,
    '--font-size-lg': theme.typography.fontSize.lg,
    '--font-size-xl': theme.typography.fontSize.xl,
    '--is-dark': isDark ? '1' : '0',
  };

  return cssVariables;
};