use super::super::{test_utils::*, *};
use std::collections::HashMap;

#[cfg(test)]
mod basic_interpolation_tests {
    use super::*;

    #[test]
    fn test_simple_variable_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "test-project".to_string());
        
        let input = "project_name: ${name}";
        let expected = "project_name: test-project";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_multiple_variables_single_value() {
        let mut variables = HashMap::new();
        variables.insert("prefix".to_string(), "my".to_string());
        variables.insert("suffix".to_string(), "project".to_string());
        
        let input = "name: ${prefix}-${suffix}";
        let expected = "name: my-project";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_multiple_variables_different_fields() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "awesome-app".to_string());
        variables.insert("version".to_string(), "1.0.0".to_string());
        variables.insert("author".to_string(), "saravenpi".to_string());
        
        let input = r#"
name: ${name}
version: ${version}
author: ${author}
"#;
        let expected = r#"
name: awesome-app
version: 1.0.0
author: saravenpi
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_string_data_type_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("description".to_string(), "A powerful CLI tool".to_string());
        
        let input = "description: \"${description}\"";
        let expected = "description: \"A powerful CLI tool\"";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_array_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("pkg1".to_string(), "utils".to_string());
        variables.insert("pkg2".to_string(), "types".to_string());
        variables.insert("pkg3".to_string(), "components".to_string());
        
        let input = r#"
packages:
  - ${pkg1}
  - ${pkg2}
  - ${pkg3}
"#;
        let expected = r#"
packages:
  - utils
  - types
  - components
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_object_value_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("build_cmd".to_string(), "npm run build".to_string());
        variables.insert("test_cmd".to_string(), "npm test".to_string());
        variables.insert("lint_cmd".to_string(), "npm run lint".to_string());
        
        let input = r#"
scripts:
  build: "${build_cmd}"
  test: "${test_cmd}"
  lint: "${lint_cmd}"
"#;
        let expected = r#"
scripts:
  build: "npm run build"
  test: "npm test"
  lint: "npm run lint"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_numeric_variables() {
        let mut variables = HashMap::new();
        variables.insert("port".to_string(), "3000".to_string());
        variables.insert("timeout".to_string(), "30".to_string());
        variables.insert("major_version".to_string(), "2".to_string());
        
        let input = r#"
server:
  port: ${port}
  timeout: ${timeout}
version: "${major_version}.0.0"
"#;
        let expected = r#"
server:
  port: 3000
  timeout: 30
version: "2.0.0"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_boolean_variables() {
        let mut variables = HashMap::new();
        variables.insert("debug_enabled".to_string(), "true".to_string());
        variables.insert("production".to_string(), "false".to_string());
        
        let input = r#"
debug: ${debug_enabled}
production: ${production}
"#;
        let expected = r#"
debug: true
production: false
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_empty_variables_section() {
        let variables = HashMap::new(); // Empty variables
        
        let input = r#"
name: static-name
version: 1.0.0
description: No variables here
"#;
        let expected = input; // Should remain unchanged
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_mixed_interpolated_and_static() {
        let mut variables = HashMap::new();
        variables.insert("app_name".to_string(), "dynamic-app".to_string());
        variables.insert("env".to_string(), "development".to_string());
        
        let input = r#"
name: ${app_name}
version: 1.0.0
environment: ${env}
static_field: "this will not change"
"#;
        let expected = r#"
name: dynamic-app
version: 1.0.0
environment: development
static_field: "this will not change"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_variables_in_yaml_comments_ignored() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "test-app".to_string());
        
        let input = r#"
# This is a comment with ${name} that should not be interpolated
name: ${name}
# Another comment with ${undefined_var}
"#;
        let expected = r#"
# This is a comment with ${name} that should not be interpolated
name: test-app
# Another comment with ${undefined_var}
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod variable_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_config_with_variables_section() {
        let yaml_content = r#"
variables:
  project_name: "knot-test"
  version: "1.0.0"
  author: "saravenpi"

name: ${project_name}
version: ${version}
description: "Test config for ${project_name}"
"#;

        let interpolator = YamlInterpolator::new();
        let result: TestConfig = interpolator.parse_yaml_with_interpolation(yaml_content).unwrap();

        assert_eq!(result.name, "knot-test");
        assert_eq!(result.version, Some("1.0.0".to_string()));
        assert_eq!(result.description, Some("Test config for knot-test".to_string()));
    }

    #[test]
    fn test_config_without_variables_section() {
        let yaml_content = r#"
name: static-name
version: 1.0.0
description: "A static config"
"#;

        let interpolator = YamlInterpolator::new();
        let result: TestConfig = interpolator.parse_yaml_with_interpolation(yaml_content).unwrap();

        assert_eq!(result.name, "static-name");
        assert_eq!(result.version, Some("1.0.0".to_string()));
        assert_eq!(result.description, Some("A static config".to_string()));
    }

    #[test]
    fn test_built_in_variables() {
        let yaml_content = r#"
name: ${project_name}
workspace: ${workspace_root}
timestamp: ${timestamp}
"#;

        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(yaml_content, &HashMap::new()).unwrap();

        assert!(result.contains("knot-test"));
        assert!(result.contains("/test/workspace"));
        assert!(result.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn test_user_variables_override_builtin() {
        let mut variables = HashMap::new();
        variables.insert("project_name".to_string(), "custom-project".to_string());

        let yaml_content = "name: ${project_name}";
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(yaml_content, &variables).unwrap();

        assert_eq!(result, "name: custom-project");
    }
}

#[cfg(test)]
mod data_type_tests {
    use super::*;

    #[test]
    fn test_interpolation_preserves_yaml_structure() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "test-app".to_string());
        variables.insert("version".to_string(), "2.1.0".to_string());
        variables.insert("debug".to_string(), "true".to_string());

        let yaml_content = r#"
variables:
  name: "${name}"
  version: "${version}"
  debug: "${debug}"

config:
  app_name: ${name}
  version: ${version}
  debug_mode: ${debug}
  nested:
    deep_name: ${name}
    deep_version: ${version}
"#;

        let interpolator = YamlInterpolator::new();
        let interpolated = interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(yaml_content).unwrap();

        let config = interpolated.get("config").unwrap();
        assert_eq!(config.get("app_name").unwrap().as_str().unwrap(), "test-app");
        assert_eq!(config.get("version").unwrap().as_str().unwrap(), "2.1.0");
        assert_eq!(config.get("debug_mode").unwrap().as_str().unwrap(), "true");

        let nested = config.get("nested").unwrap();
        assert_eq!(nested.get("deep_name").unwrap().as_str().unwrap(), "test-app");
        assert_eq!(nested.get("deep_version").unwrap().as_str().unwrap(), "2.1.0");
    }

    #[test]
    fn test_array_of_interpolated_values() {
        let mut variables = HashMap::new();
        variables.insert("dep1".to_string(), "lodash".to_string());
        variables.insert("dep2".to_string(), "express".to_string());
        variables.insert("dep3".to_string(), "react".to_string());

        let yaml_content = r#"
variables:
  dep1: "${dep1}"
  dep2: "${dep2}"
  dep3: "${dep3}"

dependencies:
  - ${dep1}
  - ${dep2}
  - ${dep3}
  - static-dep
"#;

        let interpolator = YamlInterpolator::new();
        let result = interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(yaml_content).unwrap();

        let deps = result.get("dependencies").unwrap().as_sequence().unwrap();
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].as_str().unwrap(), "lodash");
        assert_eq!(deps[1].as_str().unwrap(), "express");
        assert_eq!(deps[2].as_str().unwrap(), "react");
        assert_eq!(deps[3].as_str().unwrap(), "static-dep");
    }

    #[test]
    fn test_object_with_interpolated_keys_and_values() {
        let mut variables = HashMap::new();
        variables.insert("env_key".to_string(), "NODE_ENV".to_string());
        variables.insert("env_value".to_string(), "production".to_string());
        variables.insert("port_key".to_string(), "PORT".to_string());
        variables.insert("port_value".to_string(), "8080".to_string());

        let yaml_content = r#"
variables:
  env_key: "${env_key}"
  env_value: "${env_value}"
  port_key: "${port_key}"
  port_value: "${port_value}"

environment:
  "${env_key}": "${env_value}"
  "${port_key}": "${port_value}"
  STATIC_KEY: "static_value"
"#;

        let interpolator = YamlInterpolator::new();
        let result = interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(yaml_content).unwrap();

        let env = result.get("environment").unwrap();
        assert_eq!(env.get("NODE_ENV").unwrap().as_str().unwrap(), "production");
        assert_eq!(env.get("PORT").unwrap().as_str().unwrap(), "8080");
        assert_eq!(env.get("STATIC_KEY").unwrap().as_str().unwrap(), "static_value");
    }
}