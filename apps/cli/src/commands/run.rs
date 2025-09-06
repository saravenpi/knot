use crate::config::{AppConfig, PackageConfig};
use crate::project::Project;
use crate::utils;
use anyhow::{Context, Result};
use console::style;
use inquire::Select;
use std::io::IsTerminal;
use std::path::Path;

// Check if running in interactive environment
fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}

pub async fn run_script(script_name: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // First try to find script in current directory config files
    // Priority: app.yml/yaml > package.yml/yaml > knot.yml/yaml (project root)

    // Check if we're in an app directory (has app.yml or app.yaml)
    if let Some(app_config_path) = utils::find_yaml_file(&current_dir, "app") {
        if let Some(script_command) = find_script_in_app(&app_config_path, script_name)? {
            return execute_script(script_name, &script_command, &current_dir, "app").await;
        }
    }

    // Check if we're in a package directory (has package.yml or package.yaml)
    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        if let Some(script_command) = find_script_in_package(&package_config_path, script_name)? {
            return execute_script(script_name, &script_command, &current_dir, "package").await;
        }
    }

    // Check project root for knot.yml
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            println!("❌ No knot.yml/yaml, app.yml/yaml, or package.yml/yaml found");
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

pub async fn run_script_interactive() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    
    // Check if we're in an interactive environment
    if !is_interactive() {
        println!("❌ Interactive mode requires a terminal");
        println!("💡 Use 'knot run <script_name>' to run a specific script");
        return Ok(());
    }

    // Collect all available scripts
    let mut all_scripts = Vec::new();

    // Load project first to get context
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => Some(project),
        Err(_) => {
            println!("❌ No knot.yml/yaml found");
            println!("💡 Run from a directory containing knot.yml/yaml, app.yml/yaml, or package.yml/yaml");
            return Ok(());
        }
    };

    // Check if we're in an app directory (has app.yml or app.yaml)
    if let Some(app_config_path) = utils::find_yaml_file(&current_dir, "app") {
        if let Ok(app_config) = AppConfig::from_file(&app_config_path) {
            if let Some(scripts) = &app_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!("📱 {} (app: {})", name, app_config.name),
                        name.clone(),
                        command.clone(),
                        "app".to_string(),
                    ));
                }
            }
        }
    }

    // Check if we're in a package directory (has package.yml or package.yaml)
    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        if let Ok(package_config) = PackageConfig::from_file(&package_config_path) {
            if let Some(scripts) = &package_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!("📦 {} (package: {})", name, package_config.name),
                        name.clone(),
                        command.clone(),
                        "package".to_string(),
                    ));
                }
            }
        }
    }

    // Add project scripts
    if let Some(project) = &project {
        if let Some(scripts) = &project.config.scripts {
            for (name, command) in scripts {
                all_scripts.push((
                    format!("🏗️ {} (project: {})", name, project.config.name),
                    name.clone(),
                    command.clone(),
                    "project".to_string(),
                ));
            }
        }
    }

    if all_scripts.is_empty() {
        println!("❌ No scripts found");
        println!("💡 Add scripts to knot.yml/yaml, app.yml/yaml, or package.yml/yaml");
        println!("\nExample:");
        println!("scripts:");
        println!("  build: \"npm run build\"");
        println!("  test: \"npm test\"");
        println!("  dev: \"npm run dev\"");
        return Ok(());
    }

    // Show interactive selection
    println!("{}", style("🚀 Select a script to run:").bold().cyan());
    
    let script_options: Vec<String> = all_scripts
        .iter()
        .map(|(display_name, _, command, _)| format!("{} → {}", display_name, style(command).dim()))
        .collect();

    let selection = Select::new("Select a script:", script_options.clone())
        .with_vim_mode(true)
        .with_help_message("Use arrow keys or j/k to navigate, Enter to select, Esc to cancel")
        .prompt();

    match selection {
        Ok(selected_text) => {
            // Find the index of the selected option
            let selected_index = script_options.iter()
                .position(|opt| opt == &selected_text)
                .unwrap_or(0);
            let selected_script = &all_scripts[selected_index];
            let (_, script_name, script_command, context) = selected_script;
            
            println!("✨ Running script: {}", style(script_name).green().bold());
            println!("📍 Command: {}", style(script_command).dim());
            println!("🔧 Context: {}", context);
            println!();

            let working_dir = if let Some(ref proj) = project {
                if context == "project" {
                    &proj.root
                } else {
                    &current_dir
                }
            } else {
                &current_dir
            };

            execute_script(script_name, script_command, working_dir, context).await?;
        }
        Err(_) => {
            println!("❌ Script selection cancelled");
        }
    }

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
    if let Some(app_config_path) = utils::find_yaml_file(current_dir, "app") {
        if let Ok(app_config) = AppConfig::from_file(&app_config_path) {
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
    if let Some(package_config_path) = utils::find_yaml_file(current_dir, "package") {
        if let Ok(package_config) = PackageConfig::from_file(&package_config_path) {
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
    }

    Ok(())
}