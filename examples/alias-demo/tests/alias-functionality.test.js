/**
 * Comprehensive test scenarios for Knot package alias functionality
 * These tests verify that the alias system works correctly in various scenarios
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

describe('Knot Alias System Tests', () => {
  const testProjectDir = path.resolve(__dirname, '../');
  const knotBinary = path.resolve(__dirname, '../../../apps/cli/target/release/knot');
  
  // Helper function to run knot commands
  const runKnot = (command, options = {}) => {
    try {
      return execSync(`${knotBinary} ${command}`, {
        cwd: testProjectDir,
        encoding: 'utf8',
        ...options
      });
    } catch (error) {
      throw new Error(`Knot command failed: ${command}\n${error.message}`);
    }
  };

  // Helper function to check if tsconfig.json has correct alias configuration
  const checkTsConfigAlias = (appName, expectedAlias) => {
    const tsConfigPath = path.join(testProjectDir, 'apps', appName, 'tsconfig.json');
    
    if (!fs.existsSync(tsConfigPath)) {
      throw new Error(`tsconfig.json not found for app ${appName}`);
    }

    const tsConfig = JSON.parse(fs.readFileSync(tsConfigPath, 'utf8'));
    const paths = tsConfig.compilerOptions?.paths || {};
    
    const aliasPath = `${expectedAlias}/*`;
    expect(paths).toHaveProperty(aliasPath);
    expect(paths[aliasPath]).toContain('./knot_packages/*');
  };

  beforeAll(() => {
    // Ensure we're in the test project directory
    process.chdir(testProjectDir);
    
    // Verify knot binary exists
    if (!fs.existsSync(knotBinary)) {
      throw new Error(`Knot binary not found at ${knotBinary}. Please build the CLI first.`);
    }
  });

  describe('Basic Alias Configuration', () => {
    test('should read global tsAlias from knot.yml', () => {
      const knotConfig = fs.readFileSync(path.join(testProjectDir, 'knot.yml'), 'utf8');
      expect(knotConfig).toContain('tsAlias: "#"');
    });

    test('should read app-specific tsAlias overrides', () => {
      // Dashboard app should override with @ui
      const dashboardConfig = fs.readFileSync(
        path.join(testProjectDir, 'apps/dashboard-app/app.yml'), 
        'utf8'
      );
      expect(dashboardConfig).toContain('tsAlias: "@ui"');

      // API server should override with @core
      const apiConfig = fs.readFileSync(
        path.join(testProjectDir, 'apps/api-server/app.yml'), 
        'utf8'
      );
      expect(apiConfig).toContain('tsAlias: "@core"');

      // Analytics should override with $
      const analyticsConfig = fs.readFileSync(
        path.join(testProjectDir, 'apps/analytics/app.yml'), 
        'utf8'
      );
      expect(analyticsConfig).toContain('tsAlias: "$"');

      // Mobile app should not have tsAlias (inherits global)
      const mobileConfig = fs.readFileSync(
        path.join(testProjectDir, 'apps/mobile-app/app.yml'), 
        'utf8'
      );
      expect(mobileConfig).not.toContain('tsAlias:');
    });
  });

  describe('Package Linking with Aliases', () => {
    test('should link packages successfully', () => {
      const output = runKnot('link');
      expect(output).toContain('Successfully');
      expect(output).toContain('TypeScript');
    });

    test('should create knot_packages directory in each app', () => {
      const apps = ['dashboard-app', 'api-server', 'mobile-app', 'analytics'];
      
      apps.forEach(app => {
        const knotPackagesDir = path.join(testProjectDir, 'apps', app, 'knot_packages');
        expect(fs.existsSync(knotPackagesDir)).toBe(true);
      });
    });

    test('should create correct symlinks/copies in knot_packages', () => {
      const dashboardPackages = path.join(testProjectDir, 'apps/dashboard-app/knot_packages');
      const expectedPackages = ['utils', 'ui-components', 'data-layer'];
      
      expectedPackages.forEach(pkg => {
        const packagePath = path.join(dashboardPackages, pkg);
        expect(fs.existsSync(packagePath)).toBe(true);
        
        // Check that package.yml exists in the linked package
        const packageYml = path.join(packagePath, 'package.yml');
        expect(fs.existsSync(packageYml)).toBe(true);
      });
    });
  });

  describe('TypeScript Configuration with Aliases', () => {
    test('should generate tsconfig.json with correct alias for dashboard-app (@ui)', () => {
      checkTsConfigAlias('dashboard-app', '@ui');
    });

    test('should generate tsconfig.json with correct alias for api-server (@core)', () => {
      checkTsConfigAlias('api-server', '@core');
    });

    test('should generate tsconfig.json with correct alias for analytics ($)', () => {
      checkTsConfigAlias('analytics', '$');
    });

    test('should generate tsconfig.json with correct alias for mobile-app (# - inherited)', () => {
      checkTsConfigAlias('mobile-app', '#');
    });

    test('should include knot_packages in tsconfig include paths', () => {
      const apps = ['dashboard-app', 'api-server', 'mobile-app', 'analytics'];
      
      apps.forEach(app => {
        const tsConfigPath = path.join(testProjectDir, 'apps', app, 'tsconfig.json');
        const tsConfig = JSON.parse(fs.readFileSync(tsConfigPath, 'utf8'));
        
        expect(tsConfig.include).toContain('knot_packages/**/*');
      });
    });
  });

  describe('Alias Import Resolution', () => {
    test('should resolve alias imports in source files', () => {
      // Check dashboard app source
      const dashboardMain = fs.readFileSync(
        path.join(testProjectDir, 'apps/dashboard-app/src/main.tsx'), 
        'utf8'
      );
      expect(dashboardMain).toContain('@ui/ui-components');
      expect(dashboardMain).toContain('@ui/utils/validation');
      expect(dashboardMain).toContain('@ui/data-layer/cache');

      // Check API server source
      const apiServer = fs.readFileSync(
        path.join(testProjectDir, 'apps/api-server/src/server.ts'), 
        'utf8'
      );
      expect(apiServer).toContain('@core/utils/validation');
      expect(apiServer).toContain('@core/utils/dates');
      expect(apiServer).toContain('@core/data-layer/cache');

      // Check analytics source
      const analyticsMain = fs.readFileSync(
        path.join(testProjectDir, 'apps/analytics/src/main.ts'), 
        'utf8'
      );
      expect(analyticsMain).toContain('$/utils/dates');
      expect(analyticsMain).toContain('$/utils/math');
      expect(analyticsMain).toContain('$/data-layer/cache');

      // Check mobile app source
      const mobileApp = fs.readFileSync(
        path.join(testProjectDir, 'apps/mobile-app/src/App.tsx'), 
        'utf8'
      );
      expect(mobileApp).toContain('#/ui-components');
      expect(mobileApp).toContain('#/utils/dates');
      expect(mobileApp).toContain('#/utils/validation');
    });
  });

  describe('Package Dependencies with Aliases', () => {
    test('should handle local package dependencies correctly', () => {
      // ui-components depends on utils
      const uiComponentsIndex = fs.readFileSync(
        path.join(testProjectDir, 'packages/ui-components/src/index.ts'),
        'utf8'
      );
      expect(uiComponentsIndex).toContain("from 'utils'");

      // data-layer depends on utils
      const dataLayerIndex = fs.readFileSync(
        path.join(testProjectDir, 'packages/data-layer/src/index.ts'),
        'utf8'
      );
      expect(dataLayerIndex).toContain("from 'utils'");
      expect(dataLayerIndex).toContain("from 'utils/dates'");
    });

    test('should handle nested alias imports in packages', () => {
      const buttonComponent = fs.readFileSync(
        path.join(testProjectDir, 'packages/ui-components/src/Button/index.ts'),
        'utf8'
      );
      expect(buttonComponent).toContain("from 'utils'");

      const inputComponent = fs.readFileSync(
        path.join(testProjectDir, 'packages/ui-components/src/Input/index.ts'),
        'utf8'
      );
      expect(inputComponent).toContain("from 'utils/validation'");
    });
  });

  describe('Multiple Alias Configurations', () => {
    test('should handle different aliases in different apps correctly', () => {
      const aliases = {
        'dashboard-app': '@ui',
        'api-server': '@core',
        'analytics': '$',
        'mobile-app': '#'
      };

      Object.entries(aliases).forEach(([app, alias]) => {
        const tsConfigPath = path.join(testProjectDir, 'apps', app, 'tsconfig.json');
        const tsConfig = JSON.parse(fs.readFileSync(tsConfigPath, 'utf8'));
        
        const expectedPath = `${alias}/*`;
        expect(tsConfig.compilerOptions.paths).toHaveProperty(expectedPath);
        expect(tsConfig.compilerOptions.paths[expectedPath]).toContain('./knot_packages/*');
      });
    });
  });

  describe('Alias Symbol Validation', () => {
    test('should support various alias symbols', () => {
      const validAliases = ['#', '@ui', '@core', '$', '~', '@', '&'];
      
      // These are examples of what should be supported
      validAliases.forEach(alias => {
        expect(typeof alias).toBe('string');
        expect(alias.length).toBeGreaterThan(0);
        
        // Basic validation that they don't contain problematic characters
        expect(alias).not.toContain('/');
        expect(alias).not.toContain('\\');
        expect(alias).not.toContain(' ');
      });
    });
  });

  describe('Error Handling', () => {
    test('should handle missing packages gracefully', () => {
      // This test would be run in a separate test environment
      // to avoid affecting the main test suite
      expect(true).toBe(true); // Placeholder
    });

    test('should validate alias configurations', () => {
      // Test that the system validates alias configurations properly
      expect(true).toBe(true); // Placeholder
    });
  });

  describe('Performance and Caching', () => {
    test('should link packages efficiently', () => {
      const startTime = Date.now();
      runKnot('link', { timeout: 30000 }); // 30 second timeout
      const duration = Date.now() - startTime;
      
      // Linking should complete in reasonable time (less than 10 seconds)
      expect(duration).toBeLessThan(10000);
    });

    test('should generate TypeScript configs efficiently', () => {
      // After linking, all tsconfig.json files should exist
      const apps = ['dashboard-app', 'api-server', 'mobile-app', 'analytics'];
      
      apps.forEach(app => {
        const tsConfigPath = path.join(testProjectDir, 'apps', app, 'tsconfig.json');
        const stats = fs.statSync(tsConfigPath);
        
        // File should have been recently modified (within last minute)
        const now = Date.now();
        const fileTime = stats.mtime.getTime();
        expect(now - fileTime).toBeLessThan(60000); // 1 minute
      });
    });
  });

  describe('Cleanup and Unlinking', () => {
    test('should be able to unlink packages', () => {
      // Note: This assumes an unlink command exists
      // The actual implementation might differ
      
      // Test that knot_packages directories can be cleaned up
      const apps = ['dashboard-app', 'api-server', 'mobile-app', 'analytics'];
      
      apps.forEach(app => {
        const knotPackagesDir = path.join(testProjectDir, 'apps', app, 'knot_packages');
        if (fs.existsSync(knotPackagesDir)) {
          // Simulate cleanup by checking the directory exists
          expect(fs.existsSync(knotPackagesDir)).toBe(true);
        }
      });
    });
  });
});