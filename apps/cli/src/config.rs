use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
fn validate_safe_name_common(name: &str, context: &str) -> anyhow::Result<()> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        anyhow::bail!("{} contains invalid path characters: '{}'", context, name);
    }

    if name.contains('\0') || name.starts_with('.') || name.starts_with('-') {
        anyhow::bail!("{} contains unsafe characters: '{}'", context, name);
    }

    if name.len() > 100 {
        anyhow::bail!("{} is too long (max 100 characters): '{}'", context, name);
    }

    Ok(())
}

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
    #[allow(dead_code)]
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariable::Simple(value) => value,
            ConfigVariable::Complex { value, .. } => value,
        }
    }
    
    /// Get the description of the variable
    #[allow(dead_code)]
    pub fn get_description(&self) -> Option<&str> {
        match self {
            ConfigVariable::Simple(_) => None,
            ConfigVariable::Complex { description, .. } => description.as_deref(),
        }
    }
}

#[allow(dead_code)]
pub fn parse_yaml_error_to_user_friendly(error: &serde_yaml::Error) -> String {
    parse_yaml_error_with_context(error, ConfigType::Unknown)
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    Knot,
    Package,
    App,
    #[allow(dead_code)]
    Unknown,
}

pub fn parse_yaml_error_with_context(error: &serde_yaml::Error, config_type: ConfigType) -> String {
    let error_msg = error.to_string();
    let location_info = extract_error_location(&error_msg);

    // Extract field name from error message if available
    let field_name = extract_field_name(&error_msg);

    // Check for missing required fields
    if error_msg.contains("missing field") {
        if let Some(field) = field_name {
            let suggestion = get_field_suggestion_with_context(&field, config_type);
            return format_error_with_location(
                &format_missing_field_error_with_context(&field, suggestion, config_type),
                &location_info
            );
        }
        return format_error_with_location("Missing required field", &location_info);
    }

    // Check for invalid type errors with detailed context
    if error_msg.contains("invalid type") {
        let type_error = parse_type_error(&error_msg, field_name.as_deref());
        return format_error_with_location(&type_error, &location_info);
    }

    // Check for unknown field errors
    if error_msg.contains("unknown field") {
        if let Some(field) = field_name {
            let suggestion = suggest_similar_field(&field);
            return format_error_with_location(
                &format!("Unknown field '{}'{}", field, suggestion),
                &location_info
            );
        }
        return format_error_with_location("Unknown field found", &location_info);
    }

    // Check for duplicate key errors
    if error_msg.contains("duplicate key") {
        if let Some(field) = field_name {
            return format_error_with_location(
                &format!("Duplicate field '{}' found. Each field can only be defined once.", field),
                &location_info
            );
        }
        return format_error_with_location("Duplicate field found", &location_info);
    }

    // Check for YAML syntax errors
    if error_msg.contains("while parsing") {
        let syntax_error = parse_yaml_syntax_error(&error_msg);
        return format_error_with_location(&syntax_error, &location_info);
    }

    // Check for indentation errors
    if error_msg.contains("bad indentation") || error_msg.contains("found character that cannot start any token") {
        return format_error_with_location(
            "Invalid YAML indentation. Make sure you use consistent spaces (not tabs) for indentation.",
            &location_info
        );
    }

    // Check for unclosed quotes/brackets
    if error_msg.contains("while scanning") {
        if error_msg.contains("quote") {
            return format_error_with_location(
                "Unclosed quotes detected. Make sure all strings are properly quoted.",
                &location_info
            );
        }
        if error_msg.contains("mapping") {
            return format_error_with_location(
                "Invalid mapping structure. Check for missing colons or incorrect nesting.",
                &location_info
            );
        }
    }

    // Check for value parsing errors
    if error_msg.contains("invalid value") {
        return format_error_with_location(
            "Invalid value format. Check the data type and format requirements.",
            &location_info
        );
    }

    // Fallback with enhanced context
    if !error_msg.is_empty() {
        return format_error_with_location(&error_msg, &location_info);
    }

    "Unknown YAML parsing error occurred".to_string()
}

fn extract_error_location(error_msg: &str) -> Option<(usize, usize)> {
    // Try to extract line and column information from error message
    if let Some(line_start) = error_msg.find("line ") {
        if let Some(line_end) = error_msg[line_start + 5..].find(char::is_whitespace) {
            let line_str = &error_msg[line_start + 5..line_start + 5 + line_end];
            if let Ok(line) = line_str.parse::<usize>() {
                // Try to find column info
                if let Some(col_start) = error_msg.find("column ") {
                    if let Some(col_end) = error_msg[col_start + 7..].find(char::is_whitespace) {
                        let col_str = &error_msg[col_start + 7..col_start + 7 + col_end];
                        if let Ok(col) = col_str.parse::<usize>() {
                            return Some((line, col));
                        }
                    }
                }
                return Some((line, 0));
            }
        }
    }
    None
}

fn extract_field_name(error_msg: &str) -> Option<String> {
    // Try to extract field name from various error message formats
    if let Some(start) = error_msg.find("`") {
        if let Some(end) = error_msg[start + 1..].find("`") {
            return Some(error_msg[start + 1..start + 1 + end].to_string());
        }
    }

    // Alternative patterns for field extraction
    if let Some(start) = error_msg.find("field \"") {
        if let Some(end) = error_msg[start + 7..].find("\"") {
            return Some(error_msg[start + 7..start + 7 + end].to_string());
        }
    }

    None
}

fn format_error_with_location(message: &str, location: &Option<(usize, usize)>) -> String {
    match location {
        Some((line, 0)) => format!("{} (line {})", message, line),
        Some((line, col)) => format!("{} (line {}, column {})", message, line, col),
        None => message.to_string(),
    }
}

#[allow(dead_code)]
fn format_missing_field_error(field_name: &str, suggestion: Option<String>) -> String {
    format_missing_field_error_with_context(field_name, suggestion, ConfigType::Unknown)
}

fn format_missing_field_error_with_context(field_name: &str, suggestion: Option<String>, config_type: ConfigType) -> String {
    let base_message = match (field_name, config_type) {
        ("name", ConfigType::Knot) => "Missing required field 'name'. Every knot.yml configuration must have a project name.",
        ("name", ConfigType::Package) => "Missing required field 'name'. Every package.yml configuration must have a package name.",
        ("name", ConfigType::App) => "Missing required field 'name'. Every app.yml configuration must have an app name.",
        ("name", _) => "Missing required field 'name'. Every configuration must have a name.",

        ("version", ConfigType::Package) => "Missing required field 'version'. Package configurations must specify a version.",
        ("version", _) => "Missing required field 'version'. This configuration requires a version.",

        _ => return format!("Missing required field '{}'", field_name),
    };

    match suggestion {
        Some(suggestion) => format!("{}\n{}", base_message, suggestion),
        None => base_message.to_string(),
    }
}

#[allow(dead_code)]
fn get_field_suggestion(field_name: &str) -> Option<String> {
    get_field_suggestion_with_context(field_name, ConfigType::Unknown)
}

fn get_field_suggestion_with_context(field_name: &str, config_type: ConfigType) -> Option<String> {
    match (field_name, config_type) {
        ("name", ConfigType::Knot) => Some("Suggestion: Add 'name: \"your-project-name\"' to your knot.yml file.".to_string()),
        ("name", ConfigType::Package) => Some("Suggestion: Add 'name: \"your-package-name\"' to your package.yml file.".to_string()),
        ("name", ConfigType::App) => Some("Suggestion: Add 'name: \"your-app-name\"' to your app.yml file.".to_string()),
        ("name", _) => Some("Suggestion: Add 'name: \"your-configuration-name\"' to your configuration.".to_string()),

        ("version", ConfigType::Package) => Some("Suggestion: Add 'version: \"1.0.0\"' using semantic versioning format (major.minor.patch).".to_string()),
        ("version", _) => Some("Suggestion: Add 'version: \"1.0.0\"' using semantic versioning format.".to_string()),

        ("description", _) => Some("Suggestion: Add 'description: \"Brief description of your project\"' (optional but recommended).".to_string()),

        ("scripts", ConfigType::Knot) => Some("Suggestion: Add scripts like 'scripts:\n  start: \"command to start\"' to define project-level scripts.".to_string()),
        ("scripts", ConfigType::Package) => Some("Suggestion: Add scripts like 'scripts:\n  build: \"build command\"' to define package-level scripts.".to_string()),
        ("scripts", ConfigType::App) => Some("Suggestion: Add scripts like 'scripts:\n  dev: \"development command\"' to define app-level scripts.".to_string()),

        ("packages", ConfigType::App) => Some("Suggestion: Add packages like 'packages:\n  - package-name' to define app dependencies.".to_string()),

        ("apps", ConfigType::Knot) => Some("Suggestion: Add apps like 'apps:\n  app-name:\n    - package-name' to define project apps.".to_string()),

        _ => None,
    }
}

fn parse_type_error(error_msg: &str, field_name: Option<&str>) -> String {
    let base_error = if error_msg.contains("expected a string") {
        "Expected string value"
    } else if error_msg.contains("expected a sequence") || error_msg.contains("expected array") {
        "Expected array/list value"
    } else if error_msg.contains("expected a map") || error_msg.contains("expected object") {
        "Expected object/mapping value"
    } else if error_msg.contains("expected a boolean") {
        "Expected boolean value (true/false)"
    } else if error_msg.contains("expected a number") || error_msg.contains("expected integer") {
        "Expected numeric value"
    } else {
        "Invalid field type"
    };

    match field_name {
        Some("scripts") => format!("{} for 'scripts'. Use format: scripts:\n  script-name: \"command\"", base_error),
        Some("packages") => format!("{} for 'packages'. Use format: packages:\n  - package-name", base_error),
        Some("apps") => format!("{} for 'apps'. Use either list format or object format with tsAlias and packages.", base_error),
        Some("variables") => format!("{} for 'variables'. Use format: variables:\n  var-name: \"value\"", base_error),
        Some("tsAlias") => "Expected boolean or string for 'tsAlias'. Use: true, false, or \"@\"".to_string(),
        Some(field) => format!("{} for field '{}'", base_error, field),
        None => base_error.to_string(),
    }
}

fn suggest_similar_field(field_name: &str) -> String {
    let suggestions = match field_name.to_lowercase().as_str() {
        "script" => Some("scripts"),
        "package" => Some("packages"),
        "app" => Some("apps"),
        "variable" => Some("variables"),
        "tsalias" => Some("tsAlias"),
        "desc" | "description" => Some("description"),
        "ver" | "version" => Some("version"),
        "dependencies" => Some("packages (for app config) or dependencies (for package config)"),
        _ => None,
    };

    match suggestions {
        Some(suggestion) => format!(". Did you mean '{}'?", suggestion),
        None => ". Check the field name for typos.".to_string(),
    }
}

fn parse_yaml_syntax_error(error_msg: &str) -> String {
    if error_msg.contains("mapping") {
        "Invalid YAML mapping. Check for missing colons (:) after field names."
    } else if error_msg.contains("sequence") {
        "Invalid YAML sequence. Check for proper list formatting with dashes (-)."
    } else if error_msg.contains("scalar") {
        "Invalid YAML scalar value. Check for proper quoting of string values."
    } else if error_msg.contains("anchor") {
        "Invalid YAML anchor or alias. Check anchor definitions and references."
    } else {
        "Invalid YAML syntax. Check indentation, colons, and list formatting."
    }.to_string()
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn get_packages(&self) -> Vec<String> {
        match self {
            AppDependencies::List(packages) => packages.clone(),
            AppDependencies::Object { packages, .. } => packages.clone().unwrap_or_default(),
        }
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read knot.{} file: {}", file_extension, path.display()))?;

        if content.is_empty() {
            anyhow::bail!("Configuration file knot.{} is empty at '{}'\n💡 A valid knot.{} file should contain at least a 'name' field\n💡 Example:\n   name: my-project\n   description: My awesome project",
                file_extension, path.display(), file_extension);
        }

        let config: KnotConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_with_context(&e, ConfigType::Knot)))
            .with_context(|| format!("Failed to parse knot.{} file: {}", file_extension, path.display()))?;

        config.validate()?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Project name cannot be empty in knot.yml configuration\n💡 Add a name field to your knot.yml file\n💡 Example: name: my-awesome-project");
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

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }
}

impl PackageConfig {
    #[allow(dead_code)]
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read package.{} file: {}", file_extension, path.display()))?;

        if content.is_empty() {
            anyhow::bail!("Package configuration file package.{} is empty at '{}'\n💡 A valid package.{} file should contain at least 'name' and 'version' fields\n💡 Example:\n   name: my-package\n   version: 1.0.0\n   description: My awesome package",
                file_extension, path.display(), file_extension);
        }

        let config: PackageConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_with_context(&e, ConfigType::Package)))
            .with_context(|| format!("Failed to parse package.{} file: {}", file_extension, path.display()))?;

        config.validate()?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Package name cannot be empty in package.yml configuration\n💡 Add a name field to your package.yml file\n💡 Example: name: my-package");
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
            anyhow::bail!("Package version cannot be empty in package.yml configuration\n💡 Add a version field using semantic versioning\n💡 Example: version: 1.0.0");
        }

        // Basic semver validation
        let version_parts: Vec<&str> = self.version.split('.').collect();
        if version_parts.len() != 3 {
            anyhow::bail!(
                "Package version '{}' must follow semantic versioning format (major.minor.patch)\n💡 Current format has {} parts, but semver requires exactly 3\n💡 Examples: 1.0.0, 2.1.5, 0.3.12",
                self.version, version_parts.len()
            );
        }

        for (i, part) in version_parts.iter().enumerate() {
            if part.parse::<u32>().is_err() {
                let part_name = match i {
                    0 => "major",
                    1 => "minor",
                    2 => "patch",
                    _ => "unknown"
                };
                anyhow::bail!("Invalid {} version number '{}' in version '{}'\n💡 Each part must be a non-negative integer\n💡 Example: 1.0.0 (not 1.a.0 or 1.0.beta)",
                    part_name, part, self.version);
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

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("yml");
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read app.{} file: {}", file_extension, path.display()))?;

        if content.is_empty() {
            anyhow::bail!("App configuration file app.{} is empty at '{}'\n💡 A valid app.{} file should contain at least a 'name' field\n💡 Example:\n   name: my-app\n   description: My awesome app\n   packages:\n     - utils",
                file_extension, path.display(), file_extension);
        }

        let config: AppConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("{}", parse_yaml_error_with_context(&e, ConfigType::App)))
            .with_context(|| format!("Failed to parse app.{} file: {}", file_extension, path.display()))?;

        config.validate()?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("App name cannot be empty in app.yml configuration\n💡 Add a name field to your app.yml file\n💡 Example: name: my-app");
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

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }

    #[allow(dead_code)]
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