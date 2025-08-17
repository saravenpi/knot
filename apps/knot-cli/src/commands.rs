use crate::config::{AppConfig, KnotConfig, PackageConfig};
use crate::linker::Linker;
use crate::project::Project;
use crate::templates::TemplateManager;
use crate::typescript::TypeScriptManager;
use anyhow::{Context, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

pub fn init_project(name: &str, description: Option<&str>) -> Result<()> {
    let knot_yml_path = Path::new("knot.yml");

    if knot_yml_path.exists() {
        anyhow::bail!("knot.yml already exists in current directory");
    }

    let config = KnotConfig {
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        ts_alias: Some(crate::config::TsAlias::Boolean(false)),
        apps: None,
        scripts: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(knot_yml_path, yaml_content).context("Failed to create knot.yml")?;

    fs::create_dir_all("packages")?;
    fs::create_dir_all("apps")?;

    println!("✅ Initialized new Knot project: {}", name);
    println!("📁 Created directories: packages/, apps/");
    println!("💡 Use 'knot init:package <name>' to create packages");
    println!("💡 Use 'knot init:app <name>' to create apps");
    Ok(())
}

pub fn init_package(name: &str, team: Option<&str>, version: Option<&str>, template: Option<&str>, description: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let knot_yml_path = current_dir.join("knot.yml");
    
    // Determine where to create the package
    let package_dir = if knot_yml_path.exists() {
        // We're in project root, create package in packages/ directory
        let packages_dir = current_dir.join("packages");
        if !packages_dir.exists() {
            fs::create_dir_all(&packages_dir)?;
        }
        packages_dir.join(name)
    } else {
        // Not in project root, create in current directory
        current_dir.join(name)
    };

    if package_dir.exists() {
        anyhow::bail!("Package directory '{}' already exists", name);
    }

    fs::create_dir_all(&package_dir)?;

    // Use template if specified
    if let Some(template_name) = template {
        let templates = TemplateManager::get_package_templates();
        if let Some(template) = templates.get(template_name) {
            let pkg_version = version.unwrap_or("0.1.0");
            let pkg_description = description.unwrap_or("Package description");
            
            TemplateManager::create_from_template(
                template,
                &package_dir,
                name,
                pkg_version,
                pkg_description,
            )?;
            
            println!("📦 Initialized new {} package: {}", template_name, name);
            println!("📁 Created at: {}", package_dir.display());
            println!("🎯 Template: {} - {}", template.name, template.description);
        } else {
            let available = TemplateManager::list_package_templates();
            anyhow::bail!("Unknown template '{}'. Available templates: {}", template_name, available.join(", "));
        }
    } else {
        // Create basic package without template
        let package_yml_path = package_dir.join("package.yml");
        let config = PackageConfig {
            name: name.to_string(),
            team: team.map(|s| s.to_string()),
            version: version.unwrap_or("0.1.0").to_string(),
            description: Some(description.unwrap_or("Package description").to_string()),
            tags: Some(vec!["utilities".to_string()]),
            scripts: None,
        };

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content).context("Failed to create package.yml")?;
        
        println!("📦 Initialized new package: {}", name);
        println!("📁 Created at: {}", package_dir.display());
        println!("💡 Use '--template typescript' or '--template react' for structured templates");
    }
    
    Ok(())
}

pub fn init_app(name: &str, template: Option<&str>, description: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let knot_yml_path = current_dir.join("knot.yml");
    
    // Determine where to create the app
    let app_dir = if knot_yml_path.exists() {
        // We're in project root, create app in apps/ directory
        let apps_dir = current_dir.join("apps");
        if !apps_dir.exists() {
            fs::create_dir_all(&apps_dir)?;
        }
        apps_dir.join(name)
    } else {
        // Not in project root, create in current directory
        current_dir.join(name)
    };

    if app_dir.exists() {
        anyhow::bail!("App directory '{}' already exists", name);
    }

    fs::create_dir_all(&app_dir)?;

    // Use template if specified
    if let Some(template_name) = template {
        let templates = TemplateManager::get_app_templates();
        if let Some(template) = templates.get(template_name) {
            let app_version = "0.1.0";
            let app_description = description.unwrap_or("App description");
            
            TemplateManager::create_from_template(
                template,
                &app_dir,
                name,
                app_version,
                app_description,
            )?;
            
            // Create app.yml for Knot configuration
            let app_yml_path = app_dir.join("app.yml");
            let build_cmd = match template_name {
                "react" => Some("npm run build".to_string()),
                "svelte" => Some("npm run build".to_string()),
                _ => Some("npm run build".to_string()),
            };
            
            let config = AppConfig {
                name: name.to_string(),
                description: description.map(|s| s.to_string()),
                ts_alias: None,
                packages: None,
                build: build_cmd,
                scripts: None,
            };

            let yaml_content = serde_yaml::to_string(&config)?;
            fs::write(app_yml_path, yaml_content).context("Failed to create app.yml")?;
            
            println!("🚀 Initialized new {} app: {}", template_name, name);
            println!("📁 Created at: {}", app_dir.display());
            println!("🎯 Template: {} - {}", template.name, template.description);
            println!("💡 Run 'cd {}' then 'npm install' to get started", name);
        } else {
            let available = TemplateManager::list_app_templates();
            anyhow::bail!("Unknown template '{}'. Available templates: {}", template_name, available.join(", "));
        }
    } else {
        // Create basic app without template
        let app_yml_path = app_dir.join("app.yml");
        let config = AppConfig {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            ts_alias: None,
            packages: None,
            build: None,
            scripts: None,
        };

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(app_yml_path, yaml_content).context("Failed to create app.yml")?;

        let src_dir = app_dir.join("src");
        fs::create_dir_all(src_dir)?;

        println!("🚀 Initialized new app: {}", name);
        println!("📁 Created at: {}", app_dir.display());
        println!("💡 Use '--template react' or '--template svelte' for structured templates");
    }

    Ok(())
}

pub async fn link_packages(use_symlinks: bool) -> Result<()> {
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

    let mode = if use_symlinks { "symlinked" } else { "copied" };
    println!("🔗 Successfully {} all packages and updated TypeScript configurations", mode);
    Ok(())
}

pub fn show_status() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            println!("❌ No knot.yml found in current directory or any parent directory");
            println!("💡 Run 'knot init <project-name>' to initialize a new Knot project");
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
        if let Some(build_cmd) = &config.build {
            println!("    Build command: {}", build_cmd);
        }
    }

    Ok(())
}

pub fn show_info() -> Result<()> {
    println!("🪢 Knot - Monorepo package manager");
    println!("📦 Version: 0.2.0");
    println!();
    println!("📋 Commands:");
    println!("  🆕 init <name>              Initialize a new project");
    println!("  📦 init:package <name>      Initialize a new package");
    println!("  🚀 init:app <name>          Initialize a new app");
    println!("  ➕ add <package> [--link]   Add package dependency to current app");
    println!("  📥 install                  Install all dependencies (link packages)");
    println!("  🔗 link [--symlink]         Copy packages to apps (use --symlink for symlinks)");
    println!("  🔨 build                    Build app(s) using configured build commands");
    println!("  ▶️  run <script>            Run a script from config files");
    println!("  📦 publish [--team <name>]  Publish package to Knot Space");
    println!("  🗑️  delete <name> <version> Delete package from Knot Space");
    println!("  👥 team <subcommand>        Team management");
    println!("  🔑 auth                     Check authentication status");
    println!("  📊 status                   Show project status");
    println!("  ℹ️  info                     Show this information");
    println!("  ❓ help                     Show help for commands");
    println!();
    println!("📖 Examples:");
    println!("  knot init MyProject");
    println!("  knot init:package utils --team myteam");
    println!("  knot init:app frontend --description 'Frontend app'");
    println!("  knot add utils                    # Add local package dependency");
    println!("  knot add @jwt --link              # Add online package and auto-link");
    println!("  knot add @team/package            # Add team package");
    println!("  knot install                      # Install all dependencies");
    println!("  knot link                         # Copy packages (default)");
    println!("  knot link --symlink              # Use symlinks instead");
    println!("  knot build                        # Build all apps from project root");
    println!("  knot run test                     # Run test script");
    println!("  knot run dev                      # Run development script");
    Ok(())
}

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

            // Use shell execution for complex commands
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
            println!(
                "⚠️  No build command configured for app '{}'",
                app_config.name
            );
            println!("💡 Add a 'build' field to app.yml to configure the build command");
        }
    }

    Ok(())
}

async fn build_all_apps(project: &Project) -> Result<()> {
    println!(
        "🔨 Building all apps for project '{}'...",
        project.config.name
    );

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

    println!(
        "📋 Found {} app(s) with build commands",
        apps_with_builds.len()
    );

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

            // Use shell execution for complex commands
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
                    println!(
                        "❌ Build failed for app '{}' with exit code: {}",
                        app_name, exit_code
                    );
                }
                Err(e) => {
                    println!(
                        "❌ Failed to execute build command for app '{}': {}",
                        app_name, e
                    );
                }
            }
        }
    }

    println!("\n📊 Build Summary:");
    println!(
        "✅ Successfully built: {}/{} apps",
        success_count, total_count
    );

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
    owner_id: String,
    created_at: String,
    updated_at: String,
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
        .ok_or_else(|| anyhow::anyhow!("Authentication required. Set KNOT_TOKEN environment variable or run 'knot login' for instructions."))
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
        anyhow::bail!("Publish metadata failed ({}): {}", status, text);
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
        anyhow::bail!("Publish failed ({}): {}", status, text);
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
        anyhow::bail!("Delete failed ({}): {}", status, text);
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
        let team: Team = response.json().await?;
        println!("👥 Created team: {}", team.name);
        if let Some(desc) = team.description {
            println!("   Description: {}", desc);
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Team creation failed ({}): {}", status, text);
    }

    Ok(())
}

pub async fn list_teams() -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let teams: Vec<Team> = response.json().await?;

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
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to list teams ({}): {}", status, text);
    }

    Ok(())
}

pub async fn team_info(name: &str) -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams/{}", base_url, name);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let team: Team = response.json().await?;
        println!("👥 Team: {}", team.name);
        if let Some(desc) = team.description {
            println!("   Description: {}", desc);
        }
        println!("   Created: {}", team.created_at);

        // Get team members
        let members_url = format!("{}/api/teams/{}/members", base_url, name);
        let members_response = client.get(&members_url).send().await?;

        if members_response.status().is_success() {
            let members: Vec<serde_json::Value> = members_response.json().await?;
            println!("   Members:");
            for member in members {
                let role = member["role"].as_str().unwrap_or("unknown");
                let username = member["user"]["username"].as_str().unwrap_or("unknown");
                println!("     • {} ({})", username, role);
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to get team info ({}): {}", status, text);
    }

    Ok(())
}

pub async fn add_team_member(team: &str, username: &str, role: &str) -> Result<()> {
    let token = require_auth_token()?;

    let request = AddTeamMemberRequest {
        username: username.to_string(),
        role: role.to_string(),
    };

    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams/{}/members", base_url, team);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Added {} to team {} as {}", username, team, role);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to add team member ({}): {}", status, text);
    }

    Ok(())
}

pub async fn remove_team_member(team: &str, username: &str) -> Result<()> {
    let token = require_auth_token()?;

    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams/{}/members/{}", base_url, team, username);

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Removed {} from team {}", username, team);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to remove team member ({}): {}", status, text);
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

    // Add current directory contents to tarball (excluding certain files)
    tar.append_dir_all(".", ".")?;
    tar.finish()?;

    println!("📦 Created package tarball: {}", output_path);
    Ok(())
}

// Version management commands
pub async fn version_bump(bump_type: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    
    // Look for package.yml first (we're in a package)
    let package_yml_path = current_dir.join("package.yml");
    if package_yml_path.exists() {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_yml_path)?
        )?;
        
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
        )?;
        
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
        )?;
        
        config.version = version.to_string();
        
        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_yml_path, yaml_content)?;
        
        println!("📌 Set version to {}", version);
        return Ok(());
    }
    
    anyhow::bail!("No package.yml found in current directory");
}

pub async fn login(token: Option<&str>) -> Result<()> {
    if let Some(token) = token {
        // Login with provided token
        save_auth_token(token)?;
        println!("🔑 Logged in with provided token");
        
        // Verify the token works
        match verify_auth_token().await {
            Ok(user_info) => {
                println!("✅ Authentication successful");
                println!("👤 User: {}", user_info);
            }
            Err(_) => {
                anyhow::bail!("❌ Invalid authentication token");
            }
        }
    } else {
        // OAuth flow - open browser for authentication
        println!("🌐 Opening browser for authentication...");
        println!("If browser doesn't open, visit: https://knot-space.com/auth/cli");
        
        // For now, just prompt for manual token entry
        println!("\n📝 After authenticating, copy your token and run:");
        println!("   knot login --token <your-token>");
    }
    
    Ok(())
}

pub async fn whoami() -> Result<()> {
    match get_auth_token()? {
        Some(token) => {
            match verify_auth_token().await {
                Ok(user_info) => {
                    println!("👤 Logged in as: {}", user_info);
                    println!("🔑 Token: {}...{}", &token[..8], &token[token.len()-8..]);
                }
                Err(_) => {
                    println!("❌ Authentication token is invalid or expired");
                    println!("💡 Run 'knot login' to authenticate");
                }
            }
        }
        None => {
            println!("❌ Not authenticated");
            println!("💡 Run 'knot login' to authenticate");
        }
    }
    
    Ok(())
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

fn save_auth_token(token: &str) -> Result<()> {
    let home_dir = dirs::home_dir().context("Could not find home directory")?;
    let knot_dir = home_dir.join(".knot");
    fs::create_dir_all(&knot_dir)?;
    
    let config_file = knot_dir.join("config");
    fs::write(config_file, format!("token={}", token))?;
    
    Ok(())
}

async fn verify_auth_token() -> Result<String> {
    let token = get_auth_token()?
        .ok_or_else(|| anyhow::anyhow!("No authentication token found"))?;
    
    let knot_space_url = env::var("KNOT_SPACE_URL").unwrap_or_else(|_| "https://knot-space.com".to_string());
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/api/auth/verify", knot_space_url))
        .bearer_auth(&token)
        .send()
        .await?;
    
    if response.status().is_success() {
        let user_info = response.text().await?;
        Ok(user_info)
    } else {
        anyhow::bail!("Token verification failed")
    }
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
