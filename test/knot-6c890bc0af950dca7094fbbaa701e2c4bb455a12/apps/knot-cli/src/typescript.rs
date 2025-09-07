use crate::project::Project;
use anyhow::{Context, Result};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct TypeScriptManager<'a> {
    project: &'a Project,
}

impl<'a> TypeScriptManager<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    pub fn setup_aliases_for_all_apps(&self) -> Result<()> {
        let app_names = self.project.get_app_names();

        for app_name in app_names {
            if let Some(alias) = self.project.get_app_ts_alias(&app_name) {
                self.setup_tsconfig_aliases(&app_name, &alias)?;
            }
        }
        Ok(())
    }

    pub fn setup_tsconfig_aliases(&self, app_name: &str, alias_prefix: &str) -> Result<()> {
        let app_dir = self.project.root.join("apps").join(app_name);
        let tsconfig_path = app_dir.join("tsconfig.json");

        // Get the packages for this app
        let package_names = self.project.get_app_dependencies(app_name);
        
        // Generate package-specific aliases
        let package_aliases = self.generate_package_aliases(&package_names, alias_prefix)?;

        if tsconfig_path.exists() {
            self.update_existing_tsconfig_with_aliases(&tsconfig_path, &package_aliases)?
        } else {
            self.create_default_tsconfig_with_aliases(&tsconfig_path, &package_aliases)?
        }

        Ok(())
    }

    /// Generate package-specific aliases with validation
    fn generate_package_aliases(&self, package_names: &[String], alias_prefix: &str) -> Result<HashMap<String, String>> {
        let mut aliases = HashMap::new();
        
        for package_name in package_names {
            // Skip online packages (those starting with @)
            if package_name.starts_with('@') {
                continue;
            }

            let alias_name = format!("{}{}", alias_prefix, package_name);
            
            // Validate that the alias is a valid TypeScript identifier
            self.validate_typescript_identifier(&alias_name, package_name)?;
            
            let package_path = format!("./knot_packages/{}/*", package_name);
            aliases.insert(alias_name, package_path);
        }
        
        Ok(aliases)
    }

    /// Validate that an alias is a valid TypeScript identifier and not a reserved word
    fn validate_typescript_identifier(&self, alias: &str, package_name: &str) -> Result<()> {
        // Check if it's a valid identifier format
        if !self.is_valid_identifier(alias) {
            anyhow::bail!(
                "Invalid TypeScript alias '{}' for package '{}'. Aliases must be valid JavaScript identifiers.",
                alias, package_name
            );
        }

        // Check if it's a TypeScript reserved word
        if self.is_typescript_reserved_word(alias) {
            anyhow::bail!(
                "TypeScript alias '{}' for package '{}' conflicts with a reserved word. Please use a different alias prefix.",
                alias, package_name
            );
        }

        Ok(())
    }

    /// Check if a string is a valid JavaScript/TypeScript identifier
    fn is_valid_identifier(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let mut chars = name.chars();
        let first_char = chars.next().unwrap();
        
        // First character must be a letter, underscore, or dollar sign
        if !first_char.is_ascii_alphabetic() && first_char != '_' && first_char != '$' {
            return false;
        }

        // Remaining characters must be letters, digits, underscores, or dollar signs
        for ch in chars {
            if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '$' {
                return false;
            }
        }

        true
    }

    /// Check if a name is a TypeScript reserved word
    fn is_typescript_reserved_word(&self, name: &str) -> bool {
        const TYPESCRIPT_RESERVED_WORDS: &[&str] = &[
            // JavaScript reserved words
            "break", "case", "catch", "class", "const", "continue", "debugger",
            "default", "delete", "do", "else", "enum", "export", "extends",
            "false", "finally", "for", "function", "if", "import", "in",
            "instanceof", "new", "null", "return", "super", "switch",
            "this", "throw", "true", "try", "typeof", "var", "void",
            "while", "with", "yield", "let", "static", "implements",
            "interface", "package", "private", "protected", "public",
            // TypeScript specific keywords
            "abstract", "any", "boolean", "constructor", "declare",
            "get", "module", "require", "number", "set", "string",
            "symbol", "type", "from", "of", "as", "async", "await",
            "namespace", "readonly", "keyof", "unique", "infer",
            "is", "asserts", "never", "object", "unknown", "bigint",
        ];

        TYPESCRIPT_RESERVED_WORDS.contains(&name)
    }

    fn update_existing_tsconfig_with_aliases(&self, tsconfig_path: &Path, package_aliases: &HashMap<String, String>) -> Result<()> {
        let content = fs::read_to_string(tsconfig_path)
            .with_context(|| format!("Failed to read {:?}", tsconfig_path))?;

        // Try to parse JSON directly first, only remove comments if parsing fails
        let mut tsconfig: Value = match serde_json::from_str(&content) {
            Ok(config) => config,
            Err(_) => {
                // If direct parsing fails, try removing comments and parsing again
                let clean_content = self.remove_json_comments(&content)?;
                serde_json::from_str(&clean_content).with_context(|| {
                    format!(
                        "Failed to parse JSON in {:?}. Content may be malformed.",
                        tsconfig_path
                    )
                })?
            }
        };

        // Ensure we have a proper object structure
        if !tsconfig.is_object() {
            tsconfig = json!({});
        }

        let tsconfig_obj = tsconfig
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("Failed to get mutable object from tsconfig"))?;

        // Get or create compilerOptions
        let compiler_options = tsconfig_obj
            .entry("compilerOptions")
            .or_insert_with(|| json!({}));

        let compiler_obj = compiler_options
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("compilerOptions is not an object"))?;

        // Get or create paths
        let paths = compiler_obj.entry("paths").or_insert_with(|| json!({}));

        let paths_obj = paths
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("paths is not an object"))?;

        // Clear existing knot_packages paths and add package-specific aliases
        // Remove any existing knot_packages related paths with similar patterns
        paths_obj.retain(|key, _| {
            !key.contains("knot_packages") && !key.starts_with("#")
        });

        // Add package-specific aliases
        for (alias_name, package_path) in package_aliases {
            let alias_pattern = format!("{}/*", alias_name);
            paths_obj.insert(alias_pattern, json!([package_path]));
        }

        // Handle the include array - add knot_packages/**/* if not present
        let include = tsconfig_obj.entry("include").or_insert_with(|| json!(["src/**/*"]));
        
        match include {
            Value::Array(arr) => {
                // Always ensure src/**/* is present (most projects need this)
                if !self.has_include_pattern(arr, "src") {
                    // Insert at the beginning to maintain conventional order
                    arr.insert(0, Value::String("src/**/*".to_string()));
                }
                
                // Check if knot_packages is already included in any form
                if !self.has_include_pattern(arr, "knot_packages") {
                    arr.push(Value::String("knot_packages/**/*".to_string()));
                }
            }
            Value::String(single_path) => {
                // Handle case where include is a single string instead of array
                let existing_path = single_path.clone();
                *include = json!([existing_path, "knot_packages/**/*"]);
                
                // Add src if not already the existing path
                if !self.is_pattern_match(&existing_path, "src") {
                    if let Value::Array(arr) = include {
                        arr.insert(0, Value::String("src/**/*".to_string()));
                    }
                }
            }
            _ => {
                // If it's null, undefined, or any other type, create proper array
                *include = json!(["src/**/*", "knot_packages/**/*"]);
            }
        }

        // Validate the final JSON before writing
        let updated_content = match serde_json::to_string_pretty(&tsconfig) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to serialize updated tsconfig: {}", e);
                eprintln!("💡 Creating a fresh tsconfig with minimal configuration...");
                
                // Fallback: create a minimal but correct tsconfig
                let mut paths_obj = serde_json::Map::new();
                for (alias_name, package_path) in package_aliases {
                    let alias_pattern = format!("{}/*", alias_name);
                    paths_obj.insert(alias_pattern, json!([package_path]));
                }
                
                let fallback_tsconfig = json!({
                    "compilerOptions": {
                        "target": "es2020",
                        "module": "esnext",
                        "moduleResolution": "node",
                        "esModuleInterop": true,
                        "allowSyntheticDefaultImports": true,
                        "strict": true,
                        "skipLibCheck": true,
                        "forceConsistentCasingInFileNames": true,
                        "paths": paths_obj
                    },
                    "include": ["src/**/*", "knot_packages/**/*"],
                    "exclude": ["node_modules", "dist"]
                });
                
                serde_json::to_string_pretty(&fallback_tsconfig)
                    .context("Failed to create fallback tsconfig")?
            }
        };

        fs::write(tsconfig_path, updated_content)
            .with_context(|| format!("Failed to write updated tsconfig to {:?}", tsconfig_path))?;

        let alias_count = package_aliases.len();
        if alias_count > 0 {
            println!("✅ Updated tsconfig.json with {} package aliases and knot_packages include", alias_count);
        } else {
            println!("✅ Updated tsconfig.json (no local packages found to alias)");
        }
        Ok(())
    }

    fn has_include_pattern(&self, include_array: &[Value], pattern: &str) -> bool {
        include_array.iter().any(|item| {
            if let Some(path_str) = item.as_str() {
                self.is_pattern_match(path_str, pattern)
            } else {
                false
            }
        })
    }

    fn is_pattern_match(&self, path_str: &str, pattern: &str) -> bool {
        // Normalize paths: remove leading ./, convert backslashes, remove trailing slashes, handle multiple slashes
        let normalized = path_str
            .replace('\\', "/")
            .trim_start_matches("./")
            .trim_end_matches('/')
            .replace("//", "/")  // Handle multiple consecutive slashes
            .to_string();
        
        let pattern_base = pattern
            .replace('\\', "/")
            .trim_start_matches("./")
            .trim_end_matches('/')
            .replace("//", "/")
            .to_string();

        // Check for exact matches or pattern matches
        match pattern_base.as_str() {
            "knot_packages" => {
                // Check for various knot_packages patterns
                let knot_patterns = [
                    "knot_packages",
                    "knot_packages/*",
                    "knot_packages/**",
                    "knot_packages/**/*",
                    "knot_packages/**/*.ts",
                    "knot_packages/**/*.tsx",
                    "knot_packages/**/*.js",
                    "knot_packages/**/*.jsx",
                ];
                knot_patterns.iter().any(|p| normalized == *p) || 
                (normalized.starts_with("knot_packages/") && 
                 !normalized.starts_with("knot_packages_") && 
                 normalized != "knot_packages")
            }
            "src" => {
                // Check for various src patterns
                let src_patterns = [
                    "src",
                    "src/*",
                    "src/**",
                    "src/**/*",
                    "src/**/*.ts",
                    "src/**/*.tsx",
                    "src/**/*.js",
                    "src/**/*.jsx",
                ];
                src_patterns.iter().any(|p| normalized == *p) || 
                (normalized.starts_with("src/") && 
                 !normalized.starts_with("src_") && 
                 normalized != "src")
            }
            _ => {
                // For other patterns, do exact match or directory prefix match
                normalized == pattern_base || 
                (normalized.starts_with(&format!("{}/", pattern_base)) && 
                 !normalized.starts_with(&format!("{}_", pattern_base)))
            }
        }
    }

    fn remove_json_comments(&self, content: &str) -> Result<String> {
        // For tsconfig.json parsing, we need to be very careful with comment removal
        // Let's use a more robust approach that properly handles JSON with potential comments
        
        let mut clean = String::new();
        let mut in_string = false;
        let mut escaped = false;
        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            
            match ch {
                '"' if !escaped => {
                    in_string = !in_string;
                    clean.push(ch);
                }
                '\\' if in_string => {
                    escaped = !escaped;
                    clean.push(ch);
                }
                '/' if !in_string && !escaped => {
                    // Check for single-line comment
                    if i + 1 < chars.len() && chars[i + 1] == '/' {
                        // Skip to end of line
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                        if i < chars.len() {
                            clean.push('\n'); // Keep the newline
                        }
                    }
                    // Check for multi-line comment
                    else if i + 1 < chars.len() && chars[i + 1] == '*' {
                        i += 2; // Skip /*
                        // Skip until */
                        while i + 1 < chars.len() {
                            if chars[i] == '*' && chars[i + 1] == '/' {
                                i += 2; // Skip */
                                break;
                            }
                            i += 1;
                        }
                        continue; // Don't increment i again
                    } else {
                        clean.push(ch);
                    }
                }
                _ => {
                    if ch != '\\' {
                        escaped = false;
                    }
                    clean.push(ch);
                }
            }
            i += 1;
        }

        // Remove trailing commas before closing braces/brackets
        let trailing_comma = Regex::new(r",(\s*[}\]])").unwrap();
        clean = trailing_comma.replace_all(&clean, "$1").to_string();

        Ok(clean)
    }


    fn create_default_tsconfig_with_aliases(&self, tsconfig_path: &Path, package_aliases: &HashMap<String, String>) -> Result<()> {
        let mut paths_obj = serde_json::Map::new();
        for (alias_name, package_path) in package_aliases {
            let alias_pattern = format!("{}/*", alias_name);
            paths_obj.insert(alias_pattern, json!([package_path]));
        }

        let tsconfig = json!({
            "compilerOptions": {
                "target": "es2020",
                "lib": ["es2020"],
                "module": "esnext",
                "moduleResolution": "node",
                "esModuleInterop": true,
                "allowSyntheticDefaultImports": true,
                "strict": true,
                "skipLibCheck": true,
                "forceConsistentCasingInFileNames": true,
                "paths": paths_obj
            },
            "include": ["src/**/*", "knot_packages/**/*"],
            "exclude": ["node_modules", "dist"]
        });

        let content = serde_json::to_string_pretty(&tsconfig)?;
        fs::write(tsconfig_path, content)
            .with_context(|| format!("Failed to create default tsconfig at {:?}", tsconfig_path))?;

        let alias_count = package_aliases.len();
        if alias_count > 0 {
            println!("✅ Created tsconfig.json with {} package aliases and knot_packages include", alias_count);
        } else {
            println!("✅ Created tsconfig.json (no local packages found to alias)");
        }
        Ok(())
    }
}
