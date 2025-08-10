use crate::project::Project;
use anyhow::{Context, Result};
use regex::Regex;
use serde_json::{json, Value};
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
                self.setup_tsconfig_alias(&app_name, &alias)?;
            }
        }
        Ok(())
    }

    pub fn setup_tsconfig_alias(&self, app_name: &str, alias: &str) -> Result<()> {
        let app_dir = self.project.root.join("apps").join(app_name);
        let tsconfig_path = app_dir.join("tsconfig.json");

        if tsconfig_path.exists() {
            self.update_existing_tsconfig(&tsconfig_path, alias)?;
        } else {
            self.create_default_tsconfig(&tsconfig_path, alias)?;
        }

        Ok(())
    }

    fn update_existing_tsconfig(&self, tsconfig_path: &Path, alias: &str) -> Result<()> {
        let content = fs::read_to_string(tsconfig_path)
            .with_context(|| format!("Failed to read {:?}", tsconfig_path))?;

        // Remove JSON comments to handle tsconfig.json files with comments
        let clean_content = self.remove_json_comments(&content)?;

        let mut tsconfig: Value = serde_json::from_str(&clean_content).with_context(|| {
            format!(
                "Failed to parse JSON in {:?}. Content may be malformed.",
                tsconfig_path
            )
        })?;

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

        // Add or append to the path alias
        let knot_packages_path = format!("{}/*", alias);
        let new_path = "./knot_packages/*";

        // Check if this path alias already exists
        if let Some(existing_paths) = paths_obj.get_mut(&knot_packages_path) {
            // If it exists, ensure it's an array and append if not already present
            match existing_paths {
                Value::Array(arr) => {
                    let new_path_value = Value::String(new_path.to_string());
                    if !arr.contains(&new_path_value) {
                        arr.push(new_path_value);
                    }
                }
                _ => {
                    // If it's not an array, convert it to one with the new path
                    *existing_paths = json!([new_path]);
                }
            }
        } else {
            // Path doesn't exist, create it
            paths_obj.insert(knot_packages_path, json!([new_path]));
        }

        let updated_content = serde_json::to_string_pretty(&tsconfig)?;
        fs::write(tsconfig_path, updated_content)
            .with_context(|| format!("Failed to write updated tsconfig to {:?}", tsconfig_path))?;

        println!("Updated tsconfig.json for app with alias '{}'", alias);
        Ok(())
    }

    fn remove_json_comments(&self, content: &str) -> Result<String> {
        // Remove single-line comments (// comment)
        let single_line_comment = Regex::new(r"//.*$").unwrap();
        let mut clean = String::new();

        for line in content.lines() {
            // Check if this line is inside a string literal to avoid removing // inside strings
            let cleaned_line = if self.line_has_unquoted_comment(line) {
                single_line_comment.replace(line, "").to_string()
            } else {
                line.to_string()
            };
            clean.push_str(&cleaned_line);
            clean.push('\n');
        }

        // Remove multi-line comments (/* comment */)
        let multi_line_comment = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
        clean = multi_line_comment.replace_all(&clean, "").to_string();

        // Remove trailing commas before closing braces/brackets
        let trailing_comma = Regex::new(r",(\s*[}\]])").unwrap();
        clean = trailing_comma.replace_all(&clean, "$1").to_string();

        Ok(clean)
    }

    fn line_has_unquoted_comment(&self, line: &str) -> bool {
        let mut in_string = false;
        let mut escaped = false;
        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            match chars[i] {
                '"' if !escaped => in_string = !in_string,
                '\\' if !escaped => escaped = true,
                '/' if !in_string && !escaped && i + 1 < chars.len() && chars[i + 1] == '/' => {
                    return true
                }
                _ => escaped = false,
            }
        }
        false
    }

    fn create_default_tsconfig(&self, tsconfig_path: &Path, alias: &str) -> Result<()> {
        let knot_packages_path = format!("{}/*", alias);

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
                "paths": {
                    knot_packages_path: ["./knot_packages/*"]
                }
            },
            "include": ["src/**/*"],
            "exclude": ["node_modules", "dist"]
        });

        let content = serde_json::to_string_pretty(&tsconfig)?;
        fs::write(tsconfig_path, content)
            .with_context(|| format!("Failed to create default tsconfig at {:?}", tsconfig_path))?;

        println!("Created default tsconfig.json with alias '{}'", alias);
        Ok(())
    }
}
