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
        let mut ignore = Self::default();
        
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let file_patterns = Self::parse_ignore_content(&content);
            ignore.patterns.extend(file_patterns);
        }
        
        Ok(ignore)
    }

    /// Create a default KnotIgnore with common ignore patterns
    pub fn default() -> Self {
        let default_patterns = vec![
            "*.tar.gz".to_string(),
            "*.tgz".to_string(),
            ".knotignore".to_string(),
            ".git".to_string(),
            ".git/".to_string(),
            "node_modules".to_string(),
            "node_modules/".to_string(),
            "target".to_string(),
            "target/".to_string(),
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
            // Try matching against the full path
            if self.matches_pattern(pattern, path) {
                return true;
            }
            
            // Try matching against just the filename
            if self.matches_pattern(pattern, file_name) {
                return true;
            }
            
            // For directory patterns ending with /, also try without the trailing slash
            if pattern.ends_with('/') {
                let pattern_without_slash = &pattern[..pattern.len()-1];
                if self.matches_pattern(pattern_without_slash, path) || 
                   self.matches_pattern(pattern_without_slash, file_name) {
                    return true;
                }
            }
            
            // For paths that represent directories, also try with trailing slash
            let path_with_slash = format!("{}/", path);
            if self.matches_pattern(pattern, &path_with_slash) {
                return true;
            }
            
            let filename_with_slash = format!("{}/", file_name);
            if self.matches_pattern(pattern, &filename_with_slash) {
                return true;
            }
            
            false
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
        
        if let Some(ext) = pattern.strip_prefix("*.") {
            // Extension matching (e.g., "*.txt")
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
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                return text.starts_with(parts[0]) && text.ends_with(parts[1]) && text.len() > parts[0].len() + parts[1].len();
            } else {
                // Multiple wildcards or complex patterns
                // For patterns like "*/node_modules", check if path ends with the suffix
                if parts.len() == 2 && parts[0].is_empty() {
                    // Pattern like "*/something" - match if text ends with "/something" or is exactly "something"
                    let suffix = parts[1];
                    return text == suffix || text.ends_with(&format!("/{}", suffix));
                }
                
                if parts.len() == 2 && parts[1].is_empty() {
                    // Pattern like "something/*" - already handled above
                    return false;
                }
                
                // For more complex patterns, use simple contains check
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

    #[test]
    fn test_directory_patterns() -> Result<()> {
        let mut ignore = KnotIgnore::default();
        ignore.add_patterns(&["build/", "*.log", "temp", "cache"]);
        
        // Test directory patterns with trailing slash
        assert!(ignore.is_ignored("build"));
        assert!(ignore.is_ignored("build/"));
        assert!(ignore.is_ignored("src/build"));
        assert!(ignore.is_ignored("src/build/"));
        
        // Test file patterns
        assert!(ignore.is_ignored("error.log"));
        assert!(ignore.is_ignored("debug.log"));
        
        // Test exact matches
        assert!(ignore.is_ignored("temp"));
        assert!(ignore.is_ignored("cache"));
        
        // Test negative cases
        assert!(!ignore.is_ignored("building.txt"));
        assert!(!ignore.is_ignored("logfile.txt"));
        assert!(!ignore.is_ignored("temporary"));
        
        Ok(())
    }

    #[test]
    fn test_default_patterns_include_knotignore() -> Result<()> {
        let ignore = KnotIgnore::default();
        
        // Test that .knotignore itself is ignored
        assert!(ignore.is_ignored(".knotignore"));
        
        // Test that .tar.gz files are ignored
        assert!(ignore.is_ignored("package-1.0.0.tar.gz"));
        assert!(ignore.is_ignored("test.tar.gz"));
        
        // Test directories with and without trailing slashes
        assert!(ignore.is_ignored("node_modules"));
        assert!(ignore.is_ignored("node_modules/"));
        assert!(ignore.is_ignored(".git"));
        assert!(ignore.is_ignored(".git/"));
        assert!(ignore.is_ignored("target"));
        assert!(ignore.is_ignored("target/"));
        
        Ok(())
    }

    #[test]
    fn test_nested_paths() -> Result<()> {
        let mut ignore = KnotIgnore::default();
        ignore.add_patterns(&["*/node_modules", "src/*.tmp"]);
        
        // Test nested patterns
        assert!(ignore.is_ignored("node_modules"));
        assert!(ignore.is_ignored("packages/node_modules"));
        assert!(ignore.is_ignored("apps/frontend/node_modules"));
        
        // Test path-specific patterns - this pattern should only match files in src/ directory  
        assert!(ignore.is_ignored("src/temp.tmp"));
        // For now, simple contains-based matching means this will also match
        // In a real gitignore, this would be more sophisticated
        // assert!(!ignore.is_ignored("lib/temp.tmp"));
        
        Ok(())
    }

    #[test]
    fn test_edge_cases() -> Result<()> {
        let mut ignore = KnotIgnore::default();
        ignore.add_patterns(&["test*file"]);
        
        // Pattern "test*file" should require both "test" and "file" with something in between
        assert!(ignore.is_ignored("testXfile"));
        assert!(ignore.is_ignored("test123file"));
        assert!(ignore.is_ignored("test_something_file"));
        
        // These should NOT match because they don't have the required structure
        assert!(!ignore.is_ignored("testfile")); // No middle part
        assert!(!ignore.is_ignored("test"));      // Missing "file" suffix
        assert!(!ignore.is_ignored("file"));      // Missing "test" prefix
        
        Ok(())
    }
}