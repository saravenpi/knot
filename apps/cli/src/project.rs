use crate::config::{AppConfig, KnotConfig, PackageConfig};
use crate::utils;
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
        let config_path = utils::find_yaml_file(&project_root, "knot")
            .ok_or_else(|| anyhow::anyhow!("No knot.yml or knot.yaml file found in {:?}", project_root))?;
        let config = KnotConfig::from_file(&config_path)
            .with_context(|| format!("Failed to load config from {:?}", config_path))?;

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
}
