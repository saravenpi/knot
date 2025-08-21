use anyhow::{Context, Result};
use std::path::Path;

use crate::config::{AppConfig, PackageConfig};
use crate::project::Project;

pub async fn build_apps() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Check if we're in an app directory (has app.yml)
    let app_yml_path = current_dir.join("app.yml");
    if app_yml_path.exists() {
        // Build single app
        build_single_app(&current_dir).await
    } else {
        // Check if we're in a project directory (has knot.yml)
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(_) => {
                println!("❌ No knot.yml or app.yml found");
                println!("💡 Run from project root to build all apps, or from app directory to build single app");
                return Ok(());
            }
        };

        // Build all apps
        build_all_apps(&project).await
    }
}

async fn build_single_app(app_dir: &Path) -> Result<()> {
    let app_yml_path = app_dir.join("app.yml");
    let app_config = AppConfig::from_file(&app_yml_path).context("Failed to load app.yml")?;

    match &app_config.build {
        Some(build_command) => {
            if build_command.is_empty() {
                println!("⚠️  Empty build command for app '{}'", app_config.name);
                return Ok(());
            }

            println!("🔨 Building app '{}'...", app_config.name);
            println!("📝 Running: {}", build_command);

            let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
            let shell_flag = if cfg!(target_os = "windows") { "/C" } else { "-c" };

            let mut cmd = tokio::process::Command::new(shell);
            cmd.arg(shell_flag);
            cmd.arg(build_command);
            cmd.current_dir(app_dir);
            cmd.stdin(std::process::Stdio::inherit());
            cmd.stdout(std::process::Stdio::inherit());
            cmd.stderr(std::process::Stdio::inherit());

            let status = cmd
                .status()
                .await
                .with_context(|| format!("Failed to execute build command: {}", build_command))?;

            if status.success() {
                println!("✅ Successfully built app '{}'", app_config.name);
            } else {
                let exit_code = status.code().unwrap_or(-1);
                anyhow::bail!(
                    "Build failed for app '{}' with exit code: {}",
                    app_config.name,
                    exit_code
                );
            }
        }
        None => {
            println!("⚠️  No build command configured for app '{}'", app_config.name);
            println!("💡 Add a 'build' field to app.yml to configure build command");
        }
    }

    Ok(())
}

async fn build_all_apps(project: &Project) -> Result<()> {
    println!("🔨 Building all apps for project '{}'...", project.config.name);

    let apps_with_builds: Vec<_> = project
        .apps
        .iter()
        .filter(|(_, config)| config.build.is_some())
        .collect();

    if apps_with_builds.is_empty() {
        println!("⚠️  No apps have build commands configured");
        println!("💡 Add 'build' fields to app.yml files to configure build commands");
        return Ok(());
    }

    println!("📋 Found {} app(s) with build commands", apps_with_builds.len());

    let mut success_count = 0;
    let mut total_count = 0;

    for (app_name, app_config) in &apps_with_builds {
        total_count += 1;

        if let Some(build_command) = &app_config.build {
            if build_command.is_empty() {
                println!("\n⚠️  Empty build command for app '{}'", app_name);
                continue;
            }

            println!("\n🔨 Building app '{}'...", app_name);
            println!("📝 Running: {}", build_command);

            let app_dir = project.root.join("apps").join(app_name);

            if !app_dir.exists() {
                println!("❌ App directory does not exist: {}", app_dir.display());
                continue;
            }

            let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
            let shell_flag = if cfg!(target_os = "windows") { "/C" } else { "-c" };

            let mut cmd = tokio::process::Command::new(shell);
            cmd.arg(shell_flag);
            cmd.arg(build_command);
            cmd.current_dir(&app_dir);
            cmd.stdin(std::process::Stdio::inherit());
            cmd.stdout(std::process::Stdio::inherit());
            cmd.stderr(std::process::Stdio::inherit());

            match cmd.status().await {
                Ok(status) if status.success() => {
                    println!("✅ Successfully built app '{}'", app_name);
                    success_count += 1;
                }
                Ok(status) => {
                    let exit_code = status.code().unwrap_or(-1);
                    println!("❌ Build failed for app '{}' with exit code: {}", app_name, exit_code);
                }
                Err(e) => {
                    println!("❌ Failed to execute build command for app '{}': {}", app_name, e);
                }
            }
        }
    }

    println!("\n📊 Build Summary:");
    println!("✅ Successfully built: {}/{} apps", success_count, total_count);

    if success_count == total_count {
        println!("🎉 All apps built successfully!");
    } else {
        anyhow::bail!(
            "Some apps failed to build ({}/{} failed)",
            total_count - success_count,
            total_count
        );
    }

    Ok(())
}

pub async fn run_script(script_name: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

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
    if script_name.is_empty() {
        anyhow::bail!("Script name cannot be empty");
    }

    if script_command.is_empty() {
        anyhow::bail!("Script command cannot be empty");
    }

    if !working_dir.exists() {
        anyhow::bail!("Working directory does not exist: {}", working_dir.display());
    }

    if !working_dir.is_dir() {
        anyhow::bail!("Working directory is not a directory: {}", working_dir.display());
    }

    println!("🚀 Running {} script '{}'...", context, script_name);
    println!("📝 Command: {}", script_command);
    println!("📂 Working directory: {}", working_dir.display());

    let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
    let shell_flag = if cfg!(target_os = "windows") { "/C" } else { "-c" };

    let mut cmd = tokio::process::Command::new(shell);
    cmd.arg(shell_flag);
    cmd.arg(script_command);
    cmd.current_dir(working_dir);
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let status = cmd.status().await.with_context(|| {
        format!("Failed to execute script '{}': {}", script_name, script_command)
    })?;

    if status.success() {
        println!("✅ Script '{}' completed successfully", script_name);
    } else {
        let exit_code = status.code().unwrap_or(-1);
        anyhow::bail!("Script '{}' failed with exit code: {}", script_name, exit_code);
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