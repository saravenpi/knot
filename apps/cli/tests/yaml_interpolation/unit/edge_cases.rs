use super::super::{test_utils::*, *};
use std::collections::HashMap;

#[cfg(test)]
mod empty_value_tests {
    use super::*;

    #[test]
    fn test_empty_string_variable() {
        let mut variables = HashMap::new();
        variables.insert("empty_var".to_string(), "".to_string());
        variables.insert("prefix".to_string(), "hello".to_string());
        
        let input = r#"
value1: ${empty_var}
value2: "${empty_var}"
value3: ${prefix}${empty_var}
value4: ${empty_var}${prefix}
"#;
        
        let expected = r#"
value1: 
value2: ""
value3: hello
value4: hello
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_whitespace_only_variable() {
        let mut variables = HashMap::new();
        variables.insert("spaces".to_string(), "   ".to_string());
        variables.insert("tabs".to_string(), "\t\t".to_string());
        variables.insert("newlines".to_string(), "\n\n".to_string());
        
        let input = r#"
spaces: "${spaces}"
tabs: "${tabs}"
newlines: "${newlines}"
"#;
        
        let expected = r#"
spaces: "   "
tabs: "		"
newlines: "

"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_null_like_values() {
        let mut variables = HashMap::new();
        variables.insert("null_str".to_string(), "null".to_string());
        variables.insert("nil_str".to_string(), "nil".to_string());
        variables.insert("undefined_str".to_string(), "undefined".to_string());
        
        let input = r#"
null_value: ${null_str}
nil_value: ${nil_str}
undefined_value: ${undefined_str}
"#;
        
        let expected = r#"
null_value: null
nil_value: nil
undefined_value: undefined
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod special_character_tests {
    use super::*;

    #[test]
    fn test_special_characters_in_values() {
        let mut variables = HashMap::new();
        variables.insert("special1".to_string(), "!@#$%^&*()".to_string());
        variables.insert("special2".to_string(), "[]{}|\\;':\",./<>?".to_string());
        variables.insert("special3".to_string(), "`~-_=+".to_string());
        
        let input = r#"
symbols1: ${special1}
symbols2: ${special2}
symbols3: ${special3}
combined: ${special1}${special2}${special3}
"#;
        
        let expected = r#"
symbols1: !@#$%^&*()
symbols2: []{}|\;':",./<>?
symbols3: `~-_=+
combined: !@#$%^&*()[]{}|\;':",./<>?`~-_=+
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_yaml_special_characters() {
        let mut variables = HashMap::new();
        variables.insert("colon".to_string(), "key: value".to_string());
        variables.insert("dash".to_string(), "- item".to_string());
        variables.insert("hash".to_string(), "# comment".to_string());
        variables.insert("pipe".to_string(), "| literal".to_string());
        variables.insert("gt".to_string(), "> folded".to_string());
        
        let input = r#"
colon_test: "${colon}"
dash_test: "${dash}"
hash_test: "${hash}"
pipe_test: "${pipe}"
gt_test: "${gt}"
"#;
        
        let expected = r#"
colon_test: "key: value"
dash_test: "- item"
hash_test: "# comment"
pipe_test: "| literal"
gt_test: "> folded"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_quotes_in_variables() {
        let mut variables = HashMap::new();
        variables.insert("single_quotes".to_string(), "It's a test".to_string());
        variables.insert("double_quotes".to_string(), "He said \"hello\"".to_string());
        variables.insert("mixed_quotes".to_string(), "It's \"quoted\" content".to_string());
        
        let input = r#"
single: ${single_quotes}
double: ${double_quotes}
mixed: ${mixed_quotes}
"#;
        
        let expected = r#"
single: It's a test
double: He said "hello"
mixed: It's "quoted" content
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_escape_sequences() {
        let mut variables = HashMap::new();
        variables.insert("newline".to_string(), "line1\nline2".to_string());
        variables.insert("tab".to_string(), "col1\tcol2".to_string());
        variables.insert("carriage_return".to_string(), "text\roverwritten".to_string());
        variables.insert("backslash".to_string(), "path\\to\\file".to_string());
        
        let input = r#"
newline_test: "${newline}"
tab_test: "${tab}"
cr_test: "${carriage_return}"
backslash_test: "${backslash}"
"#;
        
        let expected = r#"
newline_test: "line1
line2"
tab_test: "col1	col2"
cr_test: "textoverwritten"
backslash_test: "path\to\file"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod unicode_tests {
    use super::*;

    #[test]
    fn test_unicode_in_variable_names() {
        let mut variables = HashMap::new();
        variables.insert("café".to_string(), "coffee_shop".to_string());
        variables.insert("naïve".to_string(), "innocent".to_string());
        variables.insert("中文".to_string(), "chinese".to_string());
        variables.insert("العربية".to_string(), "arabic".to_string());
        variables.insert("🚀".to_string(), "rocket".to_string());
        
        let input = r#"
french1: ${café}
french2: ${naïve}
chinese: ${中文}
arabic: ${العربية}
emoji: ${🚀}
"#;
        
        let expected = r#"
french1: coffee_shop
french2: innocent
chinese: chinese
arabic: arabic
emoji: rocket
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_unicode_in_variable_values() {
        let mut variables = HashMap::new();
        variables.insert("emoji_string".to_string(), "🎉🎊🎈🎁".to_string());
        variables.insert("mixed_lang".to_string(), "Hello 世界 مرحبا".to_string());
        variables.insert("accents".to_string(), "café naïve résumé".to_string());
        variables.insert("symbols".to_string(), "∑∞∂∆µ".to_string());
        
        let input = r#"
emojis: ${emoji_string}
international: ${mixed_lang}
accented: ${accents}
mathematical: ${symbols}
"#;
        
        let expected = r#"
emojis: 🎉🎊🎈🎁
international: Hello 世界 مرحبا
accented: café naïve résumé
mathematical: ∑∞∂∆µ
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_unicode_normalization() {
        let mut variables = HashMap::new();
        // Test different Unicode normalization forms for the same visual character
        variables.insert("composed".to_string(), "é".to_string()); // Single character
        variables.insert("decomposed".to_string(), "é".to_string()); // e + combining accent
        
        let input = r#"
form1: ${composed}
form2: ${decomposed}
"#;
        
        // Both should work regardless of normalization form
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(input, &variables);
        assert!(result.is_ok(), "Unicode normalization should not prevent interpolation");
    }

    #[test]
    fn test_zero_width_characters() {
        let mut variables = HashMap::new();
        variables.insert("zero_width".to_string(), "invisible\u{200B}space".to_string()); // Zero-width space
        variables.insert("rtl_mark".to_string(), "text\u{200F}direction".to_string()); // Right-to-left mark
        
        let input = r#"
zero_width_test: ${zero_width}
rtl_test: ${rtl_mark}
"#;
        
        let expected = r#"
zero_width_test: invisible​space
rtl_test: text‏direction
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod very_long_content_tests {
    use super::*;

    #[test]
    fn test_very_long_variable_values() {
        let long_value = "x".repeat(10000);
        let mut variables = HashMap::new();
        variables.insert("long_var".to_string(), long_value.clone());
        
        let input = "value: ${long_var}";
        let expected = format!("value: {}", long_value);
        
        assert_interpolation_equals(input, &expected, variables).unwrap();
    }

    #[test]
    fn test_many_variables_in_single_line() {
        let mut variables = HashMap::new();
        for i in 0..100 {
            variables.insert(format!("var{}", i), i.to_string());
        }
        
        let mut input = String::from("value: ");
        let mut expected = String::from("value: ");
        
        for i in 0..100 {
            if i > 0 {
                input.push('-');
                expected.push('-');
            }
            input.push_str(&format!("${{var{}}}", i));
            expected.push_str(&i.to_string());
        }
        
        assert_interpolation_equals(&input, &expected, variables).unwrap();
    }

    #[test]
    fn test_deeply_nested_yaml_with_interpolation() {
        let mut variables = HashMap::new();
        variables.insert("value".to_string(), "test".to_string());
        
        let mut input = String::new();
        let mut expected = String::new();
        
        // Create deeply nested YAML structure
        for i in 0..20 {
            let indent = "  ".repeat(i);
            input.push_str(&format!("{}level{}:\n", indent, i));
            expected.push_str(&format!("{}level{}:\n", indent, i));
        }
        
        let indent = "  ".repeat(20);
        input.push_str(&format!("{}final_value: ${{value}}", indent));
        expected.push_str(&format!("{}final_value: test", indent));
        
        assert_interpolation_equals(&input, &expected, variables).unwrap();
    }
}

#[cfg(test)]
mod boundary_condition_tests {
    use super::*;

    #[test]
    fn test_variable_at_string_boundaries() {
        let mut variables = HashMap::new();
        variables.insert("start".to_string(), "beginning".to_string());
        variables.insert("end".to_string(), "finish".to_string());
        
        let input = r#"
start_only: ${start}
end_only: ${end}
both_ends: ${start} and ${end}
start_boundary: ${start}-middle
end_boundary: middle-${end}
"#;
        
        let expected = r#"
start_only: beginning
end_only: finish
both_ends: beginning and finish
start_boundary: beginning-middle
end_boundary: middle-finish
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_adjacent_variables() {
        let mut variables = HashMap::new();
        variables.insert("a".to_string(), "A".to_string());
        variables.insert("b".to_string(), "B".to_string());
        variables.insert("c".to_string(), "C".to_string());
        
        let input = "value: ${a}${b}${c}";
        let expected = "value: ABC";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_variables_with_numbers_and_underscores() {
        let mut variables = HashMap::new();
        variables.insert("var_1".to_string(), "first".to_string());
        variables.insert("var2".to_string(), "second".to_string());
        variables.insert("VAR_3".to_string(), "third".to_string());
        variables.insert("_var4".to_string(), "fourth".to_string());
        variables.insert("var5_".to_string(), "fifth".to_string());
        variables.insert("123_var".to_string(), "sixth".to_string());
        
        let input = r#"
underscore: ${var_1}
number: ${var2}
uppercase: ${VAR_3}
leading_underscore: ${_var4}
trailing_underscore: ${var5_}
number_prefix: ${123_var}
"#;
        
        let expected = r#"
underscore: first
number: second
uppercase: third
leading_underscore: fourth
trailing_underscore: fifth
number_prefix: sixth
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_case_sensitivity() {
        let mut variables = HashMap::new();
        variables.insert("var".to_string(), "lowercase".to_string());
        variables.insert("VAR".to_string(), "uppercase".to_string());
        variables.insert("Var".to_string(), "titlecase".to_string());
        variables.insert("vAr".to_string(), "mixedcase".to_string());
        
        let input = r#"
lower: ${var}
upper: ${VAR}
title: ${Var}
mixed: ${vAr}
"#;
        
        let expected = r#"
lower: lowercase
upper: uppercase
title: titlecase
mixed: mixedcase
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod malformed_yaml_with_interpolation_tests {
    use super::*;

    #[test]
    fn test_interpolation_in_invalid_yaml_context() {
        let mut variables = HashMap::new();
        variables.insert("value".to_string(), "test".to_string());
        
        // Even if the YAML structure is malformed, interpolation should still work on the string level
        let input = r#"
invalid_structure:
  - item1
    missing_dash: ${value}
  - item2
"#;
        
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(input, &variables);
        
        assert!(result.is_ok());
        assert!(result.unwrap().contains("missing_dash: test"));
    }

    #[test]
    fn test_variables_in_yaml_literals() {
        let mut variables = HashMap::new();
        variables.insert("content".to_string(), "interpolated content".to_string());
        
        let input = r#"
literal_block: |
  This is a literal block with ${content}
  It should preserve formatting.
  
folded_block: >
  This is a folded block
  with ${content} that should
  be folded into a single line.
"#;
        
        let expected = r#"
literal_block: |
  This is a literal block with interpolated content
  It should preserve formatting.
  
folded_block: >
  This is a folded block
  with interpolated content that should
  be folded into a single line.
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_variables_in_complex_yaml_structures() {
        let mut variables = HashMap::new();
        variables.insert("service_name".to_string(), "my-service".to_string());
        variables.insert("version".to_string(), "1.0.0".to_string());
        variables.insert("replicas".to_string(), "3".to_string());
        
        let input = r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ${service_name}
  labels:
    app: ${service_name}
    version: ${version}
spec:
  replicas: ${replicas}
  selector:
    matchLabels:
      app: ${service_name}
  template:
    metadata:
      labels:
        app: ${service_name}
        version: ${version}
    spec:
      containers:
      - name: ${service_name}
        image: registry.example.com/${service_name}:${version}
        ports:
        - containerPort: 8080
        env:
        - name: SERVICE_NAME
          value: ${service_name}
        - name: VERSION
          value: ${version}
---
apiVersion: v1
kind: Service
metadata:
  name: ${service_name}-service
spec:
  selector:
    app: ${service_name}
  ports:
  - port: 80
    targetPort: 8080
"#;
        
        let interpolator = YamlInterpolator::new();
        let result = interpolator.interpolate(input, &variables).unwrap();
        
        assert!(result.contains("name: my-service"));
        assert!(result.contains("version: 1.0.0"));
        assert!(result.contains("replicas: 3"));
        assert!(result.contains("image: registry.example.com/my-service:1.0.0"));
        assert!(result.contains("name: my-service-service"));
    }
}