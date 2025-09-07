use crate::config::{AppConfig, KnotConfig, PackageConfig};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Project {
    pub root: PathBuf,
    pub config: KnotConfig,
    pub packages: HashMap<String, PackageConfig>,
    pub apps: HashMap<String, AppConfig>,
}

impl Project {
    pub fn find_and_load(start_dir: &Path) -> Result<Project> {
        let project_root = Self::find_project_root(start_dir)?;
        let config_path = project_root.join("knot.yml");
        let config = KnotConfig::from_file(&config_path)
            .with_context(|| format!("Failed to load knot.yml from {:?}", config_path))?;

        let mut project = Project {
            root: project_root,
            config,
            packages: HashMap::new(),
            apps: HashMap::new(),
        };

        project.load_packages()?;
        project.load_apps()?;

        Ok(project)
    }

    pub fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            let knot_yml = current.join("knot.yml");
            if knot_yml.exists() {
                // Only canonicalize when we find the file to get the clean path
                return match current.canonicalize() {
                    Ok(canonical) => Ok(canonical),
                    Err(_) => Ok(current), // Fall back to non-canonical path
                };
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => anyhow::bail!("No knot.yml file found in directory tree"),
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
                let package_yml = path.join("package.yml");
                if package_yml.exists() {
                    let package_config = PackageConfig::from_file(&package_yml)
                        .with_context(|| format!("Failed to load {:?}", package_yml))?;

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
                let app_yml = path.join("app.yml");
                if app_yml.exists() {
                    let app_config = AppConfig::from_file(&app_yml)
                        .with_context(|| format!("Failed to load {:?}", app_yml))?;

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
                dependencies.extend(packages.iter().map(|pkg| pkg.get_name().to_string()));
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

    pub fn get_app_package_entries(&self, app_name: &str) -> Result<Vec<crate::config::PackageEntry>, anyhow::Error> {
        let mut package_entries = Vec::new();

        // Get packages from knot.yml app dependencies
        if let Some(app_deps) = self
            .config
            .apps
            .as_ref()
            .and_then(|apps| apps.get(app_name))
        {
            package_entries.extend(app_deps.get_package_entries());
        }

        // Get packages from app.yml
        if let Some(app_config) = self.apps.get(app_name) {
            if let Some(packages) = &app_config.packages {
                package_entries.extend(packages.iter().map(|pkg| pkg.to_package_entry()));
            }
        }

        // Remove duplicates, preferring entries with aliases
        let mut unique_entries: std::collections::HashMap<String, crate::config::PackageEntry> = std::collections::HashMap::new();
        for entry in package_entries {
            if let Some(existing) = unique_entries.get(&entry.name) {
                // Keep the entry that has an alias, or the first one if both have/don't have aliases
                if existing.alias.is_none() && entry.alias.is_some() {
                    unique_entries.insert(entry.name.clone(), entry);
                }
            } else {
                unique_entries.insert(entry.name.clone(), entry);
            }
        }

        let mut result: Vec<_> = unique_entries.into_values().collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));
        
        // Check for alias conflicts
        self.validate_alias_conflicts(&result)?;
        
        Ok(result)
    }

    fn validate_alias_conflicts(&self, package_entries: &[crate::config::PackageEntry]) -> Result<(), anyhow::Error> {
        let mut target_names = std::collections::HashMap::new();
        
        for entry in package_entries {
            let target_name = entry.alias.as_ref().unwrap_or(&entry.name);
            
            if let Some(existing_package) = target_names.get(target_name) {
                if *existing_package != &entry.name {
                    anyhow::bail!(
                        "Alias conflict detected: both '{}' and '{}' would be linked as '{}'",
                        existing_package, entry.name, target_name
                    );
                }
            } else {
                target_names.insert(target_name, &entry.name);
            }
        }
        
        Ok(())
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

    pub fn get_package_dependencies(&self, package_name: &str) -> Vec<String> {
        if let Some(package_config) = self.packages.get(package_name) {
            package_config.dependencies.clone().unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    pub fn resolve_all_dependencies(&self, initial_deps: &[String]) -> Result<Vec<String>, anyhow::Error> {
        use std::collections::{HashMap, HashSet, VecDeque};
        
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from(initial_deps.to_vec());
        let mut dependency_versions: HashMap<String, String> = HashMap::new();

        while let Some(dependency) = queue.pop_front() {
            if visited.contains(&dependency) {
                continue;
            }

            visited.insert(dependency.clone());
            
            // Handle version conflicts for online dependencies
            if dependency.starts_with('@') {
                let (dep_name, version) = self.parse_dependency_spec(&dependency);
                
                if let Some(existing_version) = dependency_versions.get(&dep_name) {
                    if let Some(current_version) = &version {
                        if existing_version != current_version {
                            eprintln!(
                                "Warning: Version conflict for {}: {} vs {}. Using {}",
                                dep_name, existing_version, current_version, existing_version
                            );
                        }
                    }
                } else if let Some(version) = version {
                    dependency_versions.insert(dep_name.clone(), version);
                }
            }

            resolved.push(dependency.clone());

            // If it's a local package, get its dependencies recursively
            if !dependency.starts_with('@') {
                let package_deps = self.get_package_dependencies(&dependency);
                for dep in package_deps {
                    if !visited.contains(&dep) {
                        queue.push_back(dep);
                    }
                }
            }
        }

        Ok(resolved)
    }

    fn parse_dependency_spec(&self, dependency: &str) -> (String, Option<String>) {
        if dependency.starts_with('@') {
            // Handle scoped packages like @hono-modules-loader@0.2.5
            if let Some(at_pos) = dependency[1..].find('@') {
                // Found a second @ after the first one
                let at_pos = at_pos + 1; // Adjust for the skipped first character
                let name = dependency[..at_pos].to_string();
                let version = dependency[at_pos + 1..].to_string();
                (name, Some(version))
            } else {
                // No version specified, just the scoped package name
                (dependency.to_string(), None)
            }
        } else {
            // Handle regular packages like package-name@1.0.0
            if let Some(at_pos) = dependency.rfind('@') {
                let name = dependency[..at_pos].to_string();
                let version = dependency[at_pos + 1..].to_string();
                (name, Some(version))
            } else {
                (dependency.to_string(), None)
            }
        }
    }
}
