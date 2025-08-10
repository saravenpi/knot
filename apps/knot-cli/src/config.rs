use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct KnotConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub apps: Option<HashMap<String, AppDependencies>>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TsAlias {
    Boolean(bool),
    String(String),
}

impl TsAlias {
    pub fn get_alias(&self) -> Option<String> {
        match self {
            TsAlias::Boolean(true) => Some("#".to_string()),
            TsAlias::Boolean(false) => None,
            TsAlias::String(alias) => Some(alias.clone()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppDependencies {
    List(Vec<String>),
    Object {
        #[serde(rename = "tsAlias")]
        ts_alias: Option<TsAlias>,
        packages: Option<Vec<String>>,
    },
}

impl AppDependencies {
    pub fn get_packages(&self) -> Vec<String> {
        match self {
            AppDependencies::List(packages) => packages.clone(),
            AppDependencies::Object { packages, .. } => packages.clone().unwrap_or_default(),
        }
    }

    pub fn get_ts_alias(&self) -> Option<&TsAlias> {
        match self {
            AppDependencies::List(_) => None,
            AppDependencies::Object { ts_alias, .. } => ts_alias.as_ref(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub team: Option<String>,
    pub version: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub packages: Option<Vec<String>>,
    pub build: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
}

impl KnotConfig {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read knot.yml file: {}", path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("knot.yml file is empty: {}", path.display());
        }

        let config: KnotConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse knot.yml file: {}", path.display()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Project name cannot be empty");
        }

        // Validate project name for path safety
        self.validate_safe_name(&self.name, "Project name")?;

        // Validate script names and commands
        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        // Validate app names
        if let Some(apps) = &self.apps {
            for app_name in apps.keys() {
                self.validate_safe_name(app_name, "App name")?;
            }
        }

        Ok(())
    }

    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        // Check for path traversal attempts
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            anyhow::bail!("{} contains invalid path characters: '{}'", context, name);
        }

        // Check for dangerous characters
        if name.contains('\0') || name.starts_with('.') || name.starts_with('-') {
            anyhow::bail!("{} contains unsafe characters: '{}'", context, name);
        }

        // Check name length
        if name.len() > 100 {
            anyhow::bail!("{} is too long (max 100 characters): '{}'", context, name);
        }

        Ok(())
    }
}

impl PackageConfig {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read package.yml file: {}", path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("package.yml file is empty: {}", path.display());
        }

        let config: PackageConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse package.yml file: {}", path.display()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Package name cannot be empty");
        }

        // Validate package name for path safety
        self.validate_safe_name(&self.name, "Package name")?;

        // Validate team name if present
        if let Some(team) = &self.team {
            if !team.trim().is_empty() {
                self.validate_safe_name(team, "Team name")?;
            }
        }

        // Validate version format (basic semver check)
        if self.version.trim().is_empty() {
            anyhow::bail!("Package version cannot be empty");
        }

        // Basic semver validation
        let version_parts: Vec<&str> = self.version.split('.').collect();
        if version_parts.len() != 3 {
            anyhow::bail!(
                "Package version must follow semver format (x.y.z): '{}'",
                self.version
            );
        }

        for part in &version_parts {
            if part.parse::<u32>().is_err() {
                anyhow::bail!("Invalid version number in '{}': '{}'", self.version, part);
            }
        }

        // Validate tags
        if let Some(tags) = &self.tags {
            for tag in tags {
                if tag.trim().is_empty() {
                    anyhow::bail!("Tag cannot be empty");
                }
                if !tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                    anyhow::bail!("Tag '{}' contains invalid characters. Only lowercase alphanumeric and hyphens allowed", tag);
                }
                if tag.starts_with('-') || tag.ends_with('-') {
                    anyhow::bail!("Tag '{}' cannot start or end with hyphen", tag);
                }
                if tag.len() > 50 {
                    anyhow::bail!("Tag '{}' is too long (max 50 characters)", tag);
                }
            }
        }

        // Validate scripts
        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        Ok(())
    }

    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        // Check for path traversal attempts
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            anyhow::bail!("{} contains invalid path characters: '{}'", context, name);
        }

        // Check for dangerous characters
        if name.contains('\0') || name.starts_with('.') || name.starts_with('-') {
            anyhow::bail!("{} contains unsafe characters: '{}'", context, name);
        }

        // Check name length
        if name.len() > 100 {
            anyhow::bail!("{} is too long (max 100 characters): '{}'", context, name);
        }

        Ok(())
    }
}

impl AppConfig {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read app.yml file: {}", path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("app.yml file is empty: {}", path.display());
        }

        let config: AppConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse app.yml file: {}", path.display()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("App name cannot be empty");
        }

        // Validate app name for path safety
        self.validate_safe_name(&self.name, "App name")?;

        // Validate build command
        if let Some(build_cmd) = &self.build {
            if build_cmd.trim().is_empty() {
                anyhow::bail!("Build command cannot be empty if specified");
            }
        }

        // Validate scripts
        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        // Validate packages
        if let Some(packages) = &self.packages {
            for package in packages {
                if package.trim().is_empty() {
                    anyhow::bail!("Package name cannot be empty");
                }
                // Package names can contain @ for online packages, so use different validation
                self.validate_package_name(package)?;
            }
        }

        Ok(())
    }

    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        // Check for path traversal attempts
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            anyhow::bail!("{} contains invalid path characters: '{}'", context, name);
        }

        // Check for dangerous characters
        if name.contains('\0') || name.starts_with('.') || name.starts_with('-') {
            anyhow::bail!("{} contains unsafe characters: '{}'", context, name);
        }

        // Check name length
        if name.len() > 100 {
            anyhow::bail!("{} is too long (max 100 characters): '{}'", context, name);
        }

        Ok(())
    }

    fn validate_package_name(&self, package: &str) -> anyhow::Result<()> {
        // Check for dangerous characters (allow @ for online packages)
        if package.contains('\0') || package.contains("..") || package.contains('\\') {
            anyhow::bail!("Package name contains unsafe characters: '{}'", package);
        }

        // Online packages must follow @name or @team/name format
        if let Some(package_part) = package.strip_prefix('@') {
            if package_part.is_empty() {
                anyhow::bail!("Invalid online package name format: '{}'", package);
            }
            if package_part.contains("..")
                || package_part.contains('\\')
                || package_part.contains('\0')
            {
                anyhow::bail!("Invalid online package name: '{}'", package);
            }
        } else {
            // Local packages use same validation as other names
            self.validate_safe_name(package, "Package name")?;
        }

        // Check name length
        if package.len() > 100 {
            anyhow::bail!(
                "Package name is too long (max 100 characters): '{}'",
                package
            );
        }

        Ok(())
    }
}
