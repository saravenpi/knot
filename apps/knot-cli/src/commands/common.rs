use anyhow::Result;
use inquire::{Confirm, Select, Text};
use serde_json;
use std::path::{Path, PathBuf};

use crate::project::Project;

// Check if running in interactive environment
pub fn is_interactive() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal()
}

// Helper function for interactive input with beautiful UI
pub fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            anyhow::bail!("Interactive input required but running in non-interactive environment. Please provide values via command line arguments.");
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    let is_required = default.is_none();
    text_prompt = text_prompt.with_validator(move |input: &str| {
        if input.trim().is_empty() && is_required {
            Ok(inquire::validator::Validation::Invalid(
                "This field is required".into(),
            ))
        } else if !input.trim().is_empty()
            && !input
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ')
        {
            Ok(inquire::validator::Validation::Invalid(
                "Please use only letters, numbers, hyphens, underscores, and spaces".into(),
            ))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    });

    Ok(text_prompt.prompt()?)
}

// Enhanced input for descriptions (allows more characters)
pub fn prompt_for_description(prompt: &str, default: Option<&str>) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            return Ok(String::new()); // Optional descriptions can be empty
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    Ok(text_prompt.prompt()?)
}

// Enhanced select prompt
pub fn prompt_for_select(prompt: &str, options: Vec<&str>) -> Result<String> {
    // Fallback to first option if not interactive
    if !is_interactive() {
        return Ok(options.first().unwrap_or(&"basic").to_string());
    }

    let selection = Select::new(prompt, options).prompt()?;
    Ok(selection.to_string())
}

// Enhanced confirm prompt
pub fn prompt_for_confirm(prompt: &str, default: Option<bool>) -> Result<bool> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val);
        } else {
            return Ok(false); // Default to false if no default provided
        }
    }

    let mut confirm_prompt = Confirm::new(prompt);

    if let Some(default_val) = default {
        confirm_prompt = confirm_prompt.with_default(default_val);
    }

    Ok(confirm_prompt.prompt()?)
}

// Helper function to determine the best directory for creating packages/apps
pub fn determine_target_directory(
    current_dir: &Path,
    item_type: &str,
) -> Result<(PathBuf, String, bool)> {
    // Try to find project root
    match Project::find_project_root(current_dir) {
        Ok(project_root) => {
            // We're in a Knot project
            let target_dir = match item_type {
                "packages" => project_root.join("packages"),
                "apps" => project_root.join("apps"),
                _ => current_dir.to_path_buf(),
            };

            let context = if project_root == *current_dir {
                format!("in project root, will create in {}/ directory", item_type)
            } else {
                format!(
                    "in Knot project, will create in {}/ directory relative to project root",
                    item_type
                )
            };

            Ok((target_dir, context, true))
        }
        Err(_) => {
            // Not in a Knot project, create in current directory
            let context = "outside Knot project, will create in current directory".to_string();
            Ok((current_dir.to_path_buf(), context, false))
        }
    }
}

// Helper function to format API error responses in a user-friendly way
#[allow(dead_code)]
pub fn format_api_error(status: reqwest::StatusCode, response_text: &str) -> String {
    // Try to parse as JSON first to extract the actual error message
    let error_message = if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response_text) {
        json_value
            .get("error")
            .or_else(|| json_value.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or(response_text)
            .to_string()
    } else {
        response_text.to_string()
    };

    // Provide user-friendly context based on status code and add appropriate emoji
    match status.as_u16() {
        400 => format!("❌ {}", error_message),
        401 => "🔐 Authentication failed. Please log in again with 'knot auth'".to_string(),
        403 => "🚫 You don't have permission to perform this action".to_string(),
        404 => format!("🔍 Resource not found. {}", error_message),
        409 => format!("⚠️  {}", error_message),
        500..=599 => "🔥 Server error. Please try again later".to_string(),
        _ => format!("❌ {}", error_message),
    }
}