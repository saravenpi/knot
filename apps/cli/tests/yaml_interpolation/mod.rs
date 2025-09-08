use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mock configuration structure for testing YAML variable interpolation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestConfig {
    pub variables: Option<HashMap<String, String>>,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub packages: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}

/// YAML Variable Interpolation Engine for testing
/// This would typically be part of the main codebase but is implemented here for testing
pub struct YamlInterpolator {
    pub built_in_vars: HashMap<String, String>,
    pub env_fallbacks: bool,
}

impl Default for YamlInterpolator {
    fn default() -> Self {
        Self::new()
    }
}

impl YamlInterpolator {
    pub fn new() -> Self {
        let mut built_in_vars = HashMap::new();
        built_in_vars.insert("project_name".to_string(), "knot-test".to_string());
        built_in_vars.insert("workspace_root".to_string(), "/test/workspace".to_string());
        built_in_vars.insert("timestamp".to_string(), "2024-01-01T00:00:00Z".to_string());

        Self {
            built_in_vars,
            env_fallbacks: false,
        }
    }

    pub fn with_env_fallbacks(mut self, enabled: bool) -> Self {
        self.env_fallbacks = enabled;
        self
    }

    pub fn add_built_in_var<S: Into<String>>(&mut self, key: S, value: S) {
        self.built_in_vars.insert(key.into(), value.into());
    }

    /// Interpolate variables in YAML content
    pub fn interpolate(&self, yaml_content: &str, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = yaml_content.to_string();
        let mut resolved_vars = HashMap::new();
        
        // Add built-in variables first
        for (k, v) in &self.built_in_vars {
            resolved_vars.insert(k.clone(), v.clone());
        }
        
        // Add user-defined variables (can override built-ins)
        for (k, v) in variables {
            resolved_vars.insert(k.clone(), v.clone());
        }
        
        // Resolve nested variables with circular dependency detection
        resolved_vars = self.resolve_nested_variables(&resolved_vars)?;
        
        // Perform interpolation
        result = self.interpolate_string(&result, &resolved_vars)?;
        
        Ok(result)
    }

    /// Resolve nested variable references like ${var1} -> ${var2} -> value
    fn resolve_nested_variables(&self, variables: &HashMap<String, String>) -> Result<HashMap<String, String>> {
        let mut resolved = HashMap::new();
        let mut visiting = std::collections::HashSet::new();
        let mut visited = std::collections::HashSet::new();

        for (key, _) in variables {
            if !visited.contains(key) {
                self.resolve_variable_recursive(key, variables, &mut resolved, &mut visiting, &mut visited)?;
            }
        }

        Ok(resolved)
    }

    fn resolve_variable_recursive(
        &self,
        key: &str,
        variables: &HashMap<String, String>,
        resolved: &mut HashMap<String, String>,
        visiting: &mut std::collections::HashSet<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visiting.contains(key) {
            return Err(anyhow::anyhow!("Circular dependency detected in variable: {}", key));
        }

        if visited.contains(key) {
            return Ok(());
        }

        visiting.insert(key.to_string());

        if let Some(value) = variables.get(key) {
            let interpolated_value = self.interpolate_string(value, resolved)?;
            resolved.insert(key.to_string(), interpolated_value);
        }

        visiting.remove(key);
        visited.insert(key.to_string());
        Ok(())
    }

    /// Interpolate variables in a string using ${variable_name} syntax
    fn interpolate_string(&self, input: &str, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = input.to_string();
        let variable_pattern = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();

        // Keep interpolating until no more variables are found (for nested cases)
        let mut changed = true;
        let mut iteration_count = 0;
        const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

        while changed && iteration_count < MAX_ITERATIONS {
            changed = false;
            iteration_count += 1;

            let captures: Vec<_> = variable_pattern.captures_iter(&result).collect();
            
            for capture in captures {
                if let Some(var_match) = capture.get(0) {
                    if let Some(var_name_match) = capture.get(1) {
                        let var_name = var_name_match.as_str();
                        
                        if let Some(var_value) = variables.get(var_name) {
                            result = result.replace(var_match.as_str(), var_value);
                            changed = true;
                        } else if self.env_fallbacks {
                            if let Ok(env_value) = std::env::var(var_name) {
                                result = result.replace(var_match.as_str(), &env_value);
                                changed = true;
                            } else {
                                return Err(anyhow::anyhow!("Undefined variable: {}", var_name));
                            }
                        } else {
                            return Err(anyhow::anyhow!("Undefined variable: {}", var_name));
                        }
                    }
                }
            }
        }

        if iteration_count >= MAX_ITERATIONS {
            return Err(anyhow::anyhow!("Maximum interpolation iterations reached - possible infinite recursion"));
        }

        Ok(result)
    }

    /// Parse YAML with variable interpolation
    pub fn parse_yaml_with_interpolation<T>(&self, yaml_content: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // First, parse to extract variables section
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
        
        let variables = if let Some(vars_section) = yaml_value.get("variables") {
            self.extract_variables(vars_section)?
        } else {
            HashMap::new()
        };

        // Interpolate the YAML content
        let interpolated_yaml = self.interpolate(yaml_content, &variables)?;

        // Parse the interpolated YAML into the target type
        let result: T = serde_yaml::from_str(&interpolated_yaml)?;
        Ok(result)
    }

    /// Extract variables from YAML variables section
    fn extract_variables(&self, vars_section: &serde_yaml::Value) -> Result<HashMap<String, String>> {
        let mut variables = HashMap::new();

        if let Some(vars_map) = vars_section.as_mapping() {
            for (key, value) in vars_map {
                if let Some(key_str) = key.as_str() {
                    let value_str = match value {
                        serde_yaml::Value::String(s) => s.clone(),
                        serde_yaml::Value::Number(n) => n.to_string(),
                        serde_yaml::Value::Bool(b) => b.to_string(),
                        _ => serde_yaml::to_string(value)?.trim().to_string(),
                    };
                    variables.insert(key_str.to_string(), value_str);
                }
            }
        }

        Ok(variables)
    }
}

/// Test utilities and helpers
pub mod test_utils {
    use super::*;
    use std::fs;
    use std::path::Path;

    pub fn load_test_yaml(filename: &str) -> Result<String> {
        let test_file_path = Path::new("tests/yaml_interpolation/fixtures").join(filename);
        fs::read_to_string(test_file_path).map_err(|e| anyhow::anyhow!("Failed to read test file {}: {}", filename, e))
    }

    pub fn create_test_config(variables: HashMap<String, String>) -> TestConfig {
        TestConfig {
            variables: Some(variables),
            name: "${project_name}".to_string(),
            description: Some("Test config for ${project_name}".to_string()),
            version: Some("${version}".to_string()),
            scripts: Some({
                let mut scripts = HashMap::new();
                scripts.insert("build".to_string(), "${build_command}".to_string());
                scripts.insert("test".to_string(), "${test_command}".to_string());
                scripts
            }),
            packages: Some(vec!["${package1}".to_string(), "${package2}".to_string()]),
            metadata: None,
        }
    }

    pub fn assert_interpolation_equals(input: &str, expected: &str, variables: HashMap<String, String>) -> Result<()> {
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(input, &variables)?;
        assert_eq!(result.trim(), expected.trim(), "Interpolation result doesn't match expected");
        Ok(())
    }

    pub fn assert_interpolation_error(input: &str, variables: HashMap<String, String>, expected_error: &str) -> Result<()> {
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(input, &variables);
        assert!(result.is_err(), "Expected interpolation to fail but it succeeded");
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains(expected_error), 
                "Error message '{}' doesn't contain expected text '{}'", error_msg, expected_error);
        Ok(())
    }
}