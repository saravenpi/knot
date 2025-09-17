use anyhow::{Context, Result};
use crate::config::{PackageConfig, parse_yaml_error_to_user_friendly};
use crate::utils;
use crate::validation::{validate_semver, validate_prerelease_id, sanitize_input};
use std::fs;

// Version management commands
pub async fn version_bump(bump_type: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Look for package.yml/yaml first (we're in a package)
    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_config_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        let new_version = bump_version(&config.version, bump_type)?;
        config.version = new_version.clone();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_config_path, yaml_content)?;

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

    anyhow::bail!("Cannot set version: No package.yml/yaml or package.json found in current directory\n💡 Navigate to a package directory that contains package.yml\n💡 Or use 'knot init:package' to create a new package\n💡 Version management is only available for packages");
}

pub async fn version_prerelease(preid: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let preid = match preid {
        Some(id) => {
            let sanitized = sanitize_input(id);
            validate_prerelease_id(&sanitized)?;
            sanitized
        },
        None => "alpha".to_string(),
    };

    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_config_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        let new_version = bump_prerelease(&config.version, &preid)?;
        config.version = new_version.clone();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_config_path, yaml_content)?;

        println!("📈 Bumped prerelease version to {} ({})", new_version, preid);
        return Ok(());
    }

    anyhow::bail!("Cannot manage version: No package.yml/yaml found in current directory\n💡 Navigate to a package directory that contains package.yml\n💡 Or use 'knot init:package' to create a new package\n💡 Version management is only available for packages");
}

pub async fn version_set(version: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Validate and sanitize version format
    let sanitized_version = sanitize_input(version);
    validate_semver(&sanitized_version)?;

    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        let mut config: PackageConfig = serde_yaml::from_str(
            &fs::read_to_string(&package_config_path)?
        ).map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))?;

        config.version = version.to_string();

        let yaml_content = serde_yaml::to_string(&config)?;
        fs::write(&package_config_path, yaml_content)?;

        println!("📌 Set version to {}", version);
        return Ok(());
    }

    anyhow::bail!("Cannot manage version: No package.yml/yaml found in current directory\n💡 Navigate to a package directory that contains package.yml\n💡 Or use 'knot init:package' to create a new package\n💡 Version management is only available for packages");
}

// Helper functions for version management
fn bump_version(current: &str, bump_type: &str) -> Result<String> {
    let parts: Vec<&str> = current.split('.').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid version format '{}' in package.yml\n💡 Current version must follow semantic versioning (major.minor.patch)\n💡 Example: 1.0.0, 2.1.3, 0.1.0\n💡 Fix the version in package.yml before bumping", current);
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

#[allow(dead_code)]
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