use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A configuration variable with its value and optional description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariable {
    /// Simple string value
    Simple(String),
    /// Complex variable with value and metadata
    Complex {
        value: String,
        description: Option<String>,
    },
}

impl ConfigVariable {
    /// Get the value of the variable
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariable::Simple(value) => value,
            ConfigVariable::Complex { value, .. } => value,
        }
    }
    
}

pub fn parse_yaml_error_to_user_friendly(error: &serde_yaml::Error) -> String {
    let error_msg = error.to_string();
    
    // Check for missing fields
    if error_msg.contains("missing field") {
        if let Some(field_start) = error_msg.find("`") {
            if let Some(field_end) = error_msg[field_start + 1..].find("`") {
                let field_name = &error_msg[field_start + 1..field_start + 1 + field_end];
                return match field_name {
                    "name" => "Missing name field".to_string(),
                    "version" => "Missing version field".to_string(),
                    _ => format!("Missing {} field", field_name),
                };
            }
        }
        return "Missing required field".to_string();
    }
    
    // Check for invalid type errors
    if error_msg.contains("invalid type") {
        if error_msg.contains("expected a string") {
            return "Expected string value".to_string();
        }
        if error_msg.contains("expected a sequence") {
            return "Expected array/list value".to_string();
        }
        if error_msg.contains("expected a map") {
            return "Expected object/mapping value".to_string();
        }
        return "Invalid field type".to_string();
    }
    
    // Check for duplicate key errors
    if error_msg.contains("duplicate key") {
        return "Duplicate field found".to_string();
    }
    
    // Check for invalid YAML syntax
    if error_msg.contains("while parsing") {
        return "Invalid YAML syntax".to_string();
    }
    
    // Fallback to original error for unknown cases
    error_msg
}

/// Project-level configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct KnotConfig {
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// TypeScript alias configuration
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    /// App dependencies configuration
    pub apps: Option<HashMap<String, AppDependencies>>,
    /// Project-level scripts
    pub scripts: Option<HashMap<String, String>>,
    /// Project-level variables available to all apps and packages
    /// These variables can be referenced using {{variable_name}} syntax
    /// Example:
    /// variables:
    ///   project_version: "1.0.0"
    ///   api_url: "https://api.example.com"
    pub variables: Option<HashMap<String, ConfigVariable>>,
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

/// Package-level configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Package name
    pub name: String,
    /// Team owning this package
    pub team: Option<String>,
    /// Package version
    pub version: String,
    /// Package description
    pub description: Option<String>,
    /// Package author
    pub author: Option<String>,
    /// Package license
    pub license: Option<String>,
    /// Package repository URL
    pub repository: Option<String>,
    /// Package keywords
    pub keywords: Option<Vec<String>>,
    /// Package tags
    pub tags: Option<Vec<String>>,
    /// Package-level scripts
    pub scripts: Option<HashMap<String, String>>,
    /// Package dependencies
    pub dependencies: Option<Vec<String>>,
    /// Development dependencies
    pub dev_dependencies: Option<Vec<String>>,
    /// Optional dependencies
    pub optional_dependencies: Option<Vec<String>>,
    /// Peer dependencies
    pub peer_dependencies: Option<Vec<String>>,
    /// Package exports
    pub exports: Option<HashMap<String, String>>,
    /// Package features
    pub features: Option<Vec<String>>,
    /// Package-level variables that can override project and app variables
    /// These variables can be referenced using {{variable_name}} syntax
    /// Example:
    /// variables:
    ///   package_description: "Custom package description"
    ///   build_target: "es2020"
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

/// App-level configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// App name
    pub name: String,
    /// App description
    pub description: Option<String>,
    /// TypeScript alias configuration
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    /// Package dependencies for this app
    pub packages: Option<Vec<String>>,
    /// App-level scripts
    pub scripts: Option<HashMap<String, String>>,
    /// App-level variables that can override project variables
    /// These variables can be referenced using {{variable_name}} syntax
    /// Example:
    /// variables:
    ///   app_port: "3000"
    ///   app_title: "My Application"
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

impl KnotConfig {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read knot.{} file: {}", file_extension, path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("knot.{} file is empty: {}", file_extension, path.display());
        }

        let config: KnotConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
            .with_context(|| format!("Failed to parse knot.{} file: {}", file_extension, path.display()))?;

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
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read package.{} file: {}", file_extension, path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("package.{} file is empty: {}", file_extension, path.display());
        }

        let config: PackageConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
            .with_context(|| format!("Failed to parse package.{} file: {}", file_extension, path.display()))?;

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
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read app.{} file: {}", file_extension, path.display()))?;

        if content.trim().is_empty() {
            anyhow::bail!("app.{} file is empty: {}", file_extension, path.display());
        }

        let config: AppConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
            .with_context(|| format!("Failed to parse app.{} file: {}", file_extension, path.display()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("App name cannot be empty");
        }

        // Validate app name for path safety
        self.validate_safe_name(&self.name, "App name")?;

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

    pub fn validate_package_name(&self, package: &str) -> anyhow::Result<()> {
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