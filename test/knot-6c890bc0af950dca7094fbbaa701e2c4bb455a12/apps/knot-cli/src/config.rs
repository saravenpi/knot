use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PackageEntry {
    pub name: String,
    #[serde(rename = "as")]
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PackageSpec {
    String(String),
    Object(PackageEntry),
}

impl PackageSpec {
    pub fn get_name(&self) -> &str {
        match self {
            PackageSpec::String(name) => name,
            PackageSpec::Object(entry) => &entry.name,
        }
    }

    pub fn get_alias(&self) -> Option<&str> {
        match self {
            PackageSpec::String(_) => None,
            PackageSpec::Object(entry) => entry.alias.as_deref(),
        }
    }

    pub fn to_package_entry(&self) -> PackageEntry {
        match self {
            PackageSpec::String(name) => PackageEntry {
                name: name.clone(),
                alias: None,
            },
            PackageSpec::Object(entry) => entry.clone(),
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.get_name().starts_with(prefix)
    }

    pub fn from_string(name: String) -> Self {
        PackageSpec::String(name)
    }
}

impl PartialEq<String> for PackageSpec {
    fn eq(&self, other: &String) -> bool {
        self.get_name() == other
    }
}

impl PartialEq<&str> for PackageSpec {
    fn eq(&self, other: &&str) -> bool {
        self.get_name() == *other
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppDependencies {
    List(Vec<String>),
    Object {
        #[serde(rename = "tsAlias")]
        ts_alias: Option<TsAlias>,
        packages: Option<Vec<PackageSpec>>,
    },
}

impl AppDependencies {
    pub fn get_packages(&self) -> Vec<String> {
        match self {
            AppDependencies::List(packages) => packages.clone(),
            AppDependencies::Object { packages, .. } => {
                packages.as_ref().map_or_else(Vec::new, |pkgs| {
                    pkgs.iter().map(|pkg| pkg.get_name().to_string()).collect()
                })
            }
        }
    }

    pub fn get_package_entries(&self) -> Vec<PackageEntry> {
        match self {
            AppDependencies::List(packages) => {
                packages.iter().map(|name| PackageEntry {
                    name: name.clone(),
                    alias: None,
                }).collect()
            }
            AppDependencies::Object { packages, .. } => {
                packages.as_ref().map_or_else(Vec::new, |pkgs| {
                    pkgs.iter().map(|pkg| pkg.to_package_entry()).collect()
                })
            }
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
    pub dependencies: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub packages: Option<Vec<PackageSpec>>,
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
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
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

        // Validate app names and their package aliases
        if let Some(apps) = &self.apps {
            let mut global_aliases = std::collections::HashMap::new();
            
            for (app_name, app_deps) in apps {
                self.validate_safe_name(app_name, "App name")?;
                
                // Validate aliases within each app and globally
                let package_entries = app_deps.get_package_entries();
                for entry in package_entries {
                    if let Some(alias) = &entry.alias {
                        self.validate_alias_name(alias)?;
                        
                        // Check for global alias conflicts across apps
                        if let Some((existing_app, existing_package)) = global_aliases.get(alias) {
                            anyhow::bail!(
                                "Global alias conflict: alias '{}' is used by package '{}' in app '{}' and package '{}' in app '{}'",
                                alias,
                                existing_package,
                                existing_app,
                                entry.name,
                                app_name
                            );
                        }
                        global_aliases.insert(alias.clone(), (app_name.clone(), entry.name.clone()));
                    }
                }
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

    pub fn validate_alias_name(&self, alias: &str) -> anyhow::Result<()> {
        if alias.trim().is_empty() {
            anyhow::bail!("Alias name cannot be empty");
        }

        // Alias names must be valid identifiers (safe for use in imports)
        if !alias.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
            anyhow::bail!("Alias '{}' contains invalid characters. Only alphanumeric characters, underscores, and dollar signs allowed", alias);
        }

        if alias.starts_with(|c: char| c.is_ascii_digit()) {
            anyhow::bail!("Alias '{}' cannot start with a digit", alias);
        }

        // Check for reserved keywords
        let reserved = ["import", "export", "default", "const", "let", "var", "function", "class", "interface", "type", "namespace", "enum"];
        if reserved.contains(&alias) {
            anyhow::bail!("Alias '{}' is a reserved keyword", alias);
        }

        // Check alias length
        if alias.len() > 50 {
            anyhow::bail!("Alias '{}' is too long (max 50 characters)", alias);
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
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
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

        // Validate dependencies
        if let Some(dependencies) = &self.dependencies {
            for dep in dependencies {
                if dep.trim().is_empty() {
                    anyhow::bail!("Dependency name cannot be empty");
                }
                self.validate_dependency_name(dep)?;
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

    fn validate_dependency_name(&self, dependency: &str) -> anyhow::Result<()> {
        // Check for dangerous characters (allow @ for online packages)
        if dependency.contains('\0') || dependency.contains("..") || dependency.contains('\\') {
            anyhow::bail!("Dependency name contains unsafe characters: '{}'", dependency);
        }

        // Online dependencies must follow @name or @team/name format
        if let Some(dep_part) = dependency.strip_prefix('@') {
            if dep_part.is_empty() {
                anyhow::bail!("Invalid online dependency name format: '{}'", dependency);
            }
            if dep_part.contains("..") || dep_part.contains('\\') || dep_part.contains('\0') {
                anyhow::bail!("Invalid online dependency name: '{}'", dependency);
            }
        } else {
            // Local dependencies use same validation as other names
            self.validate_safe_name(dependency, "Dependency name")?;
        }

        // Check name length
        if dependency.len() > 100 {
            anyhow::bail!("Dependency name is too long (max 100 characters): '{}'", dependency);
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
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_to_user_friendly(&e)))
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
            let mut aliases = std::collections::HashMap::new();
            for package in packages {
                let package_entry = package.to_package_entry();
                
                if package_entry.name.trim().is_empty() {
                    anyhow::bail!("Package name cannot be empty");
                }
                
                // Package names can contain @ for online packages, so use different validation
                self.validate_package_name(&package_entry.name)?;
                
                // Validate alias if present
                if let Some(alias) = &package_entry.alias {
                    self.validate_alias_name(alias)?;
                    
                    // Check for alias conflicts
                    if let Some(existing_package) = aliases.get(alias) {
                        anyhow::bail!(
                            "Alias conflict: alias '{}' is used by both '{}' and '{}'",
                            alias,
                            existing_package,
                            package_entry.name
                        );
                    }
                    aliases.insert(alias.clone(), package_entry.name.clone());
                }
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

    pub fn validate_alias_name(&self, alias: &str) -> anyhow::Result<()> {
        if alias.trim().is_empty() {
            anyhow::bail!("Alias name cannot be empty");
        }

        // Alias names must be valid identifiers (safe for use in imports)
        if !alias.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
            anyhow::bail!("Alias '{}' contains invalid characters. Only alphanumeric characters, underscores, and dollar signs allowed", alias);
        }

        if alias.starts_with(|c: char| c.is_ascii_digit()) {
            anyhow::bail!("Alias '{}' cannot start with a digit", alias);
        }

        // Check for reserved keywords
        let reserved = ["import", "export", "default", "const", "let", "var", "function", "class", "interface", "type", "namespace", "enum"];
        if reserved.contains(&alias) {
            anyhow::bail!("Alias '{}' is a reserved keyword", alias);
        }

        // Check alias length
        if alias.len() > 50 {
            anyhow::bail!("Alias '{}' is too long (max 50 characters)", alias);
        }

        Ok(())
    }

    pub fn get_package_entries(&self) -> Vec<PackageEntry> {
        self.packages.as_ref().map_or_else(Vec::new, |pkgs| {
            pkgs.iter().map(|pkg| pkg.to_package_entry()).collect()
        })
    }

    pub fn get_packages(&self) -> Vec<String> {
        self.packages.as_ref().map_or_else(Vec::new, |pkgs| {
            pkgs.iter().map(|pkg| pkg.get_name().to_string()).collect()
        })
    }
}
