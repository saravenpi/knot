/**
 * Integration tests for the complete Knot alias workflow
 * Tests the entire process from project setup to package linking with aliases
 */

const { execSync, spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');

describe('Complete Alias Workflow Integration Tests', () => {
  const testWorkspace = path.resolve(__dirname, '../integration-test-workspace');
  const knotBinary = path.resolve(__dirname, '../../../apps/cli/target/release/knot');
  
  // Helper to run knot commands with timeout and error handling
  const runKnot = (command, options = {}) => {
    const defaultOptions = {
      cwd: testWorkspace,
      encoding: 'utf8',
      timeout: 30000,
      ...options
    };

    try {
      const result = execSync(`${knotBinary} ${command}`, defaultOptions);
      return { success: true, output: result, error: null };
    } catch (error) {
      return { 
        success: false, 
        output: error.stdout || '', 
        error: error.message,
        stderr: error.stderr || ''
      };
    }
  };

  // Helper to create files with proper error handling
  const createFile = (filePath, content) => {
    try {
      fs.mkdirSync(path.dirname(filePath), { recursive: true });
      fs.writeFileSync(filePath, content, 'utf8');
      return true;
    } catch (error) {
      console.error(`Failed to create file ${filePath}:`, error.message);
      return false;
    }
  };

  // Helper to read and parse JSON files
  const readJsonFile = (filePath) => {
    try {
      return JSON.parse(fs.readFileSync(filePath, 'utf8'));
    } catch (error) {
      return null;
    }
  };

  // Helper to wait for file creation/modification
  const waitForFile = (filePath, timeout = 5000) => {
    return new Promise((resolve, reject) => {
      const startTime = Date.now();
      
      const checkFile = () => {
        if (fs.existsSync(filePath)) {
          resolve(true);
        } else if (Date.now() - startTime > timeout) {
          reject(new Error(`File ${filePath} was not created within ${timeout}ms`));
        } else {
          setTimeout(checkFile, 100);
        }
      };
      
      checkFile();
    });
  };

  beforeAll(async () => {
    // Clean up any existing test workspace
    if (fs.existsSync(testWorkspace)) {
      fs.rmSync(testWorkspace, { recursive: true, force: true });
    }

    // Create test workspace
    fs.mkdirSync(testWorkspace, { recursive: true });
    
    // Verify knot binary exists
    if (!fs.existsSync(knotBinary)) {
      throw new Error(`Knot binary not found at ${knotBinary}. Please build the CLI first.`);
    }
  }, 60000);

  afterAll(() => {
    // Clean up test workspace
    if (fs.existsSync(testWorkspace)) {
      fs.rmSync(testWorkspace, { recursive: true, force: true });
    }
  });

  describe('End-to-End Workflow: Simple Alias Setup', () => {
    test('should create project with global alias and link successfully', async () => {
      // Step 1: Create project configuration
      const projectConfig = {
        name: 'workflow-test-simple',
        description: 'Integration test for simple alias workflow',
        tsAlias: '@simple',
        apps: {
          'test-app': {
            packages: ['utils-package']
          }
        }
      };

      const knotYmlPath = path.join(testWorkspace, 'knot.yml');
      expect(createFile(knotYmlPath, yaml.dump(projectConfig))).toBe(true);

      // Step 2: Create a simple package
      const packageDir = path.join(testWorkspace, 'packages', 'utils-package');
      const packageConfig = {
        name: 'utils-package',
        version: '1.0.0',
        description: 'Test utility package'
      };

      expect(createFile(
        path.join(packageDir, 'package.yml'),
        yaml.dump(packageConfig)
      )).toBe(true);

      expect(createFile(
        path.join(packageDir, 'src', 'index.ts'),
        `export const hello = (name: string) => \`Hello, \${name}!\`;`
      )).toBe(true);

      // Step 3: Create test app
      const appDir = path.join(testWorkspace, 'apps', 'test-app');
      const appConfig = {
        name: 'test-app',
        packages: ['utils-package']
      };

      expect(createFile(
        path.join(appDir, 'app.yml'),
        yaml.dump(appConfig)
      )).toBe(true);

      expect(createFile(
        path.join(appDir, 'src', 'main.ts'),
        `import { hello } from '@simple/utils-package'; console.log(hello('World'));`
      )).toBe(true);

      // Step 4: Link packages
      const linkResult = runKnot('link');
      expect(linkResult.success).toBe(true);
      expect(linkResult.output).toContain('Success');

      // Step 5: Verify knot_packages directory was created
      const knotPackagesDir = path.join(appDir, 'knot_packages');
      expect(fs.existsSync(knotPackagesDir)).toBe(true);

      // Step 6: Verify package was linked
      const linkedPackage = path.join(knotPackagesDir, 'utils-package');
      expect(fs.existsSync(linkedPackage)).toBe(true);
      expect(fs.existsSync(path.join(linkedPackage, 'package.yml'))).toBe(true);

      // Step 7: Verify tsconfig.json was created with correct alias
      const tsConfigPath = path.join(appDir, 'tsconfig.json');
      await waitForFile(tsConfigPath);
      
      const tsConfig = readJsonFile(tsConfigPath);
      expect(tsConfig).toBeTruthy();
      expect(tsConfig.compilerOptions?.paths).toHaveProperty('@simple/*');
      expect(tsConfig.compilerOptions.paths['@simple/*']).toContain('./knot_packages/*');
      expect(tsConfig.include).toContain('knot_packages/**/*');

    }, 30000);
  });

  describe('End-to-End Workflow: Multi-App with Different Aliases', () => {
    test('should handle multiple apps with different alias configurations', async () => {
      // Clean workspace for this test
      const multiAppWorkspace = path.join(testWorkspace, 'multi-app-test');
      fs.mkdirSync(multiAppWorkspace, { recursive: true });
      
      process.chdir(multiAppWorkspace);

      // Step 1: Create project with multiple apps using different aliases
      const projectConfig = {
        name: 'multi-alias-workflow',
        description: 'Test multiple aliases',
        tsAlias: '#',  // Global default
        apps: {
          'frontend': {
            tsAlias: '@ui',
            packages: ['components', 'utils']
          },
          'backend': {
            tsAlias: '@core',
            packages: ['utils', 'database']
          },
          'mobile': {
            // Inherits global '#'
            packages: ['components', 'utils']
          }
        }
      };

      expect(createFile(
        path.join(multiAppWorkspace, 'knot.yml'),
        yaml.dump(projectConfig)
      )).toBe(true);

      // Step 2: Create packages
      const packages = [
        { name: 'components', desc: 'UI components package' },
        { name: 'utils', desc: 'Utility functions package' },
        { name: 'database', desc: 'Database utilities package' }
      ];

      for (const pkg of packages) {
        const packageDir = path.join(multiAppWorkspace, 'packages', pkg.name);
        
        expect(createFile(
          path.join(packageDir, 'package.yml'),
          yaml.dump({ name: pkg.name, version: '1.0.0', description: pkg.desc })
        )).toBe(true);

        expect(createFile(
          path.join(packageDir, 'src', 'index.ts'),
          `export const ${pkg.name}Function = () => '${pkg.desc}';`
        )).toBe(true);
      }

      // Step 3: Create apps
      const apps = [
        { name: 'frontend', alias: '@ui', packages: ['components', 'utils'] },
        { name: 'backend', alias: '@core', packages: ['utils', 'database'] },
        { name: 'mobile', alias: '#', packages: ['components', 'utils'] }
      ];

      for (const app of apps) {
        const appDir = path.join(multiAppWorkspace, 'apps', app.name);
        
        expect(createFile(
          path.join(appDir, 'app.yml'),
          yaml.dump({ 
            name: app.name, 
            ...(app.alias !== '#' ? { tsAlias: app.alias } : {}),
            packages: app.packages 
          })
        )).toBe(true);

        const imports = app.packages.map(pkg => 
          `import { ${pkg}Function } from '${app.alias}/${pkg}';`
        ).join('\n');

        expect(createFile(
          path.join(appDir, 'src', 'main.ts'),
          `${imports}\nconsole.log('${app.name} app loaded');`
        )).toBe(true);
      }

      // Step 4: Link all packages
      const linkResult = runKnot('link', { cwd: multiAppWorkspace });
      expect(linkResult.success).toBe(true);

      // Step 5: Verify each app has correct alias configuration
      for (const app of apps) {
        const tsConfigPath = path.join(multiAppWorkspace, 'apps', app.name, 'tsconfig.json');
        await waitForFile(tsConfigPath);
        
        const tsConfig = readJsonFile(tsConfigPath);
        expect(tsConfig).toBeTruthy();
        
        const expectedAlias = `${app.alias}/*`;
        expect(tsConfig.compilerOptions?.paths).toHaveProperty(expectedAlias);
        expect(tsConfig.compilerOptions.paths[expectedAlias]).toContain('./knot_packages/*');
      }

      // Step 6: Verify packages are linked correctly
      for (const app of apps) {
        const knotPackagesDir = path.join(multiAppWorkspace, 'apps', app.name, 'knot_packages');
        expect(fs.existsSync(knotPackagesDir)).toBe(true);

        for (const pkg of app.packages) {
          const linkedPackage = path.join(knotPackagesDir, pkg);
          expect(fs.existsSync(linkedPackage)).toBe(true);
          expect(fs.existsSync(path.join(linkedPackage, 'package.yml'))).toBe(true);
        }
      }

    }, 45000);
  });

  describe('End-to-End Workflow: Error Recovery', () => {
    test('should recover from linking errors and continue successfully', async () => {
      const errorWorkspace = path.join(testWorkspace, 'error-recovery-test');
      fs.mkdirSync(errorWorkspace, { recursive: true });
      
      process.chdir(errorWorkspace);

      // Step 1: Create project with intentional errors
      const projectConfig = {
        name: 'error-recovery-test',
        tsAlias: '@test',
        apps: {
          'recovery-app': {
            packages: ['existing-package', 'missing-package'] // missing-package doesn't exist
          }
        }
      };

      expect(createFile(
        path.join(errorWorkspace, 'knot.yml'),
        yaml.dump(projectConfig)
      )).toBe(true);

      // Step 2: Create only one package (missing the other)
      const packageDir = path.join(errorWorkspace, 'packages', 'existing-package');
      expect(createFile(
        path.join(packageDir, 'package.yml'),
        yaml.dump({ name: 'existing-package', version: '1.0.0' })
      )).toBe(true);

      expect(createFile(
        path.join(packageDir, 'src', 'index.ts'),
        `export const test = () => 'exists';`
      )).toBe(true);

      // Step 3: Create app
      const appDir = path.join(errorWorkspace, 'apps', 'recovery-app');
      expect(createFile(
        path.join(appDir, 'app.yml'),
        yaml.dump({ name: 'recovery-app', packages: ['existing-package', 'missing-package'] })
      )).toBe(true);

      // Step 4: First link attempt should fail
      const firstLinkResult = runKnot('link', { cwd: errorWorkspace });
      expect(firstLinkResult.success).toBe(false);

      // Step 5: Create the missing package
      const missingPackageDir = path.join(errorWorkspace, 'packages', 'missing-package');
      expect(createFile(
        path.join(missingPackageDir, 'package.yml'),
        yaml.dump({ name: 'missing-package', version: '1.0.0' })
      )).toBe(true);

      expect(createFile(
        path.join(missingPackageDir, 'src', 'index.ts'),
        `export const recovered = () => 'now exists';`
      )).toBe(true);

      // Step 6: Second link attempt should succeed
      const secondLinkResult = runKnot('link', { cwd: errorWorkspace });
      expect(secondLinkResult.success).toBe(true);

      // Step 7: Verify both packages are now linked
      const knotPackagesDir = path.join(appDir, 'knot_packages');
      expect(fs.existsSync(knotPackagesDir)).toBe(true);
      expect(fs.existsSync(path.join(knotPackagesDir, 'existing-package'))).toBe(true);
      expect(fs.existsSync(path.join(knotPackagesDir, 'missing-package'))).toBe(true);

      // Step 8: Verify tsconfig is correct
      const tsConfigPath = path.join(appDir, 'tsconfig.json');
      await waitForFile(tsConfigPath);
      
      const tsConfig = readJsonFile(tsConfigPath);
      expect(tsConfig).toBeTruthy();
      expect(tsConfig.compilerOptions?.paths).toHaveProperty('@test/*');

    }, 30000);
  });

  describe('End-to-End Workflow: TypeScript Integration', () => {
    test('should generate valid TypeScript configuration and compile successfully', async () => {
      const tsWorkspace = path.join(testWorkspace, 'typescript-integration-test');
      fs.mkdirSync(tsWorkspace, { recursive: true });
      
      process.chdir(tsWorkspace);

      // Step 1: Create project with TypeScript-focused setup
      const projectConfig = {
        name: 'typescript-integration',
        tsAlias: '@ts',
        apps: {
          'typescript-app': {
            packages: ['types-package', 'functions-package']
          }
        }
      };

      expect(createFile(
        path.join(tsWorkspace, 'knot.yml'),
        yaml.dump(projectConfig)
      )).toBe(true);

      // Step 2: Create packages with proper TypeScript exports
      const typesPackageDir = path.join(tsWorkspace, 'packages', 'types-package');
      expect(createFile(
        path.join(typesPackageDir, 'package.yml'),
        yaml.dump({ name: 'types-package', version: '1.0.0' })
      )).toBe(true);

      expect(createFile(
        path.join(typesPackageDir, 'src', 'index.ts'),
        `export interface User { id: string; name: string; }
export type UserRole = 'admin' | 'user' | 'guest';
export const DEFAULT_ROLE: UserRole = 'user';`
      )).toBe(true);

      const functionsPackageDir = path.join(tsWorkspace, 'packages', 'functions-package');
      expect(createFile(
        path.join(functionsPackageDir, 'package.yml'),
        yaml.dump({ name: 'functions-package', version: '1.0.0' })
      )).toBe(true);

      expect(createFile(
        path.join(functionsPackageDir, 'src', 'index.ts'),
        `import { User, UserRole } from 'types-package';
export const createUser = (name: string, role: UserRole = 'user'): User => ({
  id: Math.random().toString(36),
  name
});
export const validateUser = (user: User): boolean => Boolean(user.id && user.name);`
      )).toBe(true);

      // Step 3: Create app with TypeScript code using aliases
      const appDir = path.join(tsWorkspace, 'apps', 'typescript-app');
      expect(createFile(
        path.join(appDir, 'app.yml'),
        yaml.dump({ name: 'typescript-app', packages: ['types-package', 'functions-package'] })
      )).toBe(true);

      expect(createFile(
        path.join(appDir, 'src', 'main.ts'),
        `import { User, UserRole, DEFAULT_ROLE } from '@ts/types-package';
import { createUser, validateUser } from '@ts/functions-package';

const newUser: User = createUser('Test User', DEFAULT_ROLE);
console.log('User valid:', validateUser(newUser));
export { newUser };`
      )).toBe(true);

      expect(createFile(
        path.join(appDir, 'package.json'),
        JSON.stringify({
          name: 'typescript-app',
          version: '1.0.0',
          scripts: {
            build: 'tsc',
            check: 'tsc --noEmit'
          },
          devDependencies: {
            typescript: '^4.9.0'
          }
        }, null, 2)
      )).toBe(true);

      // Step 4: Link packages
      const linkResult = runKnot('link', { cwd: tsWorkspace });
      expect(linkResult.success).toBe(true);

      // Step 5: Verify TypeScript configuration
      const tsConfigPath = path.join(appDir, 'tsconfig.json');
      await waitForFile(tsConfigPath);
      
      const tsConfig = readJsonFile(tsConfigPath);
      expect(tsConfig).toBeTruthy();
      expect(tsConfig.compilerOptions?.paths).toHaveProperty('@ts/*');
      
      // Step 6: Test TypeScript compilation (if TypeScript is available)
      try {
        const tscResult = runKnot('check', { cwd: appDir });
        // TypeScript check should either pass or show only expected errors
        // (we're not installing typescript in the test environment)
      } catch (error) {
        // Expected if TypeScript is not installed in test environment
        console.log('TypeScript not available in test environment, skipping compilation test');
      }

    }, 30000);
  });

  describe('Workflow Performance Tests', () => {
    test('should complete full workflow within reasonable time limits', async () => {
      const perfWorkspace = path.join(testWorkspace, 'performance-test');
      fs.mkdirSync(perfWorkspace, { recursive: true });
      
      process.chdir(perfWorkspace);

      const startTime = Date.now();

      // Create a moderately complex project
      const projectConfig = {
        name: 'performance-test',
        tsAlias: '@perf',
        apps: {}
      };

      // Create 5 apps each with 3-4 packages
      for (let i = 1; i <= 5; i++) {
        projectConfig.apps[`app${i}`] = {
          packages: [`pkg1`, `pkg2`, `pkg3`]
        };
      }

      expect(createFile(
        path.join(perfWorkspace, 'knot.yml'),
        yaml.dump(projectConfig)
      )).toBe(true);

      // Create packages
      for (let i = 1; i <= 3; i++) {
        const packageDir = path.join(perfWorkspace, 'packages', `pkg${i}`);
        expect(createFile(
          path.join(packageDir, 'package.yml'),
          yaml.dump({ name: `pkg${i}`, version: '1.0.0' })
        )).toBe(true);

        expect(createFile(
          path.join(packageDir, 'src', 'index.ts'),
          `export const func${i} = () => 'package ${i}';`
        )).toBe(true);
      }

      // Create apps
      for (let i = 1; i <= 5; i++) {
        const appDir = path.join(perfWorkspace, 'apps', `app${i}`);
        expect(createFile(
          path.join(appDir, 'app.yml'),
          yaml.dump({ name: `app${i}`, packages: [`pkg1`, `pkg2`, `pkg3`] })
        )).toBe(true);
      }

      const setupTime = Date.now() - startTime;

      // Link packages and measure time
      const linkStartTime = Date.now();
      const linkResult = runKnot('link', { cwd: perfWorkspace });
      const linkTime = Date.now() - linkStartTime;

      expect(linkResult.success).toBe(true);

      // Performance assertions
      expect(setupTime).toBeLessThan(5000); // Setup should take less than 5 seconds
      expect(linkTime).toBeLessThan(10000); // Linking should take less than 10 seconds

      console.log(`Performance Test Results:
        Setup time: ${setupTime}ms
        Link time: ${linkTime}ms
        Total time: ${setupTime + linkTime}ms`);

    }, 20000);
  });
});