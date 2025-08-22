#![allow(dead_code, unused_imports, clippy::all)]
use crate::config::{AppConfig, KnotConfig, PackageConfig, parse_yaml_error_to_user_friendly};
use crate::ignore::KnotIgnore;
use crate::linker::Linker;
use crate::project::Project;
use crate::templates::{TemplateManager, TemplateCategory};
use crate::typescript::TypeScriptManager;
use anyhow::{Context, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use inquire::{Text, Select, Confirm};
use console::style;

// Check if running in interactive environment
fn is_interactive() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stdin())
}

// Helper function for interactive input with beautiful UI
fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            anyhow::bail!("Interactive input required but running in non-interactive environment. Please provide values via command line arguments.");
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    let is_required = default.is_none();
    text_prompt = text_prompt.with_validator(move |input: &str| {
        if input.trim().is_empty() && is_required {
            Ok(inquire::validator::Validation::Invalid("This field is required".into()))
        } else if !input.trim().is_empty() && !input.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ') {
            Ok(inquire::validator::Validation::Invalid("Please use only letters, numbers, hyphens, underscores, and spaces".into()))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    });

    Ok(text_prompt.prompt()?)
}

// Enhanced input for descriptions (allows more characters)
fn prompt_for_description(prompt: &str, default: Option<&str>) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            return Ok(String::new()); // Optional descriptions can be empty
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    Ok(text_prompt.prompt()?)
}

// Enhanced select prompt
fn prompt_for_select(prompt: &str, options: Vec<&str>) -> Result<String> {
    // Fallback to first option if not interactive
    if !is_interactive() {
        return Ok(options.first().unwrap_or(&"basic").to_string());
    }

    let selection = Select::new(prompt, options).prompt()?;
    Ok(selection.to_string())
}

// Enhanced confirm prompt
fn prompt_for_confirm(prompt: &str, default: Option<bool>) -> Result<bool> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val);
        } else {
            return Ok(false); // Default to false if no default provided
        }
    }

    let mut confirm_prompt = Confirm::new(prompt);

    if let Some(default_val) = default {
        confirm_prompt = confirm_prompt.with_default(default_val);
    }

    Ok(confirm_prompt.prompt()?)
}


// Helper function to determine the best directory for creating packages/apps
fn determine_target_directory(current_dir: &Path, item_type: &str) -> Result<(PathBuf, String, bool)> {
    // Try to find project root
    match Project::find_project_root(current_dir) {
        Ok(project_root) => {
            // We're in a Knot project
            let target_dir = match item_type {
                "packages" => project_root.join("packages"),
                "apps" => project_root.join("apps"),
                _ => current_dir.to_path_buf(),
            };

            let context = if project_root == *current_dir {
                format!("in project root, will create in {}/ directory", item_type)
            } else {
                format!("in Knot project, will create in {}/ directory relative to project root", item_type)
            };

            Ok((target_dir, context, true))
        },
        Err(_) => {
            // Not in a Knot project, create in current directory
            let context = format!("outside Knot project, will create in current directory");
            Ok((current_dir.to_path_buf(), context, false))
        }
    }
}

// Helper function to format API error responses in a user-friendly way
fn format_api_error(status: reqwest::StatusCode, response_text: &str) -> String {
    // Try to parse as JSON first to extract the actual error message
    let error_message = if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response_text) {
        json_value
            .get("error")
            .or_else(|| json_value.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or(response_text)
            .to_string()
    } else {
        response_text.to_string()
    };

    // Provide user-friendly context based on status code and add appropriate emoji
    match status.as_u16() {
        400 => format!("❌ {}", error_message),
        401 => format!("🔐 Authentication failed. Please log in again with 'knot auth'"),
        403 => format!("🚫 You don't have permission to perform this action"),
        404 => format!("🔍 Resource not found. {}", error_message),
        409 => format!("⚠️  {}", error_message),
        500..=599 => format!("🔥 Server error. Please try again later"),
        _ => format!("❌ {}", error_message),
    }
}

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

pub fn init_package(name: Option<&str>, team: Option<&str>, version: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none() || path.is_none();

    // Pretty header
    println!();
    println!("{}", style("📦 Welcome to Knot Package Creator").bold().magenta());
    println!("{}", style("Let's create a new package!").dim());
    println!();

    let package_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your package name?", None)?,
    };

    // Enhanced team selection with interactive prompt
    let _package_team = match team {
        Some(t) => Some(t.to_string()),
        None if interactive => {
            let use_team = prompt_for_confirm("🏢 Is this package for a team?", Some(false))?;
            if use_team {
                Some(prompt_for_input("Team name", None)?)
            } else {
                None
            }
        },
        None => None,
    };

    // Enhanced version with better default
    let _package_version = match version {
        Some(v) => v.to_string(),
        None => {
            if interactive {
                prompt_for_input("📌 Initial version", Some("1.0.0"))?
            } else {
                "1.0.0".to_string()
            }
        }
    };

    // Enhanced template selection
    let _package_template = match template {
        Some(t) => Some(t.to_string()),
        None if interactive => {
            let available_templates = vec!["basic", "typescript", "react", "vue", "svelte"];
            let template_choice = prompt_for_select("🎨 Choose a template", available_templates)?;
            Some(template_choice)
        },
        None => None,
    };

    let _package_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 Package description (optional)", Some(""))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    // Determine target directory based on options
    let (base_dir, context, in_project) = if let Some(custom_path) = path {
        let target_path = if custom_path == "." {
            current_dir.clone()
        } else {
            Path::new(custom_path).to_path_buf()
        };

        if !target_path.exists() {
            anyhow::bail!("Specified path '{}' does not exist", custom_path);
        }

        (target_path, format!("using custom path '{}'", custom_path), false)
    } else if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else if interactive {
        let (suggested_base, _, in_project) = determine_target_directory(&current_dir, "packages")?;
        let suggested_path = suggested_base.join(&package_name);
        let path_str = prompt_for_input("📁 Target directory", Some(suggested_path.to_str().unwrap_or(".")))?;
        (PathBuf::from(path_str), "using custom path".to_string(), in_project)
    } else {
        determine_target_directory(&current_dir, "packages")?
    };

    // Create target directory if needed
    if !base_dir.exists() {
        fs::create_dir_all(&base_dir)?;
    }

    // Determine final package directory
    let package_dir = if here || (path.is_some() && path.unwrap() == ".") {
        // Create package files directly in the specified directory
        base_dir
    } else {
        // Create a new subdirectory for the package
        let pkg_dir = base_dir.join(&package_name);
        if pkg_dir.exists() {
            anyhow::bail!("Package directory '{}' already exists", package_name);
        }
        pkg_dir
    };

    // Create package directory if it doesn't exist
    if !package_dir.exists() {
        fs::create_dir_all(&package_dir)?;
    }

    // Check if package.yml already exists (when using --here or --path .)
    let package_yml_path = package_dir.join("package.yml");
    if package_yml_path.exists() {
        anyhow::bail!("package.yml already exists in the target directory. Choose a different location or remove the existing file.");
    }

    println!("📦 Creating package '{}' {}", package_name, context);
    if in_project {
        println!("✨ Detected Knot project - package will be available for linking");
    }

    // Use template if specified
    if let Some(template_name) = template {
        let pkg_version = version.unwrap_or("0.1.0");
        let pkg_description = description.unwrap_or("Package description");

        TemplateManager::create_from_template(
            &package_name,
            pkg_version,
            pkg_description,
            template_name,
            TemplateCategory::Package,
            &package_dir,
        )?;

        println!("✅ Initialized new {} package: {}", template_name, package_name);
        println!("📁 Created at: {}", package_dir.display());
        
        if let Some(template_info) = TemplateManager::get_package_template_info(template_name) {
            println!("🎯 Template: {} - {}", template_info.name, template_info.description);
        }
    } else {
        // Create basic package without template
        let config = PackageConfig {
            name: package_name.clone(),
            team: team.map(|s| s.to_string()),
            version: version.unwrap_or("0.1.0").to_string(),
            description: Some(description.unwrap_or("Package description").to_string()),
            tags: Some(vec!["utilities".to_string()]),
            scripts: None,
        };

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content).context("Failed to create package.yml")?;

        println!("✅ Initialized new package: {}", package_name);
        println!("📁 Created at: {}", package_dir.display());
        println!("💡 Use '--template typescript' or '--template react' for structured templates");
    }

    Ok(())
}

pub fn init_app(name: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none() || path.is_none();

    println!("🚀 Let's create a new app!");

    let app_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ App name", None)?,
    };

    // Determine target directory based on options
    let (base_dir, context, in_project) = if let Some(custom_path) = path {
        let target_path = if custom_path == "." {
            current_dir.clone()
        } else {
            Path::new(custom_path).to_path_buf()
        };

        if !target_path.exists() {
            anyhow::bail!("Specified path '{}' does not exist", custom_path);
        }

        (target_path, format!("using custom path '{}'", custom_path), false)
    } else if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else if interactive {
        let (suggested_base, _, in_project) = determine_target_directory(&current_dir, "apps")?;
        let suggested_path = suggested_base.join(&app_name);
        let path_str = prompt_for_input("📁 Target directory", Some(suggested_path.to_str().unwrap_or(".")))?;
        (PathBuf::from(path_str), "using custom path".to_string(), in_project)
    } else {
        determine_target_directory(&current_dir, "apps")?
    };

    // Create target directory if needed
    if !base_dir.exists() {
        fs::create_dir_all(&base_dir)?;
    }

    // Determine final app directory
    let app_dir = if here || (path.is_some() && path.unwrap() == ".") {
        // Create app files directly in the specified directory
        base_dir
    } else {
        // Create a new subdirectory for the app
        let app_subdir = base_dir.join(&app_name);
        if app_subdir.exists() {
            anyhow::bail!("App directory '{}' already exists", app_name);
        }
        app_subdir
    };

    // Create app directory if it doesn't exist
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    // Check if app.yml already exists (when using --here or --path .)
    let app_yml_path = app_dir.join("app.yml");
    if app_yml_path.exists() {
        anyhow::bail!("app.yml already exists in the target directory. Choose a different location or remove the existing file.");
    }

    println!("🚀 Creating app '{}' {}", app_name, context);
    if in_project {
        println!("✨ Detected Knot project - app will be available for building and linking");
    }

    // Use template if specified
    if let Some(template_name) = template {
        let app_version = "0.1.0";
        let app_description = description.unwrap_or("App description");

        TemplateManager::create_from_template(
            &app_name,
            app_version,
            app_description,
            template_name,
            TemplateCategory::App,
            &app_dir,
        )?;

        // Create app.yml for Knot configuration
        let config = AppConfig {
            name: app_name.clone(),
            description: description.map(|s| s.to_string()),
            ts_alias: None,
            packages: None,
            scripts: None,
        };

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&app_yml_path, yaml_content).context("Failed to create app.yml")?;

        println!("✅ Initialized new {} app: {}", template_name, app_name);
        println!("📁 Created at: {}", app_dir.display());
        
        if let Some(template_info) = TemplateManager::get_app_template_info(template_name) {
            println!("🎯 Template: {} - {}", template_info.name, template_info.description);
        }

        if !here && path.map(|p| p == ".").unwrap_or(false) == false {
            println!("💡 Run 'cd {}' then 'npm install' to get started", app_name);
        } else {
            println!("💡 Run 'npm install' to get started");
        }
    } else {
        // Create basic app without template
        let config = AppConfig {
            name: app_name.clone(),
            description: description.map(|s| s.to_string()),
            ts_alias: None,
            packages: None,
            scripts: None,
        };

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&app_yml_path, yaml_content).context("Failed to create app.yml")?;

        let src_dir = app_dir.join("src");
        fs::create_dir_all(src_dir)?;

        println!("✅ Initialized new app: {}", app_name);
        println!("📁 Created at: {}", app_dir.display());
        println!("💡 Use '--template react' or '--template svelte' for structured templates");
    }

    Ok(())
}

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

pub fn show_status() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(e) => {
            // First check if we can find the project root (file exists)
            match Project::find_project_root(&current_dir) {
                Ok(project_root) => {
                    let config_path = project_root.join("knot.yml");
                    println!("🔍 Found knot.yml at: {}", config_path.display());
                    println!("❌ Failed to load knot.yml: {}", e);
                    println!("💡 This could be due to:");
                    println!("   • Invalid YAML syntax");
                    println!("   • Missing required fields");
                    println!("   • File permission issues");
                    println!("   • Corrupted file content");
                    println!("\n🛠️  Try running 'knot init --help' to create a new valid knot.yml");
                }
                Err(_) => {
                    println!("❌ No knot.yml found in current directory or any parent directory");
                    println!("💡 Run 'knot init <project-name>' to initialize a new Knot project");
                }
            }
            return Ok(());
        }
    };

    println!("📋 Project: {}", project.config.name);
    if let Some(desc) = &project.config.description {
        println!("Description: {}", desc);
    }
    println!("Root: {}", project.root.display());

    println!("\n📦 Packages ({})", project.packages.len());
    for (name, config) in &project.packages {
        print!("  {} (v{})", name, config.version);
        if let Some(team) = &config.team {
            print!(" - team: {}", team);
        }
        println!();
    }

    println!("\n🚀 Apps ({})", project.apps.len());
    for (name, config) in &project.apps {
        println!("  {}", name);
        if let Some(desc) = &config.description {
            println!("    Description: {}", desc);
        }
        let deps = project.get_app_dependencies(name);
        if !deps.is_empty() {
            println!("    Dependencies: {}", deps.join(", "));
        }
        if let Some(alias) = project.get_app_ts_alias(name) {
            println!("    TypeScript alias: {}", alias);
        }
    }

    Ok(())
}



pub async fn run_script(script_name: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // First try to find script in current directory config files
    // Priority: app.yml > package.yml > knot.yml (project root)

    // Check if we're in an app directory (has app.yml)
    let app_yml_path = current_dir.join("app.yml");
    if app_yml_path.exists() {
        if let Some(script_command) = find_script_in_app(&app_yml_path, script_name)? {
            return execute_script(script_name, &script_command, &current_dir, "app").await;
        }
    }

    // Check if we're in a package directory (has package.yml)
    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        if let Some(script_command) = find_script_in_package(&package_yml_path, script_name)? {
            return execute_script(script_name, &script_command, &current_dir, "package").await;
        }
    }

    // Check project root for knot.yml
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            println!("❌ No knot.yml, app.yml, or package.yml found");
            println!("💡 Run from a directory containing one of these config files");
            return Ok(());
        }
    };

    // Check knot.yml for scripts
    if let Some(scripts) = &project.config.scripts {
        if let Some(script_command) = scripts.get(script_name) {
            return execute_script(script_name, script_command, &project.root, "project").await;
        }
    }

    // If script not found anywhere, show available scripts
    println!("❌ Script '{}' not found", script_name);
    show_available_scripts(&current_dir, &project).await?;

    Ok(())
}

fn find_script_in_app(app_yml_path: &Path, script_name: &str) -> Result<Option<String>> {
    let app_config = AppConfig::from_file(app_yml_path)?;
    Ok(app_config
        .scripts
        .as_ref()
        .and_then(|scripts| scripts.get(script_name))
        .cloned())
}

fn find_script_in_package(package_yml_path: &Path, script_name: &str) -> Result<Option<String>> {
    let package_config = PackageConfig::from_file(package_yml_path)?;
    Ok(package_config
        .scripts
        .as_ref()
        .and_then(|scripts| scripts.get(script_name))
        .cloned())
}

async fn execute_script(
    script_name: &str,
    script_command: &str,
    working_dir: &Path,
    context: &str,
) -> Result<()> {
    // Validate script name and command
    if script_name.is_empty() {
        anyhow::bail!("Script name cannot be empty");
    }

    if script_command.is_empty() {
        anyhow::bail!("Script command cannot be empty");
    }

    // Check working directory exists and is accessible
    if !working_dir.exists() {
        anyhow::bail!(
            "Working directory does not exist: {}",
            working_dir.display()
        );
    }

    if !working_dir.is_dir() {
        anyhow::bail!(
            "Working directory is not a directory: {}",
            working_dir.display()
        );
    }

    println!("🚀 Running {} script '{}'...", context, script_name);
    println!("📝 Command: {}", script_command);
    println!("📂 Working directory: {}", working_dir.display());

    // Use shell execution for complex commands (safer than manual parsing)
    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };

    let shell_flag = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };

    let mut cmd = tokio::process::Command::new(shell);
    cmd.arg(shell_flag);
    cmd.arg(script_command);
    cmd.current_dir(working_dir);
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let status = cmd.status().await.with_context(|| {
        format!(
            "Failed to execute script '{}': {}",
            script_name, script_command
        )
    })?;

    if status.success() {
        println!("✅ Script '{}' completed successfully", script_name);
    } else {
        let exit_code = status.code().unwrap_or(-1);
        anyhow::bail!(
            "Script '{}' failed with exit code: {}",
            script_name,
            exit_code
        );
    }

    Ok(())
}

async fn show_available_scripts(current_dir: &Path, project: &Project) -> Result<()> {
    println!("💡 Available scripts:");

    let mut found_any = false;

    // Show app scripts if in app directory
    let app_yml_path = current_dir.join("app.yml");
    if app_yml_path.exists() {
        if let Ok(app_config) = AppConfig::from_file(&app_yml_path) {
            if let Some(scripts) = &app_config.scripts {
                if !scripts.is_empty() {
                    println!("  📱 App scripts ({})", app_config.name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        }
    }

    // Show package scripts if in package directory
    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        if let Ok(package_config) = PackageConfig::from_file(&package_yml_path) {
            if let Some(scripts) = &package_config.scripts {
                if !scripts.is_empty() {
                    println!("  📦 Package scripts ({})", package_config.name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        }
    }

    // Show project scripts
    if let Some(scripts) = &project.config.scripts {
        if !scripts.is_empty() {
            println!("  🏗️  Project scripts ({})", project.config.name);
            for (name, command) in scripts {
                println!("    • {} - {}", name, command);
            }
            found_any = true;
        }
    }

    if !found_any {
        println!("  (No scripts defined in any config files)");
        println!("  💡 Add scripts to knot.yml, app.yml, or package.yml");
        println!("  Example:");
        println!("    scripts:");
        println!("      test: \"npm test\"");
        println!("      lint: \"eslint src/\"");
        println!("      dev: \"npm run dev\"");
    }

    Ok(())
}

// API Models

#[derive(Serialize, Deserialize)]
struct UserProfile {
    id: String,
    username: String,
    email: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user: UserProfile,
}

#[derive(Serialize, Deserialize)]
struct CreateTeamRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Team {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "ownerId")]
    owner_id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    owner: UserProfile,
    _count: TeamCounts,
    members: Vec<TeamMember>,
}

#[derive(Serialize, Deserialize)]
struct TeamCounts {
    members: u32,
    packages: u32,
}

#[derive(Serialize, Deserialize)]
struct TeamMember {
    id: String,
    #[serde(rename = "teamId")]
    team_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    role: String,
    #[serde(rename = "joinedAt")]
    joined_at: String,
    user: UserProfile,
}

#[derive(Serialize, Deserialize)]
struct AddTeamMemberRequest {
    username: String,
    role: String, // Should be "owner", "admin", or "member"
}

#[derive(Serialize, Deserialize)]
struct PublishPackageRequest {
    name: String,
    version: String,
    description: Option<String>,
    #[serde(rename = "teamName")]
    team_name: Option<String>,
    tags: Option<Vec<String>>,
}

// Helper functions
fn get_knot_space_url() -> String {
    env::var("KNOT_SPACE_URL").unwrap_or_else(|_| "https://knot-space-production.up.railway.app".to_string())
}

fn get_auth_token() -> Result<Option<String>> {
    // Try environment variable first
    if let Ok(token) = env::var("KNOT_TOKEN") {
        return Ok(Some(token));
    }

    // Try config file
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return Ok(None),
    };

    let config_file = home_dir.join(".knot").join("config");
    if config_file.exists() {
        let content = fs::read_to_string(config_file)?;
        for line in content.lines() {
            if line.starts_with("token=") {
                return Ok(Some(line[6..].to_string()));
            }
        }
    }

    Ok(None)
}


fn require_auth_token() -> Result<String> {
    get_auth_token()?
        .ok_or_else(|| anyhow::anyhow!("Authentication required. Set KNOT_TOKEN environment variable."))
}

// API Commands
pub async fn auth_status() -> Result<()> {
    match get_auth_token()? {
        Some(token) => {
            // Verify token by making a request to the profile endpoint
            let base_url = get_knot_space_url();
            let url = format!("{}/api/auth/profile", base_url);

            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;

            if response.status().is_success() {
                let api_response: ApiResponse<AuthResponse> = response.json().await?;
                if let Some(auth_data) = api_response.data {
                    println!("✅ Authenticated as: {}", auth_data.user.username);
                    println!("📧 Email: {}", auth_data.user.email);
                    println!("🔑 Token: {}", if token.len() > 20 {
                        format!("{}...{}", &token[..8], &token[token.len()-8..])
                    } else {
                        token
                    });
                    println!("🌐 Server: {}", base_url);
                } else {
                    println!("❌ Invalid response from server");
                }
            } else {
                println!("❌ Authentication token is invalid or expired");
                println!("💡 Get a new token from the Knot Space web interface at {}", base_url);
                println!("   Then set it with: export KNOT_TOKEN=<your-token>");
            }
        }
        None => {
            println!("❌ Not authenticated");
            println!("💡 Set your KNOT_TOKEN environment variable:");
            println!("   1. Visit {} and go to Settings", get_knot_space_url());
            println!("   2. Copy your API token");
            println!("   3. Set the token: export KNOT_TOKEN=<your-token>");
        }
    }

    Ok(())
}


pub async fn publish_package(team: Option<&str>, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    // Check if we're in a package directory
    let package_yml = Path::new("package.yml");
    if !package_yml.exists() {
        anyhow::bail!("No package.yml found. Run this command from a package directory.");
    }

    // Load package config
    let package_config = PackageConfig::from_file(package_yml)?;

    // Validate version is not a development version (should not end with -dev, -alpha, -beta without publishing intentionally)
    validate_publish_version(&package_config.version)?;

    // Check if package has been published before with this version
    if let Err(e) = check_version_exists(&package_config.name, &package_config.version, &token).await {
        println!("⚠️  Warning: Could not verify version uniqueness: {}", e);
        println!("💡 Continuing with publish attempt...");
    }

    // Create package tarball
    let tarball_path = format!("{}-{}.tar.gz", package_config.name, package_config.version);
    create_package_tarball(&package_config.name, &tarball_path)?;

    // Prepare metadata
    let metadata = PublishPackageRequest {
        name: package_config.name.clone(),
        version: package_config.version,
        description: description
            .map(|s| s.to_string())
            .or(package_config.description.clone()),
        team_name: team.map(|s| s.to_string()).or(package_config.team),
        tags: Some(package_config.tags.unwrap_or_default()),
    };

    let base_url = get_knot_space_url();
    let client = reqwest::Client::new();

    // 1. Send metadata
    let metadata_url = format!("{}/api/packages", base_url);
    let metadata_response = client
        .post(&metadata_url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&metadata)
        .send()
        .await?;

    if !metadata_response.status().is_success() {
        let status = metadata_response.status();
        let text = metadata_response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Failed to publish package: {}", formatted_error);
    }

    // 2. Upload tarball
    let upload_url = format!("{}/api/packages/upload", base_url);
    let file_content = std::fs::read(&tarball_path)?;
    let file_part = multipart::Part::bytes(file_content)
        .file_name(tarball_path.clone())
        .mime_str("application/gzip")?;

    let form = multipart::Form::new()
        .part("file", file_part)
        .text("packageName", metadata.name.clone())
        .text("version", metadata.version.clone());

    let upload_response = client
        .post(&upload_url)
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await?;

    // Clean up tarball
    let _ = std::fs::remove_file(&tarball_path);

    if upload_response.status().is_success() {
        println!(
            "📦 Successfully published {} v{}",
            metadata.name, metadata.version
        );
        if let Some(team_name) = &metadata.team_name {
            println!("   Team: {}", team_name);
        }
    } else {
        let status = upload_response.status();
        let text = upload_response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Failed to upload package files: {}", formatted_error);
    }

    Ok(())
}

pub async fn delete_package(name: &str, version: &str) -> Result<()> {
    let token = require_auth_token()?;

    let base_url = get_knot_space_url();
    let url = format!("{}/api/packages/{}/{}", base_url, name, version);

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        println!("🗑️  Successfully deleted {} v{}", name, version);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Failed to delete package: {}", formatted_error);
    }

    Ok(())
}

pub async fn create_team(name: &str, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    let request = CreateTeamRequest {
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
    };

    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Team>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(team) = api_response.data {
                        println!("👥 Created team: {}", team.name);
                        if let Some(desc) = team.description {
                            println!("   Description: {}", desc);
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to create team: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to create team: {}", message);
                        }
                        else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Failed to create team: {}", formatted_error);
    }

    Ok(())
}

pub async fn list_teams() -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Vec<Team>>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(teams) = api_response.data {
                        if teams.is_empty() {
                            println!("No teams found.");
                        } else {
                            println!("👥 Teams:");
                            for team in teams {
                                println!(
                                    "  • {} - {}",
                                    team.name,
                                    team.description.unwrap_or("No description".to_string())
                                );
                            }
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                // Attempt to parse as a generic JSON value to provide a better error message
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to list teams: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to list teams: {}", message);
                        } else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not retrieve teams: {}", formatted_error);
    }

    Ok(())
}

pub async fn team_info(name: &str) -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams/{}", base_url, name);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Team>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(team) = api_response.data {
                        println!("👥 Team: {}", team.name);
                        if let Some(desc) = team.description {
                            println!("   Description: {}", desc);
                        }
                        println!("   Created: {}", team.created_at);
                        println!("   Members:");
                        for member in team.members {
                            println!("     • {} ({})", member.user.username, member.role);
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to get team info: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to get team info: {}", message);
                        } else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not retrieve team information: {}", formatted_error);
    }

    Ok(())
}

pub async fn add_team_member(team: Option<&str>, username: Option<&str>, role: &str) -> Result<()> {
    let token = require_auth_token()?;

    // Interactive prompts for missing arguments
    let team_name = match team {
        Some(t) => t.to_string(),
        None => {
            if is_interactive() {
                prompt_for_input("🏢 Team name", None)?
            } else {
                anyhow::bail!("Team name is required. Use: knot team add-member <team> <username>");
            }
        }
    };

    let username_str = match username {
        Some(u) => u.to_string(),
        None => {
            if is_interactive() {
                prompt_for_input("👤 Username to add", None)?
            } else {
                anyhow::bail!("Username is required. Use: knot team add-member <team> <username>");
            }
        }
    };

    let request = AddTeamMemberRequest {
        username: username_str.clone(),
        role: role.to_string(),
    };

    let base_url = get_knot_space_url();
    // URL encode the team name to handle special characters
    let encoded_team = urlencoding::encode(&team_name);
    let url = format!("{}/api/teams/{}/members", base_url, encoded_team);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Added {} to team {} as {}", username_str, team_name, role);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not add team member: {}", formatted_error);
    }

    Ok(())
}

pub async fn remove_team_member(team: Option<&str>, username: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    // Interactive prompts for missing arguments
    let team_name = match team {
        Some(t) => t.to_string(),
        None => {
            if is_interactive() {
                prompt_for_input("🏢 Team name", None)?
            } else {
                anyhow::bail!("Team name is required. Use: knot team remove-member <team> <username>");
            }
        }
    };

    let username_str = match username {
        Some(u) => u.to_string(),
        None => {
            if is_interactive() {
                prompt_for_input("👤 Username to remove", None)?
            } else {
                anyhow::bail!("Username is required. Use: knot team remove-member <team> <username>");
            }
        }
    };

    let base_url = get_knot_space_url();
    // URL encode the team name and username to handle special characters
    let encoded_team = urlencoding::encode(&team_name);
    let encoded_username = urlencoding::encode(&username_str);
    let url = format!("{}/api/teams/{}/members/{}", base_url, encoded_team, encoded_username);

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Removed {} from team {}", username_str, team_name);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not remove team member: {}", formatted_error);
    }

    Ok(())
}

pub async fn add_package(package_name: &str, auto_link: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Check if we're in an app directory
    let app_yml_path = current_dir.join("app.yml");
    if !app_yml_path.exists() {
        // Check if we're in project root and user specified app
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(_) => {
                anyhow::bail!("❌ Not in an app directory or project root. Run 'knot add' from an app directory.");
            }
        };

        // List available apps for user reference
        if !project.apps.is_empty() {
            println!("📋 Available apps:");
            for app_name in project.apps.keys() {
                println!("  • {}", app_name);
            }
            println!("💡 Navigate to an app directory and run 'knot add' there");
        } else {
            println!("💡 Create an app first with 'knot init:app <name>'");
        }
        return Ok(());
    }

    // Load current app config
    let mut app_config = AppConfig::from_file(&app_yml_path)?;

    // Validate package name
    app_config.validate_package_name(package_name)?;

    // Initialize packages vector if it doesn't exist
    if app_config.packages.is_none() {
        app_config.packages = Some(Vec::new());
    }

    let packages = app_config.packages.as_mut().unwrap();

    // Check if package is already added
    if packages.contains(&package_name.to_string()) {
        println!("📦 Package '{}' is already added to app '{}'", package_name, app_config.name);
        return Ok(());
    }

    // Add the package
    packages.push(package_name.to_string());

    // Save updated config
    let yaml_content = serde_yaml::to_string(&app_config)?;
    fs::write(&app_yml_path, yaml_content)
        .context("Failed to update app.yml")?;

    println!("✅ Added package '{}' to app '{}'", package_name, app_config.name);

    // Auto-link if requested
    if auto_link {
        println!("🔗 Linking packages...");
        link_packages(false).await?;
    } else {
        println!("💡 Run 'knot link' to apply the changes");
    }

    Ok(())
}

pub async fn install_dependencies() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Check if we're in an app directory
    let app_yml_path = current_dir.join("app.yml");
    if app_yml_path.exists() {
        // Install dependencies for current app
        let app_config = AppConfig::from_file(&app_yml_path)?;
        println!("📦 Installing dependencies for app '{}'...", app_config.name);

        if let Some(packages) = &app_config.packages {
            if packages.is_empty() {
                println!("✅ No dependencies to install");
                return Ok(());
            }

            println!("📋 Dependencies: {}", packages.join(", "));
            link_packages(false).await?;
        } else {
            println!("✅ No dependencies configured");
        }
        return Ok(());
    }

    // Check if we're in project root
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            anyhow::bail!("❌ Not in an app directory or project root");
        }
    };

    // Install dependencies for all apps
    println!("📦 Installing dependencies for all apps in project '{}'...", project.config.name);

    let mut apps_with_deps = 0;
    let mut total_deps = 0;

    for (app_name, app_config) in &project.apps {
        if let Some(packages) = &app_config.packages {
            if !packages.is_empty() {
                apps_with_deps += 1;
                total_deps += packages.len();
                println!("  📱 {}: {} packages", app_name, packages.len());
            }
        }
    }

    if apps_with_deps == 0 {
        println!("✅ No dependencies configured in any app");
        return Ok(());
    }

    println!("📊 Total: {} dependencies across {} apps", total_deps, apps_with_deps);
    link_packages(false).await?;

    Ok(())
}

fn create_package_tarball(_package_name: &str, output_path: &str) -> Result<()> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use tar::Builder;

    let tar_gz = std::fs::File::create(output_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    let current_dir = std::env::current_dir()?;
    let knotignore_path = current_dir.join(".knotignore");

    // Load ignore patterns from .knotignore file or use defaults
    let ignore = match KnotIgnore::from_file(&knotignore_path) {
        Ok(ignore) => {
            if knotignore_path.exists() {
                println!("📋 Using .knotignore file with {} patterns", ignore.patterns().len());
            } else {
                println!("📋 Using default ignore patterns (create .knotignore to customize)");
            }
            ignore
        }
        Err(e) => {
            println!("⚠️  Failed to read .knotignore file: {}", e);
            println!("📋 Using default ignore patterns");
            KnotIgnore::default()
        }
    };

    // Walk through directory and add files that don't match ignore patterns
    add_directory_to_tar(&mut tar, &current_dir, ".", &ignore)?;

    tar.finish()?;

    println!("📦 Created package tarball: {}", output_path);
    Ok(())
}

fn add_directory_to_tar(
    tar: &mut tar::Builder<flate2::write::GzEncoder<std::fs::File>>,
    base_path: &std::path::Path,
    relative_path: &str,
    ignore: &KnotIgnore,
) -> Result<()> {
    let dir_path = base_path.join(relative_path);

    for entry in std::fs::read_dir(&dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();

        let entry_relative_path = if relative_path == "." {
            file_name.to_string()
        } else {
            format!("{}/{}", relative_path, file_name)
        };

        // Check if this file/directory should be ignored
        if ignore.is_ignored(&entry_relative_path) || ignore.is_ignored(&file_name) {
            continue;
        }

        if file_path.is_dir() {
            // Recursively add directory contents
            add_directory_to_tar(tar, base_path, &entry_relative_path, ignore)?;
        } else {
            // Add file to tarball with better error handling
            let file = std::fs::File::open(&file_path)
                .with_context(|| format!("Failed to open file: {}", file_path.display()))?;

            // Get file metadata for proper tar entry
            let metadata = file.metadata()
                .with_context(|| format!("Failed to get metadata for file: {}", file_path.display()))?;

            let mut header = tar::Header::new_gnu();
            header.set_size(metadata.len());
            header.set_mode(0o644); // Standard file permissions
            header.set_cksum();

            tar.append_data(&mut header, &entry_relative_path, file)
                .with_context(|| format!("Failed to add file to archive: {}", entry_relative_path))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_create_package_tarball_ignores_correctly() -> Result<()> {
        let dir = tempdir()?;
        let package_dir = dir.path();

        // Create test files and directories
        fs::create_dir_all(package_dir.join("src"))?;
        fs::create_dir_all(package_dir.join("node_modules"))?;
        fs::create_dir_all(package_dir.join(".git"))?;
        fs::create_dir_all(package_dir.join("build"))?;

        // Create test files
        File::create(package_dir.join("README.md"))?.write_all(b"readme")?;
        File::create(package_dir.join("package.yml"))?.write_all(b"name: test")?;
        File::create(package_dir.join(".knotignore"))?.write_all(b"build/\n*.log")?;
        File::create(package_dir.join("error.log"))?.write_all(b"error")?;
        File::create(package_dir.join("test-1.0.0.tar.gz"))?.write_all(b"old tarball")?;
        File::create(package_dir.join("src/main.js"))?.write_all(b"console.log('test')")?;
        File::create(package_dir.join("node_modules/package.json"))?.write_all(b"{}")?;
        File::create(package_dir.join(".git/config"))?.write_all(b"[core]")?;
        File::create(package_dir.join("build/output.js"))?.write_all(b"built")?;

        // Change to the package directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(package_dir)?;

        // Create tarball
        let tarball_path = "test-package.tar.gz";
        create_package_tarball("test-package", tarball_path)?;

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        // Verify tarball was created
        assert!(package_dir.join(tarball_path).exists());

        // Extract and verify contents
        let extract_dir = tempdir()?;
        let tarball_file = std::fs::File::open(package_dir.join(tarball_path))?;
        let tar = flate2::read::GzDecoder::new(tarball_file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(extract_dir.path())?;

        // Check that correct files are included
        assert!(extract_dir.path().join("README.md").exists());
        assert!(extract_dir.path().join("package.yml").exists());
        assert!(extract_dir.path().join("src/main.js").exists());

        // Check that ignored files are NOT included
        assert!(!extract_dir.path().join(".knotignore").exists());
        assert!(!extract_dir.path().join("error.log").exists());
        assert!(!extract_dir.path().join("test-1.0.0.tar.gz").exists());
        assert!(!extract_dir.path().join("node_modules").exists());
        assert!(!extract_dir.path().join(".git").exists());
        assert!(!extract_dir.path().join("build").exists());

        Ok(())
    }

    #[test]
    fn test_create_package_tarball_with_custom_knotignore() -> Result<()> {
        let dir = tempdir()?;
        let package_dir = dir.path();

        // Create test structure
        fs::create_dir_all(package_dir.join("src"))?;
        fs::create_dir_all(package_dir.join("docs"))?;
        fs::create_dir_all(package_dir.join("temp_cache"))?;

        // Create files
        File::create(package_dir.join("README.md"))?.write_all(b"readme")?;
        File::create(package_dir.join("src/index.js"))?.write_all(b"code")?;
        File::create(package_dir.join("docs/api.md"))?.write_all(b"docs")?;
        File::create(package_dir.join("debug.log"))?.write_all(b"debug")?;
        File::create(package_dir.join("temp_cache/data.json"))?.write_all(b"cache")?;

        // Create custom .knotignore
        let mut knotignore = File::create(package_dir.join(".knotignore"))?;
        writeln!(knotignore, "# Custom ignore rules")?;
        writeln!(knotignore, "docs/")?;
        writeln!(knotignore, "*.log")?;
        writeln!(knotignore, "temp_*")?;

        // Change to package directory and create tarball
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(package_dir)?;

        let tarball_path = "custom-test.tar.gz";
        create_package_tarball("custom-test", tarball_path)?;

        std::env::set_current_dir(original_dir)?;

        // Extract and verify
        let extract_dir = tempdir()?;
        let tarball_file = std::fs::File::open(package_dir.join(tarball_path))?;
        let tar = flate2::read::GzDecoder::new(tarball_file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(extract_dir.path())?;

        // Should include
        assert!(extract_dir.path().join("README.md").exists());
        assert!(extract_dir.path().join("src/index.js").exists());

        // Should exclude based on custom .knotignore
        assert!(!extract_dir.path().join(".knotignore").exists());
        assert!(!extract_dir.path().join("docs").exists());
        assert!(!extract_dir.path().join("debug.log").exists());
        assert!(!extract_dir.path().join("temp_cache").exists());

        Ok(())
    }
}

// Version management commands
pub async fn version_bump(bump_type: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Look for package.yml first (we're in a package)
    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_yml_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        let new_version = bump_version(&config.version, bump_type)?;
        config.version = new_version.clone();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content)?;

        println!("📈 Bumped {} version to {}", bump_type, new_version);
        return Ok(());
    }

    // Look for package.json as fallback
    let package_json_path = current_dir.join("package.json");
    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path)?;
        let mut package_json: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(current_version) = package_json.get("version").and_then(|v| v.as_str()) {
            let new_version = bump_version(current_version, bump_type)?;
            package_json["version"] = serde_json::Value::String(new_version.clone());

            let json_content = serde_json::to_string_pretty(&package_json)?;
            fs::write(&package_json_path, json_content)?;

            println!("📈 Bumped {} version to {}", bump_type, new_version);
            return Ok(());
        }
    }

    anyhow::bail!("No package.yml or package.json found in current directory");
}

pub async fn version_prerelease(preid: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let preid = preid.unwrap_or("alpha");

    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_yml_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        let new_version = bump_prerelease(&config.version, preid)?;
        config.version = new_version.clone();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content)?;

        println!("📈 Bumped prerelease version to {} ({})", new_version, preid);
        return Ok(());
    }

    anyhow::bail!("No package.yml found in current directory");
}

pub async fn version_set(version: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Validate version format
    if !is_valid_semver(version) {
        anyhow::bail!("Invalid version format. Please use semantic versioning (e.g., 1.2.3)");
    }

    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_yml_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        config.version = version.to_string();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content)?;

        println!("📌 Set version to {}", version);
        return Ok(());
    }

    anyhow::bail!("No package.yml found in current directory");
}



// Helper functions for version management
fn bump_version(current: &str, bump_type: &str) -> Result<String> {
    let parts: Vec<&str> = current.split('.').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid version format: {}", current);
    }

    let mut major: u32 = parts[0].parse().context("Invalid major version")?;
    let mut minor: u32 = parts[1].parse().context("Invalid minor version")?;
    let mut patch: u32 = parts[2].parse().context("Invalid patch version")?;

    match bump_type {
        "major" => {
            major += 1;
            minor = 0;
            patch = 0;
        }
        "minor" => {
            minor += 1;
            patch = 0;
        }
        "patch" => {
            patch += 1;
        }
        _ => anyhow::bail!("Invalid bump type: {}", bump_type),
    }

    Ok(format!("{}.{}.{}", major, minor, patch))
}

fn bump_prerelease(current: &str, preid: &str) -> Result<String> {
    // Simple prerelease implementation
    if current.contains("-") {
        // Already a prerelease, increment counter
        let parts: Vec<&str> = current.split('-').collect();
        if parts.len() >= 2 {
            let version_part = parts[0];
            let prerelease_part = parts[1];

            if prerelease_part.starts_with(preid) {
                // Extract number and increment
                let number_part = prerelease_part.strip_prefix(preid).unwrap_or("0");
                let current_num: u32 = number_part.parse().unwrap_or(0);
                return Ok(format!("{}-{}{}", version_part, preid, current_num + 1));
            }
        }
    }

    // Create new prerelease from current version
    Ok(format!("{}-{}1", current, preid))
}

fn is_valid_semver(version: &str) -> bool {
    // Handle prerelease versions (e.g., 1.2.3-alpha.1)
    let version_main = if let Some(hyphen_pos) = version.find('-') {
        &version[..hyphen_pos]
    } else {
        version
    };

    let parts: Vec<&str> = version_main.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    // Check that each part is a valid number and doesn't have leading zeros (except for "0")
    for part in &parts {
        if part.is_empty() {
            return false;
        }

        // Check for leading zeros (except for "0" itself)
        if part.len() > 1 && part.starts_with('0') {
            return false;
        }

        if part.parse::<u32>().is_err() {
            return false;
        }
    }

    // If there's a prerelease part, validate it's not empty
    if let Some(hyphen_pos) = version.find('-') {
        let prerelease = &version[hyphen_pos + 1..];
        if prerelease.is_empty() {
            return false;
        }

        // Prerelease can contain alphanumeric characters, hyphens, and dots
        if !prerelease.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-') {
            return false;
        }
    }

    true
}



fn validate_publish_version(version: &str) -> Result<()> {
    // Check if version is empty or just whitespace
    if version.trim().is_empty() {
        anyhow::bail!("Version cannot be empty");
    }

    // Validate semver format
    if !is_valid_semver(version) {
        anyhow::bail!("Invalid version format '{}'. Must follow semantic versioning (e.g., 1.2.3)", version);
    }

    // Check for development versions - warn user about publishing prerelease versions
    if version.contains("-dev") || version.contains("-development") {
        anyhow::bail!(
            "Cannot publish development version '{}'. \n💡 Use 'knot version set <version>' to set a proper release version first.",
            version
        );
    }

    if version.contains("-alpha") || version.contains("-beta") || version.contains("-rc") {
        println!("⚠️  Publishing prerelease version: {}", version);
        println!("💡 Use 'knot version bump major|minor|patch' for stable releases");
    }

    // Version 0.0.0 is typically invalid for publishing
    if version == "0.0.0" {
        anyhow::bail!(
            "Cannot publish version '0.0.0'. \n💡 Use 'knot version set <version>' to set a proper version first."
        );
    }

    println!("📋 Publishing version: {}", version);
    Ok(())
}

async fn check_version_exists(package_name: &str, version: &str, token: &str) -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/packages/{}/{}/exists", base_url, package_name, version);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            // Version exists
            anyhow::bail!(
                "Version '{}' of package '{}' already exists. \n💡 Use 'knot version bump major|minor|patch' to create a new version.",
                version,
                package_name
            );
        }
        404 => {
            // Version doesn't exist, good to proceed
            println!("✅ Version {} is available for publishing", version);
            Ok(())
        }
        _ => {
            // API error, but don't block publishing
            anyhow::bail!("Could not verify version existence (server error)")
        }
    }
}

// CLI Self-Update
pub async fn update_cli(force: bool) -> Result<()> {
    println!("🔄 Checking for Knot CLI updates...");

    let current_version = env!("CARGO_PKG_VERSION");
    println!("📦 Current version: {}", current_version);

    // Check for latest version from GitHub releases or repository
    match check_latest_version().await {
        Ok(latest_version) => {
            if !force && latest_version != "latest" && current_version == latest_version {
                println!("✅ You already have the latest version ({})", current_version);
                return Ok(());
            }

            if !force {
                if latest_version == "latest" {
                    println!("🎯 Updating to latest development version from main branch");
                    println!("⬆️ Updating from {}...", current_version);
                } else {
                    println!("🎯 Latest version available: {}", latest_version);
                    println!("⬆️ Updating from {} to {}...", current_version, latest_version);
                }
            } else {
                if latest_version == "latest" {
                    println!("🔄 Force updating to latest development version...");
                } else {
                    println!("🔄 Force updating to {}...", latest_version);
                }
            }

            update_binary_with_animation().await?;
            println!("✅ Update completed successfully!");
            println!("🎉 Run 'knot --version' to verify the new version");
        }
        Err(e) => {
            println!("❌ Failed to check for updates: {}", e);
            println!("💡 You can try updating manually by running the install script:");
            println!("   curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash");
        }
    }

    Ok(())
}

async fn check_latest_version() -> Result<String> {
    // Try to get latest version from GitHub API
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/saravenpi/knot/releases/latest")
        .header("User-Agent", "knot-cli")
        .send()
        .await?;

    if !response.status().is_success() {
        // Fallback to tags if no releases found
        return check_latest_tag().await;
    }

    let release: serde_json::Value = response.json().await?;
    let version_name = release["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid release format"))?;

    // Remove 'v' prefix if present
    let version = version_name.strip_prefix('v').unwrap_or(version_name);
    Ok(version.to_string())
}

async fn check_latest_tag() -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/saravenpi/knot/tags")
        .header("User-Agent", "knot-cli")
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch version information from GitHub");
    }

    let tags: serde_json::Value = response.json().await?;
    let tags_array = tags.as_array().ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

    if tags_array.is_empty() {
        // No tags/releases found, assume latest development version available
        println!("ℹ️  No tagged releases found, will update to latest development version");
        return Ok("latest".to_string());
    }

    // Get the latest version (first in the list)
    let latest_tag = &tags_array[0];
    let version_name = latest_tag["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid tag format"))?;

    // Remove 'v' prefix if present
    let version = version_name.strip_prefix('v').unwrap_or(version_name);
    Ok(version.to_string())
}

async fn update_binary_with_animation() -> Result<()> {
    use std::env;
    use std::time::Duration;

    // Start the loading animation in the background
    let (animation_sender, mut animation_receiver) = tokio::sync::mpsc::channel(1);
    let animation_task = tokio::spawn(async move {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut frame_index = 0;

        loop {
            tokio::select! {
                _ = animation_receiver.recv() => break,
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    print!("\r{} Downloading and installing latest version...", frames[frame_index]);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
                    frame_index = (frame_index + 1) % frames.len();
                }
            }
        }
        print!("\r✅ Downloaded and installed latest version          \n");
        std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
    });

    // Get current binary path
    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent()
        .ok_or_else(|| anyhow::anyhow!("Could not determine current binary directory"))?;

    // Use incremental compilation directly (faster and more reliable)
    println!("📦 Using incremental compilation...");
    let result = compile_from_source(&current_dir).await;

    // Stop the animation
    let _ = animation_sender.send(()).await;
    let _ = animation_task.await;

    result
}

async fn try_binary_download(target_dir: &std::path::Path) -> Result<()> {
    // Detect current platform
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    // Map to GitHub release asset names
    let platform_name = match (os, arch) {
        ("macos", "x86_64") => "knot-macos-x86_64",
        ("macos", "aarch64") => "knot-macos-aarch64",
        ("linux", "x86_64") => "knot-linux-x86_64",
        ("linux", "aarch64") => "knot-linux-aarch64",
        ("windows", "x86_64") => "knot-windows-x86_64.exe",
        _ => {
            anyhow::bail!("No pre-built binary available for {}-{}", os, arch);
        }
    };

    // Get latest release info
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/saravenpi/knot/releases/latest")
        .header("User-Agent", "knot-cli")
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch latest release information");
    }

    let release: serde_json::Value = response.json().await?;
    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("No assets found in release"))?;

    // Find matching asset for current platform
    let asset = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map(|name| name == platform_name)
                .unwrap_or(false)
        })
        .ok_or_else(|| anyhow::anyhow!("No binary found for platform: {}", platform_name))?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid download URL"))?;

    println!("📦 Downloading pre-built binary for {}-{}...", os, arch);

    // Download the binary
    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        anyhow::bail!("Failed to download binary from {}", download_url);
    }

    let binary_data = response.bytes().await?;

    // Create backup and write new binary
    let current_binary = target_dir.join("knot");
    let backup_binary = target_dir.join("knot.backup");
    let temp_binary = target_dir.join("knot.new");

    // Backup current binary
    if current_binary.exists() {
        std::fs::copy(&current_binary, &backup_binary)?;
    }

    // Write new binary
    std::fs::write(&temp_binary, &binary_data)?;

    // Make executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&temp_binary)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&temp_binary, perms)?;
    }

    // Test new binary
    let test_output = tokio::process::Command::new(&temp_binary)
        .arg("--version")
        .output()
        .await?;

    if !test_output.status.success() {
        let _ = std::fs::remove_file(&temp_binary);
        anyhow::bail!("Downloaded binary failed verification test");
    }

    // Replace current binary with new one
    std::fs::rename(&temp_binary, &current_binary)?;

    // Clean up backup
    let _ = std::fs::remove_file(&backup_binary);

    Ok(())
}

async fn compile_from_source(target_dir: &std::path::Path) -> Result<()> {
    // Enhanced source compilation with persistent cache
    let cache_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".knot")
        .join("build-cache");

    // Create cache directory if it doesn't exist
    std::fs::create_dir_all(&cache_dir)?;

    let src_dir = cache_dir.join("src");
    let build_dir = cache_dir.join("build");

    // Clone or update source
    if src_dir.exists() {
        println!("🔄 Updating existing source cache...");

        // Update existing repository
        let output = tokio::process::Command::new("git")
            .args(&["pull", "origin", "main"])
            .current_dir(&src_dir)
            .output()
            .await?;

        if !output.status.success() {
            println!("⚠️  Git pull failed, re-cloning...");
            std::fs::remove_dir_all(&src_dir)?;
            clone_source(&src_dir).await?;
        }
    } else {
        println!("📦 Cloning source to cache...");
        clone_source(&src_dir).await?;
    }

    // Build with cache
    println!("🔨 Building (using incremental compilation)...");

    let knot_cli_dir = src_dir.join("apps").join("knot-cli");
    let output = tokio::process::Command::new("cargo")
        .args(&["build", "--release"])
        .env("CARGO_TARGET_DIR", &build_dir)
        .current_dir(&knot_cli_dir)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Build failed: {}", stderr);
    }

    // Copy binary to target location
    let built_binary = build_dir.join("release").join("knot");
    let current_binary = target_dir.join("knot");
    let backup_binary = target_dir.join("knot.backup");

    // Backup and replace
    if current_binary.exists() {
        std::fs::copy(&current_binary, &backup_binary)?;
    }

    std::fs::copy(&built_binary, &current_binary)?;

    // Clean up backup
    let _ = std::fs::remove_file(&backup_binary);

    println!("🎉 Successfully compiled and installed from source!");
    Ok(())
}

async fn clone_source(target_dir: &std::path::Path) -> Result<()> {
    let output = tokio::process::Command::new("git")
        .args(&[
            "clone",
            "--depth", "1",
            "https://github.com/saravenpi/knot.git",
            target_dir.to_str().unwrap()
        ])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git clone failed: {}", stderr);
    }

    Ok(())
}
