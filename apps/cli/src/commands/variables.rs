use crate::project::Project;
use crate::variables::VariableSource;
use anyhow::Result;
use console::style;
use std::env;

/// List all variables available in the project
pub fn vars_list(app_name: Option<&str>, package_name: Option<&str>) -> Result<()> {
    let project = Project::find_and_load(&env::current_dir()?)?;
    
    match (app_name, package_name) {
        (Some(app), Some(pkg)) => {
            // Show variables for a specific app and package context
            println!("🔧 Variables for app '{}' and package '{}':", style(app).green(), style(pkg).cyan());
            let context = project.get_full_variable_context(app, pkg);
            list_variables_table(&context.list_variables());
        }
        (Some(app), None) => {
            // Show variables for a specific app
            println!("📱 Variables for app '{}':", style(app).green());
            let context = project.get_app_variable_context(app);
            list_variables_table(&context.list_variables());
        }
        (None, Some(pkg)) => {
            // Show variables for a specific package
            println!("📦 Variables for package '{}':", style(pkg).cyan());
            let context = project.get_package_variable_context(pkg);
            list_variables_table(&context.list_variables());
        }
        (None, None) => {
            // Show all project-level variables
            println!("🌍 Project variables:");
            list_variables_table(&project.variable_context.list_variables());
        }
    }
    
    Ok(())
}

/// Get the value of a specific variable
pub fn vars_get(var_name: &str, app_name: Option<&str>, package_name: Option<&str>) -> Result<()> {
    let project = Project::find_and_load(&env::current_dir()?)?;
    
    let context = match (app_name, package_name) {
        (Some(app), Some(pkg)) => project.get_full_variable_context(app, pkg),
        (Some(app), None) => project.get_app_variable_context(app),
        (None, Some(pkg)) => project.get_package_variable_context(pkg),
        (None, None) => project.variable_context.clone(),
    };
    
    if let Some(value) = context.get_variable(var_name) {
        // Find the variable info to show its source
        let var_info = context.list_variables()
            .into_iter()
            .find(|v| v.name == var_name);
        
        match var_info {
            Some(info) => {
                println!("Variable: {}", style(var_name).yellow());
                println!("Value:    {}", style(value).green());
                println!("Source:   {}", style(info.source.to_string()).blue());
            }
            None => {
                // This shouldn't happen, but just in case
                println!("{} = {}", style(var_name).yellow(), style(value).green());
            }
        }
    } else {
        println!("❌ Variable '{}' not found", style(var_name).red());
        
        // Show available variables as suggestions
        let available_vars = context.list_variables();
        if !available_vars.is_empty() {
            println!("\n💡 Available variables:");
            for var in available_vars.iter().take(5) {
                println!("  • {}", var.name);
            }
            if available_vars.len() > 5 {
                println!("  ... and {} more", available_vars.len() - 5);
            }
        }
        
        anyhow::bail!("Variable not found");
    }
    
    Ok(())
}

fn list_variables_table(variables: &[crate::variables::VariableInfo]) {
    if variables.is_empty() {
        println!("  No variables defined");
        return;
    }
    
    // Calculate column widths for pretty printing
    let max_name_len = variables.iter().map(|v| v.name.len()).max().unwrap_or(0);
    let max_value_len = variables.iter().map(|v| v.value.len()).max().unwrap_or(0).min(50); // Cap at 50 chars
    let max_source_len = variables.iter().map(|v| v.source.to_string().len()).max().unwrap_or(0);
    
    // Print header
    println!("  {:<width_name$} {:<width_value$} {:<width_source$}",
        "NAME", "VALUE", "SOURCE",
        width_name = max_name_len.max(4),
        width_value = max_value_len.max(5),
        width_source = max_source_len.max(6)
    );
    println!("  {:<width_name$} {:<width_value$} {:<width_source$}",
        "─".repeat(max_name_len.max(4)),
        "─".repeat(max_value_len.max(5)),
        "─".repeat(max_source_len.max(6)),
        width_name = max_name_len.max(4),
        width_value = max_value_len.max(5),
        width_source = max_source_len.max(6)
    );
    
    // Print variables
    for var in variables {
        let display_value = if var.value.len() > 47 {
            format!("{}...", &var.value[..47])
        } else {
            var.value.clone()
        };
        
        let name_color = match var.source {
            VariableSource::BuiltIn => style(&var.name).blue(),
            VariableSource::Project => style(&var.name).cyan(),
            VariableSource::App => style(&var.name).green(),
            VariableSource::Package => style(&var.name).yellow(),
        };
        
        let source_color = match var.source {
            VariableSource::BuiltIn => style(var.source.to_string()).blue(),
            VariableSource::Project => style(var.source.to_string()).cyan(),
            VariableSource::App => style(var.source.to_string()).green(),
            VariableSource::Package => style(var.source.to_string()).yellow(),
        };
        
        println!("  {:<width_name$} {:<width_value$} {}",
            name_color,
            display_value,
            source_color,
            width_name = max_name_len.max(4),
            width_value = max_value_len.max(5)
        );
    }
    
    println!();
    
    // Show precedence note
    println!("💡 Variable precedence (highest to lowest): {} > {} > {} > {}",
        style("package").yellow(),
        style("app").green(),
        style("project").cyan(),
        style("built-in").blue()
    );
}