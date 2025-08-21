use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateManifest {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub files: Vec<TemplateFile>,
    pub variables: Vec<TemplateVariable>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TemplateCategory {
    Package,
    App,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub source_path: String,
    pub target_path: String,
    pub is_template: bool, // whether to process template variables
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

pub fn get_package_templates() -> HashMap<String, TemplateManifest> {
    let mut templates = HashMap::new();

    // TypeScript package template
    templates.insert("typescript".to_string(), TemplateManifest {
        name: "typescript".to_string(),
        description: "TypeScript package with build configuration".to_string(),
        category: TemplateCategory::Package,
        files: vec![
            TemplateFile {
                source_path: "packages/typescript/src/index.ts".to_string(),
                target_path: "src/index.ts".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/typescript/tsconfig.json".to_string(),
                target_path: "tsconfig.json".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/typescript/.gitignore".to_string(),
                target_path: ".gitignore".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/typescript/package.json".to_string(),
                target_path: "package.json".to_string(),
                is_template: true,
            },
        ],
        variables: vec![
            TemplateVariable {
                name: "name".to_string(),
                description: "Package name".to_string(),
                default_value: None,
                required: true,
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Package version".to_string(),
                default_value: Some("1.0.0".to_string()),
                required: true,
            },
            TemplateVariable {
                name: "description".to_string(),
                description: "Package description".to_string(),
                default_value: Some("A TypeScript package".to_string()),
                required: false,
            },
        ],
    });

    // React component library template
    templates.insert("react".to_string(), TemplateManifest {
        name: "react".to_string(),
        description: "React component library with TypeScript".to_string(),
        category: TemplateCategory::Package,
        files: vec![
            TemplateFile {
                source_path: "packages/react/src/index.ts".to_string(),
                target_path: "src/index.ts".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/react/src/Button.tsx".to_string(),
                target_path: "src/Button.tsx".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/react/tsconfig.json".to_string(),
                target_path: "tsconfig.json".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/react/.gitignore".to_string(),
                target_path: ".gitignore".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "packages/react/package.json".to_string(),
                target_path: "package.json".to_string(),
                is_template: true,
            },
        ],
        variables: vec![
            TemplateVariable {
                name: "name".to_string(),
                description: "Package name".to_string(),
                default_value: None,
                required: true,
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Package version".to_string(),
                default_value: Some("1.0.0".to_string()),
                required: true,
            },
            TemplateVariable {
                name: "description".to_string(),
                description: "Package description".to_string(),
                default_value: Some("A React component library".to_string()),
                required: false,
            },
        ],
    });

    templates
}

pub fn get_app_templates() -> HashMap<String, TemplateManifest> {
    let mut templates = HashMap::new();

    // React with Vite template
    templates.insert("react".to_string(), TemplateManifest {
        name: "react".to_string(),
        description: "React application with Vite".to_string(),
        category: TemplateCategory::App,
        files: vec![
            TemplateFile {
                source_path: "apps/react/src/main.tsx".to_string(),
                target_path: "src/main.tsx".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/src/App.tsx".to_string(),
                target_path: "src/App.tsx".to_string(),
                is_template: true,
            },
            TemplateFile {
                source_path: "apps/react/src/App.css".to_string(),
                target_path: "src/App.css".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/src/index.css".to_string(),
                target_path: "src/index.css".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/index.html".to_string(),
                target_path: "index.html".to_string(),
                is_template: true,
            },
            TemplateFile {
                source_path: "apps/react/vite.config.ts".to_string(),
                target_path: "vite.config.ts".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/tsconfig.json".to_string(),
                target_path: "tsconfig.json".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/tsconfig.node.json".to_string(),
                target_path: "tsconfig.node.json".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/.gitignore".to_string(),
                target_path: ".gitignore".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/react/package.json".to_string(),
                target_path: "package.json".to_string(),
                is_template: true,
            },
        ],
        variables: vec![
            TemplateVariable {
                name: "name".to_string(),
                description: "Application name".to_string(),
                default_value: None,
                required: true,
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Application version".to_string(),
                default_value: Some("0.1.0".to_string()),
                required: true,
            },
        ],
    });

    // SvelteKit template
    templates.insert("svelte".to_string(), TemplateManifest {
        name: "svelte".to_string(),
        description: "SvelteKit application".to_string(),
        category: TemplateCategory::App,
        files: vec![
            TemplateFile {
                source_path: "apps/svelte/src/app.html".to_string(),
                target_path: "src/app.html".to_string(),
                is_template: true,
            },
            TemplateFile {
                source_path: "apps/svelte/src/routes/+page.svelte".to_string(),
                target_path: "src/routes/+page.svelte".to_string(),
                is_template: true,
            },
            TemplateFile {
                source_path: "apps/svelte/src/routes/+layout.svelte".to_string(),
                target_path: "src/routes/+layout.svelte".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/svelte/svelte.config.js".to_string(),
                target_path: "svelte.config.js".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/svelte/vite.config.ts".to_string(),
                target_path: "vite.config.ts".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/svelte/tsconfig.json".to_string(),
                target_path: "tsconfig.json".to_string(),
                is_template: false,
            },
            TemplateFile {
                source_path: "apps/svelte/package.json".to_string(),
                target_path: "package.json".to_string(),
                is_template: true,
            },
        ],
        variables: vec![
            TemplateVariable {
                name: "name".to_string(),
                description: "Application name".to_string(),
                default_value: None,
                required: true,
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Application version".to_string(),
                default_value: Some("0.1.0".to_string()),
                required: true,
            },
        ],
    });

    templates
}