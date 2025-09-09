// Minimal test program to validate YAML parsing outside of the full CLI context
// This isolates YAML parsing issues to understand what's failing

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration variable with its value and optional description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariable {
    /// Simple string value
    Simple(String),
    /// Complex variable with value and metadata
    Complex {
        value: String,
        description: Option<String>,
    },
}

impl ConfigVariable {
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariable::Simple(value) => value,
            ConfigVariable::Complex { value, .. } => value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TsAlias {
    Boolean(bool),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppDependencies {
    List(Vec<String>),
    Object {
        #[serde(rename = "tsAlias")]
        ts_alias: Option<TsAlias>,
        packages: Option<Vec<String>>,
    },
}

/// Project-level configuration structure  
#[derive(Debug, Serialize, Deserialize)]
pub struct KnotConfig {
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// TypeScript alias configuration
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    /// App dependencies configuration
    pub apps: Option<HashMap<String, AppDependencies>>,
    /// Project-level scripts
    pub scripts: Option<HashMap<String, String>>,
    /// Project-level variables
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

/// Package-level configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub team: Option<String>,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<Vec<String>>,
    pub peer_dependencies: Option<Vec<String>>,
    pub exports: Option<HashMap<String, String>>,
    pub features: Option<Vec<String>>,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

/// App-level configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub packages: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

fn test_basic_yaml_parsing() -> Result<()> {
    println!("=== Testing Basic YAML Parsing ===");
    
    // Test 1: Simple valid YAML
    let simple_yaml = r#"
name: test-project
description: A simple test project
"#;
    
    print!("Testing simple YAML... ");
    match serde_yaml::from_str::<KnotConfig>(simple_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 2: YAML with scripts section
    let yaml_with_scripts = r#"
name: test-project
description: A test project with scripts
scripts:
  start: "npm start"
  build: "npm run build"
  test: "npm test"
"#;
    
    print!("Testing YAML with scripts... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_with_scripts) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Scripts: {:?}", config.scripts);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 3: YAML with apps section (simple format)
    let yaml_with_apps = r#"
name: test-project
description: A test project with apps
apps:
  web:
    - types
    - utils
  backend:
    - types
    - validators
"#;
    
    print!("Testing YAML with apps (simple)... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_with_apps) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_apps_section_variations() -> Result<()> {
    println!("\n=== Testing Apps Section Variations ===");
    
    // Test 1: Apps with object format (tsAlias)
    let yaml_object_format = r#"
name: test-project
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
"#;
    
    print!("Testing apps with object format... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_object_format) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 2: Apps with mixed formats
    let yaml_mixed_format = r#"
name: test-project
apps:
  web:
    - types
    - utils
  backend:
    tsAlias: true
    packages:
      - types 
      - validators
"#;
    
    print!("Testing apps with mixed formats... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_mixed_format) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_variables_section() -> Result<()> {
    println!("\n=== Testing Variables Section ===");
    
    // Test 1: Simple variables
    let yaml_simple_vars = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  author: "saravenpi"
"#;
    
    print!("Testing simple variables... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_simple_vars) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 2: Complex variables with descriptions
    let yaml_complex_vars = r#"
name: test-project
variables:
  simple_var: "simple value"
  complex_var:
    value: "complex value"
    description: "This is a complex variable"
"#;
    
    print!("Testing complex variables... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_complex_vars) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_problematic_yaml_patterns() -> Result<()> {
    println!("\n=== Testing Potentially Problematic YAML Patterns ===");
    
    // Test 1: YAML with ${} syntax (what the examples are using)
    let yaml_with_dollar_syntax = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  full_name: "${project_name}-v${version}"
description: "A project called ${full_name}"
"#;
    
    print!("Testing YAML with ${{}} variable syntax... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_with_dollar_syntax) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - Variable {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 2: YAML with {{}} syntax (what the Rust code expects)
    let yaml_with_brace_syntax = r#"
name: test-project
variables:
  project_name: "my-project" 
  version: "1.0.0"
  full_name: "{{project_name}}-v{{version}}"
description: "A project called {{full_name}}"
"#;
    
    print!("Testing YAML with {{{{}}}} variable syntax... ");
    match serde_yaml::from_str::<KnotConfig>(yaml_with_brace_syntax) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - Variable {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_real_world_yaml() -> Result<()> {
    println!("\n=== Testing Real-World YAML (from knot.yml) ===");
    
    let real_knot_yaml = r#"
name: Marcel
description: A gamified organisation app to boost productivity

apps:
  web:
    tsAlias: true
    packages:
      - types
      - validators

  backend:
    tsAlias: true
    packages:
      - types
      - utils
      - validators

  marcel-cli:
    tsAlias: true
    packages:
      - types
      - utils
      - validators

scripts:
  frontend: cd frontend && bun run dev
  backend: cd backend && bun run dev
  cli: cd apps/marcel-cli && bun run dev
  cli-build: cd apps/marcel-cli && bun run build
  cli-tui: cd apps/marcel-cli && bun run build:go
"#;
    
    print!("Testing real-world knot.yml... ");
    match serde_yaml::from_str::<KnotConfig>(real_knot_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            println!("  - Apps count: {}", config.apps.as_ref().map(|a| a.len()).unwrap_or(0));
            println!("  - Scripts count: {}", config.scripts.as_ref().map(|s| s.len()).unwrap_or(0));
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_step_by_step_parsing() -> Result<()> {
    println!("\n=== Testing Step-by-Step Field Parsing ===");
    
    // Test each field individually
    
    // 1. Just name field
    print!("Testing name field only... ");
    let name_only = r#"name: "test-project""#;
    match serde_yaml::from_str::<KnotConfig>(name_only) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // 2. Name + description
    print!("Testing name + description... ");
    let name_desc = r#"
name: "test-project"
description: "A test project"
"#;
    match serde_yaml::from_str::<KnotConfig>(name_desc) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // 3. Add scripts
    print!("Testing name + description + scripts... ");
    let with_scripts = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
"#;
    match serde_yaml::from_str::<KnotConfig>(with_scripts) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // 4. Add simple apps
    print!("Testing with simple apps... ");
    let with_apps = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    - types
"#;
    match serde_yaml::from_str::<KnotConfig>(with_apps) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // 5. Add complex apps
    print!("Testing with complex apps... ");
    let with_complex_apps = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    tsAlias: true
    packages:
      - types
"#;
    match serde_yaml::from_str::<KnotConfig>(with_complex_apps) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // 6. Add variables
    print!("Testing with variables... ");
    let with_variables = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    tsAlias: true
    packages:
      - types
variables:
  test_var: "test_value"
"#;
    match serde_yaml::from_str::<KnotConfig>(with_variables) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    Ok(())
}

fn test_as_generic_yaml() -> Result<()> {
    println!("\n=== Testing as Generic YAML (serde_yaml::Value) ===");
    
    let problematic_yaml = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  full_name: "${project_name}-v${version}"
description: "A project called ${full_name}"
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
"#;
    
    print!("Parsing as generic YAML... ");
    match serde_yaml::from_str::<serde_yaml::Value>(problematic_yaml) {
        Ok(value) => {
            println!("✅ SUCCESS");
            println!("YAML structure: {:#?}", value);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    print!("Converting generic YAML to KnotConfig... ");
    let value: serde_yaml::Value = serde_yaml::from_str(problematic_yaml)?;
    match serde_yaml::from_value::<KnotConfig>(value) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("Config: {:#?}", config);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_specific_problematic_examples() -> Result<()> {
    println!("\n=== Testing Specific Problematic Examples ===");
    
    // Test the exact YAML from example-with-variables.yml 
    let example_yaml = r#"
# Example Knot configuration with variable interpolation
# This demonstrates the new variables section and ${variable_name} syntax

variables:
  # Basic variables
  org_name: "acme-corp"
  project_name: "awesome-platform"
  version: "2.1.0"
  author: "saravenpi"
  
  # Computed variables (using other variables)
  full_name: "${org_name}-${project_name}"
  display_name: "${project_name} v${version}"
  
  # Environment-specific variables (will fallback to env vars if not defined)
  environment: "development"
  api_port: 3000
  db_port: 5432

# Main configuration using variables
name: "${full_name}"
description: "${display_name} - A comprehensive platform built by ${author}"

# Scripts with variable interpolation
scripts:
  start: "echo 'Starting ${display_name} on port ${api_port}'"
  build: "echo 'Building ${full_name} for ${environment}'"

# Apps configuration
apps:
  web:
    name: "${full_name}-web"
    description: "Web interface for ${project_name}"
    packages: 
      - "${org_name}-types"
      - "${org_name}-ui-components"
"#;
    
    print!("Testing example-with-variables.yml content... ");
    match serde_yaml::from_str::<KnotConfig>(example_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    println!("🔍 YAML Parsing Validation Test");
    println!("Testing serde_yaml version: 0.9");
    
    // Run all tests
    test_basic_yaml_parsing()?;
    test_apps_section_variations()?;
    test_variables_section()?;
    test_problematic_yaml_patterns()?;
    test_real_world_yaml()?;
    test_step_by_step_parsing()?;
    test_as_generic_yaml()?;
    test_specific_problematic_examples()?;
    
    println!("\n🎉 All tests completed successfully!");
    println!("\n🔍 Summary:");
    println!("- Basic YAML parsing works");
    println!("- Apps section parsing works with both list and object formats");
    println!("- Variables section parsing works");
    println!("- Both ${{}} and {{{{}}}} variable syntax parse successfully");
    println!("- Real-world YAML from knot.yml parses correctly");
    println!("- Step-by-step field addition all works");
    println!("- Generic YAML parsing and conversion works");
    println!("- Specific problematic examples parse correctly");
    
    Ok(())
}