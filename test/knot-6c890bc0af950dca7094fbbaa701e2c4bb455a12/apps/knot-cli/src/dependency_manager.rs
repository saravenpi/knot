use crate::project::Project;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct DependencyManager<'a> {
    project: &'a Project,
}

impl<'a> DependencyManager<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    /// Install dependencies for a given package in a target directory
    pub async fn install_package_dependencies(
        &self,
        package_name: &str,
        target_dir: &Path,
    ) -> Result<()> {
        let dependencies = self.project.get_package_dependencies(package_name);
        if dependencies.is_empty() {
            return Ok(());
        }

        println!("📦 Installing dependencies for package '{}'", package_name);

        let resolved_deps = self.project.resolve_all_dependencies(&dependencies)
            .with_context(|| format!("Failed to resolve dependencies for package '{}'", package_name))?;

        for dep in &resolved_deps {
            self.install_single_dependency(dep, target_dir).await
                .with_context(|| format!("Failed to install dependency '{}' for package '{}'", dep, package_name))?;
        }

        println!("✅ Successfully installed {} dependencies for package '{}'", 
                 resolved_deps.len(), package_name);
        Ok(())
    }

    /// Install dependencies for all packages linked to an app
    pub async fn install_app_package_dependencies(
        &self,
        app_name: &str,
        app_dir: &Path,
    ) -> Result<()> {
        let app_dependencies = self.project.get_app_dependencies(app_name);
        let local_packages: Vec<String> = app_dependencies.iter()
            .filter(|dep| !dep.starts_with('@'))
            .cloned()
            .collect();

        if local_packages.is_empty() {
            return Ok(());
        }

        println!("🔧 Installing online dependencies for app '{}'", app_name);

        // Collect all dependencies from all local packages
        let mut all_online_deps = HashSet::new();
        for package in &local_packages {
            let package_deps = self.project.get_package_dependencies(package);
            let resolved_deps = self.project.resolve_all_dependencies(&package_deps)
                .with_context(|| format!("Failed to resolve dependencies for package '{}'", package))?;
            
            // Only add online dependencies
            for dep in resolved_deps {
                if dep.starts_with('@') {
                    all_online_deps.insert(dep);
                }
            }
        }

        if all_online_deps.is_empty() {
            println!("No online dependencies to install for app '{}'", app_name);
            return Ok(());
        }

        // Install to node_modules in the app directory
        let node_modules_dir = app_dir.join("node_modules");
        fs::create_dir_all(&node_modules_dir)
            .with_context(|| format!("Failed to create node_modules directory for app '{}'", app_name))?;

        for dep in &all_online_deps {
            self.install_npm_dependency(dep, app_dir).await
                .with_context(|| format!("Failed to install npm dependency '{}' for app '{}'", dep, app_name))?;
        }

        println!("✅ Successfully installed {} online dependencies for app '{}'", 
                 all_online_deps.len(), app_name);
        Ok(())
    }

    async fn install_single_dependency(&self, dependency: &str, target_dir: &Path) -> Result<()> {
        if dependency.starts_with('@') {
            // This is an online dependency, install via npm
            self.install_npm_dependency(dependency, target_dir).await
        } else {
            // This is a local knot package, no need to install separately
            // as it will be linked by the linker
            Ok(())
        }
    }

    async fn install_npm_dependency(&self, dependency: &str, project_dir: &Path) -> Result<()> {
        println!("📦 Installing npm dependency: {}", dependency);

        // Use npm to install the dependency
        let output = Command::new("npm")
            .arg("install")
            .arg(dependency)
            .arg("--save")
            .current_dir(project_dir)
            .output()
            .with_context(|| format!("Failed to execute npm install for dependency '{}'", dependency))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            anyhow::bail!(
                "npm install failed for dependency '{}'\nSTDOUT: {}\nSTDERR: {}",
                dependency, stdout, stderr
            );
        }

        println!("✅ Successfully installed npm dependency: {}", dependency);
        Ok(())
    }

    /// Check if a package.json exists and create one if not
    pub fn ensure_package_json(&self, app_dir: &Path, app_name: &str) -> Result<()> {
        let package_json_path = app_dir.join("package.json");
        
        if !package_json_path.exists() {
            println!("📄 Creating package.json for app '{}'", app_name);
            
            let package_json_content = serde_json::json!({
                "name": app_name,
                "version": "1.0.0",
                "description": format!("Generated package.json for Knot app '{}'", app_name),
                "main": "index.js",
                "private": true,
                "dependencies": {}
            });

            fs::write(&package_json_path, serde_json::to_string_pretty(&package_json_content)?)
                .with_context(|| format!("Failed to create package.json for app '{}'", app_name))?;
            
            println!("✅ Created package.json for app '{}'", app_name);
        }

        Ok(())
    }

    /// Validate that required tools (npm) are available
    pub fn validate_prerequisites(&self) -> Result<()> {
        // Check if npm is available
        let output = Command::new("npm")
            .arg("--version")
            .output()
            .context("Failed to check npm version. Make sure npm is installed and available in PATH")?;

        if !output.status.success() {
            anyhow::bail!("npm is not available or not working properly. Please ensure npm is installed.");
        }

        let npm_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("✅ npm version {} detected", npm_version);

        Ok(())
    }

    /// Get a summary of dependencies that would be installed
    pub fn get_dependency_summary(&self, app_name: &str) -> Result<DependencySummary> {
        let app_dependencies = self.project.get_app_dependencies(app_name);
        let local_packages: Vec<String> = app_dependencies.iter()
            .filter(|dep| !dep.starts_with('@'))
            .cloned()
            .collect();
        
        let online_packages: Vec<String> = app_dependencies.iter()
            .filter(|dep| dep.starts_with('@'))
            .cloned()
            .collect();

        let mut indirect_online_deps = HashSet::new();
        for package in &local_packages {
            let package_deps = self.project.get_package_dependencies(package);
            let resolved_deps = self.project.resolve_all_dependencies(&package_deps)?;
            
            for dep in resolved_deps {
                if dep.starts_with('@') {
                    indirect_online_deps.insert(dep);
                }
            }
        }

        Ok(DependencySummary {
            local_packages,
            direct_online_packages: online_packages,
            indirect_online_packages: indirect_online_deps.into_iter().collect(),
        })
    }
}

#[derive(Debug)]
pub struct DependencySummary {
    pub local_packages: Vec<String>,
    pub direct_online_packages: Vec<String>,
    pub indirect_online_packages: Vec<String>,
}

impl DependencySummary {
    pub fn total_packages(&self) -> usize {
        self.local_packages.len() + self.direct_online_packages.len() + self.indirect_online_packages.len()
    }

    pub fn has_online_dependencies(&self) -> bool {
        !self.direct_online_packages.is_empty() || !self.indirect_online_packages.is_empty()
    }
}