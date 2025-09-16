use anyhow::{Context, Result};
use crate::config::PackageConfig;
use crate::ignore::KnotIgnore;
use crate::utils;
use crate::validation::{validate_package_name, validate_semver, confirm_destructive_action, sanitize_input};
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tar::Builder;

// API structures
#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
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
    match env::var("KNOT_TOKEN") {
        Ok(token) if !token.trim().is_empty() => Ok(Some(token)),
        _ => Ok(None),
    }
}

fn require_auth_token() -> Result<String> {
    get_auth_token()?
        .ok_or_else(|| anyhow::anyhow!("Authentication required. Set KNOT_TOKEN environment variable."))
}

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

    // Provide user-friendly context based on status code
    match status.as_u16() {
        400 => format!("❌ {}", error_message),
        401 => format!("🔐 Authentication failed: {}", error_message),
        403 => format!("🚫 Permission denied: {}", error_message),
        404 => format!("🔍 Not found: {}", error_message),
        409 => format!("⚠️  Conflict: {}", error_message),
        500 => format!("💥 Server error: {}", error_message),
        502..=504 => format!("🌐 Service temporarily unavailable: {}", error_message),
        _ => format!("❌ Error ({}): {}", status.as_u16(), error_message),
    }
}

fn is_valid_semver(version: &str) -> bool {
    // Simple semantic versioning check
    // Supports x.y.z and x.y.z-prerelease formats
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

fn create_package_tarball(_package_name: &str, output_path: &str) -> Result<()> {
    let tar_gz = fs::File::create(output_path)?;
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
    tar: &mut Builder<GzEncoder<fs::File>>,
    base_path: &Path,
    relative_path: &str,
    ignore: &KnotIgnore,
) -> Result<()> {
    let dir_path = base_path.join(relative_path);

    for entry in fs::read_dir(&dir_path)? {
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
            let file = fs::File::open(&file_path)
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

// Public commands
pub async fn publish_package(team: Option<&str>, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    // Check if we're in a package directory
    let current_dir = std::env::current_dir()?;
    let package_config_path = utils::find_yaml_file(&current_dir, "package")
        .ok_or_else(|| anyhow::anyhow!("No package.yml or package.yaml found. Run this command from a package directory."))?;

    // Load package config
    let package_config = PackageConfig::from_file(&package_config_path)?;

    // Validate version is not a development version
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
    let file_content = fs::read(&tarball_path)?;
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
    let _ = fs::remove_file(&tarball_path);

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

    // Validate inputs
    let sanitized_name = sanitize_input(name);
    let sanitized_version = sanitize_input(version);
    validate_package_name(&sanitized_name)?;
    validate_semver(&sanitized_version)?;

    // Confirm destructive action
    let package_spec = format!("{}@{}", sanitized_name, sanitized_version);
    if !confirm_destructive_action("delete package", &package_spec)? {
        println!("❌ Delete operation cancelled");
        return Ok(());
    }

    let base_url = get_knot_space_url();
    let url = format!("{}/api/packages/{}/{}", base_url, sanitized_name, sanitized_version);

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