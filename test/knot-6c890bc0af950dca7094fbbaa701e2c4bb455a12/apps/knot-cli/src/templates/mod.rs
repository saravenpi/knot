pub mod engine;
pub mod manifest;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

pub use engine::TemplateEngine;
pub use manifest::TemplateCategory;

pub struct TemplateManager;

impl TemplateManager {
    pub fn create_from_template(
        name: &str,
        version: &str,
        description: &str,
        template_name: &str,
        category: TemplateCategory,
        target_dir: &Path,
    ) -> Result<()> {
        let engine = TemplateEngine::new();
        
        let template = engine.get_template(template_name, category)
            .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))?;
        
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), name.to_string());
        variables.insert("version".to_string(), version.to_string());
        variables.insert("description".to_string(), description.to_string());
        
        let prepared_variables = engine.prepare_variables(&template, variables);
        
        engine.create_from_template(&template, target_dir, &prepared_variables)
            .with_context(|| format!("Failed to create {} from template '{}'", 
                match category {
                    TemplateCategory::Package => "package",
                    TemplateCategory::App => "app",
                }, template_name))?;
        
        Ok(())
    }
}