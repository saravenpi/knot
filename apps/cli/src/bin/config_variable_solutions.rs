// Test program showing different solutions for ConfigVariable numeric values issue

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Original ConfigVariable (fails with numeric values)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OriginalConfigVariable {
    Simple(String),
    Complex {
        value: String,
        description: Option<String>,
    },
}

// Solution 1: Add numeric variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariableV1 {
    SimpleString(String),
    SimpleNumber(f64),
    SimpleBool(bool),
    Complex {
        value: String,
        description: Option<String>,
    },
}

impl ConfigVariableV1 {
    pub fn get_value(&self) -> String {
        match self {
            ConfigVariableV1::SimpleString(s) => s.clone(),
            ConfigVariableV1::SimpleNumber(n) => n.to_string(),
            ConfigVariableV1::SimpleBool(b) => b.to_string(),
            ConfigVariableV1::Complex { value, .. } => value.clone(),
        }
    }
}

// Solution 2: Use serde_yaml::Value for maximum flexibility
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariableV2 {
    Simple(serde_yaml::Value),
    Complex {
        value: String,
        description: Option<String>,
    },
}

impl ConfigVariableV2 {
    pub fn get_value(&self) -> String {
        match self {
            ConfigVariableV2::Simple(value) => {
                match value {
                    serde_yaml::Value::String(s) => s.clone(),
                    serde_yaml::Value::Number(n) => n.to_string(),
                    serde_yaml::Value::Bool(b) => b.to_string(),
                    _ => format!("{:?}", value),
                }
            }
            ConfigVariableV2::Complex { value, .. } => value.clone(),
        }
    }
}

// Solution 3: Custom deserializer that converts everything to strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariableV3 {
    #[serde(deserialize_with = "deserialize_to_string")]
    Simple(String),
    Complex {
        value: String,
        description: Option<String>,
    },
}

fn deserialize_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_yaml::Value = serde::Deserialize::deserialize(deserializer)?;
    Ok(match value {
        serde_yaml::Value::String(s) => s,
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        _ => format!("{:?}", value),
    })
}

impl ConfigVariableV3 {
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariableV3::Simple(s) => s,
            ConfigVariableV3::Complex { value, .. } => value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConfig<T> {
    pub name: String,
    pub variables: Option<HashMap<String, T>>,
}

fn main() -> Result<()> {
    println!("🔧 ConfigVariable Solutions Test");
    
    let problematic_yaml = r#"
name: test
variables:
  org_name: "acme-corp"
  version: "2.1.0"
  api_port: 3000
  db_port: 5432
  enabled: true
"#;
    
    println!("\nTesting problematic YAML:");
    println!("{}", problematic_yaml);
    
    // Test Original (should fail)
    print!("\n1. Original ConfigVariable (string only)... ");
    match serde_yaml::from_str::<TestConfig<OriginalConfigVariable>>(problematic_yaml) {
        Ok(_) => println!("✅ SUCCESS (unexpected!)"),
        Err(e) => println!("❌ FAILED (expected): {}", e),
    }
    
    // Test Solution 1 (specific types)
    print!("2. Solution 1 (specific numeric/bool types)... ");
    match serde_yaml::from_str::<TestConfig<ConfigVariableV1>>(problematic_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("   - {}: {} ({})", key, value.get_value(), 
                        match value {
                            ConfigVariableV1::SimpleString(_) => "string",
                            ConfigVariableV1::SimpleNumber(_) => "number", 
                            ConfigVariableV1::SimpleBool(_) => "bool",
                            ConfigVariableV1::Complex { .. } => "complex",
                        }
                    );
                }
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // Test Solution 2 (serde_yaml::Value)
    print!("3. Solution 2 (serde_yaml::Value)... ");
    match serde_yaml::from_str::<TestConfig<ConfigVariableV2>>(problematic_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("   - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // Test Solution 3 (custom deserializer)
    print!("4. Solution 3 (custom deserializer)... ");
    match serde_yaml::from_str::<TestConfig<ConfigVariableV3>>(problematic_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("   - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    // Test with complex variables too
    let complex_yaml = r#"
name: test
variables:
  simple_string: "hello"
  simple_number: 42
  simple_bool: true
  complex_var:
    value: "complex value"
    description: "A complex variable"
"#;
    
    println!("\nTesting complex variables YAML:");
    print!("Solution 1 with complex variables... ");
    match serde_yaml::from_str::<TestConfig<ConfigVariableV1>>(complex_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("   - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
    
    println!("\n🎯 Recommendations:");
    println!("- Solution 1 (specific types) is type-safe and clear");
    println!("- Solution 2 (serde_yaml::Value) is most flexible");
    println!("- Solution 3 (custom deserializer) maintains string interface");
    println!("- The issue is that numeric values in YAML aren't being quoted");
    
    Ok(())
}