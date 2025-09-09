// Focused test to isolate the ConfigVariable parsing issue

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariable {
    Simple(String),
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
pub struct MinimalConfig {
    pub name: String,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

fn main() -> Result<()> {
    println!("🔍 Focused ConfigVariable Parsing Test");
    
    // Test 1: This works - simple variables
    let simple_vars_yaml = r#"
name: test
variables:
  simple_var: "value"
  another_var: "another value"
"#;
    
    print!("Testing simple variables... ");
    match serde_yaml::from_str::<MinimalConfig>(simple_vars_yaml) {
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
        }
    }
    
    // Test 2: This fails - numeric values 
    let numeric_vars_yaml = r#"
name: test
variables:
  api_port: 3000
  db_port: 5432
"#;
    
    print!("Testing numeric variables... ");
    match serde_yaml::from_str::<MinimalConfig>(numeric_vars_yaml) {
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
        }
    }
    
    // Test 3: Try numeric values as strings
    let numeric_as_string_yaml = r#"
name: test
variables:
  api_port: "3000"
  db_port: "5432"
"#;
    
    print!("Testing numeric variables as strings... ");
    match serde_yaml::from_str::<MinimalConfig>(numeric_as_string_yaml) {
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
        }
    }
    
    // Test 4: Complex variables format
    let complex_vars_yaml = r#"
name: test
variables:
  simple_var: "simple value"
  complex_var:
    value: "complex value"
    description: "A complex variable"
"#;
    
    print!("Testing complex variables format... ");
    match serde_yaml::from_str::<MinimalConfig>(complex_vars_yaml) {
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
        }
    }
    
    // Test 5: The exact problematic section from example-with-variables.yml
    let problematic_section = r#"
name: test
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
"#;
    
    print!("Testing exact problematic section... ");
    match serde_yaml::from_str::<MinimalConfig>(problematic_section) {
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
            println!("Error details: {:?}", e);
        }
    }
    
    // Test 6: Parse as generic value to see the structure
    print!("Parsing problematic section as generic YAML... ");
    match serde_yaml::from_str::<serde_yaml::Value>(problematic_section) {
        Ok(value) => {
            println!("✅ SUCCESS");
            if let Some(variables) = value.get("variables") {
                println!("Variables structure: {:#?}", variables);
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
        }
    }
    
    Ok(())
}