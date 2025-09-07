use crate::config::KnotConfig;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_alias_configuration() {
        let yaml_content = r#"
name: Test Project
description: Test project with aliases
apps:
  web-app:
    packages:
      - name: long-package-name
        alias: utils
      - name: "@team/complex-package@1.0.0"
        alias: api
      - simple-package
"#;
        
        let config: KnotConfig = serde_yaml::from_str(yaml_content).unwrap();
        
        // Test that apps are parsed correctly
        assert_eq!(config.name, "Test Project");
        assert!(config.apps.is_some());
        
        let apps = config.apps.as_ref().unwrap();
        assert!(apps.contains_key("web-app"));
        
        let web_app = &apps["web-app"];
        let entries = web_app.get_package_entries();
        
        // Check that we have 3 entries
        assert_eq!(entries.len(), 3);
        
        // Check alias entries
        let utils_entry = entries.iter().find(|e| e.name == "long-package-name").unwrap();
        assert_eq!(utils_entry.alias, Some("utils".to_string()));
        
        let api_entry = entries.iter().find(|e| e.name == "@team/complex-package@1.0.0").unwrap();
        assert_eq!(api_entry.alias, Some("api".to_string()));
        
        let simple_entry = entries.iter().find(|e| e.name == "simple-package").unwrap();
        assert_eq!(simple_entry.alias, None);
    }

    #[test]
    fn test_alias_validation() {
        let yaml_content = r#"
name: Test Project
apps:
  web-app:
    packages:
      - name: package1
        alias: valid_alias
      - name: package2
        alias: invalid-alias-with-dash!
"#;
        
        let config: KnotConfig = serde_yaml::from_str(yaml_content).unwrap();
        
        // This should fail validation due to invalid alias
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_duplicate_alias_detection() {
        let yaml_content = r#"
name: Test Project
apps:
  web-app:
    packages:
      - name: package1
        alias: utils
      - name: package2
        alias: utils
"#;
        
        let config: KnotConfig = serde_yaml::from_str(yaml_content).unwrap();
        
        // This should fail validation due to duplicate alias
        assert!(config.validate().is_err());
    }
}