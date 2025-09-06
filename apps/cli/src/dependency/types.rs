use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageId {
    pub name: String,
    pub source: PackageSource,
}

impl PackageId {
    pub fn local(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: PackageSource::Local,
        }
    }

    pub fn remote(name: impl Into<String>, registry: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: PackageSource::Remote {
                registry: registry.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageSource {
    Local,
    Remote { registry: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub id: PackageId,
    pub version: Version,
    pub dependencies: Vec<DependencySpec>,
    pub dev_dependencies: Vec<DependencySpec>,
    pub optional_dependencies: Vec<DependencySpec>,
    pub peer_dependencies: Vec<DependencySpec>,
    pub source_path: Option<PathBuf>,
    pub metadata: Option<PackageMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySpec {
    pub id: PackageId,
    pub version_req: VersionReq,
    pub optional: bool,
    pub dev_only: bool,
    pub conditions: Option<DependencyConditions>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConditions {
    pub platform: Option<Vec<String>>,
    pub arch: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub node_version: Option<VersionReq>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub exports: Option<HashMap<String, String>>,
    pub features: Option<Vec<String>>,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolutionContext {
    pub strategy: ResolutionStrategy,
    pub allow_prerelease: bool,
    pub max_depth: usize,
    pub include_dev: bool,
    pub include_optional: bool,
    pub platform: Option<String>,
    pub arch: Option<String>,
    pub environment: Option<String>,
}

impl Default for ResolutionContext {
    fn default() -> Self {
        Self {
            strategy: ResolutionStrategy::Compatible,
            allow_prerelease: false,
            max_depth: 50,
            include_dev: false,
            include_optional: false,
            platform: None,
            arch: None,
            environment: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResolutionStrategy {
    Latest,        // Always prefer latest compatible version
    Strict,        // Exact version matching only
    Compatible,    // Semantic versioning compatible ranges
    Conservative,  // Prefer stable, older compatible versions
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub packages: HashMap<PackageId, Vec<PackageVersion>>,
    pub resolution: HashMap<PackageId, PackageVersion>,
    pub constraints: HashMap<PackageId, Vec<VersionReq>>,
    pub dependency_order: Vec<PackageId>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            resolution: HashMap::new(),
            constraints: HashMap::new(),
            dependency_order: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolutionRequest {
    pub dependencies: Vec<DependencySpec>,
    pub context: ResolutionContext,
    pub overrides: HashMap<PackageId, Version>,
    pub excludes: Vec<PackageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub resolved_packages: HashMap<PackageId, PackageVersion>,
    pub dependency_order: Vec<PackageId>,
    pub conflicts: Vec<DependencyConflict>,
    pub warnings: Vec<String>,
    pub lock_file_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConflict {
    pub package_id: PackageId,
    pub conflicting_requirements: Vec<(VersionReq, PackageId)>,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    UseVersion(Version),
    RequiresManualResolution(String),
    Ignore,
}

// Cache structures
#[derive(Debug, Clone)]
pub struct CacheKey {
    pub request_hash: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone)]
pub struct CacheValue {
    pub result: ResolutionResult,
    pub timestamp: SystemTime,
    pub ttl: Duration,
}

impl CacheValue {
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed().unwrap_or(Duration::MAX) > self.ttl
    }
}

// Helper trait for package identification
pub trait PackageIdentifiable {
    fn package_id(&self) -> &PackageId;
    fn version(&self) -> &Version;
}

impl PackageIdentifiable for PackageVersion {
    fn package_id(&self) -> &PackageId {
        &self.id
    }

    fn version(&self) -> &Version {
        &self.version
    }
}

// Utility functions
impl DependencySpec {
    pub fn matches_conditions(&self, context: &ResolutionContext) -> bool {
        let Some(conditions) = &self.conditions else {
            return true;
        };

        // Check platform condition
        if let Some(platforms) = &conditions.platform {
            if let Some(current_platform) = &context.platform {
                if !platforms.contains(current_platform) {
                    return false;
                }
            }
        }

        // Check architecture condition
        if let Some(arches) = &conditions.arch {
            if let Some(current_arch) = &context.arch {
                if !arches.contains(current_arch) {
                    return false;
                }
            }
        }

        // Check environment condition
        if let Some(envs) = &conditions.env {
            if let Some(current_env) = &context.environment {
                if !envs.contains(current_env) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_applicable(&self, context: &ResolutionContext) -> bool {
        if self.dev_only && !context.include_dev {
            return false;
        }

        if self.optional && !context.include_optional {
            return false;
        }

        self.matches_conditions(context)
    }
}

impl PackageVersion {
    pub fn get_applicable_dependencies(&self, context: &ResolutionContext) -> Vec<&DependencySpec> {
        let mut deps = Vec::new();

        // Runtime dependencies are always included
        deps.extend(self.dependencies.iter().filter(|dep| dep.is_applicable(context)));

        // Development dependencies if requested
        if context.include_dev {
            deps.extend(self.dev_dependencies.iter().filter(|dep| dep.is_applicable(context)));
        }

        // Optional dependencies if requested
        if context.include_optional {
            deps.extend(self.optional_dependencies.iter().filter(|dep| dep.is_applicable(context)));
        }

        deps
    }
}