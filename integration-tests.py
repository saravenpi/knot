#!/usr/bin/env python3
"""
Knot Integration Test Suite

This script runs comprehensive integration tests for the Knot dependency system.
It verifies that all components work together correctly.
"""

import os
import subprocess
import tempfile
import shutil
import json
import time
from pathlib import Path
from typing import List, Dict, Tuple, Optional


class TestResult:
    def __init__(self, name: str, passed: bool, message: str = "", duration: float = 0.0):
        self.name = name
        self.passed = passed
        self.message = message
        self.duration = duration


class KnotIntegrationTester:
    def __init__(self):
        self.test_results: List[TestResult] = []
        self.temp_dirs: List[str] = []

    def run_command(self, cmd: List[str], cwd: str = None, capture_output: bool = True) -> Tuple[int, str, str]:
        """Run a command and return (returncode, stdout, stderr)"""
        try:
            result = subprocess.run(
                cmd,
                cwd=cwd,
                capture_output=capture_output,
                text=True,
                timeout=30
            )
            return result.returncode, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return -1, "", "Command timed out"
        except Exception as e:
            return -1, "", str(e)

    def create_test_project(self, name: str, config: Dict) -> str:
        """Create a test project with the given configuration"""
        test_dir = tempfile.mkdtemp(prefix=f"knot_test_{name}_")
        self.temp_dirs.append(test_dir)
        
        # Create knot.yml
        knot_yml_content = f"""name: {config['name']}
description: {config.get('description', 'Test project')}

apps:
"""
        for app_name, app_config in config.get('apps', {}).items():
            knot_yml_content += f"""  {app_name}:
    description: {app_config.get('description', 'Test app')}
"""
            if 'packages' in app_config:
                knot_yml_content += f"    packages: {app_config['packages']}\n"
            if 'tsAlias' in app_config:
                knot_yml_content += f"    tsAlias: {app_config['tsAlias']}\n"

        knot_yml_content += "\npackages:\n"
        for pkg_name, pkg_config in config.get('packages', {}).items():
            knot_yml_content += f"""  {pkg_name}:
    description: {pkg_config.get('description', 'Test package')}
"""

        with open(os.path.join(test_dir, 'knot.yml'), 'w') as f:
            f.write(knot_yml_content)

        # Create packages
        for pkg_name, pkg_config in config.get('packages', {}).items():
            pkg_dir = os.path.join(test_dir, 'packages', pkg_name)
            os.makedirs(pkg_dir, exist_ok=True)
            
            # Create package.json
            package_json = {
                "name": pkg_name,
                "version": pkg_config.get('version', '1.0.0'),
                "description": pkg_config.get('description', 'Test package'),
                "main": "index.ts"
            }
            if 'dependencies' in pkg_config:
                package_json['dependencies'] = pkg_config['dependencies']
            
            with open(os.path.join(pkg_dir, 'package.json'), 'w') as f:
                json.dump(package_json, f, indent=2)
            
            # Create index.ts
            index_ts = pkg_config.get('code', f'export const {pkg_name}Function = () => "Hello from {pkg_name}!";')
            with open(os.path.join(pkg_dir, 'index.ts'), 'w') as f:
                f.write(index_ts)

        # Create apps
        for app_name, app_config in config.get('apps', {}).items():
            app_dir = os.path.join(test_dir, 'apps', app_name)
            os.makedirs(app_dir, exist_ok=True)
            
            # Create app.yml
            app_yml_content = f"""name: {app_name}
description: {app_config.get('description', 'Test app')}
"""
            if 'packages' in app_config:
                app_yml_content += f"packages:\n"
                for pkg in app_config['packages']:
                    app_yml_content += f"  - {pkg}\n"

            with open(os.path.join(app_dir, 'app.yml'), 'w') as f:
                f.write(app_yml_content)
            
            # Create package.json
            app_package_json = {
                "name": app_name,
                "version": "1.0.0",
                "description": app_config.get('description', 'Test app')
            }
            with open(os.path.join(app_dir, 'package.json'), 'w') as f:
                json.dump(app_package_json, f, indent=2)

        return test_dir

    def test_basic_linking(self):
        """Test 1: Basic local package linking"""
        start_time = time.time()
        
        config = {
            'name': 'Basic Linking Test',
            'apps': {
                'test-app': {
                    'packages': ['utils', 'helpers']
                }
            },
            'packages': {
                'utils': {
                    'description': 'Utility functions'
                },
                'helpers': {
                    'description': 'Helper functions'
                }
            }
        }
        
        test_dir = self.create_test_project('basic_linking', config)
        
        # Run knot link
        returncode, stdout, stderr = self.run_command(['knot', 'link'], cwd=test_dir)
        
        duration = time.time() - start_time
        
        if returncode == 0:
            # Verify packages were linked
            knot_packages_dir = os.path.join(test_dir, 'apps', 'test-app', 'knot_packages')
            if (os.path.exists(os.path.join(knot_packages_dir, 'utils')) and 
                os.path.exists(os.path.join(knot_packages_dir, 'helpers'))):
                self.test_results.append(TestResult(
                    "Basic Linking", True, "Both packages linked successfully", duration
                ))
            else:
                self.test_results.append(TestResult(
                    "Basic Linking", False, "Packages not found in knot_packages", duration
                ))
        else:
            self.test_results.append(TestResult(
                "Basic Linking", False, f"Command failed: {stderr}", duration
            ))

    def test_symlink_mode(self):
        """Test 2: Symlink mode linking"""
        start_time = time.time()
        
        config = {
            'name': 'Symlink Test',
            'apps': {
                'test-app': {
                    'packages': ['utils']
                }
            },
            'packages': {
                'utils': {
                    'description': 'Utility functions'
                }
            }
        }
        
        test_dir = self.create_test_project('symlink_test', config)
        
        # Run knot link --symlink
        returncode, stdout, stderr = self.run_command(['knot', 'link', '--symlink'], cwd=test_dir)
        
        duration = time.time() - start_time
        
        if returncode == 0:
            # Check if symlink was created
            symlink_path = os.path.join(test_dir, 'apps', 'test-app', 'knot_packages', 'utils')
            if os.path.islink(symlink_path):
                self.test_results.append(TestResult(
                    "Symlink Mode", True, "Symlink created successfully", duration
                ))
            else:
                self.test_results.append(TestResult(
                    "Symlink Mode", False, "Symlink not created", duration
                ))
        else:
            self.test_results.append(TestResult(
                "Symlink Mode", False, f"Command failed: {stderr}", duration
            ))

    def test_multi_app_linking(self):
        """Test 3: Multiple apps with different dependencies"""
        start_time = time.time()
        
        config = {
            'name': 'Multi-App Test',
            'apps': {
                'frontend': {
                    'packages': ['ui-components', 'utils']
                },
                'backend': {
                    'packages': ['database', 'utils']
                }
            },
            'packages': {
                'ui-components': {'description': 'UI components'},
                'utils': {'description': 'Shared utilities'},
                'database': {'description': 'Database helpers'}
            }
        }
        
        test_dir = self.create_test_project('multi_app', config)
        
        # Run knot link
        returncode, stdout, stderr = self.run_command(['knot', 'link'], cwd=test_dir)
        
        duration = time.time() - start_time
        
        if returncode == 0:
            # Check both apps have their dependencies
            frontend_deps = os.path.join(test_dir, 'apps', 'frontend', 'knot_packages')
            backend_deps = os.path.join(test_dir, 'apps', 'backend', 'knot_packages')
            
            frontend_has_deps = (os.path.exists(os.path.join(frontend_deps, 'ui-components')) and
                               os.path.exists(os.path.join(frontend_deps, 'utils')))
            backend_has_deps = (os.path.exists(os.path.join(backend_deps, 'database')) and
                              os.path.exists(os.path.join(backend_deps, 'utils')))
            
            if frontend_has_deps and backend_has_deps:
                self.test_results.append(TestResult(
                    "Multi-App Linking", True, "All apps have correct dependencies", duration
                ))
            else:
                self.test_results.append(TestResult(
                    "Multi-App Linking", False, "Some dependencies missing", duration
                ))
        else:
            self.test_results.append(TestResult(
                "Multi-App Linking", False, f"Command failed: {stderr}", duration
            ))

    def test_missing_package_error(self):
        """Test 4: Missing package error handling"""
        start_time = time.time()
        
        config = {
            'name': 'Missing Package Test',
            'apps': {
                'test-app': {
                    'packages': ['utils', 'nonexistent']
                }
            },
            'packages': {
                'utils': {'description': 'Utility functions'}
                # Note: 'nonexistent' package is not defined
            }
        }
        
        test_dir = self.create_test_project('missing_package', config)
        
        # Run knot link - this should fail gracefully
        returncode, stdout, stderr = self.run_command(['knot', 'link'], cwd=test_dir)
        
        duration = time.time() - start_time
        
        if returncode != 0:
            # Check if error message mentions the missing package
            error_output = stderr + stdout
            if 'nonexistent' in error_output.lower() and ('does not exist' in error_output.lower() or 'not found' in error_output.lower()):
                self.test_results.append(TestResult(
                    "Missing Package Error", True, "Proper error message for missing package", duration
                ))
            else:
                self.test_results.append(TestResult(
                    "Missing Package Error", False, f"Unclear error message: {error_output}", duration
                ))
        else:
            self.test_results.append(TestResult(
                "Missing Package Error", False, "Command should have failed but didn't", duration
            ))

    def test_typescript_aliases(self):
        """Test 5: TypeScript alias configuration"""
        start_time = time.time()
        
        config = {
            'name': 'TypeScript Aliases Test',
            'apps': {
                'ts-app': {
                    'packages': ['utils'],
                    'tsAlias': True
                }
            },
            'packages': {
                'utils': {'description': 'Utility functions'}
            }
        }
        
        test_dir = self.create_test_project('typescript_aliases', config)
        
        # Create a tsconfig.json in the app
        app_dir = os.path.join(test_dir, 'apps', 'ts-app')
        tsconfig = {
            "compilerOptions": {
                "target": "ES2020",
                "module": "commonjs",
                "strict": True
            }
        }
        with open(os.path.join(app_dir, 'tsconfig.json'), 'w') as f:
            json.dump(tsconfig, f, indent=2)
        
        # Run knot link
        returncode, stdout, stderr = self.run_command(['knot', 'link'], cwd=test_dir)
        
        duration = time.time() - start_time
        
        if returncode == 0:
            # Check if tsconfig.json was updated with paths
            with open(os.path.join(app_dir, 'tsconfig.json'), 'r') as f:
                updated_tsconfig = json.load(f)
            
            if ('compilerOptions' in updated_tsconfig and 
                'paths' in updated_tsconfig['compilerOptions']):
                self.test_results.append(TestResult(
                    "TypeScript Aliases", True, "tsconfig.json updated with paths", duration
                ))
            else:
                self.test_results.append(TestResult(
                    "TypeScript Aliases", False, "tsconfig.json not updated with paths", duration
                ))
        else:
            self.test_results.append(TestResult(
                "TypeScript Aliases", False, f"Command failed: {stderr}", duration
            ))

    def test_performance_large_project(self):
        """Test 6: Performance with many packages"""
        start_time = time.time()
        
        # Create a project with many packages
        packages = {}
        app_packages = []
        for i in range(20):
            pkg_name = f"package-{i}"
            packages[pkg_name] = {'description': f'Package {i}'}
            app_packages.append(pkg_name)
        
        config = {
            'name': 'Large Project Test',
            'apps': {
                'large-app': {
                    'packages': app_packages
                }
            },
            'packages': packages
        }
        
        test_dir = self.create_test_project('large_project', config)
        
        # Run knot link and measure time
        link_start = time.time()
        returncode, stdout, stderr = self.run_command(['knot', 'link'], cwd=test_dir)
        link_duration = time.time() - link_start
        
        total_duration = time.time() - start_time
        
        if returncode == 0:
            # Count linked packages
            knot_packages_dir = os.path.join(test_dir, 'apps', 'large-app', 'knot_packages')
            linked_count = len([d for d in os.listdir(knot_packages_dir) 
                              if os.path.isdir(os.path.join(knot_packages_dir, d))])
            
            if linked_count == 20:
                self.test_results.append(TestResult(
                    "Large Project Performance", True, 
                    f"Linked 20 packages in {link_duration:.2f}s", total_duration
                ))
            else:
                self.test_results.append(TestResult(
                    "Large Project Performance", False, 
                    f"Expected 20 packages, found {linked_count}", total_duration
                ))
        else:
            self.test_results.append(TestResult(
                "Large Project Performance", False, f"Command failed: {stderr}", total_duration
            ))

    def cleanup(self):
        """Clean up temporary directories"""
        for temp_dir in self.temp_dirs:
            if os.path.exists(temp_dir):
                shutil.rmtree(temp_dir)

    def run_all_tests(self):
        """Run all integration tests"""
        print("Starting Knot Integration Tests...")
        print("=" * 50)
        
        # Check if knot command is available
        returncode, _, _ = self.run_command(['knot', '--version'])
        if returncode != 0:
            print("❌ knot command not found. Please install Knot CLI first.")
            return
        
        tests = [
            self.test_basic_linking,
            self.test_symlink_mode,
            self.test_multi_app_linking,
            self.test_missing_package_error,
            self.test_typescript_aliases,
            self.test_performance_large_project
        ]
        
        for test in tests:
            print(f"Running {test.__doc__}...")
            try:
                test()
            except Exception as e:
                self.test_results.append(TestResult(
                    test.__doc__ or test.__name__, False, f"Test threw exception: {e}"
                ))
            print()
        
        self.print_results()

    def print_results(self):
        """Print test results summary"""
        print("=" * 50)
        print("Integration Test Results")
        print("=" * 50)
        
        passed = sum(1 for r in self.test_results if r.passed)
        failed = len(self.test_results) - passed
        
        for result in self.test_results:
            status = "✅ PASS" if result.passed else "❌ FAIL"
            duration_str = f" ({result.duration:.2f}s)" if result.duration > 0 else ""
            print(f"{status} {result.name}{duration_str}")
            if result.message:
                print(f"    {result.message}")
        
        print("-" * 50)
        print(f"Total: {len(self.test_results)}, Passed: {passed}, Failed: {failed}")
        
        if failed == 0:
            print("🎉 All integration tests passed!")
        else:
            print(f"⚠️  {failed} test(s) failed. Please review the results above.")


if __name__ == "__main__":
    tester = KnotIntegrationTester()
    try:
        tester.run_all_tests()
    finally:
        tester.cleanup()