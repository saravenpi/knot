use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::config::{KnotConfig, AppConfig, PackageConfig};
use crate::downloader::PackageDownloader;
use super::common::*;

pub fn init_project(name: Option<&str>, path: Option<&str>, description: Option<&str>) -> Result<()> {
    let interactive = name.is_none() || path.is_none();

    // Pretty header
    println!();
    println!("{}", style("🚀 Welcome to Knot Project Initializer").bold().cyan());
    println!("{}", style("Let's create a new Knot project together!").dim());
    println!();

    let project_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your project name?", None)?,
    };

    let project_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 Project description (optional)", Some("A new Knot project"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    let target_dir = match path {
        Some(p) => PathBuf::from(p),
        None if interactive => {
            let suggested_path = PathBuf::from(format!("./{}", project_name));
            let path_str = prompt_for_input("📁 Where should we create the project?", Some(suggested_path.to_str().unwrap_or(".")))?;
            PathBuf::from(path_str)
        }
        None => std::env::current_dir()?,
    };

    // Show summary and confirm
    if interactive {
        println!();
        println!("{}", style("📋 Project Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&project_name).bold());
        if let Some(desc) = &project_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Location:").dim(), target_dir.display());
        println!();

        if !prompt_for_confirm("Create this project?", Some(true))? {
            println!("{}", style("❌ Project creation cancelled").red());
            return Ok(());
        }
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let knot_yml_path = target_dir.join("knot.yml");

    if knot_yml_path.exists() {
        anyhow::bail!("knot.yml already exists in the target directory");
    }

    let config = KnotConfig {
        name: project_name.clone(),
        description: project_description,
        ts_alias: Some(crate::config::TsAlias::Boolean(false)),
        apps: None,
        scripts: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(knot_yml_path, yaml_content).context("Failed to create knot.yml")?;

    fs::create_dir_all(target_dir.join("packages"))?;
    fs::create_dir_all(target_dir.join("apps"))?;

    println!("✅ Initialized new Knot project: {}", project_name);
    println!("📁 Created directories: packages/, apps/");
    println!("💡 Use 'knot init:package <name>' to create packages");
    println!("💡 Use 'knot init:app <name>' to create apps");
    Ok(())
}

pub async fn init_package(name: Option<&str>, team: Option<&str>, version: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    // Pretty header for interactive mode
    if interactive {
        println!();
        println!("{}", style("📦 Welcome to Knot Package Creator").bold().cyan());
        println!("{}", style("Let's create a new package together!").dim());
        println!();
    }

    let package_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your package name?", None)?,
    };

    let team_name = match team {
        Some(t) => Some(t.to_string()),
        None if interactive => {
            let team = prompt_for_input("👥 Team name (optional, for namespaced packages)", None).ok();
            team.filter(|t| !t.trim().is_empty())
        },
        None => None,
    };

    let package_version = match version {
        Some(v) => v.to_string(),
        None if interactive => prompt_for_input("🏷️  Package version", Some("1.0.0"))?,
        None => "1.0.0".to_string(),
    };

    let package_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 Package description (optional)", Some("A new Knot package"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    // Ask if user wants to use a template
    let use_template = match template {
        Some(_) => true, // If template is provided via CLI, we're using it
        None if interactive => prompt_for_confirm("📦 Do you want to use a template from Knot Space?", Some(false))?,
        None => false,
    };

    let package_template = if use_template {
        match template {
            Some(t) => {
                // Ensure template starts with @
                if !t.starts_with('@') {
                    format!("@{}", t)
                } else {
                    t.to_string()
                }
            },
            None if interactive => {
                println!();
                println!("{}", style("Enter a package name to use as template.").dim());
                println!("{}", style("Examples: @svelte-starter, @react-with-auth, @my-team/template").dim());
                println!();
                let template_input = prompt_for_input("🎨 Template package name", None)?;
                // Ensure template starts with @
                if !template_input.starts_with('@') {
                    format!("@{}", template_input)
                } else {
                    template_input
                }
            },
            None => return Ok(()), // No template
        }
    } else {
        // No template - just create basic structure
        "".to_string()
    };

    // Determine target directory
    let (base_target_dir, context, _in_project) = if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else {
        match path {
            Some(p) => (PathBuf::from(p), format!("in specified path: {}", p), false),
            None => determine_target_directory(&current_dir, "packages")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&package_name)
    };

    // Show summary and confirm for interactive mode
    if interactive {
        println!();
        println!("{}", style("📋 Package Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&package_name).bold());
        if let Some(team) = &team_name {
            println!("   {} @{}/{}", style("Namespaced:").dim(), team, package_name);
        }
        println!("   {} {}", style("Version:").dim(), package_version);
        if let Some(desc) = &package_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Template:").dim(), package_template);
        println!("   {} {} ({})", style("Location:").dim(), target_dir.display(), context);
        println!();

        if !prompt_for_confirm("Create this package?", Some(true))? {
            println!("{}", style("❌ Package creation cancelled").red());
            return Ok(());
        }
    }

    // Handle path collision for non-here mode
    if !here && target_dir.exists() && !path.map(|p| p == ".").unwrap_or(false) {
        anyhow::bail!("Directory '{}' already exists", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let package_yml_path = target_dir.join("package.yml");

    if package_yml_path.exists() {
        anyhow::bail!("package.yml already exists in the target directory");
    }

    let config = PackageConfig {
        name: package_name.clone(),
        team: team_name.clone(),
        version: package_version.clone(),
        description: package_description.clone(),
        author: None,
        license: None,
        repository: None,
        keywords: None,
        tags: None,
        scripts: None,
        dependencies: None,
        dev_dependencies: None,
        optional_dependencies: None,
        peer_dependencies: None,
        exports: None,
        features: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&package_yml_path, yaml_content).context("Failed to create package.yml")?;

    // Generate package files using templates
    if !package_template.is_empty() {
        // Use published package as template
        use_package_as_template(
            &package_template,
            &target_dir,
            Some(&package_name),
            Some(&package_version),
            package_description.as_deref(),
        ).await?;
    } else {
        // Create basic structure without template
        create_basic_structure(&target_dir, true)?;
    }

    let display_name = if let Some(team) = &team_name {
        format!("@{}/{}", team, package_name)
    } else {
        package_name.clone()
    };

    println!("✅ Created package: {}", display_name);
    println!("📁 Location: {}", target_dir.display());

    if let Some(team) = &team_name {
        println!("👥 Team: @{}", team);
        println!("💡 Use 'knot publish --team {}' to publish this package", team);
    } else {
        println!("💡 Use 'knot publish' to publish this package");
    }

    Ok(())
}

pub async fn init_app(name: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    // Pretty header for interactive mode
    if interactive {
        println!();
        println!("{}", style("🚀 Welcome to Knot App Creator").bold().cyan());
        println!("{}", style("Let's create a new app together!").dim());
        println!();
    }

    let app_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your app name?", None)?,
    };

    let app_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 App description (optional)", Some("A new Knot app"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    // Ask if user wants to use a template
    let use_template = match template {
        Some(_) => true, // If template is provided via CLI, we're using it
        None if interactive => prompt_for_confirm("📦 Do you want to use a template from Knot Space?", Some(false))?,
        None => false,
    };

    let app_template = if use_template {
        match template {
            Some(t) => {
                // Ensure template starts with @
                if !t.starts_with('@') {
                    format!("@{}", t)
                } else {
                    t.to_string()
                }
            },
            None if interactive => {
                println!();
                println!("{}", style("Enter a package name to use as template.").dim());
                println!("{}", style("Examples: @svelte-app-starter, @nextjs-with-auth, @my-team/app-template").dim());
                println!();
                let template_input = prompt_for_input("🎨 Template package name", None)?;
                // Ensure template starts with @
                if !template_input.starts_with('@') {
                    format!("@{}", template_input)
                } else {
                    template_input
                }
            },
            None => return Ok(()), // No template
        }
    } else {
        // No template - just create basic structure
        "".to_string()
    };

    // Determine target directory
    let (base_target_dir, context, _in_project) = if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else {
        match path {
            Some(p) => (PathBuf::from(p), format!("in specified path: {}", p), false),
            None => determine_target_directory(&current_dir, "apps")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&app_name)
    };

    // Show summary and confirm for interactive mode
    if interactive {
        println!();
        println!("{}", style("📋 App Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&app_name).bold());
        if let Some(desc) = &app_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Template:").dim(), app_template);
        println!("   {} {} ({})", style("Location:").dim(), target_dir.display(), context);
        println!();

        if !prompt_for_confirm("Create this app?", Some(true))? {
            println!("{}", style("❌ App creation cancelled").red());
            return Ok(());
        }
    }

    // Handle path collision for non-here mode
    if !here && target_dir.exists() && !path.map(|p| p == ".").unwrap_or(false) {
        anyhow::bail!("Directory '{}' already exists", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let app_yml_path = target_dir.join("app.yml");

    if app_yml_path.exists() {
        anyhow::bail!("app.yml already exists in the target directory");
    }

    let config = AppConfig {
        name: app_name.clone(),
        description: app_description.clone(),
        ts_alias: None,
        packages: None,
        scripts: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&app_yml_path, yaml_content).context("Failed to create app.yml")?;

    // Generate app files using templates
    if !app_template.is_empty() {
        // Use published package as template
        use_package_as_template(
            &app_template,
            &target_dir,
            Some(&app_name),
            Some("1.0.0"),
            app_description.as_deref(),
        ).await?;
    } else {
        // Create basic structure without template
        create_basic_structure(&target_dir, false)?;
    }

    println!("✅ Created app: {}", app_name);
    println!("📁 Location: {}", target_dir.display());
    println!("💡 Use 'knot install <package>' to add dependencies");
    println!("💡 Use 'knot link' to install and link packages");

    Ok(())
}

// Function to download and use a published package as a template
async fn use_package_as_template(
    package_spec: &str,
    target_dir: &Path,
    name: Option<&str>,
    version: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    println!("📥 Downloading template package: {}", package_spec);

    // Create a temporary directory to download the package
    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path();

    // Download the package
    PackageDownloader::download_package(package_spec, download_path).await?;

    // Copy all files from the downloaded package to the target directory
    copy_template_files(download_path, target_dir)?;

    // Update package.yml or app.yml with the new name, version, and description if they exist
    update_config_files(target_dir, name, version, description)?;

    println!("✅ Successfully applied template from package: {}", package_spec);
    Ok(())
}

// Helper function to copy template files
fn copy_template_files(source: &Path, destination: &Path) -> Result<()> {
    use walkdir::WalkDir;

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        
        // Skip the source directory itself
        if path == source {
            continue;
        }

        let relative_path = path.strip_prefix(source)?;
        let dest_path = destination.join(relative_path);

        if entry.file_type().is_dir() {
            // Create directory if it doesn't exist
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }
        } else {
            // Skip config files, we'll handle them separately
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name == "package.yml" || file_name == "app.yml" || file_name == "knot.yml" {
                continue;
            }

            // Copy file
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }

    Ok(())
}

// Helper function to update config files with new values
fn update_config_files(
    target_dir: &Path,
    name: Option<&str>,
    version: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    // Try to update package.json if it exists
    let package_json_path = target_dir.join("package.json");
    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path)?;
        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(name) = name {
                json["name"] = serde_json::Value::String(name.to_string());
            }
            if let Some(version) = version {
                json["version"] = serde_json::Value::String(version.to_string());
            }
            if let Some(description) = description {
                json["description"] = serde_json::Value::String(description.to_string());
            }
            let updated_content = serde_json::to_string_pretty(&json)?;
            fs::write(&package_json_path, updated_content)?;
        }
    }

    Ok(())
}

// Create basic structure when no template is used
fn create_basic_structure(target_dir: &Path, is_package: bool) -> Result<()> {
    // Create src directory
    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    // Create a basic index file
    let index_path = src_dir.join("index.ts");
    let content = if is_package {
        "// Your package code here\nexport {};"
    } else {
        "// Your app code here\nconsole.log('Hello from Knot!');"
    };
    fs::write(index_path, content)?;

    // Create basic package.json
    let package_json = serde_json::json!({
        "name": target_dir.file_name().and_then(|n| n.to_str()).unwrap_or("my-project"),
        "version": "1.0.0",
        "main": "src/index.ts",
        "scripts": {
            "dev": "node src/index.ts"
        }
    });
    let package_json_path = target_dir.join("package.json");
    fs::write(&package_json_path, serde_json::to_string_pretty(&package_json)?)?;

    Ok(())
}