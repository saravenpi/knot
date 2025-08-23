use anyhow::{Context, Result};
use std::fs;

use crate::config::AppConfig;
use crate::linker::Linker;
use crate::project::Project;
use crate::typescript::TypeScriptManager;

pub async fn link_packages(use_symlinks: bool) -> Result<()> {
    let start_time = std::time::Instant::now();

    let current_dir = std::env::current_dir()?;
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            println!("❌ No knot.yml found in current directory or any parent directory");
            println!("💡 Run 'knot init <project-name>' to initialize a new Knot project");
            return Ok(());
        }
    };

    let linker = Linker::new(&project);
    linker
        .link_all_apps(use_symlinks)
        .await
        .context("Failed to link packages to apps")?;

    let ts_manager = TypeScriptManager::new(&project);
    ts_manager
        .setup_aliases_for_all_apps()
        .context("Failed to setup TypeScript aliases")?;

    let duration = start_time.elapsed();
    let mode = if use_symlinks { "symlinked" } else { "copied" };
    println!("🔗 Successfully {} all packages and updated TypeScript configurations", mode);
    println!("⚡ Linked in {}ms", duration.as_millis());
    Ok(())
}

pub async fn add_package(package_spec: &str, auto_link: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Parse package specification (name@version or name@latest or just name)
    let (package_name, version) = parse_package_spec(package_spec);
    let display_name = if let Some(v) = &version {
        format!("{}@{}", package_name, v)
    } else {
        format!("{}@latest", package_name)
    };

    // Check if we're in an app directory
    let app_yml_path = current_dir.join("app.yml");
    if !app_yml_path.exists() {
        // Check if we're in project root and user specified app
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(_) => {
                anyhow::bail!("❌ Not in an app directory or project root. Run 'knot install' from an app directory.");
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

    // Initialize packages vector if it doesn't exist
    if app_config.packages.is_none() {
        app_config.packages = Some(Vec::new());
    }

    let packages = app_config.packages.as_mut().unwrap();

    // Check if package (with same version) is already added
    if packages.contains(&package_spec.to_string()) {
        println!("📦 Package '{}' is already added to app '{}'", display_name, app_config.name);
        return Ok(());
    }

    // Remove any existing versions of the same package
    packages.retain(|p| !p.starts_with(&format!("{}@", package_name)) && p != &package_name);

    // Add the package with version specification
    packages.push(package_spec.to_string());

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
    if let Some(at_pos) = package_spec.rfind('@') {
        let name = package_spec[..at_pos].to_string();
        let version = package_spec[at_pos + 1..].to_string();
        (name, Some(version))
    } else {
        (package_spec.to_string(), None)
    }
}