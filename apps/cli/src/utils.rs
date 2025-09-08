use std::path::{Path, PathBuf};

/// Find a YAML config file with either .yml or .yaml extension
/// Returns the path to the first file found, prioritizing .yml
pub fn find_yaml_file(base_path: &Path, filename: &str) -> Option<PathBuf> {
    // First try .yml
    let yml_path = base_path.join(format!("{}.yml", filename));
    if yml_path.exists() {
        return Some(yml_path);
    }

    // Then try .yaml
    let yaml_path = base_path.join(format!("{}.yaml", filename));
    if yaml_path.exists() {
        return Some(yaml_path);
    }

    None
}

/// Get the expected config file path, preferring .yml if neither exists
#[allow(dead_code)]
pub fn get_yaml_config_path(base_path: &Path, filename: &str) -> PathBuf {
    find_yaml_file(base_path, filename)
        .unwrap_or_else(|| base_path.join(format!("{}.yml", filename)))
}

/// Check if a YAML config file exists with either extension
pub fn yaml_config_exists(base_path: &Path, filename: &str) -> bool {
    find_yaml_file(base_path, filename).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_yaml_file_yml_priority() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create both .yml and .yaml files
        fs::write(base_path.join("test.yml"), "yml content").unwrap();
        fs::write(base_path.join("test.yaml"), "yaml content").unwrap();

        let result = find_yaml_file(base_path, "test");
        assert!(result.is_some());
        assert_eq!(result.unwrap().extension().unwrap(), "yml");
    }

    #[test]
    fn test_find_yaml_file_yaml_fallback() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create only .yaml file
        fs::write(base_path.join("test.yaml"), "yaml content").unwrap();

        let result = find_yaml_file(base_path, "test");
        assert!(result.is_some());
        assert_eq!(result.unwrap().extension().unwrap(), "yaml");
    }

    #[test]
    fn test_find_yaml_file_none_found() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let result = find_yaml_file(base_path, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_yaml_config_exists() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        assert!(!yaml_config_exists(base_path, "test"));

        fs::write(base_path.join("test.yml"), "content").unwrap();
        assert!(yaml_config_exists(base_path, "test"));
    }
}