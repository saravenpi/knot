use crate::config::{AppConfig, KnotConfig, PackageConfig};
use crate::dependency::{DependencyResolver, DependencySpec, PackageId, PackageVersion, ResolutionContext, ResolutionStrategy};
use crate::dependency::registry::{LocalPackageRegistry, RemotePackageRegistry};
use crate::utils;
use anyhow::{Context, Result};
use console::style;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use semver::{Version, VersionReq};

pub struct Project {
    pub root: PathBuf,
    pub config: KnotConfig,
    pub packages: HashMap<String, PackageConfig>,
    pub apps: HashMap<String, AppConfig>,
    dependency_resolver: Option<DependencyResolver>,
}

impl Project {
    pub fn find_and_load(start_dir: &Path) -> Result<Project> {
        let project_root = Self::find_project_root(start_dir)?;
        let config_path = utils::find_yaml_file(&project_root, "knot")
            .ok_or_else(|| anyhow::anyhow!("No knot.yml or knot.yaml file found in {:?}", project_root))?;
        let config = KnotConfig::from_file(&config_path)
            .with_context(|| format!("Failed to load config from {:?}", config_path))?;

        let mut project = Project {
            root: project_root,
            config,
            packages: HashMap::new(),
            apps: HashMap::new(),
            dependency_resolver: None,
        };

        project.load_packages()?;
        project.load_apps()?;

        Ok(project)
    }

    pub fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            if utils::yaml_config_exists(&current, "knot") {
                // Only canonicalize when we find the file to get the clean path
                return match current.canonicalize() {
                    Ok(canonical) => Ok(canonical),
                    Err(_) => Ok(current), // Fall back to non-canonical path
                };
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => anyhow::bail!("No knot.yml or knot.yaml file found in directory tree"),
            }
        }
    }

    fn load_packages(&mut self) -> Result<()> {
        let packages_dir = self.root.join("packages");
        if !packages_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&packages_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(package_config_path) = utils::find_yaml_file(&path, "package") {
                    let package_config = PackageConfig::from_file(&package_config_path)
                        .with_context(|| format!("Failed to load {:?}", package_config_path))?;

                    let package_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| anyhow::anyhow!("Invalid package directory name"))?
                        .to_string();

                    self.packages.insert(package_name, package_config);
                }
            }
        }

        Ok(())
    }

    fn load_apps(&mut self) -> Result<()> {
        let apps_dir = self.root.join("apps");
        if !apps_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&apps_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(app_config_path) = utils::find_yaml_file(&path, "app") {
                    let app_config = AppConfig::from_file(&app_config_path)
                        .with_context(|| format!("Failed to load {:?}", app_config_path))?;

                    let app_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| anyhow::anyhow!("Invalid app directory name"))?
                        .to_string();

                    self.apps.insert(app_name, app_config);
                }
            }
        }

        Ok(())
    }

    pub fn get_app_dependencies(&self, app_name: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        if let Some(app_deps) = self
            .config
            .apps
            .as_ref()
            .and_then(|apps| apps.get(app_name))
        {
            dependencies.extend(app_deps.get_packages());
        }

        if let Some(app_config) = self.apps.get(app_name) {
            if let Some(packages) = &app_config.packages {
                dependencies.extend(packages.clone());
            }
        }

        dependencies.sort();
        dependencies.dedup();
        dependencies
    }

    pub fn get_app_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if let Some(apps) = &self.config.apps {
            names.extend(apps.keys().cloned());
        }

        names.extend(self.apps.keys().cloned());
        names.sort();
        names.dedup();
        names
    }

    pub fn get_app_ts_alias(&self, app_name: &str) -> Option<String> {
        // First check app.yml for tsAlias
        if let Some(app_config) = self.apps.get(app_name) {
            if let Some(ts_alias) = &app_config.ts_alias {
                return ts_alias.get_alias();
            }
        }

        // Then check knot.yml app dependencies for tsAlias
        if let Some(app_deps) = self
            .config
            .apps
            .as_ref()
            .and_then(|apps| apps.get(app_name))
        {
            if let Some(ts_alias) = app_deps.get_ts_alias() {
                return ts_alias.get_alias();
            }
        }

        // Finally fall back to global tsAlias from knot.yml
        if let Some(ts_alias) = &self.config.ts_alias {
            return ts_alias.get_alias();
        }

        None
    }

    pub async fn resolve_dependencies(&mut self, app_name: &str, include_dev: bool, strategy: Option<ResolutionStrategy>) -> Result<crate::dependency::types::ResolutionResult> {
        let resolver = self.get_or_create_resolver().await?;
        let deps = self.get_app_dependency_specs(app_name, include_dev)?;
        
        println!("🔍 Resolving {} dependencies for app '{}'...", deps.len(), style(app_name).green());
        
        let result = resolver.resolve_dependencies(deps).await
            .with_context(|| format!("Failed to resolve dependencies for app '{}'", app_name))?;
            
        println!("✅ Successfully resolved {} packages", result.resolved_packages.len());
        
        if !result.conflicts.is_empty() {
            println!("⚠️  {} conflicts were automatically resolved", result.conflicts.len());
        }
        
        if !result.warnings.is_empty() {
            for warning in &result.warnings {
                println!("⚠️  {}", warning);
            }
        }
        
        Ok(result)
    }

    pub async fn install_dependencies(&mut self, app_name: &str, resolution: &crate::dependency::types::ResolutionResult) -> Result<()> {
        let app_packages_dir = self.root.join("apps").join(app_name).join("knot_packages");
        
        // Clean existing packages directory
        if app_packages_dir.exists() {
            std::fs::remove_dir_all(&app_packages_dir)
                .with_context(|| format!("Failed to clean packages directory for app '{}'", app_name))?;
        }
        std::fs::create_dir_all(&app_packages_dir)
            .with_context(|| format!("Failed to create packages directory for app '{}'", app_name))?;
            
        println!("📦 Installing {} packages for app '{}'...", resolution.resolved_packages.len(), style(app_name).green());
        
        let resolver = self.get_or_create_resolver().await?;
        
        // Install packages in dependency order
        for package_id in &resolution.dependency_order {
            if let Some(package_version) = resolution.resolved_packages.get(package_id) {
                let package_dir = app_packages_dir.join(&package_id.name);
                
                print!("  📦 Installing {}@{}... ", 
                       style(&package_id.name).cyan(), 
                       style(&package_version.version).yellow());
                
                match &package_id.source {
                    crate::dependency::types::PackageSource::Local => {
                        // For local packages, create symlinks
                        if let Some(source_path) = &package_version.source_path {
                            self.create_package_link(source_path, &package_dir)
                                .with_context(|| format!("Failed to link local package '{}'", package_id.name))?;
                        }
                    }
                    crate::dependency::types::PackageSource::Remote { registry: _ } => {
                        // For remote packages, download and extract
                        // Try local registry first, then remote
                        let download_result = {
                            let local_result = resolver.get_local_registry().download_package(package_id, &package_version.version, &package_dir).await;
                            match local_result {
                                Ok(_) => Ok(()),
                                Err(_) => resolver.get_remote_registry().download_package(package_id, &package_version.version, &package_dir).await,
                            }
                        };
                        download_result.with_context(|| format!("Failed to download package '{}'", package_id.name))?;
                    }
                }
                
                println!("✅");
            }
        }
        
        println!("🎉 Successfully installed all dependencies for app '{}'", style(app_name).green());
        Ok(())
    }

    pub fn get_app_dependency_specs(&self, app_name: &str, include_dev: bool) -> Result<Vec<DependencySpec>> {
        let raw_deps = self.get_app_dependencies(app_name);
        let mut dep_specs = Vec::new();
        
        for dep_str in raw_deps {
            let dep_spec = self.parse_dependency_spec(&dep_str, false)?;
            dep_specs.push(dep_spec);
        }
        
        // Add development dependencies if requested
        if include_dev {
            if let Some(app_config) = self.apps.get(app_name) {
                // For now, we don't have dev dependencies in app config
                // but this is where we'd add them
            }
        }
        
        Ok(dep_specs)
    }

    pub async fn check_dependency_health(&mut self, app_name: &str) -> Result<DependencyHealthReport> {
        let deps = self.get_app_dependency_specs(app_name, false)?;
        let resolver = self.get_or_create_resolver().await?;
        
        let mut report = DependencyHealthReport {
            app_name: app_name.to_string(),
            total_dependencies: deps.len(),
            issues: Vec::new(),
            suggestions: Vec::new(),
            outdated_packages: Vec::new(),
        };
        
        // Try to resolve dependencies and catch issues
        match resolver.resolve_dependencies(deps.clone()).await {
            Ok(resolution) => {
                // Check for conflicts that were resolved
                for conflict in &resolution.conflicts {
                    report.issues.push(format!(
                        "Version conflict for '{}' was automatically resolved", 
                        conflict.package_id.name
                    ));
                }
                
                // Check for prerelease versions
                for (package_id, package_version) in &resolution.resolved_packages {
                    if !package_version.version.pre.is_empty() {
                        report.issues.push(format!(
                            "Package '{}' is using prerelease version {}", 
                            package_id.name, 
                            package_version.version
                        ));
                        report.suggestions.push(format!(
                            "Consider using a stable version of '{}'", 
                            package_id.name
                        ));
                    }
                }
                
                // TODO: Check for outdated packages by comparing with latest versions
            }
            Err(e) => {
                report.issues.push(format!("Dependency resolution failed: {}", e));
            }
        }
        
        Ok(report)
    }

    pub fn validate_dependency_tree(&self, app_name: &str) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        let app_deps = self.get_app_dependencies(app_name);
        
        // Check for duplicate dependencies
        let mut seen = std::collections::HashSet::new();
        for dep in &app_deps {
            let dep_name = self.extract_package_name(dep)?;
            if !seen.insert(dep_name.clone()) {
                issues.push(format!("Duplicate dependency: '{}'", dep_name));
            }
        }
        
        // Check for missing local packages
        for dep in &app_deps {
            if !dep.starts_with('@') {
                let dep_name = self.extract_package_name(dep)?;
                if !self.packages.contains_key(&dep_name) {
                    issues.push(format!(
                        "Local package '{}' not found in packages directory", 
                        dep_name
                    ));
                }
            }
        }
        
        Ok(issues)
    }

    fn parse_dependency_spec(&self, dep_str: &str, dev_only: bool) -> Result<DependencySpec> {
        let (name, version_req_str) = if let Some(at_pos) = dep_str.rfind('@') {
            let name = dep_str[..at_pos].to_string();
            let version_str = &dep_str[at_pos + 1..];
            (name, version_str)
        } else {
            (dep_str.to_string(), "*")
        };
        
        let version_req = VersionReq::parse(if version_req_str == "latest" {
            "*"
        } else {
            version_req_str
        }).with_context(|| format!("Invalid version requirement '{}' for package '{}'", version_req_str, name))?;
        
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
            optional: false,
            dev_only,
            conditions: None,
            features: None,
        })
    }

    fn extract_package_name(&self, dep_str: &str) -> Result<String> {
        let name = if let Some(at_pos) = dep_str.rfind('@') {
            dep_str[..at_pos].to_string()
        } else {
            dep_str.to_string()
        };
        
        Ok(name)
    }

    async fn get_or_create_resolver(&mut self) -> Result<&mut DependencyResolver> {
        if self.dependency_resolver.is_none() {
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
            
            let mut local_registry = LocalPackageRegistry::new(self.root.join("packages"));
            local_registry.discover_packages().await
                .context("Failed to discover local packages")?;
            
            let remote_registry = RemotePackageRegistry::new("https://knot.space".to_string());
            let cache_dir = self.root.join(".knot").join("cache");
            
            self.dependency_resolver = Some(DependencyResolver::new(
                context,
                local_registry,
                remote_registry,
                cache_dir,
            ));
        }
        
        Ok(self.dependency_resolver.as_mut().unwrap())
    }

    fn create_package_link(&self, source_path: &Path, target_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
        
        // Remove existing link/directory if it exists
        if target_path.exists() {
            if target_path.is_dir() {
                std::fs::remove_dir_all(target_path)
            } else {
                std::fs::remove_file(target_path)
            }.with_context(|| format!("Failed to remove existing path {}", target_path.display()))?;
        }
        
        // Create symlink on Unix systems
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source_path, target_path)
                .with_context(|| format!(
                    "Failed to create symlink from {} to {}", 
                    source_path.display(), 
                    target_path.display()
                ))?;
        }
        
        // Copy directory on Windows and other systems
        #[cfg(not(unix))]
        {
            self.copy_dir_recursive(source_path, target_path)
                .with_context(|| format!(
                    "Failed to copy from {} to {}", 
                    source_path.display(), 
                    target_path.display()
                ))?;
        }
        
        Ok(())
    }

    #[cfg(not(unix))]
    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::fs;
        
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct DependencyHealthReport {
    pub app_name: String,
    pub total_dependencies: usize,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
    pub outdated_packages: Vec<String>,
}

impl DependencyHealthReport {
    pub fn is_healthy(&self) -> bool {
        self.issues.is_empty()
    }
    
    pub fn print_summary(&self) {
        println!("\n📊 Dependency Health Report for app '{}':", style(&self.app_name).green());
        println!("  Total dependencies: {}", self.total_dependencies);
        
        if self.issues.is_empty() {
            println!("  ✅ No issues found");
        } else {
            println!("  ❌ {} issue(s) found:", self.issues.len());
            for issue in &self.issues {
                println!("    • {}", issue);
            }
        }
        
        if !self.suggestions.is_empty() {
            println!("  💡 Suggestions:");
            for suggestion in &self.suggestions {
                println!("    • {}", suggestion);
            }
        }
        
        if !self.outdated_packages.is_empty() {
            println!("  📅 Outdated packages:");
            for package in &self.outdated_packages {
                println!("    • {}", package);
            }
        }
    }
}
