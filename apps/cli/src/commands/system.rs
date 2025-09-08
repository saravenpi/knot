use anyhow::{Context, Result};
use std::env;
use crate::project::Project;
use serde::{Deserialize, Serialize};

// API response structures
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
struct UserProfile {
    id: String,
    username: String,
    email: String,
}

// Helper function to get Knot Space URL
fn get_knot_space_url() -> String {
    env::var("KNOT_SPACE_URL").unwrap_or_else(|_| "https://knot-space-production.up.railway.app".to_string())
}

// Get authentication token from environment
fn get_auth_token() -> Result<Option<String>> {
    match env::var("KNOT_TOKEN") {
        Ok(token) if !token.trim().is_empty() => Ok(Some(token)),
        _ => Ok(None),
    }
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

pub async fn update_cli(force: bool) -> Result<()> {
    println!("🔄 Checking for Knot CLI updates...");

    let current_version = env!("CARGO_PKG_VERSION");
    println!("📦 Current version: {}", current_version);

    // Check if cargo is available
    if !is_cargo_available() {
        println!("❌ Cargo is not available in PATH");
        println!("💡 Please install Rust and Cargo from https://rustup.rs/");
        return Ok(());
    }

    // Check for latest version from GitHub API
    match check_latest_version().await {
        Ok(latest_version) => {
            if current_version != latest_version || force {
                println!("🎉 New version available: {}", latest_version);
                perform_cargo_update(&latest_version).await?;
            } else {
                println!("✅ You are already on the latest version");
            }
        }
        Err(e) => {
            println!("⚠️  Could not check for updates: {}", e);
            if force {
                println!("💡 Forcing update anyway...");
                perform_cargo_update("latest").await?;
            } else {
                println!("💡 You can still force an update with: knot upgrade --force");
                println!("   Or update manually with: cargo install --git https://github.com/saravenpi/knot knot --bin knot --locked");
            }
        }
    }

    Ok(())
}

// Helper function to check if cargo is available
fn is_cargo_available() -> bool {
    std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Perform incremental cargo update
async fn perform_cargo_update(version: &str) -> Result<()> {
    println!("📦 Building from source using cargo...");
    println!("⏳ This may take a few minutes for incremental compilation...");
    
    let install_args = vec![
        "install".to_string(),
        "--git".to_string(),
        "https://github.com/saravenpi/knot".to_string(),
        "knot".to_string(),
        "--bin".to_string(),
        "knot".to_string(),
    ];

    // Add version tag if it's not "latest"
    let final_args = if version != "latest" {
        let mut args = install_args;
        args.push("--tag".to_string());
        args.push(format!("v{}", version));
        args
    } else {
        install_args
    };

    println!("🔧 Running: cargo {}", final_args.join(" "));
    
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(&final_args);
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let status = cmd.status()?;

    if status.success() {
        println!("✅ Successfully updated Knot CLI!");
        println!("🎉 New version is now available");
        println!("💡 You may need to restart your terminal or run 'hash -r' to refresh the command cache");
    } else {
        let exit_code = status.code().unwrap_or(-1);
        anyhow::bail!(
            "Failed to update Knot CLI. Cargo install exited with code: {}.\n💡 Try running the command manually: cargo {}",
            exit_code,
            final_args.join(" ")
        );
    }

    Ok(())
}

async fn check_latest_version() -> Result<String> {
    // Check GitHub API for latest release
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/saravenpi/knot/releases/latest")
        .header("User-Agent", "knot-cli")
        .send()
        .await
        .context("Failed to fetch latest release info")?;

    if response.status().is_success() {
        let release_info: serde_json::Value = response.json().await?;
        if let Some(tag_name) = release_info["tag_name"].as_str() {
            // Remove 'v' prefix if present
            let version = tag_name.strip_prefix('v').unwrap_or(tag_name);
            return Ok(version.to_string());
        }
    }

    anyhow::bail!("Could not determine latest version")
}