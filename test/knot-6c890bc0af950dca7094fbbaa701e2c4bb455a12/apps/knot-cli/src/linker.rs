use crate::dependency_manager::DependencyManager;
use crate::downloader::PackageDownloader;
use crate::project::Project;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct Linker<'a> {
    project: &'a Project,
}

impl<'a> Linker<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    pub async fn link_all_apps(&self, use_symlinks: bool) -> Result<()> {
        let app_names = self.project.get_app_names();

        for app_name in app_names {
            self.link_app(&app_name, use_symlinks).await?;
        }

        Ok(())
    }

    pub async fn link_app(&self, app_name: &str, use_symlinks: bool) -> Result<()> {
        let app_dir = self.project.root.join("apps").join(app_name);
        if !app_dir.exists() {
            anyhow::bail!("App directory does not exist: {}", app_name);
        }

        let knot_packages_dir = app_dir.join("knot_packages");

        if knot_packages_dir.exists() {
            fs::remove_dir_all(&knot_packages_dir).with_context(|| {
                format!(
                    "Failed to remove existing knot_packages directory for app {}",
                    app_name
                )
            })?;
        }

        fs::create_dir_all(&knot_packages_dir).with_context(|| {
            format!(
                "Failed to create knot_packages directory for app {}",
                app_name
            )
        })?;

        // Initialize dependency manager
        let dep_manager = DependencyManager::new(self.project);

        // Validate prerequisites before starting
        dep_manager.validate_prerequisites().with_context(|| {
            "Failed to validate prerequisites for dependency installation"
        })?;

        // Ensure package.json exists for npm dependency installation
        dep_manager.ensure_package_json(&app_dir, app_name).with_context(|| {
            format!("Failed to ensure package.json exists for app '{}'", app_name)
        })?;

        // Get dependency summary
        let dep_summary = dep_manager.get_dependency_summary(app_name).with_context(|| {
            format!("Failed to analyze dependencies for app '{}'", app_name)
        })?;

        println!("📊 Dependency summary for app '{}':", app_name);
        println!("  • Local packages: {}", dep_summary.local_packages.len());
        println!("  • Direct online dependencies: {}", dep_summary.direct_online_packages.len());
        println!("  • Indirect online dependencies: {}", dep_summary.indirect_online_packages.len());
        println!("  • Total packages: {}", dep_summary.total_packages());

        if dep_summary.has_online_dependencies() {
            println!("🌐 Installing online dependencies...");
        }

        let package_entries = self.project.get_app_package_entries(app_name)
            .with_context(|| format!("Failed to get package entries for app '{}'", app_name))?;
        
        // Debug: Print package entries
        println!("DEBUG: Package entries for app '{}': {:?}", app_name, package_entries);

        // Show alias information if any packages have aliases
        let aliased_packages: Vec<_> = package_entries.iter()
            .filter(|entry| entry.alias.is_some())
            .collect();
            
        if !aliased_packages.is_empty() {
            println!("🔗 Package aliases:");
            for entry in &aliased_packages {
                println!("  • {} → {}", entry.name, entry.alias.as_ref().unwrap());
            }
        }

        // Link all dependencies (both local and online)
        for package_entry in &package_entries {
            let display_name = package_entry.alias.as_ref().unwrap_or(&package_entry.name);
            self.link_package_entry(app_name, package_entry, &knot_packages_dir, use_symlinks)
                .await.with_context(|| {
                    format!("Failed to link package '{}' as '{}' for app '{}'", 
                           package_entry.name, display_name, app_name)
                })?;
        }

        // Install online dependencies for local packages
        dep_manager.install_app_package_dependencies(app_name, &app_dir).await
            .with_context(|| {
                format!("Failed to install online dependencies for app '{}'", app_name)
            })?;

        // Install dependencies for each linked local package
        for package_entry in &package_entries {
            if !package_entry.name.starts_with('@') {
                // This is a local package, install its online dependencies
                dep_manager.install_package_dependencies(&package_entry.name, &app_dir).await
                    .with_context(|| {
                        format!("Failed to install dependencies for local package '{}' in app '{}'", package_entry.name, app_name)
                    })?;
            }
        }

        let linked_count = self.count_linked_packages(&knot_packages_dir)?;
        println!(
            "✅ Successfully linked {} dependencies for app '{}'",
            linked_count,
            app_name
        );

        if dep_summary.has_online_dependencies() {
            println!("🔗 All online dependencies have been installed via npm");
        }

        Ok(())
    }

    async fn link_dependency(
        &self,
        app_name: &str,
        dependency: &str,
        knot_packages_dir: &Path,
        use_symlinks: bool,
    ) -> Result<()> {
        if let Some(folder_name) = dependency.strip_prefix('@') {
            // Remove @ prefix for folder name, but keep it for package identification
            let link_target = knot_packages_dir.join(folder_name);
            PackageDownloader::download_package(dependency, &link_target)
                .await
                .with_context(|| {
                    format!(
                        "Failed to download online package '{}' for app '{}'",
                        dependency, app_name
                    )
                })?;
            return Ok(());
        }

        let package_source = self.project.root.join("packages").join(dependency);
        if !package_source.exists() {
            anyhow::bail!(
                "Local package '{}' does not exist in packages/ directory",
                dependency
            );
        }

        let link_target = knot_packages_dir.join(dependency);

        if use_symlinks {
            self.create_symlink(&package_source, &link_target)
                .with_context(|| {
                    format!(
                        "Failed to create symlink for package '{}' in app '{}'",
                        dependency, app_name
                    )
                })?;
        } else {
            self.copy_package(&package_source, &link_target)
                .with_context(|| {
                    format!(
                        "Failed to copy package '{}' to app '{}'",
                        dependency, app_name
                    )
                })?;
        }

        Ok(())
    }

    async fn link_package_entry(
        &self,
        app_name: &str,
        package_entry: &crate::config::PackageEntry,
        knot_packages_dir: &Path,
        use_symlinks: bool,
    ) -> Result<()> {
        let target_name = package_entry.alias.as_ref().unwrap_or(&package_entry.name);
        
        if let Some(_folder_name) = package_entry.name.strip_prefix('@') {
            // Online package - remove @ prefix for folder name, but use alias if provided
            let link_target = knot_packages_dir.join(target_name);
            PackageDownloader::download_package(&package_entry.name, &link_target)
                .await
                .with_context(|| {
                    format!(
                        "Failed to download online package '{}' as '{}' for app '{}'",
                        package_entry.name, target_name, app_name
                    )
                })?;
            return Ok(());
        }

        // Local package - source is based on actual package name
        let package_source = self.project.root.join("packages").join(&package_entry.name);
        if !package_source.exists() {
            anyhow::bail!(
                "Local package '{}' does not exist in packages/ directory",
                package_entry.name
            );
        }

        // Target uses the alias name if provided
        let link_target = knot_packages_dir.join(target_name);

        if use_symlinks {
            self.create_symlink(&package_source, &link_target)
                .with_context(|| {
                    format!(
                        "Failed to create symlink for package '{}' as '{}' in app '{}'",
                        package_entry.name, target_name, app_name
                    )
                })?;
        } else {
            self.copy_package(&package_source, &link_target)
                .with_context(|| {
                    format!(
                        "Failed to copy package '{}' as '{}' to app '{}'",
                        package_entry.name, target_name, app_name
                    )
                })?;
        }

        Ok(())
    }

    #[cfg(unix)]
    fn create_symlink(&self, source: &Path, target: &Path) -> Result<()> {
        std::os::unix::fs::symlink(source, target)?;
        Ok(())
    }

    fn copy_package(&self, source: &Path, target: &Path) -> Result<()> {
        Self::copy_dir_recursively(source, target)?;
        Ok(())
    }

    fn copy_dir_recursively(source: &Path, target: &Path) -> Result<()> {
        if !source.exists() {
            anyhow::bail!("Source directory does not exist: {}", source.display());
        }

        if source.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(source, target)?;
            return Ok(());
        }

        fs::create_dir_all(target)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let target_path = target.join(entry.file_name());

            if source_path.is_dir() {
                Self::copy_dir_recursively(&source_path, &target_path)?;
            } else {
                fs::copy(&source_path, &target_path)?;
            }
        }

        Ok(())
    }

    fn count_linked_packages(&self, knot_packages_dir: &Path) -> Result<usize> {
        if !knot_packages_dir.exists() {
            return Ok(0);
        }

        let count = fs::read_dir(knot_packages_dir)?
            .filter_map(|entry| entry.ok())
            .count();

        Ok(count)
    }
}
