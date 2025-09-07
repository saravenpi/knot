pub mod resolver;
pub mod registry;
pub mod types;
pub mod cache;
pub mod error;

pub use resolver::{DependencyResolver, PackageEcosystemAnalysis, SecurityAdvisory, SecuritySeverity, MaintenanceStatus, CycleAnalysis, CycleInfo, CycleType, CycleBreakingPoint, BreakingStrategy, BreakingDifficulty, CycleSeverity, WorkspacePackageInfo, KnotPackageResolutionResult, VersionCompatibilityReport, VersionCompatibilityEntry};
pub use types::{PackageId, PackageVersion, DependencySpec, ResolutionContext, ResolutionStrategy, VersionStrategy, VersionDistance, VersionResolutionContext, VersionConstraint, ConstraintPriority, VersionCompatibilityChecker, VersionSelector};
