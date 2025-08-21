use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::PathBuf;

use crate::config::{KnotConfig, AppConfig, PackageConfig};
use crate::templates::{TemplateManager, TemplateCategory};
use super::common::*;

pub fn init_project(name: Option<&str>, path: Option<&str>, description: Option<&str>) -> Result<()> {
    let interactive = name.is_none() || path.is_none();

    // Pretty header
    println!();
    println!("{}", style("🚀 Welcome to Knot Project Initializer").bold().cyan());
    println!("{}", style("Let's create a new Knot project together!").dim());
    println!();

    let project_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your project name?", None)?,
    };

    let project_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 Project description (optional)", Some("A new Knot project"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    let target_dir = match path {
        Some(p) => PathBuf::from(p),
        None if interactive => {
            let suggested_path = PathBuf::from(format!("./{}", project_name));
            let path_str = prompt_for_input("📁 Where should we create the project?", Some(suggested_path.to_str().unwrap_or(".")))?;
            PathBuf::from(path_str)
        }
        None => std::env::current_dir()?,
    };

    // Show summary and confirm
    if interactive {
        println!();
        println!("{}", style("📋 Project Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&project_name).bold());
        if let Some(desc) = &project_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Location:").dim(), target_dir.display());
        println!();

        if !prompt_for_confirm("Create this project?", Some(true))? {
            println!("{}", style("❌ Project creation cancelled").red());
            return Ok(());
        }
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let knot_yml_path = target_dir.join("knot.yml");

    if knot_yml_path.exists() {
        anyhow::bail!("knot.yml already exists in the target directory");
    }

    let config = KnotConfig {
        name: project_name.clone(),
        description: project_description,
        ts_alias: Some(crate::config::TsAlias::Boolean(false)),
        apps: None,
        scripts: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(knot_yml_path, yaml_content).context("Failed to create knot.yml")?;

    fs::create_dir_all(target_dir.join("packages"))?;
    fs::create_dir_all(target_dir.join("apps"))?;

    println!("✅ Initialized new Knot project: {}", project_name);
    println!("📁 Created directories: packages/, apps/");
    println!("💡 Use 'knot init:package <name>' to create packages");
    println!("💡 Use 'knot init:app <name>' to create apps");
    Ok(())
}

pub fn init_package(name: Option<&str>, team: Option<&str>, version: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    // Pretty header for interactive mode
    if interactive {
        println!();
        println!("{}", style("📦 Welcome to Knot Package Creator").bold().cyan());
        println!("{}", style("Let's create a new package together!").dim());
        println!();
    }

    let package_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your package name?", None)?,
    };

    let team_name = match team {
        Some(t) => Some(t.to_string()),
        None if interactive => {
            let team = prompt_for_input("👥 Team name (optional, for namespaced packages)", None).ok();
            team.filter(|t| !t.trim().is_empty())
        },
        None => None,
    };

    let package_version = match version {
        Some(v) => v.to_string(),
        None if interactive => prompt_for_input("🏷️  Package version", Some("1.0.0"))?,
        None => "1.0.0".to_string(),
    };

    let package_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 Package description (optional)", Some("A new Knot package"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    let available_templates = vec!["typescript", "react", "basic"];
    let package_template = match template {
        Some(t) => t.to_string(),
        None if interactive => prompt_for_select("🎨 Choose a template", available_templates)?,
        None => "basic".to_string(),
    };

    // Determine target directory
    let (base_target_dir, context, _in_project) = if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else {
        match path {
            Some(p) => (PathBuf::from(p), format!("in specified path: {}", p), false),
            None => determine_target_directory(&current_dir, "packages")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&package_name)
    };

    // Show summary and confirm for interactive mode
    if interactive {
        println!();
        println!("{}", style("📋 Package Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&package_name).bold());
        if let Some(team) = &team_name {
            println!("   {} @{}/{}", style("Namespaced:").dim(), team, package_name);
        }
        println!("   {} {}", style("Version:").dim(), package_version);
        if let Some(desc) = &package_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Template:").dim(), package_template);
        println!("   {} {} ({})", style("Location:").dim(), target_dir.display(), context);
        println!();

        if !prompt_for_confirm("Create this package?", Some(true))? {
            println!("{}", style("❌ Package creation cancelled").red());
            return Ok(());
        }
    }

    // Handle path collision for non-here mode
    if !here && target_dir.exists() && !path.map(|p| p == ".").unwrap_or(false) {
        anyhow::bail!("Directory '{}' already exists", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let package_yml_path = target_dir.join("package.yml");

    if package_yml_path.exists() {
        anyhow::bail!("package.yml already exists in the target directory");
    }

    let config = PackageConfig {
        name: package_name.clone(),
        team: team_name.clone(),
        version: package_version.clone(),
        description: package_description.clone(),
        scripts: None,
        tags: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&package_yml_path, yaml_content).context("Failed to create package.yml")?;

    // Generate package files using templates
    TemplateManager::create_from_template(
        &package_name,
        &package_version,
        &package_description.unwrap_or_else(|| "A new Knot package".to_string()),
        &package_template,
        TemplateCategory::Package,
        &target_dir,
    )?;

    let display_name = if let Some(team) = &team_name {
        format!("@{}/{}", team, package_name)
    } else {
        package_name.clone()
    };

    println!("✅ Created package: {}", display_name);
    println!("📁 Location: {}", target_dir.display());

    if let Some(team) = &team_name {
        println!("👥 Team: @{}", team);
        println!("💡 Use 'knot publish --team {}' to publish this package", team);
    } else {
        println!("💡 Use 'knot publish' to publish this package");
    }

    Ok(())
}

pub fn init_app(name: Option<&str>, template: Option<&str>, description: Option<&str>, path: Option<&str>, here: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    // Pretty header for interactive mode
    if interactive {
        println!();
        println!("{}", style("🚀 Welcome to Knot App Creator").bold().cyan());
        println!("{}", style("Let's create a new app together!").dim());
        println!();
    }

    let app_name = match name {
        Some(n) => n.to_string(),
        None => prompt_for_input("✨ What's your app name?", None)?,
    };

    let app_description = match description {
        Some(d) => Some(d.to_string()),
        None if interactive => {
            let desc = prompt_for_description("📝 App description (optional)", Some("A new Knot app"))?;
            if desc.trim().is_empty() { None } else { Some(desc) }
        },
        None => None,
    };

    let available_templates = vec!["react", "svelte", "nextjs", "fastify", "express", "vanilla"];
    let app_template = match template {
        Some(t) => t.to_string(),
        None if interactive => prompt_for_select("🎨 Choose a template", available_templates)?,
        None => "vanilla".to_string(),
    };

    // Determine target directory
    let (base_target_dir, context, _in_project) = if here {
        (current_dir.clone(), "in current directory".to_string(), false)
    } else {
        match path {
            Some(p) => (PathBuf::from(p), format!("in specified path: {}", p), false),
            None => determine_target_directory(&current_dir, "apps")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&app_name)
    };

    // Show summary and confirm for interactive mode
    if interactive {
        println!();
        println!("{}", style("📋 App Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&app_name).bold());
        if let Some(desc) = &app_description {
            println!("   {} {}", style("Description:").dim(), desc);
        }
        println!("   {} {}", style("Template:").dim(), app_template);
        println!("   {} {} ({})", style("Location:").dim(), target_dir.display(), context);
        println!();

        if !prompt_for_confirm("Create this app?", Some(true))? {
            println!("{}", style("❌ App creation cancelled").red());
            return Ok(());
        }
    }

    // Handle path collision for non-here mode
    if !here && target_dir.exists() && !path.map(|p| p == ".").unwrap_or(false) {
        anyhow::bail!("Directory '{}' already exists", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let app_yml_path = target_dir.join("app.yml");

    if app_yml_path.exists() {
        anyhow::bail!("app.yml already exists in the target directory");
    }

    let config = AppConfig {
        name: app_name.clone(),
        description: app_description.clone(),
        ts_alias: None,
        packages: None,
        build: None,
        scripts: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&app_yml_path, yaml_content).context("Failed to create app.yml")?;

    // Generate app files using templates
    TemplateManager::create_from_template(
        &app_name,
        "1.0.0",
        &app_description.unwrap_or_else(|| "A new Knot app".to_string()),
        &app_template,
        TemplateCategory::App,
        &target_dir,
    )?;

    println!("✅ Created app: {}", app_name);
    println!("📁 Location: {}", target_dir.display());
    println!("💡 Use 'knot add <package>' to add dependencies");
    println!("💡 Use 'knot link' to install and link packages");

    Ok(())
}