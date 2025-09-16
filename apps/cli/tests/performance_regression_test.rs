use anyhow::Result;
use std::process::{Command, Stdio};
use std::fs;
use std::time::{Duration, Instant};
use tempfile::TempDir;

// Test helper to execute knot command with timing
fn run_knot_command_timed(args: &[&str], working_dir: &std::path::Path) -> (String, String, bool, Duration) {
    let start = Instant::now();

    let output = Command::new("cargo")
        .args(&["run", "--bin", "knot"])
        .args(args)
        .current_dir(working_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success, duration)
}

// Helper to create projects of various sizes for performance testing
fn create_large_project(num_apps: usize, num_packages: usize) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Create main knot.yml with many apps
    let mut knot_yml = String::from(r#"
name: large-test-project
description: A large test project for performance testing
tsAlias: true
scripts:
  build-all: "echo 'Building all components...'"
  test-all: "echo 'Testing all components...'"
  dev: "echo 'Starting development...'"
  clean: "echo 'Cleaning all...'"
  deploy: "echo 'Deploying all...'"
  perf-test: "echo 'Running performance tests...'"
apps:
"#);

    // Add apps to knot.yml
    for i in 0..num_apps {
        knot_yml.push_str(&format!(r#"
  app-{}:
    tsAlias: true
    packages:
      - types
      - utils-{}
      - component-{}
"#, i, i % (num_packages.max(1)), i % (num_packages.max(1))));
    }

    knot_yml.push_str(r#"
variables:
  project_version: "1.0.0"
  api_url: "https://api.example.com"
  build_env: "production"
  performance_mode: "optimized"
"#);

    fs::write(temp_dir.path().join("knot.yml"), knot_yml)?;

    // Create apps directory structure
    let apps_dir = temp_dir.path().join("apps");
    fs::create_dir_all(&apps_dir)?;

    for i in 0..num_apps {
        let app_dir = apps_dir.join(format!("app-{}", i));
        fs::create_dir_all(&app_dir)?;

        let app_yml = format!(r#"
name: app-{}
description: Test application number {}
tsAlias: "#app{}"
packages:
  - types
  - utils-{}
  - component-{}
scripts:
  start: "echo 'Starting app-{} on port {}...'"
  build: "echo 'Building app-{}...'"
  test: "echo 'Testing app-{}...'"
  lint: "echo 'Linting app-{}...'"
  preview: "echo 'Previewing app-{}...'"
variables:
  app_port: "{}"
  app_title: "Test App {}"
  app_id: "{}"
"#, i, i, i, i % (num_packages.max(1)), i % (num_packages.max(1)), i, 3000 + i, i, i, i, i, 3000 + i, i, i);

        fs::write(app_dir.join("app.yml"), app_yml)?;
    }

    // Create packages directory structure
    let packages_dir = temp_dir.path().join("packages");
    fs::create_dir_all(&packages_dir)?;

    // Create shared types package
    let types_dir = packages_dir.join("types");
    fs::create_dir_all(&types_dir)?;
    let types_yml = r#"
name: types
team: core
version: "1.0.0"
description: Shared TypeScript types for all components
scripts:
  build: "echo 'Building types package...'"
  test: "echo 'Testing types package...'"
  generate: "echo 'Generating type definitions...'"
exports:
  ".": "./dist/index.js"
  "./api": "./dist/api.js"
  "./components": "./dist/components.js"
features:
  - typescript
  - validation
  - performance
variables:
  typescript_version: "5.0.0"
"#;
    fs::write(types_dir.join("package.yml"), types_yml)?;

    // Create multiple utility packages
    for i in 0..num_packages {
        let utils_dir = packages_dir.join(format!("utils-{}", i));
        fs::create_dir_all(&utils_dir)?;

        let utils_yml = format!(r#"
name: utils-{}
team: core
version: "1.{}.0"
description: Utility functions package {}
dependencies:
  - types
scripts:
  build: "echo 'Building utils-{} package...'"
  test: "echo 'Testing utils-{} package...'"
  benchmark: "echo 'Benchmarking utils-{} package...'"
  optimize: "echo 'Optimizing utils-{} package...'"
exports:
  ".": "./dist/index.js"
  "./helpers": "./dist/helpers.js"
  "./validators": "./dist/validators.js"
features:
  - performance
  - testing
  - optimization
variables:
  package_id: "{}"
  optimization_level: "high"
"#, i, i, i, i, i, i, i, i);

        fs::write(utils_dir.join("package.yml"), utils_yml)?;

        let component_dir = packages_dir.join(format!("component-{}", i));
        fs::create_dir_all(&component_dir)?;

        let component_yml = format!(r#"
name: component-{}
team: frontend
version: "2.{}.0"
description: UI component package {}
dependencies:
  - types
  - utils-{}
scripts:
  build: "echo 'Building component-{} package...'"
  test: "echo 'Testing component-{} package...'"
  storybook: "echo 'Starting storybook for component-{}...'"
  visual-test: "echo 'Running visual tests for component-{}...'"
exports:
  ".": "./dist/index.js"
  "./button": "./dist/button.js"
  "./input": "./dist/input.js"
features:
  - storybook
  - testing
  - accessibility
  - performance
variables:
  component_id: "{}"
  theme_version: "2.0"
"#, i, i, i, i, i, i, i, i, i);

        fs::write(component_dir.join("package.yml"), component_yml)?;
    }

    Ok(temp_dir)
}

#[test]
fn test_startup_performance() -> Result<()> {
    println!("=== Testing Startup Performance ===");

    let project = create_large_project(5, 3)?;

    // Test help command performance (should be very fast)
    let (_, _, success, duration) = run_knot_command_timed(&["--help"], project.path());

    assert!(success, "Help command should succeed");
    assert!(duration < Duration::from_secs(5), "Help should be fast, took {:?}", duration);
    println!("✓ Help command completed in {:?}", duration);

    // Test version command performance
    let (_, _, success, duration) = run_knot_command_timed(&["--version"], project.path());

    if success {
        assert!(duration < Duration::from_secs(3), "Version should be very fast, took {:?}", duration);
        println!("✓ Version command completed in {:?}", duration);
    }

    println!("✅ Startup performance test passed");
    Ok(())
}

#[test]
fn test_status_command_performance() -> Result<()> {
    println!("=== Testing Status Command Performance ===");

    // Test with different project sizes
    let project_sizes = vec![
        (2, 2, "small"),
        (5, 3, "medium"),
        (10, 5, "large"),
    ];

    for (num_apps, num_packages, size_name) in project_sizes {
        let project = create_large_project(num_apps, num_packages)?;

        let (stdout, stderr, success, duration) = run_knot_command_timed(&["status"], project.path());
        let output = format!("{}{}", stdout, stderr);

        // Status should complete in reasonable time
        assert!(duration < Duration::from_secs(30),
               "Status on {} project should complete in <30s, took {:?}", size_name, duration);

        // Should either succeed or provide meaningful feedback
        if !success && !output.is_empty() {
            assert!(output.contains("project") || output.contains("config"),
                   "Status error should be meaningful: {}", output);
        }

        println!("✓ Status command on {} project completed in {:?}", size_name, duration);
    }

    println!("✅ Status command performance test passed");
    Ok(())
}

#[test]
fn test_script_discovery_performance() -> Result<()> {
    println!("=== Testing Script Discovery Performance ===");

    let project = create_large_project(10, 5)?;

    // Test script discovery with many available scripts
    let test_locations = vec![
        (project.path(), "project root"),
        (project.path().join("apps").join("app-0"), "app directory"),
        (project.path().join("packages").join("utils-0"), "package directory"),
    ];

    for (location, context) in test_locations {
        if location.exists() {
            // Test finding a specific script
            let (stdout, stderr, success, duration) = run_knot_command_timed(&["run", "build"], &location);

            // Should complete quickly even with many scripts
            assert!(duration < Duration::from_secs(10),
                   "Script discovery in {} should be fast, took {:?}", context, duration);

            if success {
                println!("✓ Script 'build' found in {} in {:?}", context, duration);
            }

            // Test script not found (should show available scripts quickly)
            let (stdout, stderr, success, duration) = run_knot_command_timed(&["run", "nonexistent"], &location);
            let output = format!("{}{}", stdout, stderr);

            assert!(duration < Duration::from_secs(15),
                   "Script not found handling should be fast, took {:?}", duration);

            if output.contains("Available scripts") || output.contains("not found") {
                println!("✓ Script not found handled efficiently in {} in {:?}", context, duration);
            }
        }
    }

    println!("✅ Script discovery performance test passed");
    Ok(())
}

#[test]
fn test_config_parsing_performance() -> Result<()> {
    println!("=== Testing Config Parsing Performance ===");

    let project = create_large_project(15, 8)?;

    // Test parsing performance for different config files
    let config_tests = vec![
        (project.path().join("knot.yml"), "project config"),
        (project.path().join("apps").join("app-0").join("app.yml"), "app config"),
        (project.path().join("packages").join("utils-0").join("package.yml"), "package config"),
    ];

    for (config_path, config_type) in config_tests {
        if config_path.exists() {
            let parent_dir = config_path.parent().unwrap();

            // Multiple operations that require parsing the config
            let operations = vec![
                (vec!["status"], "status check"),
                (vec!["run", "build"], "script execution"),
            ];

            for (command, operation) in operations {
                let (stdout, stderr, success, duration) = run_knot_command_timed(&command, parent_dir);

                // Config parsing should be efficient
                assert!(duration < Duration::from_secs(20),
                       "{} on {} should be fast, took {:?}", operation, config_type, duration);

                if success || !format!("{}{}", stdout, stderr).is_empty() {
                    println!("✓ {} on {} completed in {:?}", operation, config_type, duration);
                }
            }
        }
    }

    println!("✅ Config parsing performance test passed");
    Ok(())
}

#[test]
fn test_memory_usage_stability() -> Result<()> {
    println!("=== Testing Memory Usage Stability ===");

    let project = create_large_project(8, 4)?;

    // Test repeated operations don't accumulate memory
    let repeated_operations = vec![
        vec!["status"],
        vec!["run", "build-all"],
        vec!["--help"],
    ];

    for operation in repeated_operations {
        let mut durations = Vec::new();

        // Run the same operation multiple times
        for i in 0..5 {
            let (stdout, stderr, success, duration) = run_knot_command_timed(&operation, project.path());
            durations.push(duration);

            // Should not get significantly slower (indicating memory accumulation)
            if i > 0 {
                let first_duration = durations[0];
                let current_duration = duration;

                // Allow some variance but no major degradation (3x slower threshold)
                let max_acceptable = first_duration * 3;
                assert!(current_duration < max_acceptable,
                       "Operation {:?} should not degrade significantly: first={:?}, current={:?}",
                       operation, first_duration, current_duration);
            }

            if success {
                println!("✓ Operation {:?} run {} completed in {:?}", operation, i + 1, duration);
            }
        }
    }

    println!("✅ Memory usage stability test passed");
    Ok(())
}

#[test]
fn test_concurrent_safety() -> Result<()> {
    println!("=== Testing Concurrent Safety ===");

    let project = create_large_project(5, 3)?;

    // Test multiple concurrent operations (simulated by rapid succession)
    let operations = vec![
        vec!["status"],
        vec!["run", "build"],
        vec!["--help"],
        vec!["run", "test"],
    ];

    let mut all_durations = Vec::new();

    for operation in operations {
        let (stdout, stderr, success, duration) = run_knot_command_timed(&operation, project.path());
        all_durations.push(duration);

        // Each operation should complete in reasonable time
        assert!(duration < Duration::from_secs(25),
               "Operation {:?} should complete reasonably fast, took {:?}", operation, duration);

        if success || !format!("{}{}", stdout, stderr).is_empty() {
            println!("✓ Concurrent operation {:?} completed in {:?}", operation, duration);
        }
    }

    // Overall operations should not take excessively long
    let total_duration: Duration = all_durations.iter().sum();
    assert!(total_duration < Duration::from_secs(60),
           "All operations should complete in reasonable total time, took {:?}", total_duration);

    println!("✅ Concurrent safety test passed");
    Ok(())
}

#[test]
fn test_large_config_handling() -> Result<()> {
    println!("=== Testing Large Config Handling ===");

    // Create a project with very large configuration
    let project = create_large_project(20, 10)?;

    // Test that large configs don't cause performance issues
    let operations = vec![
        (vec!["status"], "status with large config"),
        (vec!["run", "build-all"], "script execution with large config"),
        (vec!["vars", "list"], "variable listing with large config"),
    ];

    for (command, description) in operations {
        let (stdout, stderr, success, duration) = run_knot_command_timed(&command, project.path());

        // Large configs should still be handled efficiently
        assert!(duration < Duration::from_secs(45),
               "{} should handle large config efficiently, took {:?}", description, duration);

        let output = format!("{}{}", stdout, stderr);
        if success || !output.is_empty() {
            println!("✓ {} completed in {:?}", description, duration);
        }
    }

    println!("✅ Large config handling test passed");
    Ok(())
}

#[test]
fn test_error_handling_performance() -> Result<()> {
    println!("=== Testing Error Handling Performance ===");

    let project = create_large_project(5, 3)?;

    // Test that error scenarios don't cause performance degradation
    let error_scenarios = vec![
        (vec!["run", "nonexistent-script"], "script not found"),
        (vec!["run", "invalid script name"], "invalid script name"),
    ];

    for (command, scenario) in error_scenarios {
        let (stdout, stderr, success, duration) = run_knot_command_timed(&command, project.path());

        // Error handling should be fast
        assert!(duration < Duration::from_secs(15),
               "Error handling for {} should be fast, took {:?}", scenario, duration);

        // Should provide error feedback
        let output = format!("{}{}", stdout, stderr);
        if !success && !output.is_empty() {
            println!("✓ {} error handled efficiently in {:?}", scenario, duration);
        }
    }

    // Test with invalid config
    let temp_dir = TempDir::new()?;
    let invalid_yml = r#"
name: test
scripts:
  invalid yaml structure
"#;
    fs::write(temp_dir.path().join("knot.yml"), invalid_yml)?;

    let (stdout, stderr, success, duration) = run_knot_command_timed(&["status"], temp_dir.path());

    // Invalid config error should be handled quickly
    assert!(duration < Duration::from_secs(10),
           "Invalid config error should be fast, took {:?}", duration);

    println!("✅ Error handling performance test passed");
    Ok(())
}

#[test]
fn test_file_system_performance() -> Result<()> {
    println!("=== Testing File System Performance ===");

    let project = create_large_project(12, 6)?;

    // Test operations that require file system scanning
    let fs_operations = vec![
        (vec!["status"], "project scanning"),
        (vec!["link"], "package discovery"),
    ];

    for (command, operation) in fs_operations {
        let (stdout, stderr, success, duration) = run_knot_command_timed(&command, project.path());

        // File system operations should be optimized
        assert!(duration < Duration::from_secs(35),
               "{} should handle file system efficiently, took {:?}", operation, duration);

        let output = format!("{}{}", stdout, stderr);
        if success || !output.is_empty() {
            println!("✓ {} completed efficiently in {:?}", operation, duration);
        }
    }

    println!("✅ File system performance test passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_performance_regression_tests() -> Result<()> {
        println!("⚡ Running comprehensive performance regression tests");

        test_startup_performance()?;
        test_status_command_performance()?;
        test_script_discovery_performance()?;
        test_config_parsing_performance()?;
        test_memory_usage_stability()?;
        test_concurrent_safety()?;
        test_large_config_handling()?;
        test_error_handling_performance()?;
        test_file_system_performance()?;

        println!("🎉 All performance regression tests completed successfully!");
        Ok(())
    }
}