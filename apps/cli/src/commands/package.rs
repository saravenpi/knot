use anyhow::{Context, Result};
use std::fs;

use crate::commands::common::{create_spinner, create_progress_bar, finish_progress, fail_progress, display_success, display_error, display_info};
use crate::config::AppConfig;
use crate::linker::Linker;
use crate::project::Project;
use crate::typescript::TypeScriptManager;
use crate::validation::{validate_package_spec, sanitize_input};

pub async fn link_packages(use_symlinks: bool) -> Result<()> {
    let start_time = std::time::Instant::now();

    let current_dir = std::env::current_dir()?;
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            display_error("Cannot link packages: No knot.yml found in current directory or any parent directory");
            display_info("You must be inside a Knot project to link packages");
            display_info("Run 'knot init <project-name>' to initialize a new Knot project");
            display_info("Or navigate to an existing Knot project directory");
            anyhow::bail!("Not in a Knot project");
        }
    };

    let mode = if use_symlinks { "symlinked" } else { "copied" };
    let spinner = create_spinner(&format!("Linking packages ({} mode)...", mode));

    let linker = Linker::new(&project);
    match linker.link_all_apps(use_symlinks).await {
        Ok(_) => {}
        Err(e) => {
            fail_progress(&spinner, "Failed to link packages");
            return Err(e.context("Failed to link packages to apps"));
        }
    }

    spinner.set_message("Setting up TypeScript aliases...".to_string());

    let ts_manager = TypeScriptManager::new(&project);
    match ts_manager.setup_aliases_for_all_apps() {
        Ok(_) => {}
        Err(e) => {
            fail_progress(&spinner, "Failed to setup TypeScript aliases");
            return Err(e.context("Failed to setup TypeScript aliases"));
        }
    }

    let duration = start_time.elapsed();
    finish_progress(&spinner, &format!("All packages {} and TypeScript configured", mode));
    display_success(&format!("Successfully {} all packages and updated TypeScript configurations", mode));
    display_info(&format!("Completed in {}ms", duration.as_millis()));
    Ok(())
}

pub async fn add_package(package_spec: &str, auto_link: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Validate and parse package specification
    let sanitized_spec = sanitize_input(package_spec);
    if sanitized_spec.is_empty() {
        anyhow::bail!("Package specification cannot be empty");
    }

    let (package_name, version) = validate_package_spec(&sanitized_spec)?;
    
    // Determine the version specification to save (inspired by npm)
    let version_spec = match version.as_ref() {
        Some(v) if v == "latest" => "latest".to_string(),
        Some(v) if v.starts_with('^') || v.starts_with('~') || v.starts_with('=') => {
            // Already has a range specifier, keep as-is
            v.to_string()
        },
        Some(v) if is_exact_version(v) => {
            // Exact version specified, save as-is for reproducible installs
            v.to_string()
        },
        Some(v) => {
            // Other version specifier, save as-is
            v.to_string()
        },
        None => "latest".to_string(), // Default to latest if no version specified
    };
    
    // Create the package specification for storage (always include version)
    let storage_spec = format!("{}@{}", package_name, version_spec);
    let display_name = storage_spec.clone();

    // Check if we're in an app directory
    let app_yml_path = current_dir.join("app.yml");
    if !app_yml_path.exists() {
        // Check if we're in project root and user specified app
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(_) => {
                anyhow::bail!("Cannot install package: Not in a Knot project directory\n💡 You must be inside a Knot project to install packages\n💡 Run 'knot init <project-name>' to initialize a new project\n💡 Or navigate to an existing Knot project directory\n💡 Then navigate to an app directory and run 'knot install' there");
            }
        };

        // List available apps for user reference
        if !project.apps.is_empty() {
            println!("📋 Available apps:");
            for app_name in project.apps.keys() {
                println!("  • {}", app_name);
            }
            println!("💡 Navigate to an app directory and run 'knot install' there");
        } else {
            println!("💡 Create an app first with 'knot init:app <name>'");
        }
        return Ok(());
    }

    // Load current app config
    let mut app_config = AppConfig::from_file(&app_yml_path)?;

    // Validate package name (without version)
    app_config.validate_package_name(&package_name)?;

    // Check if local package exists before adding to config
    if !package_name.starts_with('@') {
        // This is a local package, verify it exists
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(e) => {
                anyhow::bail!("Failed to load project configuration: {}\n💡 Ensure you're in a valid Knot project directory\n💡 Check that knot.yml exists and is properly formatted", e);
            }
        };
        
        // Check if the package exists in the project
        if !project.packages.contains_key(&package_name) {
            let available_packages = project.packages.keys().map(|s| s.as_str()).collect::<Vec<_>>();
            if available_packages.is_empty() {
                anyhow::bail!("Local package '{}' does not exist in this project\n💡 No packages found in this project\n💡 Create a package first with: knot init:package {}", package_name, package_name);
            } else {
                anyhow::bail!("Local package '{}' does not exist in this project\n💡 Available local packages: {}\n💡 Create the package with: knot init:package {}\n💡 Or install from Knot Space with: @team/{}",
                    package_name,
                    available_packages.join(", "),
                    package_name,
                    package_name
                );
            }
        }
    }

    // Initialize packages vector if it doesn't exist
    if app_config.packages.is_none() {
        app_config.packages = Some(Vec::new());
    }

    let packages = app_config.packages.as_mut().unwrap();

    // Check if package (with same version) is already added
    if packages.contains(&storage_spec) {
        println!("📦 Package '{}' is already added to app '{}'", display_name, app_config.name);
        return Ok(());
    }

    // Remove any existing versions of the same package
    packages.retain(|p| !p.starts_with(&format!("{}@", package_name)) && p != &package_name);

    // Add the package with version specification (always includes version)
    packages.push(storage_spec.clone());

    // Save updated config
    let yaml_content = serde_yaml::to_string(&app_config)?;
    fs::write(&app_yml_path, yaml_content)
        .context("Failed to update app.yml")?;

    println!("✅ Added package '{}' to app '{}'", display_name, app_config.name);

    // Auto-link if requested
    if auto_link {
        println!("🔗 Linking packages...");
        link_packages(false).await?;
    } else {
        println!("💡 Run 'knot link' to apply the changes");
    }

    Ok(())
}

// Parse package specification into name and version
fn parse_package_spec(package_spec: &str) -> (String, Option<String>) {
    if let Some(stripped) = package_spec.strip_prefix('@') {
        // Handle scoped packages like @hono-modules-loader@0.2.5
        if let Some(at_pos) = stripped.find('@') {
            // Found a second @ after the first one
            let at_pos = at_pos + 1; // Adjust for the skipped first character
            let name = package_spec[..at_pos].to_string();
            let version = package_spec[at_pos + 1..].to_string();
            (name, Some(version))
        } else {
            // No version specified, just the scoped package name
            (package_spec.to_string(), None)
        }
    } else {
        // Handle regular packages like package-name@1.0.0
        if let Some(at_pos) = package_spec.rfind('@') {
            let name = package_spec[..at_pos].to_string();
            let version = package_spec[at_pos + 1..].to_string();
            (name, Some(version))
        } else {
            (package_spec.to_string(), None)
        }
    }
}


// Check if version string is an exact semantic version (X.Y.Z format)
fn is_exact_version(version: &str) -> bool {
    // Check for basic semantic version pattern (e.g., 1.2.3, 0.1.0, 2.0.0-beta.1)
    // Split on '-' first to handle pre-release versions
    let main_version = version.split('-').next().unwrap_or(version);
    
    // Should have at least 2 dots for X.Y.Z
    let parts: Vec<&str> = main_version.split('.').collect();
    if parts.len() < 3 {
        return false;
    }
    
    // First 3 parts should be numeric
    parts.iter().take(3).all(|part| {
        !part.is_empty() && part.chars().all(|c| c.is_ascii_digit())
    })
}