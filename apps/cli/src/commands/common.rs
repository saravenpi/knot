use anyhow::Result;
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Select, Text, validator::Validation};
use serde_json;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::project::Project;

// Check if running in interactive environment
pub fn is_interactive() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal()
}

// Helper function for interactive input with beautiful UI
#[allow(dead_code)]
pub fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String> {
    prompt_for_input_with_validation(prompt, default, None, None)
}

// Enhanced input function with custom validation and help text
pub fn prompt_for_input_with_validation(
    prompt: &str,
    default: Option<&str>,
    help_text: Option<&str>,
    custom_validator: Option<fn(&str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>>>
) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            anyhow::bail!("Interactive input required but running in non-interactive environment\n💡 You are using Knot in a non-interactive terminal (like a script or CI/CD)\n💡 Please provide all required values via command line arguments\n💡 Add flags like --name, --description, etc. to your command");
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    if let Some(help) = help_text {
        text_prompt = text_prompt.with_help_message(help);
    }

    let is_required = default.is_none();

    if let Some(validator) = custom_validator {
        text_prompt = text_prompt.with_validator(validator);
    } else {
        text_prompt = text_prompt.with_validator(move |input: &str| {
            if input.trim().is_empty() && is_required {
                Ok(Validation::Invalid(
                    "⚠️  This field is required".into(),
                ))
            } else if !input.trim().is_empty()
                && !input
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ')
            {
                Ok(Validation::Invalid(
                    "⚠️  Please use only letters, numbers, hyphens, underscores, and spaces".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        });
    }

    match text_prompt.prompt() {
        Ok(result) => Ok(result),
        Err(inquire::InquireError::OperationCanceled) => {
            println!("{}", style("❌ Operation cancelled").red());
            anyhow::bail!("Operation cancelled by user")
        },
        Err(e) => Err(anyhow::anyhow!("Input error: {}", e)),
    }
}

// Enhanced input for descriptions (allows more characters)
#[allow(dead_code)]
pub fn prompt_for_description(prompt: &str, default: Option<&str>) -> Result<String> {
    prompt_for_description_with_help(prompt, default, None)
}

// Enhanced description input with help text
pub fn prompt_for_description_with_help(prompt: &str, default: Option<&str>, help_text: Option<&str>) -> Result<String> {
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

    if let Some(help) = help_text {
        text_prompt = text_prompt.with_help_message(help);
    }

    match text_prompt.prompt() {
        Ok(result) => Ok(result),
        Err(inquire::InquireError::OperationCanceled) => {
            println!("{}", style("❌ Operation cancelled").red());
            anyhow::bail!("Operation cancelled by user")
        },
        Err(e) => Err(anyhow::anyhow!("Input error: {}", e)),
    }
}

// Enhanced select prompt
#[allow(dead_code)]
pub fn prompt_for_select(prompt: &str, options: Vec<&str>) -> Result<String> {
    prompt_for_select_with_help(prompt, options, None)
}

// Enhanced select prompt with custom help text
#[allow(dead_code)]
pub fn prompt_for_select_with_help(prompt: &str, options: Vec<&str>, help_text: Option<&str>) -> Result<String> {
    // Fallback to first option if not interactive
    if !is_interactive() {
        return Ok(options.first().unwrap_or(&"basic").to_string());
    }

    if options.is_empty() {
        anyhow::bail!("No options provided for selection");
    }

    let help_message = help_text.unwrap_or("Use arrow keys or j/k to navigate, Enter to select, Esc to cancel");

    match Select::new(prompt, options)
        .with_vim_mode(true)
        .with_help_message(help_message)
        .prompt() {
        Ok(selection) => Ok(selection.to_string()),
        Err(inquire::InquireError::OperationCanceled) => {
            println!("{}", style("❌ Selection cancelled").red());
            anyhow::bail!("Operation cancelled by user")
        },
        Err(e) => Err(anyhow::anyhow!("Selection error: {}", e)),
    }
}

// Enhanced confirm prompt
pub fn prompt_for_confirm(prompt: &str, default: Option<bool>) -> Result<bool> {
    prompt_for_confirm_with_help(prompt, default, None)
}

// Enhanced confirm prompt with help text
pub fn prompt_for_confirm_with_help(prompt: &str, default: Option<bool>, help_text: Option<&str>) -> Result<bool> {
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

    if let Some(help) = help_text {
        confirm_prompt = confirm_prompt.with_help_message(help);
    }

    match confirm_prompt.prompt() {
        Ok(result) => Ok(result),
        Err(inquire::InquireError::OperationCanceled) => {
            println!("{}", style("❌ Confirmation cancelled").red());
            anyhow::bail!("Operation cancelled by user")
        },
        Err(e) => Err(anyhow::anyhow!("Confirmation error: {}", e)),
    }
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

// Progress indicator utilities
#[allow(dead_code)]
pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(message.to_string());
    pb
}

pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

pub fn finish_progress(pb: &ProgressBar, message: &str) {
    pb.finish_with_message(format!("✅ {}", message));
}

pub fn fail_progress(pb: &ProgressBar, message: &str) {
    pb.finish_with_message(format!("❌ {}", message));
}

// Enhanced error display functions
pub fn display_error(message: &str) {
    eprintln!("{} {}", style("❌ Error:").red().bold(), message);
}

#[allow(dead_code)]
pub fn display_warning(message: &str) {
    println!("{} {}", style("⚠️  Warning:").yellow().bold(), message);
}

pub fn display_success(message: &str) {
    println!("{} {}", style("✅ Success:").green().bold(), message);
}

pub fn display_info(message: &str) {
    println!("{} {}", style("ℹ️  Info:").blue().bold(), message);
}

pub fn display_tip(message: &str) {
    println!("{} {}", style("💡 Tip:").cyan().bold(), message);
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
        400 => format!("❌ Bad request: {}", error_message),
        401 => "🔐 Authentication failed. Please log in again with 'knot auth'".to_string(),
        403 => "🚫 You don't have permission to perform this action".to_string(),
        404 => format!("🔍 Resource not found: {}", error_message),
        409 => format!("⚠️  Conflict: {}", error_message),
        422 => format!("❌ Validation error: {}", error_message),
        429 => "⏳ Rate limited. Please wait and try again".to_string(),
        500..=599 => "🔥 Server error. Please try again later".to_string(),
        _ => format!("❌ HTTP {} error: {}", status.as_u16(), error_message),
    }
}

// Handle Ctrl+C gracefully
pub fn setup_ctrl_c_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        println!("\n{}", style("❌ Operation cancelled by user").red());
        std::process::exit(130); // Standard exit code for Ctrl+C
    })?;
    Ok(())
}

// Clear the current line and move cursor to beginning
#[allow(dead_code)]
pub fn clear_line() {
    let term = Term::stdout();
    let _ = term.clear_line();
    // Move cursor to bottom - height method requires TermLike trait
    use indicatif::TermLike;
    let _ = term.move_cursor_to(0, (term.height() - 1) as usize);
}