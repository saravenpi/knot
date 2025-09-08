use super::super::{test_utils::*, *};
use std::collections::HashMap;

#[cfg(test)]
mod undefined_variable_tests {
    use super::*;

    #[test]
    fn test_undefined_variable_error() {
        let variables = HashMap::new();
        let input = "name: ${undefined_var}";
        
        assert_interpolation_error(input, variables, "Undefined variable: undefined_var").unwrap();
    }

    #[test]
    fn test_multiple_undefined_variables() {
        let variables = HashMap::new();
        let input = "name: ${var1}-${var2}";
        
        // Should fail on the first undefined variable encountered
        assert_interpolation_error(input, variables, "Undefined variable").unwrap();
    }

    #[test]
    fn test_partially_defined_variables() {
        let mut variables = HashMap::new();
        variables.insert("defined_var".to_string(), "value".to_string());
        
        let input = "name: ${defined_var}-${undefined_var}";
        
        assert_interpolation_error(input, variables, "Undefined variable: undefined_var").unwrap();
    }

    #[test]
    fn test_undefined_nested_variable() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "${undefined_var}".to_string());
        
        let input = "name: ${var1}";
        
        assert_interpolation_error(input, variables, "Undefined variable: undefined_var").unwrap();
    }

    #[test]
    fn test_undefined_in_complex_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("prefix".to_string(), "app".to_string());
        variables.insert("suffix".to_string(), "v1".to_string());
        // missing 'env' variable
        
        let input = "service: ${prefix}-${env}-${suffix}";
        
        assert_interpolation_error(input, variables, "Undefined variable: env").unwrap();
    }
}

#[cfg(test)]
mod circular_dependency_tests {
    use super::*;

    #[test]
    fn test_simple_circular_dependency() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "${var2}".to_string());
        variables.insert("var2".to_string(), "${var1}".to_string());
        
        let input = "name: ${var1}";
        
        assert_interpolation_error(input, variables, "Circular dependency detected").unwrap();
    }

    #[test]
    fn test_complex_circular_dependency() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "${var2}".to_string());
        variables.insert("var2".to_string(), "${var3}".to_string());
        variables.insert("var3".to_string(), "${var4}".to_string());
        variables.insert("var4".to_string(), "${var1}".to_string()); // Circular reference
        
        let input = "name: ${var1}";
        
        assert_interpolation_error(input, variables, "Circular dependency detected").unwrap();
    }

    #[test]
    fn test_self_referencing_variable() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "prefix-${var1}-suffix".to_string());
        
        let input = "name: ${var1}";
        
        assert_interpolation_error(input, variables, "Circular dependency detected").unwrap();
    }

    #[test]
    fn test_indirect_circular_dependency() {
        let mut variables = HashMap::new();
        variables.insert("app_name".to_string(), "myapp".to_string());
        variables.insert("service_name".to_string(), "${app_name}-${version}".to_string());
        variables.insert("version".to_string(), "v1-${service_name}".to_string()); // Indirect circular reference
        
        let input = "service: ${service_name}";
        
        assert_interpolation_error(input, variables, "Circular dependency detected").unwrap();
    }

    #[test]
    fn test_valid_complex_dependencies_no_circle() {
        let mut variables = HashMap::new();
        variables.insert("base".to_string(), "myapp".to_string());
        variables.insert("env".to_string(), "prod".to_string());
        variables.insert("version".to_string(), "v1".to_string());
        variables.insert("service".to_string(), "${base}-${env}".to_string());
        variables.insert("full_name".to_string(), "${service}-${version}".to_string());
        
        let input = "name: ${full_name}";
        let expected = "name: myapp-prod-v1";
        
        // This should work fine - no circular dependencies
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod malformed_syntax_tests {
    use super::*;

    #[test]
    fn test_unclosed_variable_syntax() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "value".to_string());
        
        let input = "name: ${var1";
        
        // This should not find the variable due to malformed syntax
        assert_interpolation_error(input, variables, "Undefined variable").unwrap();
    }

    #[test]
    fn test_unopened_variable_syntax() {
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "value".to_string());
        
        let input = "name: var1}";
        let expected = "name: var1}"; // Should remain unchanged
        
        // This should not interpolate - just pass through
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_empty_variable_name() {
        let variables = HashMap::new();
        let input = "name: ${}";
        
        assert_interpolation_error(input, variables, "Undefined variable").unwrap();
    }

    #[test]
    fn test_nested_braces_in_variable() {
        let mut variables = HashMap::new();
        variables.insert("var{1}".to_string(), "value".to_string());
        
        let input = "name: ${var{1}}";
        
        // Variable names with braces are not valid
        assert_interpolation_error(input, variables, "Undefined variable").unwrap();
    }

    #[test]
    fn test_special_characters_in_variable_names() {
        let mut variables = HashMap::new();
        variables.insert("var@name".to_string(), "value1".to_string());
        variables.insert("var-name".to_string(), "value2".to_string());
        variables.insert("var_name".to_string(), "value3".to_string());
        variables.insert("var.name".to_string(), "value4".to_string());
        
        let input = r#"
test1: ${var@name}
test2: ${var-name}
test3: ${var_name}
test4: ${var.name}
"#;
        
        let expected = r#"
test1: value1
test2: value2
test3: value3
test4: value4
"#;
        
        // These should all work - special chars in variable names are allowed
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_whitespace_in_variable_names() {
        let mut variables = HashMap::new();
        variables.insert("var name".to_string(), "value".to_string());
        
        let input = "name: ${var name}";
        let expected = "name: value";
        
        // Whitespace in variable names should work
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod invalid_variable_names_tests {
    use super::*;

    #[test]
    fn test_numeric_only_variable_names() {
        let mut variables = HashMap::new();
        variables.insert("123".to_string(), "numeric_value".to_string());
        
        let input = "value: ${123}";
        let expected = "value: numeric_value";
        
        // Numeric variable names should be allowed
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_mixed_alphanumeric_variable_names() {
        let mut variables = HashMap::new();
        variables.insert("var123".to_string(), "mixed_value".to_string());
        variables.insert("123var".to_string(), "reverse_mixed".to_string());
        
        let input = r#"
test1: ${var123}
test2: ${123var}
"#;
        let expected = r#"
test1: mixed_value
test2: reverse_mixed
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_very_long_variable_names() {
        let long_name = "a".repeat(1000);
        let mut variables = HashMap::new();
        variables.insert(long_name.clone(), "long_name_value".to_string());
        
        let input = format!("value: ${{{}}}", long_name);
        let expected = "value: long_name_value";
        
        // Very long variable names should work
        assert_interpolation_equals(&input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod type_conflict_tests {
    use super::*;

    #[test]
    fn test_interpolation_preserves_yaml_types() {
        let mut variables = HashMap::new();
        variables.insert("string_var".to_string(), "hello".to_string());
        variables.insert("number_var".to_string(), "42".to_string());
        variables.insert("bool_var".to_string(), "true".to_string());
        
        let yaml_content = r#"
variables:
  string_var: "${string_var}"
  number_var: "${number_var}"
  bool_var: "${bool_var}"

config:
  name: ${string_var}
  count: ${number_var}
  enabled: ${bool_var}
  quoted_number: "${number_var}"
  quoted_bool: "${bool_var}"
"#;
        
        let interpolator = YamlInterpolator::new();
        let result = interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(yaml_content).unwrap();
        
        let config = result.get("config").unwrap();
        
        // Unquoted interpolations should preserve type inference
        assert_eq!(config.get("name").unwrap().as_str().unwrap(), "hello");
        assert_eq!(config.get("count").unwrap().as_u64().unwrap(), 42);
        assert_eq!(config.get("enabled").unwrap().as_bool().unwrap(), true);
        
        // Quoted interpolations should be strings
        assert_eq!(config.get("quoted_number").unwrap().as_str().unwrap(), "42");
        assert_eq!(config.get("quoted_bool").unwrap().as_str().unwrap(), "true");
    }

    #[test]
    fn test_complex_type_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("json_array".to_string(), "[1, 2, 3]".to_string());
        variables.insert("yaml_object".to_string(), "{key: value}".to_string());
        
        let input = r#"
array_field: ${json_array}
object_field: ${yaml_object}
"#;
        
        let expected = r#"
array_field: [1, 2, 3]
object_field: {key: value}
"#;
        
        // Complex types as strings should interpolate correctly
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod infinite_recursion_tests {
    use super::*;

    #[test]
    fn test_infinite_recursion_prevention() {
        let mut variables = HashMap::new();
        // Create a very deep but non-circular dependency chain
        for i in 0..150 {
            variables.insert(format!("var{}", i), format!("${{var{}}}", i + 1));
        }
        variables.insert("var150".to_string(), "final_value".to_string());
        
        let input = "value: ${var0}";
        
        // This should hit the maximum iteration limit
        assert_interpolation_error(input, variables, "Maximum interpolation iterations reached").unwrap();
    }

    #[test]
    fn test_reasonable_depth_works() {
        let mut variables = HashMap::new();
        // Create a reasonable depth dependency chain
        for i in 0..50 {
            variables.insert(format!("var{}", i), format!("${{var{}}}", i + 1));
        }
        variables.insert("var50".to_string(), "final_value".to_string());
        
        let input = "value: ${var0}";
        let expected = "value: final_value";
        
        // This should work fine
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_complex_interpolation_with_reasonable_nesting() {
        let mut variables = HashMap::new();
        variables.insert("base".to_string(), "myapp".to_string());
        variables.insert("env".to_string(), "prod".to_string());
        variables.insert("region".to_string(), "us-west".to_string());
        variables.insert("tier1".to_string(), "${base}-${env}".to_string());
        variables.insert("tier2".to_string(), "${tier1}-${region}".to_string());
        variables.insert("tier3".to_string(), "${tier2}-service".to_string());
        variables.insert("tier4".to_string(), "${tier3}-v1".to_string());
        variables.insert("final".to_string(), "${tier4}.example.com".to_string());
        
        let input = "url: ${final}";
        let expected = "url: myapp-prod-us-west-service-v1.example.com";
        
        // Multi-level nesting should work fine
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}