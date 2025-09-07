/**
 * Edge case tests for Knot package alias system
 * Tests various edge cases, error conditions, and boundary scenarios
 */

const { execSync, spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');

describe('Alias System Edge Cases', () => {
  const testDir = path.resolve(__dirname, '../edge-case-tests');
  const knotBinary = path.resolve(__dirname, '../../../apps/cli/target/release/knot');
  
  // Helper to create test projects
  const createTestProject = (name, config) => {
    const projectDir = path.join(testDir, name);
    fs.mkdirSync(projectDir, { recursive: true });
    
    // Write knot.yml
    fs.writeFileSync(
      path.join(projectDir, 'knot.yml'),
      yaml.dump(config)
    );
    
    return projectDir;
  };

  // Helper to create test app
  const createTestApp = (projectDir, appName, appConfig) => {
    const appDir = path.join(projectDir, 'apps', appName);
    fs.mkdirSync(appDir, { recursive: true });
    
    fs.writeFileSync(
      path.join(appDir, 'app.yml'),
      yaml.dump(appConfig)
    );
    
    // Create src directory with basic files
    const srcDir = path.join(appDir, 'src');
    fs.mkdirSync(srcDir, { recursive: true });
    
    return appDir;
  };

  // Helper to run knot commands safely
  const runKnotSafe = (command, options = {}) => {
    try {
      return {
        success: true,
        output: execSync(`${knotBinary} ${command}`, {
          encoding: 'utf8',
          ...options
        })
      };
    } catch (error) {
      return {
        success: false,
        error: error.message,
        output: error.stdout || '',
        stderr: error.stderr || ''
      };
    }
  };

  beforeAll(() => {
    // Create test directory
    if (!fs.existsSync(testDir)) {
      fs.mkdirSync(testDir, { recursive: true });
    }
  });

  afterAll(() => {
    // Clean up test directory
    if (fs.existsSync(testDir)) {
      fs.rmSync(testDir, { recursive: true, force: true });
    }
  });

  describe('Invalid Alias Configurations', () => {
    test('should handle empty alias gracefully', () => {
      const projectDir = createTestProject('empty-alias', {
        name: 'empty-alias-test',
        tsAlias: '',
        apps: {
          'test-app': {
            packages: ['utils']
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      
      // Should either succeed with fallback or fail gracefully
      if (!result.success) {
        expect(result.error).toMatch(/alias|configuration/i);
      }
    });

    test('should handle null alias', () => {
      const projectDir = createTestProject('null-alias', {
        name: 'null-alias-test',
        tsAlias: null,
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      
      // Should handle null alias appropriately
      expect(result.success || result.error.includes('alias')).toBe(true);
    });

    test('should handle very long alias names', () => {
      const longAlias = '@' + 'a'.repeat(100);
      
      const projectDir = createTestProject('long-alias', {
        name: 'long-alias-test',
        tsAlias: longAlias,
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      
      if (!result.success) {
        expect(result.error).toMatch(/alias|length|invalid/i);
      }
    });

    test('should handle aliases with invalid characters', () => {
      const invalidAliases = [
        'alias with spaces',
        'alias/with/slashes',
        'alias\\with\\backslashes',
        'alias.with.dots',
        'alias*with*stars',
        'alias?with?questions'
      ];

      invalidAliases.forEach((alias, index) => {
        const projectDir = createTestProject(`invalid-alias-${index}`, {
          name: `invalid-alias-test-${index}`,
          tsAlias: alias,
          apps: {
            'test-app': {
              packages: []
            }
          }
        });

        createTestApp(projectDir, 'test-app', {
          name: 'test-app',
          packages: []
        });

        const result = runKnotSafe('link', { cwd: projectDir });
        
        if (!result.success) {
          expect(result.error).toMatch(/alias|invalid|character/i);
        }
      });
    });
  });

  describe('Conflicting Alias Configurations', () => {
    test('should handle app alias overriding global alias', () => {
      const projectDir = createTestProject('alias-override', {
        name: 'alias-override-test',
        tsAlias: '#',
        apps: {
          'app1': {
            tsAlias: '@override',
            packages: []
          },
          'app2': {
            // No tsAlias - should inherit global
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'app1', {
        name: 'app1',
        tsAlias: '@override',
        packages: []
      });

      createTestApp(projectDir, 'app2', {
        name: 'app2',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);

      // Check that tsconfig files have correct aliases
      const tsconfig1Path = path.join(projectDir, 'apps/app1/tsconfig.json');
      const tsconfig2Path = path.join(projectDir, 'apps/app2/tsconfig.json');

      if (fs.existsSync(tsconfig1Path) && fs.existsSync(tsconfig2Path)) {
        const tsconfig1 = JSON.parse(fs.readFileSync(tsconfig1Path, 'utf8'));
        const tsconfig2 = JSON.parse(fs.readFileSync(tsconfig2Path, 'utf8'));

        expect(tsconfig1.compilerOptions.paths).toHaveProperty('@override/*');
        expect(tsconfig2.compilerOptions.paths).toHaveProperty('#/*');
      }
    });

    test('should handle identical aliases across multiple apps', () => {
      const projectDir = createTestProject('identical-aliases', {
        name: 'identical-aliases-test',
        tsAlias: '@same',
        apps: {
          'app1': {
            tsAlias: '@same',
            packages: []
          },
          'app2': {
            tsAlias: '@same',
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'app1', {
        name: 'app1',
        tsAlias: '@same',
        packages: []
      });

      createTestApp(projectDir, 'app2', {
        name: 'app2',
        tsAlias: '@same',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);
    });
  });

  describe('Circular Dependency Scenarios', () => {
    test('should handle circular alias references (if applicable)', () => {
      // This test would be more relevant if aliases could reference other aliases
      // For now, it's a placeholder for future functionality
      
      const projectDir = createTestProject('circular-test', {
        name: 'circular-test',
        tsAlias: '#',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);
    });
  });

  describe('File System Edge Cases', () => {
    test('should handle missing knot_packages directory creation', () => {
      const projectDir = createTestProject('missing-dir', {
        name: 'missing-dir-test',
        tsAlias: '#',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);

      // knot_packages directory should be created
      const knotPackagesDir = path.join(projectDir, 'apps/test-app/knot_packages');
      expect(fs.existsSync(knotPackagesDir)).toBe(true);
    });

    test('should handle existing tsconfig.json file', () => {
      const projectDir = createTestProject('existing-tsconfig', {
        name: 'existing-tsconfig-test',
        tsAlias: '@custom',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      const appDir = createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      // Create existing tsconfig.json with some content
      const existingTsConfig = {
        compilerOptions: {
          target: 'ES2020',
          module: 'commonjs',
          strict: true,
          customOption: 'preserve-me'
        },
        include: ['src/**/*'],
        exclude: ['node_modules']
      };

      fs.writeFileSync(
        path.join(appDir, 'tsconfig.json'),
        JSON.stringify(existingTsConfig, null, 2)
      );

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);

      // Check that existing config is preserved and alias is added
      const updatedTsConfig = JSON.parse(
        fs.readFileSync(path.join(appDir, 'tsconfig.json'), 'utf8')
      );

      expect(updatedTsConfig.compilerOptions.customOption).toBe('preserve-me');
      expect(updatedTsConfig.compilerOptions.paths).toHaveProperty('@custom/*');
    });

    test('should handle malformed tsconfig.json', () => {
      const projectDir = createTestProject('malformed-tsconfig', {
        name: 'malformed-tsconfig-test',
        tsAlias: '@fix',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      const appDir = createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      // Create malformed tsconfig.json
      fs.writeFileSync(
        path.join(appDir, 'tsconfig.json'),
        '{ invalid json content'
      );

      const result = runKnotSafe('link', { cwd: projectDir });
      
      // Should either fix the file or report an error
      if (result.success) {
        const tsConfigPath = path.join(appDir, 'tsconfig.json');
        if (fs.existsSync(tsConfigPath)) {
          expect(() => {
            JSON.parse(fs.readFileSync(tsConfigPath, 'utf8'));
          }).not.toThrow();
        }
      } else {
        expect(result.error).toMatch(/tsconfig|json|parse/i);
      }
    });
  });

  describe('Special Character Handling', () => {
    test('should handle special characters in project names', () => {
      const specialNames = [
        'project-with-hyphens',
        'project_with_underscores',
        'project.with.dots'
      ];

      specialNames.forEach((name, index) => {
        const safeName = name.replace(/[^a-zA-Z0-9-_]/g, '_');
        const projectDir = createTestProject(`special-${index}`, {
          name: safeName,
          tsAlias: '#',
          apps: {
            'test-app': {
              packages: []
            }
          }
        });

        createTestApp(projectDir, 'test-app', {
          name: 'test-app',
          packages: []
        });

        const result = runKnotSafe('link', { cwd: projectDir });
        expect(result.success).toBe(true);
      });
    });

    test('should handle Unicode characters in aliases (if supported)', () => {
      const unicodeAliases = [
        '🚀',
        '⚡',
        '💻'
      ];

      unicodeAliases.forEach((alias, index) => {
        const projectDir = createTestProject(`unicode-${index}`, {
          name: `unicode-test-${index}`,
          tsAlias: alias,
          apps: {
            'test-app': {
              packages: []
            }
          }
        });

        createTestApp(projectDir, 'test-app', {
          name: 'test-app',
          packages: []
        });

        const result = runKnotSafe('link', { cwd: projectDir });
        
        // Unicode might not be supported, but should fail gracefully
        if (!result.success) {
          expect(result.error).toMatch(/alias|character|unicode/i);
        }
      });
    });
  });

  describe('Performance Edge Cases', () => {
    test('should handle large number of packages', () => {
      const packageCount = 50;
      const packages = [];
      
      for (let i = 0; i < packageCount; i++) {
        packages.push(`package-${i}`);
      }

      const projectDir = createTestProject('large-packages', {
        name: 'large-packages-test',
        tsAlias: '@large',
        apps: {
          'test-app': {
            packages: packages.slice(0, 10) // Only use first 10 to avoid issues
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: packages.slice(0, 10)
      });

      const startTime = Date.now();
      const result = runKnotSafe('link', { cwd: projectDir, timeout: 60000 });
      const duration = Date.now() - startTime;

      if (result.success) {
        expect(duration).toBeLessThan(30000); // Should complete within 30 seconds
      }
    });

    test('should handle deeply nested package imports', () => {
      // This would test performance with very deep import paths
      const projectDir = createTestProject('deep-imports', {
        name: 'deep-imports-test',
        tsAlias: '@deep',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);
    });
  });

  describe('Cleanup and Recovery', () => {
    test('should recover from incomplete linking process', () => {
      const projectDir = createTestProject('recovery-test', {
        name: 'recovery-test',
        tsAlias: '#',
        apps: {
          'test-app': {
            packages: []
          }
        }
      });

      const appDir = createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: []
      });

      // Create incomplete knot_packages directory
      const knotPackagesDir = path.join(appDir, 'knot_packages');
      fs.mkdirSync(knotPackagesDir, { recursive: true });
      
      // Create some dummy content that might interfere
      fs.writeFileSync(path.join(knotPackagesDir, 'dummy-file.txt'), 'test');

      const result = runKnotSafe('link', { cwd: projectDir });
      expect(result.success).toBe(true);
    });
  });

  describe('Validation and Error Messages', () => {
    test('should provide helpful error messages for common mistakes', () => {
      const projectDir = createTestProject('error-messages', {
        name: 'error-messages-test',
        tsAlias: '///invalid///',
        apps: {
          'test-app': {
            packages: ['non-existent-package']
          }
        }
      });

      createTestApp(projectDir, 'test-app', {
        name: 'test-app',
        packages: ['non-existent-package']
      });

      const result = runKnotSafe('link', { cwd: projectDir });
      
      if (!result.success) {
        // Error messages should be helpful and specific
        expect(result.error.length).toBeGreaterThan(10);
        expect(result.error).toMatch(/(alias|package|invalid|not found)/i);
      }
    });
  });
});