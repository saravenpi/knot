use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::manifest::{TemplateManifest, TemplateCategory};

pub struct TemplateEngine {
    templates_root: String,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            templates_root: concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates").to_string(),
        }
    }

    pub fn create_from_template(
        &self,
        template: &TemplateManifest,
        target_dir: &Path,
        variables: &HashMap<String, String>,
    ) -> Result<()> {
        // Validate required variables
        for var in &template.variables {
            if var.required && !variables.contains_key(&var.name) {
                return Err(anyhow::anyhow!(
                    "Required variable '{}' is missing", 
                    var.name
                ));
            }
        }

        // Create all template files
        for file in &template.files {
            let source_path = Path::new(&self.templates_root).join(&file.source_path);
            let target_path = target_dir.join(&file.target_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }
            
            // Read source file
            let content = fs::read_to_string(&source_path)
                .with_context(|| format!("Failed to read template file: {}", source_path.display()))?;
            
            // Process template variables if needed
            let processed_content = if file.is_template {
                self.process_template_variables(&content, variables)
            } else {
                content
            };
            
            // Write target file
            fs::write(&target_path, processed_content)
                .with_context(|| format!("Failed to create file: {}", target_path.display()))?;
        }
        
        Ok(())
    }

    fn process_template_variables(
        &self,
        content: &str,
        variables: &HashMap<String, String>,
    ) -> String {
        let mut result = content.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }

    pub fn get_template(&self, name: &str, category: TemplateCategory) -> Option<TemplateManifest> {
        match category {
            TemplateCategory::Package => {
                super::manifest::get_package_templates().get(name).cloned()
            }
            TemplateCategory::App => {
                super::manifest::get_app_templates().get(name).cloned()
            }
        }
    }

    pub fn prepare_variables(
        &self,
        template: &TemplateManifest,
        provided_vars: HashMap<String, String>,
    ) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        // Set default values first
        for var in &template.variables {
            if let Some(default) = &var.default_value {
                variables.insert(var.name.clone(), default.clone());
            }
        }
        
        // Override with provided values
        for (key, value) in provided_vars {
            variables.insert(key, value);
        }
        
        variables
    }
}