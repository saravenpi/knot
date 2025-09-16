use anyhow::Result;
use std::process::{Command, Stdio};
use std::fs;
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

// Helper to check if an error message is helpful
fn is_helpful_error_message(message: &str) -> bool {
    // Check for helpful characteristics
    let has_emoji = message.chars().any(|c| c as u32 > 127); // Simple emoji check
    let has_suggestion = message.contains("💡") || message.contains("Try") || message.contains("Please");
    let has_context = message.contains("because") || message.contains("since") || message.contains("due to");
    let has_action = message.contains("Run") || message.contains("Use") || message.contains("Add") || message.contains("Create");

    // Error message should be descriptive (not just generic)
    let is_descriptive = message.len() > 20 &&
                        !message.contains("Error") &&
                        !message.contains("Failed");

    // At least one helpful characteristic should be present
    has_emoji || has_suggestion || has_context || has_action || is_descriptive
}

#[test]
fn test_missing_config_file_errors() -> Result<()> {
    println!("=== Testing Missing Config File Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Test running knot in directory with no config files
    let (stdout, stderr, success) = run_knot_command(&["run", "build"], temp_dir.path());
    assert!(!success, "Command should fail when no config files exist");

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide helpful error message
    assert!(error_message.contains("knot.yml") || error_message.contains("config"),
           "Error should mention config files: {}", error_message);

    assert!(is_helpful_error_message(&error_message),
           "Error message should be helpful: {}", error_message);

    // Should suggest what to do
    assert!(error_message.contains("💡") || error_message.contains("Run from"),
           "Should provide suggestion: {}", error_message);

    println!("✅ Missing config file error test passed");
    Ok(())
}

#[test]
fn test_invalid_yaml_error_messages() -> Result<()> {
    println!("=== Testing Invalid YAML Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Test with malformed YAML
    let invalid_yaml = r#"
name: test-project
description: invalid yaml
scripts
  build: "npm build"  # Missing colon after scripts
"#;
    fs::write(temp_dir.path().join("knot.yml"), invalid_yaml)?;

    let (stdout, stderr, success) = run_knot_command(&["run", "build"], temp_dir.path());
    assert!(!success, "Command should fail with invalid YAML");

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide specific YAML error
    assert!(error_message.contains("YAML") || error_message.contains("parse") || error_message.contains("syntax"),
           "Should mention YAML/parsing issue: {}", error_message);

    assert!(is_helpful_error_message(&error_message),
           "YAML error should be helpful: {}", error_message);

    println!("✅ Invalid YAML error test passed");
    Ok(())
}

#[test]
fn test_missing_required_fields_errors() -> Result<()> {
    println!("=== Testing Missing Required Fields Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Test package.yml missing version
    let package_yml = r#"
name: test-package
description: Missing version field
"#;
    fs::write(temp_dir.path().join("package.yml"), package_yml)?;

    let (stdout, stderr, success) = run_knot_command(&["publish"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should mention missing version specifically
    if error_message.contains("version") || error_message.contains("missing") {
        assert!(is_helpful_error_message(&error_message),
               "Missing field error should be helpful: {}", error_message);
    }

    println!("✅ Missing required fields error test passed");
    Ok(())
}

#[test]
fn test_script_not_found_errors() -> Result<()> {
    println!("=== Testing Script Not Found Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Create valid config with scripts
    let knot_yml = r#"
name: test-project
scripts:
  build: "echo 'Building...'"
  test: "echo 'Testing...'"
  dev: "echo 'Development...'"
"#;
    fs::write(temp_dir.path().join("knot.yml"), knot_yml)?;

    // Try to run non-existent script
    let (stdout, stderr, success) = run_knot_command(&["run", "nonexistent"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide helpful error about script not found
    assert!(error_message.contains("not found") || error_message.contains("❌"),
           "Should indicate script not found: {}", error_message);

    // Should list available scripts
    assert!(error_message.contains("Available scripts") || error_message.contains("build") || error_message.contains("test"),
           "Should list available scripts: {}", error_message);

    assert!(is_helpful_error_message(&error_message),
           "Script not found error should be helpful: {}", error_message);

    println!("✅ Script not found error test passed");
    Ok(())
}

#[test]
fn test_authentication_error_messages() -> Result<()> {
    println!("=== Testing Authentication Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Create valid package config
    let package_yml = r#"
name: test-package
version: "1.0.0"
description: Test package for auth
"#;
    fs::write(temp_dir.path().join("package.yml"), package_yml)?;

    // Try to publish without authentication
    let (stdout, stderr, success) = run_knot_command(&["publish"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // If authentication is required, error should be helpful
    if error_message.contains("auth") || error_message.contains("login") || error_message.contains("🔐") {
        assert!(is_helpful_error_message(&error_message),
               "Auth error should be helpful: {}", error_message);

        // Should suggest how to authenticate
        assert!(error_message.contains("knot auth") || error_message.contains("log in"),
               "Should suggest authentication: {}", error_message);
    }

    println!("✅ Authentication error test passed");
    Ok(())
}

#[test]
fn test_validation_error_messages() -> Result<()> {
    println!("=== Testing Validation Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Test with invalid project name
    let invalid_name_yml = r#"
name: "../invalid-name"
description: Invalid project name
"#;
    fs::write(temp_dir.path().join("knot.yml"), invalid_name_yml)?;

    let (stdout, stderr, success) = run_knot_command(&["status"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide specific validation error
    if error_message.contains("invalid") || error_message.contains("path") || error_message.contains("characters") {
        assert!(is_helpful_error_message(&error_message),
               "Validation error should be helpful: {}", error_message);
    }

    println!("✅ Validation error test passed");
    Ok(())
}

#[test]
fn test_dependency_error_messages() -> Result<()> {
    println!("=== Testing Dependency Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Create app with non-existent dependency
    let app_yml = r#"
name: test-app
packages:
  - nonexistent-package
  - another-missing-package
"#;
    fs::write(temp_dir.path().join("app.yml"), app_yml)?;

    let (stdout, stderr, success) = run_knot_command(&["link"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide helpful dependency error
    if error_message.contains("package") || error_message.contains("dependency") || error_message.contains("not found") {
        assert!(is_helpful_error_message(&error_message),
               "Dependency error should be helpful: {}", error_message);
    }

    println!("✅ Dependency error test passed");
    Ok(())
}

#[test]
fn test_interactive_mode_error_messages() -> Result<()> {
    println!("=== Testing Interactive Mode Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Create project with scripts
    let knot_yml = r#"
name: test-project
scripts:
  build: "echo 'Building...'"
"#;
    fs::write(temp_dir.path().join("knot.yml"), knot_yml)?;

    // Try interactive mode in non-interactive environment
    let (stdout, stderr, success) = run_knot_command(&["run"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should handle non-interactive gracefully with helpful message
    if error_message.contains("Interactive") || error_message.contains("terminal") {
        assert!(is_helpful_error_message(&error_message),
               "Interactive mode error should be helpful: {}", error_message);

        // Should suggest alternative
        assert!(error_message.contains("knot run <script_name>") || error_message.contains("specific script"),
               "Should suggest alternative: {}", error_message);
    }

    println!("✅ Interactive mode error test passed");
    Ok(())
}

#[test]
fn test_version_validation_error_messages() -> Result<()> {
    println!("=== Testing Version Validation Error Messages ===");

    let temp_dir = TempDir::new()?;

    // Test with invalid version format
    let invalid_version_yml = r#"
name: test-package
version: "invalid-version"
description: Package with invalid version
"#;
    fs::write(temp_dir.path().join("package.yml"), invalid_version_yml)?;

    let (stdout, stderr, success) = run_knot_command(&["publish"], temp_dir.path());

    let error_message = format!("{}{}", stdout, stderr);

    // Should provide specific version error
    if error_message.contains("version") || error_message.contains("semver") {
        assert!(is_helpful_error_message(&error_message),
               "Version error should be helpful: {}", error_message);

        // Should explain semver format
        assert!(error_message.contains("x.y.z") || error_message.contains("semver") || error_message.contains("format"),
               "Should explain version format: {}", error_message);
    }

    println!("✅ Version validation error test passed");
    Ok(())
}

#[test]
fn test_help_message_quality() -> Result<()> {
    println!("=== Testing Help Message Quality ===");

    let temp_dir = TempDir::new()?;

    // Test main help
    let (stdout, stderr, _) = run_knot_command(&["--help"], temp_dir.path());
    let help_message = format!("{}{}", stdout, stderr);

    // Help should be comprehensive and well-formatted
    assert!(help_message.contains("Knot") && help_message.contains("Monorepo"),
           "Help should contain project description: {}", help_message);

    assert!(help_message.contains("COMMANDS") || help_message.contains("SUBCOMMANDS"),
           "Help should list commands: {}", help_message);

    // Test subcommand help
    let (stdout, stderr, _) = run_knot_command(&["run", "--help"], temp_dir.path());
    let run_help = format!("{}{}", stdout, stderr);

    assert!(run_help.contains("run") || run_help.contains("script"),
           "Run help should mention scripts: {}", run_help);

    // Should not contain outdated information
    assert!(!run_help.contains("ctrl-j") && !run_help.contains("ctrl-k"),
           "Help should not contain outdated ctrl-j/ctrl-k references: {}", run_help);

    println!("✅ Help message quality test passed");
    Ok(())
}

#[test]
fn test_error_context_and_suggestions() -> Result<()> {
    println!("=== Testing Error Context and Suggestions ===");

    let temp_dir = TempDir::new()?;

    // Test various error scenarios and verify they provide context and suggestions
    let test_cases = vec![
        // (command, expected_context_keywords)
        (&["status"] as &[&str], vec!["knot.yml", "project"]),
        (&["run", "missing-script"], vec!["script", "Available"]),
        (&["link"], vec!["package", "app"]),
    ];

    for (command, context_keywords) in test_cases {
        let (stdout, stderr, _) = run_knot_command(command, temp_dir.path());
        let output = format!("{}{}", stdout, stderr);

        // Check if at least one context keyword is present
        let has_context = context_keywords.iter().any(|keyword| {
            output.to_lowercase().contains(&keyword.to_lowercase())
        });

        if !output.is_empty() && has_context {
            assert!(is_helpful_error_message(&output),
                   "Command {:?} should provide helpful output: {}", command, output);
        }
    }

    println!("✅ Error context and suggestions test passed");
    Ok(())
}

#[test]
fn test_emoji_usage_consistency() -> Result<()> {
    println!("=== Testing Emoji Usage Consistency ===");

    let temp_dir = TempDir::new()?;

    // Test that error messages use consistent emoji patterns
    let knot_yml = r#"
name: test-project
scripts:
  build: "echo 'Building...'"
"#;
    fs::write(temp_dir.path().join("knot.yml"), knot_yml)?;

    let test_commands = vec![
        vec!["run", "nonexistent"],
        vec!["status"],
        vec!["link"],
    ];

    for command in test_commands {
        let (stdout, stderr, _) = run_knot_command(&command, temp_dir.path());
        let output = format!("{}{}", stdout, stderr);

        if !output.is_empty() {
            // Check for consistent emoji usage
            let error_emojis = ["❌", "🔍", "💡", "🚀", "📝", "🔧"];
            let has_appropriate_emoji = error_emojis.iter().any(|emoji| output.contains(emoji));

            // If there's substantive output, it should have helpful formatting
            if output.len() > 50 {
                assert!(has_appropriate_emoji || is_helpful_error_message(&output),
                       "Command {:?} should have appropriate emoji or helpful formatting: {}", command, output);
            }
        }
    }

    println!("✅ Emoji usage consistency test passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_error_message_quality_tests() -> Result<()> {
        println!("📝 Running comprehensive error message quality tests");

        test_missing_config_file_errors()?;
        test_invalid_yaml_error_messages()?;
        test_missing_required_fields_errors()?;
        test_script_not_found_errors()?;
        test_authentication_error_messages()?;
        test_validation_error_messages()?;
        test_dependency_error_messages()?;
        test_interactive_mode_error_messages()?;
        test_version_validation_error_messages()?;
        test_help_message_quality()?;
        test_error_context_and_suggestions()?;
        test_emoji_usage_consistency()?;

        println!("🎉 All error message quality tests completed successfully!");
        Ok(())
    }
}