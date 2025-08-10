use regex::Regex;
use std::sync::OnceLock;

// Package name validation: @package or @team/package
fn package_name_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^@[a-z0-9][a-z0-9\-]*(/[a-z0-9][a-z0-9\-]*)?$").unwrap())
}

// Version validation: semantic versioning
fn version_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^[0-9]+\.[0-9]+\.[0-9]+.*$").unwrap())
}

// Team name validation: lowercase alphanumeric with hyphens
fn team_name_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^[a-z0-9][a-z0-9\-]*$").unwrap())
}

// Username validation: alphanumeric with hyphens and underscores
fn username_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-_]*$").unwrap())
}

pub fn validate_package_name(name: &str) -> bool {
    package_name_regex().is_match(name)
}

pub fn validate_version(version: &str) -> bool {
    version_regex().is_match(version)
}

pub fn validate_team_name(name: &str) -> bool {
    team_name_regex().is_match(name)
}

pub fn validate_username(username: &str) -> bool {
    username_regex().is_match(username)
}

pub fn extract_team_from_package_name(package_name: &str) -> Option<String> {
    if let Some(_captures) = package_name_regex().captures(package_name) {
        if package_name.contains('/') {
            // Format: @team/package
            let parts: Vec<&str> = package_name.splitn(2, '/').collect();
            if parts.len() == 2 {
                return Some(parts[0].trim_start_matches('@').to_string());
            }
        }
    }
    None
}

pub fn validate_package_team_consistency(package_name: &str, team_name: Option<&str>) -> bool {
    let extracted_team = extract_team_from_package_name(package_name);

    match (extracted_team, team_name) {
        (Some(extracted), Some(provided)) => extracted == provided,
        (None, None) => !package_name.contains('/'), // Public package without team
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_name_validation() {
        assert!(validate_package_name("@jwt"));
        assert!(validate_package_name("@myteam/logger"));
        assert!(validate_package_name("@axios-client"));
        assert!(!validate_package_name("jwt")); // Missing @
        assert!(!validate_package_name("@")); // Just @
        assert!(!validate_package_name("@-invalid")); // Starts with -
        assert!(!validate_package_name("@team/")); // Empty package name
    }

    #[test]
    fn test_version_validation() {
        assert!(validate_version("1.0.0"));
        assert!(validate_version("1.2.3-beta"));
        assert!(validate_version("0.1.0"));
        assert!(!validate_version("1.0")); // Missing patch
        assert!(!validate_version("v1.0.0")); // Starts with v
    }

    #[test]
    fn test_team_extraction() {
        assert_eq!(
            extract_team_from_package_name("@myteam/logger"),
            Some("myteam".to_string())
        );
        assert_eq!(extract_team_from_package_name("@jwt"), None);
        assert_eq!(extract_team_from_package_name("invalid"), None);
    }

    #[test]
    fn test_package_team_consistency() {
        assert!(validate_package_team_consistency("@jwt", None));
        assert!(validate_package_team_consistency(
            "@myteam/logger",
            Some("myteam")
        ));
        assert!(!validate_package_team_consistency("@jwt", Some("myteam")));
        assert!(!validate_package_team_consistency("@myteam/logger", None));
        assert!(!validate_package_team_consistency(
            "@myteam/logger",
            Some("otherteam")
        ));
    }
}
