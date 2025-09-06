use std::collections::HashMap;
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use semver::Version;
use serde::Deserialize;

use crate::config::PackageConfig;
use crate::dependency::types::{PackageId, PackageVersion, PackageSource, PackageMetadata, DependencySpec};
use crate::dependency::error::{ResolutionError, ResolutionResult};

#[derive(Deserialize)]
struct RemoteDependency {
    name: String,
    version: String,
    optional: Option<bool>,
}

#[async_trait]
pub trait PackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>>;
    async fn get_package_metadata(&self, package_id: &PackageId, version: &Version) -> ResolutionResult<PackageMetadata>;
    async fn download_package(&self, package_id: &PackageId, version: &Version, destination: &Path) -> ResolutionResult<()>;
    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>>;
}

pub struct LocalPackageRegistry {
    packages_path: PathBuf,
    cache: HashMap<PackageId, Vec<PackageVersion>>,
}

impl LocalPackageRegistry {
    pub fn new(packages_path: PathBuf) -> Self {
        Self {
            packages_path,
            cache: HashMap::new(),
        }
    }

    pub async fn discover_packages(&mut self) -> ResolutionResult<()> {
        self.cache.clear();
        
        if !self.packages_path.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.packages_path)
            .map_err(|e| ResolutionError::io_error(
                "reading packages directory",
                Some(self.packages_path.to_string_lossy().into_owned()),
                e.to_string()
            ))?;

        for entry in entries {
            let entry = entry.map_err(|e| ResolutionError::io_error(
                "reading directory entry",
                Some(self.packages_path.to_string_lossy().into_owned()),
                e.to_string()
            ))?;

            if entry.path().is_dir() {
                if let Ok(package_version) = self.load_local_package(&entry.path()).await {
                    let package_id = package_version.id.clone();
                    self.cache.entry(package_id).or_default().push(package_version);
                }
            }
        }

        Ok(())
    }

    async fn load_local_package(&self, package_path: &Path) -> ResolutionResult<PackageVersion> {
        let package_yml = package_path.join("package.yml");
        
        if !package_yml.exists() {
            return Err(ResolutionError::configuration_error(
                format!("package.yml not found in {}", package_path.display()),
                None
            ));
        }

        let config = PackageConfig::from_file(&package_yml)
            .map_err(|e| ResolutionError::configuration_error(
                format!("Failed to load package configuration: {}", e),
                Some("package.yml".to_string())
            ))?;

        let package_id = PackageId {
            name: config.name.clone(),
            source: PackageSource::Local,
        };

        let version = Version::parse(&config.version)
            .map_err(|e| ResolutionError::invalid_version(
                package_id.clone(),
                config.version.clone(),
                e.to_string()
            ))?;

        // Parse dependencies from enhanced package config
        let dependencies = self.parse_dependencies(&config.dependencies.unwrap_or_default(), false)?;
        let dev_dependencies = self.parse_dependencies(&config.dev_dependencies.unwrap_or_default(), true)?;
        let optional_dependencies = self.parse_dependencies(&config.optional_dependencies.unwrap_or_default(), false)?;
        let peer_dependencies = self.parse_dependencies(&config.peer_dependencies.unwrap_or_default(), false)?;

        let metadata = Some(PackageMetadata {
            name: config.name.clone(),
            version: config.version.clone(),
            description: config.description.clone(),
            author: config.author.clone(),
            license: config.license.clone(),
            repository: config.repository.clone(),
            keywords: config.keywords.clone(),
            exports: config.exports.clone(),
            features: config.features.clone(),
            checksum: None, // Local packages don't need checksums
        });

        Ok(PackageVersion {
            id: package_id,
            version,
            dependencies,
            dev_dependencies,
            optional_dependencies,
            peer_dependencies,
            source_path: Some(package_path.to_path_buf()),
            metadata,
        })
    }

    fn parse_dependencies(&self, deps: &[String], is_dev: bool) -> ResolutionResult<Vec<DependencySpec>> {
        let mut dependency_specs = Vec::new();

        for dep_str in deps {
            let dependency_spec = self.parse_dependency_spec(dep_str, is_dev)?;
            dependency_specs.push(dependency_spec);
        }

        Ok(dependency_specs)
    }

    fn parse_dependency_spec(&self, dep_str: &str, is_dev: bool) -> ResolutionResult<DependencySpec> {
        // Parse dependency string: "name" or "name@version" or "@remote/name@version"
        let (name, version_req_str) = if let Some(at_pos) = dep_str.rfind('@') {
            let name = dep_str[..at_pos].to_string();
            let version_str = &dep_str[at_pos + 1..];
            (name, version_str)
        } else {
            (dep_str.to_string(), "*")
        };

        let version_req = semver::VersionReq::parse(if version_req_str == "latest" {
            "*"
        } else {
            version_req_str
        }).map_err(|e| ResolutionError::invalid_version(
            PackageId::local(&name),
            version_req_str.to_string(),
            e.to_string()
        ))?;

        let source = if name.starts_with('@') {
            PackageSource::Remote {
                registry: "knot-space".to_string(),
            }
        } else {
            PackageSource::Local
        };

        Ok(DependencySpec {
            id: PackageId { name, source },
            version_req,
            optional: false,
            dev_only: is_dev,
            conditions: None,
            features: None,
        })
    }
}

#[async_trait]
impl PackageRegistry for LocalPackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        if let Some(versions) = self.cache.get(package_id) {
            Ok(versions.clone())
        } else {
            Ok(Vec::new())
        }
    }

    async fn get_package_metadata(&self, package_id: &PackageId, version: &Version) -> ResolutionResult<PackageMetadata> {
        let versions = self.list_versions(package_id).await?;
        
        let package_version = versions.iter()
            .find(|v| v.version == *version)
            .ok_or_else(|| ResolutionError::package_not_found(
                package_id.clone(),
                vec![format!("Local packages directory: {}", self.packages_path.display())],
                vec![]
            ))?;

        package_version.metadata.clone().ok_or_else(|| ResolutionError::configuration_error(
            "Package metadata not available",
            None
        ))
    }

    async fn download_package(&self, package_id: &PackageId, _version: &Version, destination: &Path) -> ResolutionResult<()> {
        let versions = self.list_versions(package_id).await?;
        let package_version = versions.first().ok_or_else(|| ResolutionError::package_not_found(
            package_id.clone(),
            vec![format!("Local packages directory: {}", self.packages_path.display())],
            vec![]
        ))?;

        let source_path = package_version.source_path.as_ref().ok_or_else(|| ResolutionError::configuration_error(
            "No source path for local package",
            None
        ))?;

        // For local packages, we can symlink or copy
        if destination.exists() {
            std::fs::remove_dir_all(destination)?;
        }

        // Create a symlink for local packages (faster than copying)
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source_path, destination)
                .map_err(|e| ResolutionError::io_error(
                    "creating symlink",
                    Some(destination.to_string_lossy().into_owned()),
                    e.to_string()
                ))?;
        }

        #[cfg(not(unix))]
        {
            // Fallback to copying on non-Unix systems
            copy_dir_recursive(source_path, destination)?;
        }

        Ok(())
    }

    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>> {
        let mut matches = Vec::new();
        
        for package_id in self.cache.keys() {
            if package_id.name.contains(query) {
                matches.push(package_id.name.clone());
            }
        }

        matches.sort();
        Ok(matches)
    }
}

pub struct RemotePackageRegistry {
    base_url: String,
    client: reqwest::Client,
    cache: HashMap<PackageId, Vec<PackageVersion>>,
    auth_token: Option<String>,
}

impl RemotePackageRegistry {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
            cache: HashMap::new(),
            auth_token: None,
        }
    }

    pub fn with_auth(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut request = self.client.get(url);
        
        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        request
    }
}

#[async_trait]
impl PackageRegistry for RemotePackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        if let Some(cached) = self.cache.get(package_id) {
            return Ok(cached.clone());
        }

        let package_name = package_id.name.trim_start_matches('@');
        let url = format!("{}/api/packages/{}/versions", self.base_url, package_name);
        
        let response = self.build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))?;

        if !response.status().is_success() {
            return Ok(Vec::new()); // Package not found
        }

        #[derive(Deserialize)]
        struct VersionsResponse {
            data: Vec<RemotePackageVersion>,
        }

        #[derive(Deserialize)]
        struct RemotePackageVersion {
            version: String,
            dependencies: Option<Vec<RemoteDependency>>,
            dev_dependencies: Option<Vec<RemoteDependency>>,
            metadata: Option<serde_json::Value>,
        }


        let versions_data: VersionsResponse = response.json().await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), 
                format!("Failed to parse response: {}", e)))?;

        let mut versions = Vec::new();
        for version_data in versions_data.data {
            let version = Version::parse(&version_data.version)
                .map_err(|e| ResolutionError::invalid_version(
                    package_id.clone(),
                    version_data.version,
                    e.to_string()
                ))?;

            let dependencies = self.parse_remote_dependencies(&version_data.dependencies.unwrap_or_default(), false)?;
            let dev_dependencies = self.parse_remote_dependencies(&version_data.dev_dependencies.unwrap_or_default(), true)?;

            versions.push(PackageVersion {
                id: package_id.clone(),
                version,
                dependencies,
                dev_dependencies,
                optional_dependencies: Vec::new(),
                peer_dependencies: Vec::new(),
                source_path: None,
                metadata: version_data.metadata.and_then(|m| serde_json::from_value(m).ok()),
            });
        }

        Ok(versions)
    }

    async fn get_package_metadata(&self, package_id: &PackageId, version: &Version) -> ResolutionResult<PackageMetadata> {
        let package_name = package_id.name.trim_start_matches('@');
        let url = format!("{}/api/packages/{}/{}", self.base_url, package_name, version);
        
        let response = self.build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))?;

        if !response.status().is_success() {
            return Err(ResolutionError::package_not_found(
                package_id.clone(),
                vec![format!("Remote registry: {}", self.base_url)],
                vec![]
            ));
        }

        let metadata: PackageMetadata = response.json().await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), 
                format!("Failed to parse metadata: {}", e)))?;

        Ok(metadata)
    }

    async fn download_package(&self, package_id: &PackageId, version: &Version, destination: &Path) -> ResolutionResult<()> {
        let package_name = package_id.name.trim_start_matches('@');
        let url = format!("{}/api/packages/{}/{}/download", self.base_url, package_name, version);
        
        let response = self.build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))?;

        if !response.status().is_success() {
            return Err(ResolutionError::network_error(package_id.clone(), 
                format!("Download failed with status: {}", response.status())));
        }

        let bytes = response.bytes().await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), 
                format!("Failed to download package data: {}", e)))?;

        // Extract the package (assuming it's a tar.gz)
        extract_package(&bytes, destination)
            .map_err(|e| ResolutionError::io_error(
                "extracting package",
                Some(destination.to_string_lossy().into_owned()),
                e.to_string()
            ))?;

        Ok(())
    }

    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>> {
        let url = format!("{}/api/search?q={}", self.base_url, urlencoding::encode(query));
        
        let response = self.build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(
                PackageId::local("search"),
                e.to_string()
            ))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            packages: Vec<SearchResult>,
        }

        #[derive(Deserialize)]
        struct SearchResult {
            name: String,
        }

        let search_data: SearchResponse = response.json().await
            .map_err(|e| ResolutionError::network_error(
                PackageId::local("search"),
                format!("Failed to parse search response: {}", e)
            ))?;

        Ok(search_data.packages.into_iter().map(|p| p.name).collect())
    }
}

impl RemotePackageRegistry {
    fn parse_remote_dependencies(&self, deps: &[RemoteDependency], is_dev: bool) -> ResolutionResult<Vec<DependencySpec>> {
        let mut dependency_specs = Vec::new();

        for dep in deps {
            let version_req = semver::VersionReq::parse(&dep.version)
                .map_err(|e| ResolutionError::invalid_version(
                    PackageId::remote(&dep.name, "knot-space"),
                    dep.version.clone(),
                    e.to_string()
                ))?;

            let source = if dep.name.starts_with('@') {
                PackageSource::Remote {
                    registry: "knot-space".to_string(),
                }
            } else {
                PackageSource::Local
            };

            dependency_specs.push(DependencySpec {
                id: PackageId { name: dep.name.clone(), source },
                version_req,
                optional: dep.optional.unwrap_or(false),
                dev_only: is_dev,
                conditions: None,
                features: None,
            });
        }

        Ok(dependency_specs)
    }
}

// Helper function to copy directories recursively
#[cfg(not(unix))]
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    use std::fs;
    
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

// Helper function to extract packages
fn extract_package(data: &[u8], destination: &Path) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Cursor;
    
    // Create destination directory
    std::fs::create_dir_all(destination)?;
    
    // For now, assume the package is just a JSON manifest + files
    // In a real implementation, you'd handle tar.gz extraction
    let manifest_path = destination.join("package.json");
    let mut file = File::create(manifest_path)?;
    std::io::copy(&mut Cursor::new(data), &mut file)?;
    
    Ok(())
}