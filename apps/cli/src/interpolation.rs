use anyhow::{Context, Result};
use regex::Regex;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::env;

/// Variable interpolation system for YAML configurations
/// Supports ${variable_name} syntax with environment variable fallbacks
pub struct VariableInterpolator {
    variables: HashMap<String, String>,
    env_fallback: bool,
    circular_detection: HashSet<String>,
}

impl VariableInterpolator {
    /// Create a new interpolator with variables extracted from YAML
    pub fn new(yaml_content: &str, env_fallback: bool) -> Result<Self> {
        let mut interpolator = Self {
            variables: HashMap::new(),
            env_fallback,
            circular_detection: HashSet::new(),
        };
        
        interpolator.extract_variables(yaml_content)?;
        Ok(interpolator)
    }

    /// Extract variables section from YAML and populate the variables map
    fn extract_variables(&mut self, yaml_content: &str) -> Result<()> {
        // Parse YAML to extract variables section
        if let Ok(value) = serde_yaml::from_str::<Value>(yaml_content) {
            if let Some(vars_section) = value.get("variables") {
                self.parse_variables_section(vars_section)?;
            }
        }
        
        // Add built-in variables
        self.add_builtin_variables()?;
        
        Ok(())
    }

    /// Parse the variables section and convert to string map
    fn parse_variables_section(&mut self, vars: &Value) -> Result<()> {
        if let Some(vars_map) = vars.as_mapping() {
            for (key, value) in vars_map {
                if let (Some(key_str), Some(value_str)) = (key.as_str(), self.value_to_string(value)) {
                    self.variables.insert(key_str.to_string(), value_str);
                }
            }
        }
        Ok(())
    }

    /// Convert YAML value to string representation
    fn value_to_string(&self, value: &Value) -> Option<String> {
        match value {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            Value::Null => Some("".to_string()),
            _ => None, // Complex types not supported for interpolation
        }
    }

    /// Add built-in system variables
    fn add_builtin_variables(&mut self) -> Result<()> {
        // System information
        self.variables.insert("OS".to_string(), env::consts::OS.to_string());
        self.variables.insert("ARCH".to_string(), env::consts::ARCH.to_string());
        self.variables.insert("FAMILY".to_string(), env::consts::FAMILY.to_string());
        
        // Current working directory
        if let Ok(cwd) = env::current_dir() {
            if let Some(cwd_str) = cwd.to_str() {
                self.variables.insert("PWD".to_string(), cwd_str.to_string());
            }
        }

        Ok(())
    }

    /// Interpolate variables in YAML content
    pub fn interpolate(&mut self, yaml_content: &str) -> Result<String> {
        // Regex to match ${variable_name} patterns
        let var_regex = Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}")
            .context("Failed to create variable regex")?;

        let mut result = yaml_content.to_string();
        let mut max_iterations = 100; // Prevent infinite loops
        
        loop {
            let mut found_substitution = false;
            max_iterations -= 1;
            
            if max_iterations <= 0 {
                anyhow::bail!("Variable interpolation exceeded maximum iterations - possible circular dependency");
            }

            // Find all variable references  
            let result_clone = result.clone();
            let captures: Vec<_> = var_regex.captures_iter(&result_clone).collect();
            
            for capture in captures {
                let full_match = capture.get(0).unwrap().as_str();
                let var_name = capture.get(1).unwrap().as_str();
                
                // Check for circular dependency
                if self.circular_detection.contains(var_name) {
                    anyhow::bail!("Circular dependency detected for variable: {}", var_name);
                }

                // Add to circular detection before resolving
                self.circular_detection.insert(var_name.to_string());
                
                let resolve_result = self.resolve_variable(var_name);
                
                // Always remove from circular detection after resolving (success or failure)
                self.circular_detection.remove(var_name);
                
                let var_value = resolve_result
                    .with_context(|| format!("Failed to resolve variable: {}", var_name))?;
                
                result = result.replace(full_match, &var_value);
                found_substitution = true;
            }
            
            if !found_substitution {
                break;
            }
        }

        Ok(result)
    }

    /// Resolve a single variable value with fallbacks
    fn resolve_variable(&self, name: &str) -> Result<String> {
        // First try explicit variables
        if let Some(value) = self.variables.get(name) {
            return Ok(value.clone());
        }

        // Fall back to environment variables if enabled
        if self.env_fallback {
            if let Ok(env_value) = env::var(name) {
                return Ok(env_value);
            }
        }

        anyhow::bail!("Variable '{}' is not defined", name);
    }

    /// Add or update a variable
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    /// Get all defined variables
    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Check if a variable is defined
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name) || (self.env_fallback && env::var(name).is_ok())
    }

    /// Validate that all variables in the content can be resolved
    pub fn validate_variables(&mut self, yaml_content: &str) -> Result<Vec<String>> {
        let var_regex = Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}")
            .context("Failed to create variable regex")?;

        let mut undefined_vars = Vec::new();
        let mut seen_vars = HashSet::new();

        for capture in var_regex.captures_iter(yaml_content) {
            let var_name = capture.get(1).unwrap().as_str();
            
            if !seen_vars.contains(var_name) {
                seen_vars.insert(var_name);
                
                if self.resolve_variable(var_name).is_err() {
                    undefined_vars.push(var_name.to_string());
                }
            }
        }

        Ok(undefined_vars)
    }
}

/// Interpolate variables in YAML content with default settings
pub fn interpolate_yaml(yaml_content: &str) -> Result<String> {
    let mut interpolator = VariableInterpolator::new(yaml_content, true)?;
    interpolator.interpolate(yaml_content)
}

/// Interpolate variables in YAML content with custom variable map
pub fn interpolate_yaml_with_vars(yaml_content: &str, variables: HashMap<String, String>) -> Result<String> {
    let mut interpolator = VariableInterpolator::new(yaml_content, true)?;
    
    // Add custom variables
    for (key, value) in variables {
        interpolator.set_variable(key, value);
    }
    
    interpolator.interpolate(yaml_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_interpolation() {
        let yaml_content = r#"
variables:
  name: "test-project"
  version: "1.0.0"

project:
  name: "${name}"
  version: "${version}"
  description: "Project ${name} version ${version}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("name: \"test-project\""));
        assert!(result.contains("version: \"1.0.0\""));
        assert!(result.contains("description: \"Project test-project version 1.0.0\""));
    }

    #[test]
    fn test_nested_interpolation() {
        let yaml_content = r#"
variables:
  base_name: "my-app"
  env: "prod"
  full_name: "${base_name}-${env}"

project:
  name: "${full_name}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("name: \"my-app-prod\""));
    }

    #[test]
    fn test_builtin_variables() {
        let yaml_content = r#"
project:
  platform: "${OS}"
  architecture: "${ARCH}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("platform:"));
        assert!(result.contains("architecture:"));
    }

    #[test]
    fn test_undefined_variable_error() {
        let yaml_content = r#"
project:
  name: "${undefined_var}"
"#;

        let result = interpolate_yaml(yaml_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("undefined_var"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let yaml_content = r#"
variables:
  var1: "${var2}"
  var2: "${var1}"

project:
  name: "${var1}"
"#;

        let result = interpolate_yaml(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // The error should mention circular dependency somewhere in the chain
        assert!(error_msg.contains("Circular dependency") || error_msg.contains("Failed to resolve variable") || error_msg.contains("maximum iterations"));
    }

    #[test]
    fn test_custom_variables() {
        let yaml_content = r#"
project:
  name: "${custom_name}"
  version: "${custom_version}"
"#;

        let mut custom_vars = HashMap::new();
        custom_vars.insert("custom_name".to_string(), "test-app".to_string());
        custom_vars.insert("custom_version".to_string(), "2.0.0".to_string());

        let result = interpolate_yaml_with_vars(yaml_content, custom_vars).unwrap();
        assert!(result.contains("name: \"test-app\""));
        assert!(result.contains("version: \"2.0.0\""));
    }

    #[test]
    fn test_variable_validation() {
        let yaml_content = r#"
project:
  name: "${defined_var}"
  version: "${undefined_var}"
"#;

        let mut interpolator = VariableInterpolator::new("", true).unwrap();
        interpolator.set_variable("defined_var".to_string(), "test".to_string());

        let undefined_vars = interpolator.validate_variables(yaml_content).unwrap();
        assert_eq!(undefined_vars.len(), 1);
        assert!(undefined_vars.contains(&"undefined_var".to_string()));
    }

    #[test]
    fn test_environment_variable_fallback() {
        env::set_var("TEST_ENV_VAR", "env_value");

        let yaml_content = r#"
project:
  name: "${TEST_ENV_VAR}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("name: \"env_value\""));

        env::remove_var("TEST_ENV_VAR");
    }

    #[test]
    fn test_different_data_types() {
        let yaml_content = r#"
variables:
  debug: true
  port: 3000
  timeout: 5.5

config:
  debug: "${debug}"
  port: "${port}"
  timeout: "${timeout}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("debug: \"true\""));
        assert!(result.contains("port: \"3000\""));
        assert!(result.contains("timeout: \"5.5\""));
    }

    #[test]
    fn test_empty_variables_section() {
        let yaml_content = r#"
project:
  name: "simple"
  platform: "${OS}"
"#;

        let result = interpolate_yaml(yaml_content).unwrap();
        assert!(result.contains("name: \"simple\""));
        assert!(result.contains("platform:"));
    }

    #[test]
    fn test_malformed_variable_syntax() {
        let yaml_content = r#"
project:
  name: "$incomplete"
  version: "${valid_var}"
"#;

        // Should not fail - malformed syntax should be left as-is
        let mut interpolator = VariableInterpolator::new("", true).unwrap();
        interpolator.set_variable("valid_var".to_string(), "1.0.0".to_string());
        
        let result = interpolator.interpolate(yaml_content).unwrap();
        assert!(result.contains("name: \"$incomplete\""));
        assert!(result.contains("version: \"1.0.0\""));
    }
}