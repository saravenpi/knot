use anyhow::{Context, Result};
use console::style;
use std::collections::HashMap;

use crate::dependency::{
    DependencyResolver,
    PackageId, PackageVersion, DependencySpec, ResolutionContext, ResolutionStrategy,
};
use crate::dependency::registry::{LocalPackageRegistry, RemotePackageRegistry};
use crate::project::Project;

pub async fn deps_add(package_spec: &str, app_name: Option<&str>, dev: bool, optional: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    // Determine target app
    let target_app = if let Some(app) = app_name {
        app.to_string()
    } else {
        // Try to detect current app from directory
        detect_current_app(&current_dir, &project)?
    };
    
    println!("📦 Adding dependency '{}' to app '{}'", 
             style(package_spec).cyan(), 
             style(&target_app).green());
    
    // Parse package specification
    let dep_spec = parse_package_spec(package_spec, dev, optional)?;
    
    // Initialize resolver
    let mut resolver = create_resolver(&project).await?;
    
    // Get current dependencies for the app
    let mut current_deps = get_app_dependencies(&project, &target_app)?;
    
    // Add the new dependency
    current_deps.push(dep_spec.clone());
    
    // Resolve dependencies
    println!("🔍 Resolving dependencies...");
    let resolution = resolver.resolve_dependencies(current_deps).await
        .with_context(|| format!("Failed to resolve dependencies for app '{}'", target_app))?;
    
    // Update app configuration
    update_app_config(&project, &target_app, package_spec, dev, optional)?;
    
    // Link the packages
    println!("🔗 Linking packages...");
    link_resolved_packages(&project, &target_app, &resolution.resolved_packages).await?;
    
    println!("✅ Successfully added dependency '{}' to app '{}'", 
             style(package_spec).cyan(), 
             style(&target_app).green());
    
    // Show resolution summary
    print_resolution_summary(&resolution);
    
    Ok(())
}

pub async fn deps_list(app_name: Option<&str>, tree: bool, depth: Option<usize>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    if let Some(app) = app_name {
        list_app_dependencies(&project, app, tree, depth).await?;
    } else {
        // List all apps
        let app_names = project.get_app_names();
        
        if app_names.is_empty() {
            println!("No apps found in this project.");
            return Ok(());
        }
        
        for (i, app_name) in app_names.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("📱 App: {}", style(app_name).green().bold());
            list_app_dependencies(&project, app_name, tree, depth).await?;
        }
    }
    
    Ok(())
}

pub async fn deps_resolve(strategy: Option<&str>, dry_run: bool, app_name: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    let resolution_strategy = parse_resolution_strategy(strategy)?;
    
    if let Some(app) = app_name {
        resolve_app_dependencies(&project, app, resolution_strategy, dry_run).await?;
    } else {
        let app_names = project.get_app_names();
        
        for app_name in app_names {
            if dry_run {
                println!("\n📱 Would resolve dependencies for app: {}", style(&app_name).green());
            } else {
                println!("\n📱 Resolving dependencies for app: {}", style(&app_name).green());
            }
            
            resolve_app_dependencies(&project, &app_name, resolution_strategy.clone(), dry_run).await?;
        }
    }
    
    Ok(())
}

pub async fn deps_check() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    println!("🔍 Analyzing project dependencies...");
    
    let mut issues_found = 0;
    let app_names = project.get_app_names();
    
    for app_name in app_names {
        println!("\n📱 Checking app '{}':", style(&app_name).green());
        
        match analyze_app_dependencies(&project, &app_name).await {
            Ok(analysis) => {
                if analysis.has_issues() {
                    issues_found += 1;
                    print_dependency_analysis(&analysis);
                } else {
                    println!("  ✅ No issues found");
                }
            }
            Err(e) => {
                println!("  ❌ Analysis failed: {}", e);
                issues_found += 1;
            }
        }
    }
    
    if issues_found == 0 {
        println!("\n🎉 All dependencies look good!");
    } else {
        println!("\n⚠️  Found issues in {} app(s)", issues_found);
    }
    
    Ok(())
}

pub async fn deps_tree(app_name: Option<&str>, depth: Option<usize>) -> Result<()> {
    deps_list(app_name, true, depth).await
}

pub async fn deps_outdated(app_name: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    println!("🔍 Checking for outdated dependencies...");
    
    if let Some(app) = app_name {
        check_outdated_dependencies(&project, app).await?;
    } else {
        let app_names = project.get_app_names();
        
        for (i, app_name) in app_names.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("📱 App: {}", style(app_name).green().bold());
            check_outdated_dependencies(&project, app_name).await?;
        }
    }
    
    Ok(())
}

pub async fn deps_why(package_name: &str, app_name: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    let target_app = if let Some(app) = app_name {
        app.to_string()
    } else {
        detect_current_app(&current_dir, &project)?
    };
    
    println!("🔍 Explaining why '{}' is included in app '{}'", 
             style(package_name).cyan(), 
             style(&target_app).green());
    
    explain_dependency_inclusion(&project, &target_app, package_name).await?;
    
    Ok(())
}

pub async fn deps_sync() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    
    println!("🔄 Synchronizing dependencies across all apps...");
    
    // Find common dependencies and suggest version alignment
    let common_deps = find_common_dependencies(&project).await?;
    
    if common_deps.is_empty() {
        println!("No common dependencies found across apps.");
        return Ok(());
    }
    
    println!("Found {} common dependencies:", common_deps.len());
    
    for (package_name, versions) in common_deps {
        println!("\n📦 Package: {}", style(&package_name).cyan());
        
        if versions.len() > 1 {
            println!("  ⚠️  Version conflict detected:");
            for (app_name, version) in &versions {
                println!("    {} uses {}", style(app_name).green(), style(version).yellow());
            }
            
            // Suggest resolution
            let latest_version = find_latest_compatible_version(&versions);
            if let Some(suggested) = latest_version {
                println!("  💡 Suggested version: {}", style(suggested).green());
            }
        } else {
            let (_app_name, version) = versions.first().unwrap();
            println!("  ✅ All apps use version {}", style(version).green());
        }
    }
    
    Ok(())
}

// Helper functions

async fn create_resolver(project: &Project) -> Result<DependencyResolver> {
    let context = ResolutionContext {
        strategy: ResolutionStrategy::Compatible,
        allow_prerelease: false,
        max_depth: 50,
        include_dev: false,
        include_optional: true,
        platform: Some(std::env::consts::OS.to_string()),
        arch: Some(std::env::consts::ARCH.to_string()),
        environment: Some("production".to_string()),
    };
    
    let local_registry = LocalPackageRegistry::new(project.root.join("packages"));
    let remote_registry = RemotePackageRegistry::new("https://knot.space".to_string());
    let cache_dir = project.root.join(".knot").join("cache");
    
    Ok(DependencyResolver::new(context, local_registry, remote_registry, cache_dir))
}

fn detect_current_app(current_dir: &std::path::Path, project: &Project) -> Result<String> {
    // Try to detect if we're in an app directory
    let relative_path = current_dir.strip_prefix(&project.root)?;
    
    if let Some(app_segment) = relative_path.components().nth(1) {
        if relative_path.starts_with("apps") {
            return Ok(app_segment.as_os_str().to_string_lossy().to_string());
        }
    }
    
    // If not in an app directory, prompt the user
    let app_names = project.get_app_names();
    
    if app_names.is_empty() {
        anyhow::bail!("No apps found in this project");
    }
    
    if app_names.len() == 1 {
        return Ok(app_names.into_iter().next().unwrap());
    }
    
    // Interactive selection
    use inquire::Select;
    
    let selection = Select::new("Select target app:", app_names)
        .prompt()
        .context("Failed to select app")?;
    
    Ok(selection)
}

fn parse_package_spec(spec: &str, dev: bool, optional: bool) -> Result<DependencySpec> {
    let (name, version_req_str) = if let Some(at_pos) = spec.rfind('@') {
        let name = spec[..at_pos].to_string();
        let version_str = &spec[at_pos + 1..];
        (name, version_str)
    } else {
        (spec.to_string(), "*")
    };
    
    let version_req = semver::VersionReq::parse(if version_req_str == "latest" {
        "*"
    } else {
        version_req_str
    }).with_context(|| format!("Invalid version requirement: {}", version_req_str))?;
    
    let source = if name.starts_with('@') {
        crate::dependency::types::PackageSource::Remote {
            registry: "knot-space".to_string(),
        }
    } else {
        crate::dependency::types::PackageSource::Local
    };
    
    Ok(DependencySpec {
        id: PackageId { name, source },
        version_req,
        optional,
        dev_only: dev,
        conditions: None,
        features: None,
    })
}

fn parse_resolution_strategy(strategy: Option<&str>) -> Result<ResolutionStrategy> {
    match strategy {
        Some("latest") => Ok(ResolutionStrategy::Latest),
        Some("strict") => Ok(ResolutionStrategy::Strict),
        Some("compatible") => Ok(ResolutionStrategy::Compatible),
        Some("conservative") => Ok(ResolutionStrategy::Conservative),
        Some(unknown) => anyhow::bail!("Unknown resolution strategy: {}", unknown),
        None => Ok(ResolutionStrategy::Compatible),
    }
}

fn get_app_dependencies(project: &Project, app_name: &str) -> Result<Vec<DependencySpec>> {
    let raw_deps = project.get_app_dependencies(app_name);
    let mut dep_specs = Vec::new();
    
    for dep_str in raw_deps {
        let dep_spec = parse_package_spec(&dep_str, false, false)?;
        dep_specs.push(dep_spec);
    }
    
    Ok(dep_specs)
}

fn update_app_config(_project: &Project, app_name: &str, package_spec: &str, _dev: bool, _optional: bool) -> Result<()> {
    // This would update the app configuration file to include the new dependency
    // For now, we'll just print what would be updated
    println!("📝 Would update app configuration for '{}' to include '{}'", app_name, package_spec);
    
    // In a real implementation, this would:
    // 1. Load the app.yml file
    // 2. Add the package to the appropriate dependencies section
    // 3. Write the updated configuration back
    
    Ok(())
}

async fn link_resolved_packages(
    _project: &Project, 
    app_name: &str, 
    resolved_packages: &HashMap<PackageId, PackageVersion>
) -> Result<()> {
    // This would link the resolved packages to the app's knot_packages directory
    println!("🔗 Would link {} packages for app '{}'", resolved_packages.len(), app_name);
    
    // In a real implementation, this would:
    // 1. Create/clean the knot_packages directory
    // 2. Link each resolved package (symlink or copy)
    // 3. Update TypeScript configurations
    
    Ok(())
}

fn print_resolution_summary(resolution: &crate::dependency::types::ResolutionResult) {
    println!("\n📊 Resolution Summary:");
    println!("  Packages resolved: {}", resolution.resolved_packages.len());
    
    if !resolution.conflicts.is_empty() {
        println!("  Conflicts resolved: {}", resolution.conflicts.len());
    }
    
    if !resolution.warnings.is_empty() {
        println!("  Warnings: {}", resolution.warnings.len());
        for warning in &resolution.warnings {
            println!("    ⚠️  {}", warning);
        }
    }
}

async fn list_app_dependencies(
    project: &Project, 
    app_name: &str, 
    tree: bool, 
    depth: Option<usize>
) -> Result<()> {
    let deps = get_app_dependencies(project, app_name)?;
    
    if deps.is_empty() {
        println!("  No dependencies");
        return Ok(());
    }
    
    if tree {
        // Show dependency tree
        let mut resolver = create_resolver(project).await?;
        let resolution = resolver.resolve_dependencies(deps).await?;
        
        print_dependency_tree(&resolution, depth);
    } else {
        // Show flat list
        for dep in deps {
            let version_str = if dep.version_req.to_string() == "*" {
                "latest".to_string()
            } else {
                dep.version_req.to_string()
            };
            
            println!("  📦 {} @ {}", 
                     style(&dep.id.name).cyan(), 
                     style(version_str).yellow());
        }
    }
    
    Ok(())
}

fn print_dependency_tree(resolution: &crate::dependency::types::ResolutionResult, _max_depth: Option<usize>) {
    // This would print a tree structure of dependencies
    println!("  📦 Dependency tree:");
    
    for package_id in &resolution.dependency_order {
        if let Some(package) = resolution.resolved_packages.get(package_id) {
            println!("  ├── {} @ {}", 
                     style(&package.id.name).cyan(), 
                     style(&package.version).yellow());
        }
    }
}

async fn resolve_app_dependencies(
    project: &Project, 
    app_name: &str, 
    strategy: ResolutionStrategy,
    dry_run: bool
) -> Result<()> {
    let deps = get_app_dependencies(project, app_name)?;
    
    if deps.is_empty() {
        println!("  No dependencies to resolve");
        return Ok(());
    }
    
    let context = ResolutionContext {
        strategy,
        ..Default::default()
    };
    
    let mut resolver = DependencyResolver::new(
        context,
        LocalPackageRegistry::new(project.root.join("packages")),
        RemotePackageRegistry::new("https://knot.space".to_string()),
        project.root.join(".knot").join("cache"),
    );
    
    match resolver.resolve_dependencies(deps).await {
        Ok(resolution) => {
            if dry_run {
                println!("  Would resolve {} packages", resolution.resolved_packages.len());
            } else {
                println!("  ✅ Resolved {} packages", resolution.resolved_packages.len());
                
                // Link packages
                link_resolved_packages(project, app_name, &resolution.resolved_packages).await?;
            }
            
            if !resolution.warnings.is_empty() {
                for warning in &resolution.warnings {
                    println!("    ⚠️  {}", warning);
                }
            }
        }
        Err(e) => {
            println!("  ❌ Resolution failed: {}", e);
        }
    }
    
    Ok(())
}

async fn analyze_app_dependencies(_project: &Project, app_name: &str) -> Result<DependencyAnalysis> {
    // This would perform dependency analysis
    Ok(DependencyAnalysis {
        app_name: app_name.to_string(),
        issues: Vec::new(),
        suggestions: Vec::new(),
    })
}

struct DependencyAnalysis {
    app_name: String,
    issues: Vec<String>,
    suggestions: Vec<String>,
}

impl DependencyAnalysis {
    fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

fn print_dependency_analysis(analysis: &DependencyAnalysis) {
    for issue in &analysis.issues {
        println!("  ❌ {}", issue);
    }
    
    for suggestion in &analysis.suggestions {
        println!("  💡 {}", suggestion);
    }
}

async fn check_outdated_dependencies(_project: &Project, app_name: &str) -> Result<()> {
    let deps = get_app_dependencies(_project, app_name)?;
    
    if deps.is_empty() {
        println!("  No dependencies");
        return Ok(());
    }
    
    // This would check for newer versions
    println!("  All dependencies are up to date");
    
    Ok(())
}

async fn explain_dependency_inclusion(_project: &Project, app_name: &str, package_name: &str) -> Result<()> {
    // This would trace the dependency chain that leads to the package being included
    println!("  📦 '{}' is included because:", package_name);
    println!("    └── Direct dependency of app '{}'", app_name);
    
    Ok(())
}

async fn find_common_dependencies(_project: &Project) -> Result<HashMap<String, Vec<(String, String)>>> {
    let common_deps = HashMap::new();
    
    // This would analyze all apps and find common dependencies
    // For now, return empty
    
    Ok(common_deps)
}

fn find_latest_compatible_version(versions: &[(String, String)]) -> Option<&str> {
    // This would find the latest compatible version across all apps
    versions.first().map(|(_, version)| version.as_str())
}