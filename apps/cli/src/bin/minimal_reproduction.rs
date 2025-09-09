// Minimal reproduction case for YAML parsing failure

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

#[derive(Debug, Serialize, Deserialize)]
pub struct KnotConfig {
    pub name: String,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

fn main() {
    println!("🔬 Minimal Reproduction Case");
    
    // This works fine - all strings
    let working_yaml = r#"
name: test-project
variables:
  api_port: "3000"
  db_port: "5432"
"#;
    
    print!("✅ YAML with quoted numbers (strings): ");
    match serde_yaml::from_str::<KnotConfig>(working_yaml) {
        Ok(_) => println!("SUCCESS"),
        Err(e) => println!("FAILED: {}", e),
    }
    
    // This fails - has unquoted numbers
    let failing_yaml = r#"
name: test-project
variables:
  api_port: 3000
  db_port: 5432
"#;
    
    print!("❌ YAML with unquoted numbers: ");
    match serde_yaml::from_str::<KnotConfig>(failing_yaml) {
        Ok(_) => println!("SUCCESS (unexpected!)"),
        Err(e) => println!("FAILED: {}", e),
    }
    
    println!("\n🔍 Root Cause:");
    println!("- ConfigVariable enum only accepts String values");
    println!("- YAML parser treats unquoted numbers as numeric types");
    println!("- Serde can't deserialize Number(3000) into String");
    
    println!("\n💡 Solutions:");
    println!("1. Modify ConfigVariable to accept numeric types");
    println!("2. Quote all numeric values in YAML files");
    println!("3. Use custom deserializer to convert all types to strings");
    
    println!("\n📋 Example YAML files affected:");
    println!("- example-with-variables.yml (api_port: 3000, db_port: 5432)");
    println!("- Any YAML with unquoted numeric variables");
}