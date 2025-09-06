use std::fmt;
use std::error::Error;
use crate::dependency::types::PackageId;
use semver::VersionReq;

pub type ResolutionResult<T> = Result<T, ResolutionError>;

#[derive(Debug)]
pub enum ResolutionError {
    VersionConflict {
        package: PackageId,
        conflicts: Vec<(VersionReq, PackageId)>,
        suggestion: Option<String>,
    },
    CircularDependency {
        cycle: Vec<PackageId>,
        suggestion: String,
    },
    PackageNotFound {
        package: PackageId,
        searched_in: Vec<String>,
        similar_packages: Vec<String>,
    },
    NetworkError {
        package: PackageId,
        error: String,
        retry_suggestion: String,
    },
    InvalidVersion {
        package: PackageId,
        version: String,
        error: String,
    },
    ConfigurationError {
        message: String,
        field: Option<String>,
    },
    CacheError {
        operation: String,
        error: String,
    },
    IOError {
        operation: String,
        path: Option<String>,
        error: String,
    },
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolutionError::VersionConflict { package, conflicts, suggestion } => {
                writeln!(f, "❌ Version conflict for package '{}':", package.name)?;
                for (constraint, requested_by) in conflicts {
                    writeln!(f, "  • {} (required by {})", constraint, requested_by.name)?;
                }
                
                if let Some(suggestion) = suggestion {
                    writeln!(f, "\n💡 Suggestion: {}", suggestion)?;
                } else {
                    writeln!(f, "\n💡 Try updating package versions or using compatible version ranges.")?;
                }
                Ok(())
            }
            ResolutionError::CircularDependency { cycle, suggestion } => {
                write!(f, "❌ Circular dependency detected:\n  ")?;
                for (i, package) in cycle.iter().enumerate() {
                    if i > 0 { write!(f, " → ")?; }
                    write!(f, "{}", package.name)?;
                }
                writeln!(f, "\n\n💡 {}", suggestion)
            }
            ResolutionError::PackageNotFound { package, searched_in, similar_packages } => {
                writeln!(f, "❌ Package '{}' not found.\n\nSearched in:", package.name)?;
                for location in searched_in {
                    writeln!(f, "  • {}", location)?;
                }
                
                if !similar_packages.is_empty() {
                    writeln!(f, "\nDid you mean:")?;
                    for similar in similar_packages {
                        writeln!(f, "  • {}", similar)?;
                    }
                }
                Ok(())
            }
            ResolutionError::NetworkError { package, error, retry_suggestion } => {
                write!(f, "❌ Network error downloading '{}': {}\n\n💡 {}", 
                      package.name, error, retry_suggestion)
            }
            ResolutionError::InvalidVersion { package, version, error } => {
                write!(f, "❌ Invalid version '{}' for package '{}': {}", 
                      version, package.name, error)
            }
            ResolutionError::ConfigurationError { message, field } => {
                if let Some(field) = field {
                    write!(f, "❌ Configuration error in field '{}': {}", field, message)
                } else {
                    write!(f, "❌ Configuration error: {}", message)
                }
            }
            ResolutionError::CacheError { operation, error } => {
                write!(f, "❌ Cache error during {}: {}", operation, error)
            }
            ResolutionError::IOError { operation, path, error } => {
                if let Some(path) = path {
                    write!(f, "❌ IO error during {} ({}): {}", operation, path, error)
                } else {
                    write!(f, "❌ IO error during {}: {}", operation, error)
                }
            }
        }
    }
}

impl Error for ResolutionError {}

impl From<std::io::Error> for ResolutionError {
    fn from(error: std::io::Error) -> Self {
        ResolutionError::IOError {
            operation: "file operation".to_string(),
            path: None,
            error: error.to_string(),
        }
    }
}

impl From<semver::Error> for ResolutionError {
    fn from(error: semver::Error) -> Self {
        ResolutionError::InvalidVersion {
            package: PackageId::local("unknown"),
            version: "unknown".to_string(),
            error: error.to_string(),
        }
    }
}

impl ResolutionError {
    pub fn to_user_friendly_message(&self) -> String {
        self.to_string()
    }

    pub fn version_conflict(
        package: PackageId,
        conflicts: Vec<(VersionReq, PackageId)>,
        suggestion: Option<String>,
    ) -> Self {
        ResolutionError::VersionConflict {
            package,
            conflicts,
            suggestion,
        }
    }

    pub fn circular_dependency(cycle: Vec<PackageId>) -> Self {
        let suggestion = "Consider refactoring the dependency structure to break the circular dependency. \
             You might need to extract shared functionality into a separate package.".to_string();
        
        ResolutionError::CircularDependency { cycle, suggestion }
    }

    pub fn package_not_found(
        package: PackageId,
        searched_in: Vec<String>,
        similar_packages: Vec<String>,
    ) -> Self {
        ResolutionError::PackageNotFound {
            package,
            searched_in,
            similar_packages,
        }
    }

    pub fn network_error(package: PackageId, error: impl Into<String>) -> Self {
        ResolutionError::NetworkError {
            package,
            error: error.into(),
            retry_suggestion: "Check your network connection and try again. You can also try clearing the cache with 'knot cache clean'.".to_string(),
        }
    }

    pub fn invalid_version(package: PackageId, version: impl Into<String>, error: impl Into<String>) -> Self {
        ResolutionError::InvalidVersion {
            package,
            version: version.into(),
            error: error.into(),
        }
    }

    pub fn configuration_error(message: impl Into<String>, field: Option<String>) -> Self {
        ResolutionError::ConfigurationError {
            message: message.into(),
            field,
        }
    }

    pub fn cache_error(operation: impl Into<String>, error: impl Into<String>) -> Self {
        ResolutionError::CacheError {
            operation: operation.into(),
            error: error.into(),
        }
    }

    pub fn io_error(operation: impl Into<String>, path: Option<String>, error: impl Into<String>) -> Self {
        ResolutionError::IOError {
            operation: operation.into(),
            path,
            error: error.into(),
        }
    }
}

// Helper trait for adding context to errors
pub trait ErrorContext<T> {
    fn with_package_context(self, package: &PackageId) -> ResolutionResult<T>;
    fn with_operation_context(self, operation: &str) -> ResolutionResult<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: Into<ResolutionError>,
{
    fn with_package_context(self, package: &PackageId) -> ResolutionResult<T> {
        self.map_err(|e| {
            let mut error = e.into();
            // Add package context to the error if it doesn't already have it
            match &mut error {
                ResolutionError::IOError { operation, path, .. } => {
                    if path.is_none() {
                        *operation = format!("{} (for package '{}')", operation, package.name);
                    }
                }
                ResolutionError::ConfigurationError { message, .. } => {
                    *message = format!("{} (for package '{}')", message, package.name);
                }
                _ => {}
            }
            error
        })
    }

    fn with_operation_context(self, operation: &str) -> ResolutionResult<T> {
        self.map_err(|e| {
            let mut error = e.into();
            match &mut error {
                ResolutionError::IOError { operation: op, .. } => {
                    *op = operation.to_string();
                }
                ResolutionError::CacheError { operation: op, .. } => {
                    *op = operation.to_string();
                }
                _ => {}
            }
            error
        })
    }
}