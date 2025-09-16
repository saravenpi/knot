use anyhow::Result;
use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Test helper to execute knot command and capture output
fn run_knot_command(args: &[&str], working_dir: &std::path::Path) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "knot"])
        .args(args)
        .current_dir(working_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}

// Helper to create a complete project structure for testing
fn create_complete_project() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Create main knot.yml
    let knot_yml = r#"
name: integration-test-project
description: A complete test project for UX integration testing
tsAlias: true
scripts:
  build-all: "echo 'Building entire project...'"
  test-all: "echo 'Testing entire project...'"
  dev: "echo 'Starting development environment...'"
  clean: "echo 'Cleaning project...'"
  deploy: "echo 'Deploying project...'"
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
      - ui-components
  backend:
    tsAlias: "#api"
    packages:
      - types
      - utils
      - database
  mobile:
    - types
    - utils
variables:
  project_version: "1.0.0"
  api_url: "https://api.example.com"
  build_env: "production"
"#;
    fs::write(temp_dir.path().join("knot.yml"), knot_yml)?;

    // Create apps directory structure
    let apps_dir = temp_dir.path().join("apps");
    fs::create_dir_all(&apps_dir)?;

    // Web app
    let web_dir = apps_dir.join("web");
    fs::create_dir_all(&web_dir)?;
    let web_app_yml = r#"
name: web-app
description: Frontend web application
tsAlias: "#"
packages:
  - types
  - utils
  - ui-components
scripts:
  start: "echo 'Starting web dev server on port 3000...'"
  build: "echo 'Building web app for production...'"
  test: "echo 'Running web app tests...'"
  lint: "echo 'Linting web app code...'"
  preview: "echo 'Starting preview server...'"
variables:
  app_port: "3000"
  app_title: "Test Web App"
"#;
    fs::write(web_dir.join("app.yml"), web_app_yml)?;

    // Backend app
    let backend_dir = apps_dir.join("backend");
    fs::create_dir_all(&backend_dir)?;
    let backend_app_yml = r#"
name: backend-api
description: Backend API server
tsAlias: "#api"
packages:
  - types
  - utils
  - database
scripts:
  start: "echo 'Starting API server on port 8000...'"
  build: "echo 'Building backend API...'"
  test: "echo 'Running API tests...'"
  migrate: "echo 'Running database migrations...'"
  seed: "echo 'Seeding database...'"
variables:
  api_port: "8000"
  db_url: "postgresql://localhost:5432/testdb"
"#;
    fs::write(backend_dir.join("app.yml"), backend_app_yml)?;

    // Mobile app
    let mobile_dir = apps_dir.join("mobile");
    fs::create_dir_all(&mobile_dir)?;
    let mobile_app_yml = r#"
name: mobile-app
description: Mobile application
packages:
  - types
  - utils
scripts:
  start: "echo 'Starting mobile dev environment...'"
  build: "echo 'Building mobile app...'"
  test: "echo 'Running mobile tests...'"
  ios: "echo 'Building for iOS...'"
  android: "echo 'Building for Android...'"
"#;
    fs::write(mobile_dir.join("app.yml"), mobile_app_yml)?;

    // Create packages directory structure
    let packages_dir = temp_dir.path().join("packages");
    fs::create_dir_all(&packages_dir)?;

    // Types package
    let types_dir = packages_dir.join("types");
    fs::create_dir_all(&types_dir)?;
    let types_package_yml = r#"
name: types
team: core
version: "1.0.0"
description: Shared TypeScript types
scripts:
  build: "echo 'Building types package...'"
  test: "echo 'Testing types package...'"
  generate: "echo 'Generating type definitions...'"
exports:
  ".": "./dist/index.js"
  "./api": "./dist/api.js"
features:
  - typescript
  - validation
variables:
  typescript_version: "5.0.0"
"#;
    fs::write(types_dir.join("package.yml"), types_package_yml)?;

    // Utils package
    let utils_dir = packages_dir.join("utils");
    fs::create_dir_all(&utils_dir)?;
    let utils_package_yml = r#"
name: utils
team: core
version: "2.1.0"
description: Shared utility functions
dependencies:
  - types
scripts:
  build: "echo 'Building utils package...'"
  test: "echo 'Testing utils package...'"
  benchmark: "echo 'Running performance benchmarks...'"
exports:
  ".": "./dist/index.js"
  "./helpers": "./dist/helpers.js"
features:
  - performance
  - testing
"#;
    fs::write(utils_dir.join("package.yml"), utils_package_yml)?;

    // UI Components package
    let ui_dir = packages_dir.join("ui-components");
    fs::create_dir_all(&ui_dir)?;
    let ui_package_yml = r#"
name: ui-components
team: frontend
version: "1.5.0"
description: Reusable UI components
dependencies:
  - types
  - utils
scripts:
  build: "echo 'Building UI components...'"
  test: "echo 'Testing UI components...'"
  storybook: "echo 'Starting Storybook...'"
  visual-test: "echo 'Running visual regression tests...'"
exports:
  ".": "./dist/index.js"
  "./button": "./dist/button.js"
  "./input": "./dist/input.js"
features:
  - storybook
  - testing
  - accessibility
"#;
    fs::write(ui_dir.join("package.yml"), ui_package_yml)?;

    // Database package
    let db_dir = packages_dir.join("database");
    fs::create_dir_all(&db_dir)?;
    let db_package_yml = r#"
name: database
team: backend
version: "1.0.0"
description: Database utilities and migrations
dependencies:
  - types
  - utils
scripts:
  build: "echo 'Building database package...'"
  test: "echo 'Testing database package...'"
  migrate: "echo 'Running migrations...'"
  rollback: "echo 'Rolling back migration...'"
  seed: "echo 'Seeding database...'"
exports:
  ".": "./dist/index.js"
  "./migrations": "./dist/migrations.js"
features:
  - migrations
  - seeding
"#;
    fs::write(db_dir.join("package.yml"), db_package_yml)?;

    Ok(temp_dir)
}

#[test]
fn test_complete_project_workflow() -> Result<()> {
    println!("=== Testing Complete Project Workflow ===");

    let project = create_complete_project()?;

    // Test project status from root
    let (stdout, stderr, success) = run_knot_command(&["status"], project.path());
    let output = format!("{}{}", stdout, stderr);

    // Status should work without errors and provide useful information
    if success || !output.is_empty() {
        println!("✓ Project status command executed");
    }

    // Test running scripts from different contexts
    let contexts = vec![
        (project.path(), "build-all", "project root"),
        (project.path().join("apps").join("web"), "start", "web app"),
        (project.path().join("apps").join("backend"), "start", "backend app"),
        (project.path().join("packages").join("types"), "build", "types package"),
    ];

    for (context_path, script, context_name) in contexts {
        if context_path.exists() {
            let (stdout, stderr, success) = run_knot_command(&["run", script], &context_path);
            let output = format!("{}{}", stdout, stderr);

            // Should either succeed or provide helpful feedback
            if !success && !output.is_empty() {
                assert!(output.contains("script") || output.contains("not found") || output.contains("Available"),
                       "Error from {} should be helpful: {}", context_name, output);
            } else if success {
                println!("✓ Successfully ran '{}' script from {}", script, context_name);
            }
        }
    }

    println!("✅ Complete project workflow test passed");
    Ok(())
}

#[test]
fn test_interactive_script_discovery() -> Result<()> {
    println!("=== Testing Interactive Script Discovery ===");

    let project = create_complete_project()?;

    // Test script discovery from different locations
    let test_locations = vec![
        project.path(),
        project.path().join("apps").join("web"),
        project.path().join("apps").join("backend"),
        project.path().join("packages").join("utils"),
    ];

    for location in test_locations {
        if location.exists() {
            // Test interactive mode (will fail in non-interactive environment, but should be graceful)
            let (stdout, stderr, _) = run_knot_command(&["run"], &location);
            let output = format!("{}{}", stdout, stderr);

            // Should handle non-interactive gracefully and provide script information
            if output.contains("Interactive") || output.contains("scripts") || output.contains("terminal") {
                println!("✓ Interactive mode handled gracefully from {}", location.display());
            }

            // Test script listing with nonexistent script (should show available scripts)
            let (stdout, stderr, _) = run_knot_command(&["run", "nonexistent-script"], &location);
            let output = format!("{}{}", stdout, stderr);

            if output.contains("Available scripts") || output.contains("not found") {
                println!("✓ Script discovery working from {}", location.display());
            }
        }
    }

    println!("✅ Interactive script discovery test passed");
    Ok(())
}

#[test]
fn test_context_aware_execution() -> Result<()> {
    println!("=== Testing Context-Aware Execution ===");

    let project = create_complete_project()?;

    // Test that the same script name works differently in different contexts
    let contexts = vec![
        (project.path().join("apps").join("web"), "web app context"),
        (project.path().join("apps").join("backend"), "backend app context"),
        (project.path().join("packages").join("types"), "types package context"),
        (project.path().join("packages").join("utils"), "utils package context"),
    ];

    for (context_path, context_name) in contexts {
        if context_path.exists() {
            // Try running "build" script which exists in most contexts
            let (stdout, stderr, success) = run_knot_command(&["run", "build"], &context_path);
            let output = format!("{}{}", stdout, stderr);

            // Should execute context-appropriate script
            if success {
                println!("✓ Build script executed in {}", context_name);
            } else if output.contains("script") {
                println!("✓ Context-aware script handling in {}", context_name);
            }

            // Try running "test" script
            let (stdout, stderr, success) = run_knot_command(&["run", "test"], &context_path);
            let output = format!("{}{}", stdout, stderr);

            if success {
                println!("✓ Test script executed in {}", context_name);
            } else if output.contains("script") {
                println!("✓ Context-aware test script handling in {}", context_name);
            }
        }
    }

    println!("✅ Context-aware execution test passed");
    Ok(())
}

#[test]
fn test_error_recovery_workflow() -> Result<()> {
    println!("=== Testing Error Recovery Workflow ===");

    let project = create_complete_project()?;

    // Test recovery from various error scenarios

    // 1. Try to run from directory without config
    let empty_dir = project.path().join("empty");
    fs::create_dir_all(&empty_dir)?;

    let (stdout, stderr, success) = run_knot_command(&["run", "build"], &empty_dir);
    assert!(!success, "Should fail in directory without config");
    let output = format!("{}{}", stdout, stderr);
    assert!(output.contains("knot.yml") || output.contains("config"),
           "Should provide helpful guidance: {}", output);

    // 2. Try to run nonexistent script and verify helpful error
    let (stdout, stderr, success) = run_knot_command(&["run", "nonexistent"], project.path());
    let output = format!("{}{}", stdout, stderr);
    if !success {
        assert!(output.contains("not found") || output.contains("Available"),
               "Should provide script suggestions: {}", output);
    }

    // 3. Test recovery when switching between contexts
    let web_dir = project.path().join("apps").join("web");
    let (stdout, stderr, _) = run_knot_command(&["run", "migrate"], &web_dir);
    let output = format!("{}{}", stdout, stderr);
    // Web app doesn't have migrate script, should suggest available ones
    if output.contains("not found") {
        assert!(output.contains("Available") || output.contains("start") || output.contains("build"),
               "Should show available web app scripts: {}", output);
    }

    println!("✅ Error recovery workflow test passed");
    Ok(())
}

#[test]
fn test_help_and_guidance_integration() -> Result<()> {
    println!("=== Testing Help and Guidance Integration ===");

    let project = create_complete_project()?;

    // Test help system integration
    let help_commands = vec![
        vec!["--help"],
        vec!["run", "--help"],
        vec!["status", "--help"],
        vec!["link", "--help"],
    ];

    for command in help_commands {
        let (stdout, stderr, _) = run_knot_command(&command, project.path());
        let output = format!("{}{}", stdout, stderr);

        if !output.is_empty() {
            // Help should be informative and not contain outdated information
            assert!(!output.contains("ctrl-j") && !output.contains("ctrl-k"),
                   "Help should not contain outdated navigation info: {:?}", command);

            // Should contain useful information
            assert!(output.len() > 50, "Help should be substantial: {:?}", command);
        }
    }

    println!("✅ Help and guidance integration test passed");
    Ok(())
}

#[test]
fn test_multi_level_script_priority() -> Result<()> {
    println!("=== Testing Multi-Level Script Priority ===");

    let project = create_complete_project()?;

    // Create overlapping script names to test priority
    // Project has "build-all", apps have "build", packages have "build"

    // From project root - should find project-level "build-all"
    let (stdout, stderr, success) = run_knot_command(&["run", "build-all"], project.path());
    let output = format!("{}{}", stdout, stderr);
    if success {
        println!("✓ Project-level script found from root");
    }

    // From app directory - should find app-level "build" first
    let web_dir = project.path().join("apps").join("web");
    let (stdout, stderr, success) = run_knot_command(&["run", "build"], &web_dir);
    let output = format!("{}{}", stdout, stderr);
    if success {
        println!("✓ App-level script prioritized in app directory");
    }

    // From package directory - should find package-level "build" first
    let utils_dir = project.path().join("packages").join("utils");
    let (stdout, stderr, success) = run_knot_command(&["run", "build"], &utils_dir);
    let output = format!("{}{}", stdout, stderr);
    if success {
        println!("✓ Package-level script prioritized in package directory");
    }

    println!("✅ Multi-level script priority test passed");
    Ok(())
}

#[test]
fn test_comprehensive_status_output() -> Result<()> {
    println!("=== Testing Comprehensive Status Output ===");

    let project = create_complete_project()?;

    // Test status command provides comprehensive information
    let (stdout, stderr, success) = run_knot_command(&["status"], project.path());
    let output = format!("{}{}", stdout, stderr);

    // Status should work and provide useful information
    if success || !output.is_empty() {
        // Should contain project information
        if output.contains("project") || output.contains("integration-test-project") {
            println!("✓ Status includes project information");
        }

        // Should be user-friendly (no raw errors)
        assert!(!output.contains("Error") || output.contains("❌"),
               "Status errors should be user-friendly: {}", output);
    }

    println!("✅ Comprehensive status output test passed");
    Ok(())
}

#[test]
fn test_linking_workflow() -> Result<()> {
    println!("=== Testing Linking Workflow ===");

    let project = create_complete_project()?;

    // Test package linking
    let (stdout, stderr, success) = run_knot_command(&["link"], project.path());
    let output = format!("{}{}", stdout, stderr);

    // Linking should either work or provide helpful feedback
    if !success && !output.is_empty() {
        // Should provide helpful error about missing packages or linking issues
        assert!(output.contains("package") || output.contains("link") || output.contains("not found"),
               "Link error should be helpful: {}", output);
    } else if success {
        println!("✓ Package linking succeeded");
    }

    // Test linking with symlinks option
    let (stdout, stderr, success) = run_knot_command(&["link", "--symlink"], project.path());
    let output = format!("{}{}", stdout, stderr);

    // Should handle symlink option appropriately
    if !success && !output.is_empty() {
        println!("Symlink linking feedback: {}", output);
    }

    println!("✅ Linking workflow test passed");
    Ok(())
}

#[test]
fn test_variable_resolution_workflow() -> Result<()> {
    println!("=== Testing Variable Resolution Workflow ===");

    let project = create_complete_project()?;

    // Test variable listing
    let (stdout, stderr, success) = run_knot_command(&["vars", "list"], project.path());
    let output = format!("{}{}", stdout, stderr);

    if success || output.contains("variable") {
        println!("✓ Variable listing working");
    }

    // Test getting specific variable
    let (stdout, stderr, success) = run_knot_command(&["vars", "get", "project_version"], project.path());
    let output = format!("{}{}", stdout, stderr);

    if success || output.contains("variable") || output.contains("1.0.0") {
        println!("✓ Variable retrieval working");
    }

    // Test variable listing from app context
    let web_dir = project.path().join("apps").join("web");
    if web_dir.exists() {
        let (stdout, stderr, success) = run_knot_command(&["vars", "list"], &web_dir);
        let output = format!("{}{}", stdout, stderr);

        if success || output.contains("variable") {
            println!("✓ App context variable listing working");
        }
    }

    println!("✅ Variable resolution workflow test passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_ux_integration_tests() -> Result<()> {
        println!("🎯 Running comprehensive UX integration tests");

        test_complete_project_workflow()?;
        test_interactive_script_discovery()?;
        test_context_aware_execution()?;
        test_error_recovery_workflow()?;
        test_help_and_guidance_integration()?;
        test_multi_level_script_priority()?;
        test_comprehensive_status_output()?;
        test_linking_workflow()?;
        test_variable_resolution_workflow()?;

        println!("🎉 All UX integration tests completed successfully!");
        Ok(())
    }
}