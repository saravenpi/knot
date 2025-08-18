use anyhow::Result;
use std::fs;
use std::path::Path;

/// Represents ignore patterns loaded from .knotignore file
#[derive(Debug, Clone)]
pub struct KnotIgnore {
    patterns: Vec<String>,
}

impl KnotIgnore {
    /// Create a new KnotIgnore from a .knotignore file path
    pub fn from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)?;
        let patterns = Self::parse_ignore_content(&content);
        
        Ok(Self { patterns })
    }

    /// Create a default KnotIgnore with common ignore patterns
    pub fn default() -> Self {
        let default_patterns = vec![
            "*.tar.gz".to_string(),
            "*.tgz".to_string(),
            ".git".to_string(),
            "node_modules".to_string(),
            "target".to_string(),
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            "*.tmp".to_string(),
            "*.temp".to_string(),
            ".env".to_string(),
            ".env.local".to_string(),
        ];

        Self { patterns: default_patterns }
    }

    /// Parse the content of a .knotignore file
    fn parse_ignore_content(content: &str) -> Vec<String> {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.to_string())
            .collect()
    }

    /// Check if a file or directory should be ignored
    pub fn is_ignored(&self, path: &str) -> bool {
        let file_name = Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(path);

        self.patterns.iter().any(|pattern| {
            self.matches_pattern(pattern, file_name) || self.matches_pattern(pattern, path)
        })
    }

    /// Check if a pattern matches a given path
    fn matches_pattern(&self, pattern: &str, path: &str) -> bool {
        if pattern.contains('*') {
            self.glob_match(pattern, path)
        } else {
            // Exact match for patterns without wildcards
            pattern == path
        }
    }

    /// Simple glob matching implementation
    fn glob_match(&self, pattern: &str, text: &str) -> bool {
        if pattern == "*" {
            // Match everything
            return true;
        }
        
        if pattern.starts_with("*.") {
            // Extension matching (e.g., "*.txt")
            let ext = &pattern[2..];
            return text.ends_with(ext);
        } 
        
        if pattern.ends_with("*") && !pattern.starts_with("*") {
            // Prefix matching (e.g., "temp*")
            let prefix = &pattern[..pattern.len() - 1];
            return text.starts_with(prefix);
        }
        
        if pattern.starts_with("*") && !pattern.ends_with("*") {
            // Suffix matching (e.g., "*temp")
            let suffix = &pattern[1..];
            return text.ends_with(suffix);
        }
        
        if pattern.starts_with("*") && pattern.ends_with("*") {
            // Contains matching (e.g., "*temp*")
            if pattern.len() <= 2 {
                return true; // Just "*" or "**"
            }
            let middle = &pattern[1..pattern.len()-1];
            return text.contains(middle);
        }
        
        // Pattern with * in the middle (e.g., "file*.txt")
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return text.starts_with(parts[0]) && text.ends_with(parts[1]);
            } else {
                // Multiple wildcards - more complex matching
                // For now, just check if all non-wildcard parts are present
                return parts.iter()
                    .filter(|part| !part.is_empty())
                    .all(|part| text.contains(part));
            }
        }
        
        false
    }

    /// Get all patterns for debugging
    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }

    /// Add additional patterns
    #[cfg(test)]
    pub fn add_patterns(&mut self, additional_patterns: &[&str]) {
        for pattern in additional_patterns {
            self.patterns.push(pattern.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_default_ignore() {
        let ignore = KnotIgnore::default();
        
        assert!(ignore.is_ignored("test.tar.gz"));
        assert!(ignore.is_ignored("node_modules"));
        assert!(ignore.is_ignored(".git"));
        assert!(ignore.is_ignored(".DS_Store"));
        
        assert!(!ignore.is_ignored("src"));
        assert!(!ignore.is_ignored("README.md"));
    }

    #[test]
    fn test_glob_patterns() {
        let mut ignore = KnotIgnore::default();
        ignore.add_patterns(&["test_*", "*.log", "*temp*"]);
        
        assert!(ignore.is_ignored("test_file.txt"));
        assert!(ignore.is_ignored("debug.log"));
        assert!(ignore.is_ignored("temporary"));
        assert!(ignore.is_ignored("temp_data"));
        
        assert!(!ignore.is_ignored("testing.txt"));
        assert!(!ignore.is_ignored("file.txt"));
    }

    #[test]
    fn test_knotignore_file() -> Result<()> {
        let dir = tempdir()?;
        let ignore_file = dir.path().join(".knotignore");
        
        let mut file = File::create(&ignore_file)?;
        writeln!(file, "# This is a comment")?;
        writeln!(file, "build/")?;
        writeln!(file, "*.log")?;
        writeln!(file, "")?; // Empty line
        writeln!(file, "temp_*")?;
        
        let ignore = KnotIgnore::from_file(&ignore_file)?;
        
        assert!(ignore.is_ignored("build/"));
        assert!(ignore.is_ignored("error.log"));
        assert!(ignore.is_ignored("temp_file"));
        assert!(!ignore.is_ignored("src/"));
        
        Ok(())
    }

    #[test]
    fn test_missing_knotignore_file() -> Result<()> {
        let dir = tempdir()?;
        let missing_file = dir.path().join(".knotignore");
        
        let ignore = KnotIgnore::from_file(&missing_file)?;
        
        // Should return default patterns when file doesn't exist
        assert!(ignore.is_ignored("node_modules"));
        assert!(ignore.is_ignored(".git"));
        
        Ok(())
    }
}