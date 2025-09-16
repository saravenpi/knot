use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::Path;
use url::Url;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub example: Option<String>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)?;
        if let Some(example) = &self.example {
            write!(f, "\nExample: {}", example)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

pub fn validate_package_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot be empty".to_string(),
            example: Some("my-package, utils, @team/utils".to_string()),
        }));
    }

    if name.len() > 214 {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot exceed 214 characters".to_string(),
            example: None,
        }));
    }

    if name.starts_with('.') || name.starts_with('_') {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot start with '.' or '_'".to_string(),
            example: Some("my-package, utils".to_string()),
        }));
    }

    if name.contains("..") {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot contain consecutive dots".to_string(),
            example: Some("my-package, utils".to_string()),
        }));
    }

    let scoped_regex = Regex::new(r"^@[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?/[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    let unscoped_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();

    if name.starts_with('@') {
        if !scoped_regex.is_match(name) {
            return Err(anyhow!(ValidationError {
                field: "scoped package name".to_string(),
                message: "Invalid scoped package format. Must be @scope/name with alphanumeric characters, dots, hyphens, or underscores".to_string(),
                example: Some("@team/utils, @my-org/ui-components".to_string()),
            }));
        }
    } else {
        if !unscoped_regex.is_match(name) {
            return Err(anyhow!(ValidationError {
                field: "package name".to_string(),
                message: "Invalid package name format. Must contain only alphanumeric characters, dots, hyphens, or underscores".to_string(),
                example: Some("my-package, utils, ui-components".to_string()),
            }));
        }
    }

    let reserved_names = [
        "node_modules", "favicon.ico", ".git", ".gitignore", ".npmignore",
        "package.json", "package-lock.json", "knot.yml", "package.yml", "app.yml"
    ];

    if reserved_names.contains(&name) {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: format!("'{}' is a reserved name", name),
            example: Some("my-package, utils".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_app_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot be empty".to_string(),
            example: Some("my-app, web, mobile".to_string()),
        }));
    }

    if name.len() > 100 {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot exceed 100 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, or underscores".to_string(),
            example: Some("my-app, web-dashboard, mobile_app".to_string()),
        }));
    }

    if name.starts_with('.') || name.starts_with('_') || name.starts_with('-') {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot start with '.', '_', or '-'".to_string(),
            example: Some("my-app, web, mobile".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name cannot be empty".to_string(),
            example: Some("my-project, monorepo".to_string()),
        }));
    }

    if name.len() > 100 {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name cannot exceed 100 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._\s-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name must start and end with alphanumeric characters and contain only letters, numbers, spaces, dots, hyphens, or underscores".to_string(),
            example: Some("my-project, Web Dashboard, mobile_app".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_team_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name cannot be empty".to_string(),
            example: Some("my-team, frontend, devops".to_string()),
        }));
    }

    if name.len() > 50 {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name cannot exceed 50 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, or underscores".to_string(),
            example: Some("my-team, frontend-team, dev_ops".to_string()),
        }));
    }

    let reserved_names = ["admin", "system", "root", "api", "www", "knot"];
    if reserved_names.contains(&name.to_lowercase().as_str()) {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: format!("'{}' is a reserved team name", name),
            example: Some("my-team, frontend, devops".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_username(username: &str) -> Result<()> {
    if username.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "username".to_string(),
            message: "Username cannot be empty".to_string(),
            example: Some("john_doe, developer123".to_string()),
        }));
    }

    if username.len() < 3 {
        return Err(anyhow!(ValidationError {
            field: "username".to_string(),
            message: "Username must be at least 3 characters long".to_string(),
            example: Some("john_doe, dev".to_string()),
        }));
    }

    if username.len() > 39 {
        return Err(anyhow!(ValidationError {
            field: "username".to_string(),
            message: "Username cannot exceed 39 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(username) {
        return Err(anyhow!(ValidationError {
            field: "username".to_string(),
            message: "Username must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, or underscores".to_string(),
            example: Some("john_doe, developer-123, user.name".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_semver(version: &str) -> Result<()> {
    if version.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "version".to_string(),
            message: "Version cannot be empty".to_string(),
            example: Some("1.0.0, 2.1.3, 1.0.0-alpha.1".to_string()),
        }));
    }

    let semver_regex = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();

    if !semver_regex.is_match(version) {
        return Err(anyhow!(ValidationError {
            field: "version".to_string(),
            message: "Invalid semantic version format".to_string(),
            example: Some("1.0.0, 2.1.3, 1.0.0-alpha.1, 1.0.0+build.1".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_version_constraint(constraint: &str) -> Result<()> {
    if constraint.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "version constraint".to_string(),
            message: "Version constraint cannot be empty".to_string(),
            example: Some("1.0.0, ^1.0.0, ~1.2.0, >=1.0.0, latest".to_string()),
        }));
    }

    if constraint == "latest" {
        return Ok(());
    }

    let constraint_regex = Regex::new(r"^([\^~>=<]*)(.+)$").unwrap();
    if let Some(captures) = constraint_regex.captures(constraint) {
        let version_part = captures.get(2).map_or("", |m| m.as_str());
        validate_semver(version_part)?;
    } else {
        return Err(anyhow!(ValidationError {
            field: "version constraint".to_string(),
            message: "Invalid version constraint format".to_string(),
            example: Some("1.0.0, ^1.0.0, ~1.2.0, >=1.0.0, latest".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_package_spec(spec: &str) -> Result<(String, Option<String>)> {
    if spec.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "package specification".to_string(),
            message: "Package specification cannot be empty".to_string(),
            example: Some("utils, utils@1.0.0, @team/utils@^1.0.0".to_string()),
        }));
    }

    if let Some(at_pos) = spec.rfind('@') {
        if at_pos == 0 {
            if let Some(second_at) = spec[1..].find('@') {
                let name = &spec[..second_at + 1];
                let version = &spec[second_at + 2..];
                validate_package_name(name)?;
                validate_version_constraint(version)?;
                return Ok((name.to_string(), Some(version.to_string())));
            } else {
                validate_package_name(spec)?;
                return Ok((spec.to_string(), None));
            }
        } else {
            let name = &spec[..at_pos];
            let version = &spec[at_pos + 1..];
            validate_package_name(name)?;
            validate_version_constraint(version)?;
            return Ok((name.to_string(), Some(version.to_string())));
        }
    } else {
        validate_package_name(spec)?;
        return Ok((spec.to_string(), None));
    }
}

pub fn validate_template_name(template: &str) -> Result<()> {
    if template.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "template".to_string(),
            message: "Template name cannot be empty".to_string(),
            example: Some("@svelte-starter, @team/template".to_string()),
        }));
    }

    if !template.starts_with('@') {
        return Err(anyhow!(ValidationError {
            field: "template".to_string(),
            message: "Template name must start with '@' to indicate it's from Knot Space".to_string(),
            example: Some("@svelte-starter, @team/template".to_string()),
        }));
    }

    validate_package_name(template)?;
    Ok(())
}

pub fn validate_url(url: &str, field_name: &str) -> Result<()> {
    if url.is_empty() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} cannot be empty", field_name),
            example: Some("https://example.com, https://github.com/user/repo".to_string()),
        }));
    }

    match Url::parse(url) {
        Ok(parsed_url) => {
            match parsed_url.scheme() {
                "http" | "https" => Ok(()),
                _ => Err(anyhow!(ValidationError {
                    field: field_name.to_string(),
                    message: format!("{} must use http or https protocol", field_name),
                    example: Some("https://example.com, https://github.com/user/repo".to_string()),
                }))
            }
        }
        Err(_) => Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("Invalid {} format", field_name),
            example: Some("https://example.com, https://github.com/user/repo".to_string()),
        }))
    }
}

pub fn validate_path(path: &str, field_name: &str, must_exist: bool) -> Result<()> {
    if path.is_empty() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} cannot be empty", field_name),
            example: Some("./my-project, /home/user/projects".to_string()),
        }));
    }

    if path.contains('\0') {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} contains invalid null character", field_name),
            example: None,
        }));
    }

    let path_obj = Path::new(path);

    if path.len() > 4096 {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} is too long (maximum 4096 characters)", field_name),
            example: None,
        }));
    }

    for component in path_obj.components() {
        if let std::path::Component::Normal(os_str) = component {
            if let Some(name) = os_str.to_str() {
                if name.starts_with('.') && (name == ".." || name.contains("../") || name.contains("..\\")) {
                    return Err(anyhow!(ValidationError {
                        field: field_name.to_string(),
                        message: format!("{} contains unsafe path traversal patterns", field_name),
                        example: Some("./my-project, /home/user/projects".to_string()),
                    }));
                }
            }
        }
    }

    if must_exist && !path_obj.exists() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} does not exist", field_name),
            example: None,
        }));
    }

    Ok(())
}

pub fn validate_script_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name cannot be empty".to_string(),
            example: Some("dev, build, test, start".to_string()),
        }));
    }

    if name.len() > 50 {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name cannot exceed 50 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._:-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, underscores, or colons".to_string(),
            example: Some("dev, build, test:unit, e2e:ci".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_role(role: &str) -> Result<()> {
    let valid_roles = ["admin", "member"];
    if !valid_roles.contains(&role) {
        return Err(anyhow!(ValidationError {
            field: "role".to_string(),
            message: format!("Invalid role '{}'. Must be 'admin' or 'member'", role),
            example: Some("admin, member".to_string()),
        }));
    }
    Ok(())
}

pub fn validate_prerelease_id(preid: &str) -> Result<()> {
    if preid.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "prerelease identifier".to_string(),
            message: "Prerelease identifier cannot be empty".to_string(),
            example: Some("alpha, beta, rc".to_string()),
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    if !valid_regex.is_match(preid) {
        return Err(anyhow!(ValidationError {
            field: "prerelease identifier".to_string(),
            message: "Prerelease identifier can only contain alphanumeric characters".to_string(),
            example: Some("alpha, beta, rc, dev".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_description(description: &str) -> Result<()> {
    if description.len() > 500 {
        return Err(anyhow!(ValidationError {
            field: "description".to_string(),
            message: "Description cannot exceed 500 characters".to_string(),
            example: None,
        }));
    }

    if description.trim() != description {
        return Err(anyhow!(ValidationError {
            field: "description".to_string(),
            message: "Description cannot start or end with whitespace".to_string(),
            example: Some("A useful package for utilities".to_string()),
        }));
    }

    Ok(())
}

pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\t' || *c == '\n')
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn confirm_destructive_action(action: &str, target: &str) -> Result<bool> {
    use crate::commands::common::prompt_for_confirm;

    println!("⚠️  WARNING: This is a destructive action!");
    println!("   Action: {}", action);
    println!("   Target: {}", target);
    println!("   This action cannot be undone.");
    println!();

    prompt_for_confirm(&format!("Are you sure you want to {} '{}'?", action, target), Some(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name() {
        assert!(validate_package_name("my-package").is_ok());
        assert!(validate_package_name("@team/utils").is_ok());
        assert!(validate_package_name("utils123").is_ok());
        assert!(validate_package_name("ui.components").is_ok());

        assert!(validate_package_name("").is_err());
        assert!(validate_package_name(".invalid").is_err());
        assert!(validate_package_name("_invalid").is_err());
        assert!(validate_package_name("package..name").is_err());
        assert!(validate_package_name("node_modules").is_err());
    }

    #[test]
    fn test_validate_semver() {
        assert!(validate_semver("1.0.0").is_ok());
        assert!(validate_semver("1.2.3-alpha.1").is_ok());
        assert!(validate_semver("2.0.0+build.1").is_ok());

        assert!(validate_semver("").is_err());
        assert!(validate_semver("1.0").is_err());
        assert!(validate_semver("invalid").is_err());
    }

    #[test]
    fn test_validate_package_spec() {
        assert!(validate_package_spec("utils").is_ok());
        assert!(validate_package_spec("utils@1.0.0").is_ok());
        assert!(validate_package_spec("@team/utils@^1.0.0").is_ok());

        assert!(validate_package_spec("").is_err());
        assert!(validate_package_spec("@").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("https://example.com", "URL").is_ok());
        assert!(validate_url("http://localhost:3000", "URL").is_ok());

        assert!(validate_url("", "URL").is_err());
        assert!(validate_url("ftp://example.com", "URL").is_err());
        assert!(validate_url("invalid-url", "URL").is_err());
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("  hello world  "), "hello world");
        assert_eq!(sanitize_input("test\x00invalid"), "testinvalid");
        assert_eq!(sanitize_input("normal text"), "normal text");
    }
}