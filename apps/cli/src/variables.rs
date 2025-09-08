use std::collections::HashMap;
use anyhow::{Context, Result};
use crate::config::{KnotConfig, AppConfig, PackageConfig, ConfigVariable};

/// Variable context for hierarchical variable resolution
/// Variables are resolved in this order of precedence:
/// 1. Built-in variables (highest precedence)
/// 2. Package variables
/// 3. App variables  
/// 4. Project variables (lowest precedence)
#[derive(Debug, Clone)]
pub struct VariableContext {
    /// Built-in system variables
    built_in: HashMap<String, String>,
    /// Project-level variables from knot.yml
    project: HashMap<String, String>,
    /// App-level variables from app.yml
    app: HashMap<String, String>,
    /// Package-level variables from package.yml
    package: HashMap<String, String>,
}

impl VariableContext {
    /// Create a new variable context with built-in variables
    pub fn new(project_name: &str, project_root: &std::path::Path) -> Self {
        let mut built_in = HashMap::new();
        
        // Add built-in variables
        built_in.insert("project_name".to_string(), project_name.to_string());
        built_in.insert("project_root".to_string(), project_root.display().to_string());
        
        // Add current timestamp for unique identifiers
        built_in.insert(
            "timestamp".to_string(), 
            chrono::Utc::now().timestamp().to_string()
        );
        
        // Add current date in YYYY-MM-DD format
        built_in.insert(
            "date".to_string(),
            chrono::Utc::now().format("%Y-%m-%d").to_string()
        );
        
        // Add current year
        built_in.insert(
            "year".to_string(),
            chrono::Utc::now().format("%Y").to_string()
        );

        Self {
            built_in,
            project: HashMap::new(),
            app: HashMap::new(),
            package: HashMap::new(),
        }
    }
    
    /// Add project variables from KnotConfig
    pub fn with_project_variables(mut self, config: &KnotConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.project.insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }
    
    /// Add app variables from AppConfig
    pub fn with_app_variables(mut self, config: &AppConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.app.insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }
    
    /// Add package variables from PackageConfig
    pub fn with_package_variables(mut self, config: &PackageConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.package.insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }
    
    /// Get a variable value with proper precedence
    pub fn get_variable(&self, key: &str) -> Option<&str> {
        // Check in order of precedence: built-in > package > app > project
        self.built_in.get(key)
            .or_else(|| self.package.get(key))
            .or_else(|| self.app.get(key))
            .or_else(|| self.project.get(key))
            .map(|s| s.as_str())
    }
    
    /// Get all variables as a flat HashMap (for compatibility with existing template system)
    pub fn to_variables_map(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        // Add in reverse precedence order so higher precedence overrides
        variables.extend(self.project.clone());
        variables.extend(self.app.clone());
        variables.extend(self.package.clone());
        variables.extend(self.built_in.clone());
        
        variables
    }
    
    /// List all available variables with their sources
    pub fn list_variables(&self) -> Vec<VariableInfo> {
        let mut variables = Vec::new();
        
        // Collect all unique variable names
        let mut all_names = std::collections::HashSet::new();
        all_names.extend(self.built_in.keys());
        all_names.extend(self.project.keys());
        all_names.extend(self.app.keys());
        all_names.extend(self.package.keys());
        
        for name in all_names {
            let source = if self.built_in.contains_key(name) {
                VariableSource::BuiltIn
            } else if self.package.contains_key(name) {
                VariableSource::Package
            } else if self.app.contains_key(name) {
                VariableSource::App
            } else {
                VariableSource::Project
            };
            
            let value = self.get_variable(name).unwrap_or("");
            
            variables.push(VariableInfo {
                name: name.clone(),
                value: value.to_string(),
                source,
            });
        }
        
        variables.sort_by(|a, b| a.name.cmp(&b.name));
        variables
    }
}

/// Information about a variable including its source
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub value: String,
    pub source: VariableSource,
}

/// Source of a variable
#[derive(Debug, Clone, PartialEq)]
pub enum VariableSource {
    BuiltIn,
    Project,
    App,
    Package,
}

impl std::fmt::Display for VariableSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableSource::BuiltIn => write!(f, "built-in"),
            VariableSource::Project => write!(f, "project"),
            VariableSource::App => write!(f, "app"),
            VariableSource::Package => write!(f, "package"),
        }
    }
}

/// Interpolate variables in a string using the {{variable_name}} syntax
pub fn interpolate_variables(text: &str, context: &VariableContext) -> Result<String> {
    let mut result = text.to_string();
    let mut missing_variables = Vec::new();
    
    // Find all variable references using regex
    let re = regex::Regex::new(r"\{\{(\w+)\}\}")
        .context("Failed to compile variable regex")?;
    
    for captures in re.captures_iter(text) {
        let full_match = captures.get(0).unwrap().as_str();
        let var_name = captures.get(1).unwrap().as_str();
        
        if let Some(value) = context.get_variable(var_name) {
            result = result.replace(full_match, value);
        } else {
            missing_variables.push(var_name.to_string());
        }
    }
    
    if !missing_variables.is_empty() {
        anyhow::bail!(
            "Missing variables in template: {}. Available variables: {}",
            missing_variables.join(", "),
            context.list_variables()
                .iter()
                .map(|v| v.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
    
    Ok(result)
}

/// Interpolate variables in all string fields of a configuration structure
pub trait VariableInterpolation {
    /// Apply variable interpolation to all string fields in the configuration
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()>;
}

impl VariableInterpolation for HashMap<String, String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        for (key, value) in self.iter_mut() {
            let interpolated = interpolate_variables(value, context)
                .with_context(|| format!("Failed to interpolate variables in field '{}'", key))?;
            *value = interpolated;
        }
        Ok(())
    }
}

impl VariableInterpolation for Vec<String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        for (index, value) in self.iter_mut().enumerate() {
            let interpolated = interpolate_variables(value, context)
                .with_context(|| format!("Failed to interpolate variables in array item {}", index))?;
            *value = interpolated;
        }
        Ok(())
    }
}

impl VariableInterpolation for Option<String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        if let Some(value) = self {
            let interpolated = interpolate_variables(value, context)
                .with_context(|| "Failed to interpolate variables in optional string")?;
            *value = interpolated;
        }
        Ok(())
    }
}

impl VariableInterpolation for KnotConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        // Interpolate description
        self.description.interpolate_variables(context)
            .context("Failed to interpolate project description")?;
        
        // Interpolate scripts
        if let Some(scripts) = &mut self.scripts {
            scripts.interpolate_variables(context)
                .context("Failed to interpolate project scripts")?;
        }
        
        Ok(())
    }
}

impl VariableInterpolation for AppConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        // Interpolate description
        self.description.interpolate_variables(context)
            .context("Failed to interpolate app description")?;
        
        // Interpolate packages
        if let Some(packages) = &mut self.packages {
            packages.interpolate_variables(context)
                .context("Failed to interpolate app packages")?;
        }
        
        // Interpolate scripts
        if let Some(scripts) = &mut self.scripts {
            scripts.interpolate_variables(context)
                .context("Failed to interpolate app scripts")?;
        }
        
        Ok(())
    }
}

impl VariableInterpolation for PackageConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        // Interpolate description
        self.description.interpolate_variables(context)
            .context("Failed to interpolate package description")?;
        
        // Interpolate author
        self.author.interpolate_variables(context)
            .context("Failed to interpolate package author")?;
        
        // Interpolate license
        self.license.interpolate_variables(context)
            .context("Failed to interpolate package license")?;
        
        // Interpolate repository
        self.repository.interpolate_variables(context)
            .context("Failed to interpolate package repository")?;
        
        // Interpolate keywords
        if let Some(keywords) = &mut self.keywords {
            keywords.interpolate_variables(context)
                .context("Failed to interpolate package keywords")?;
        }
        
        // Interpolate tags
        if let Some(tags) = &mut self.tags {
            tags.interpolate_variables(context)
                .context("Failed to interpolate package tags")?;
        }
        
        // Interpolate scripts
        if let Some(scripts) = &mut self.scripts {
            scripts.interpolate_variables(context)
                .context("Failed to interpolate package scripts")?;
        }
        
        // Interpolate dependencies
        if let Some(dependencies) = &mut self.dependencies {
            dependencies.interpolate_variables(context)
                .context("Failed to interpolate package dependencies")?;
        }
        
        // Interpolate dev dependencies
        if let Some(dev_dependencies) = &mut self.dev_dependencies {
            dev_dependencies.interpolate_variables(context)
                .context("Failed to interpolate package dev dependencies")?;
        }
        
        // Interpolate optional dependencies
        if let Some(optional_dependencies) = &mut self.optional_dependencies {
            optional_dependencies.interpolate_variables(context)
                .context("Failed to interpolate package optional dependencies")?;
        }
        
        // Interpolate peer dependencies
        if let Some(peer_dependencies) = &mut self.peer_dependencies {
            peer_dependencies.interpolate_variables(context)
                .context("Failed to interpolate package peer dependencies")?;
        }
        
        // Interpolate exports
        if let Some(exports) = &mut self.exports {
            exports.interpolate_variables(context)
                .context("Failed to interpolate package exports")?;
        }
        
        // Interpolate features
        if let Some(features) = &mut self.features {
            features.interpolate_variables(context)
                .context("Failed to interpolate package features")?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_variable_context_precedence() {
        let context = VariableContext::new("test-project", Path::new("/test"))
            .with_project_variables(&KnotConfig {
                name: "test".to_string(),
                description: None,
                ts_alias: None,
                apps: None,
                scripts: None,
                variables: Some({
                    let mut vars = HashMap::new();
                    vars.insert("test_var".to_string(), ConfigVariable::Simple("project".to_string()));
                    vars
                }),
            })
            .with_app_variables(&AppConfig {
                name: "test-app".to_string(),
                description: None,
                ts_alias: None,
                packages: None,
                scripts: None,
                variables: Some({
                    let mut vars = HashMap::new();
                    vars.insert("test_var".to_string(), ConfigVariable::Simple("app".to_string()));
                    vars
                }),
            });
        
        // App variable should override project variable
        assert_eq!(context.get_variable("test_var"), Some("app"));
        
        // Built-in variables should be available
        assert_eq!(context.get_variable("project_name"), Some("test-project"));
    }
    
    #[test]
    fn test_variable_interpolation() {
        let context = VariableContext::new("my-project", Path::new("/test"));
        
        let result = interpolate_variables("Hello {{project_name}}!", &context).unwrap();
        assert_eq!(result, "Hello my-project!");
        
        // Test missing variable
        let result = interpolate_variables("Hello {{missing_var}}!", &context);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_config_variable_types() {
        let simple = ConfigVariable::Simple("test".to_string());
        assert_eq!(simple.get_value(), "test");
        assert_eq!(simple.get_description(), None);
        
        let complex = ConfigVariable::Complex {
            value: "test".to_string(),
            description: Some("A test variable".to_string()),
        };
        assert_eq!(complex.get_value(), "test");
        assert_eq!(complex.get_description(), Some("A test variable"));
    }
}