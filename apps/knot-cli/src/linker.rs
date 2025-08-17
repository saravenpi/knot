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

        let dependencies = self.project.get_app_dependencies(app_name);

        for dep in dependencies {
            self.link_dependency(app_name, &dep, &knot_packages_dir, use_symlinks)
                .await?;
        }

        println!(
            "Linked {} dependencies for app '{}'",
            self.count_linked_packages(&knot_packages_dir)?,
            app_name
        );

        Ok(())
    }

    async fn link_dependency(
        &self,
        app_name: &str,
        dependency: &str,
        knot_packages_dir: &Path,
        use_symlinks: bool,
    ) -> Result<()> {
        if dependency.starts_with('@') {
            // Remove @ prefix for folder name, but keep it for package identification
            let folder_name = &dependency[1..];
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

    #[cfg(unix)]
    fn create_symlink(&self, source: &Path, target: &Path) -> Result<()> {
        std::os::unix::fs::symlink(source, target)?;
        Ok(())
    }

    #[cfg(windows)]
    fn create_symlink(&self, source: &Path, target: &Path) -> Result<()> {
        if source.is_dir() {
            std::os::windows::fs::symlink_dir(source, target)?;
        } else {
            std::os::windows::fs::symlink_file(source, target)?;
        }
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
