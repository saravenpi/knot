pub mod resolver;
pub mod registry;
pub mod types;
pub mod cache;
pub mod error;

pub use resolver::DependencyResolver;
pub use types::{PackageId, PackageVersion, DependencySpec, ResolutionContext, ResolutionStrategy};
