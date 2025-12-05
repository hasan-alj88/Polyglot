//! Import Resolution Interface
//!
//! This module defines the interface for resolving package imports.
//! The actual implementation requires database access (Epic 3) and will be
//! integrated in Epic 8 (CLI Development).
//!
//! For now, this module provides:
//! - `ImportResolver` trait defining the resolution interface
//! - `StubImportResolver` stub implementation that gracefully degrades when DB is inactive
//! - Documentation of SQL queries that will be executed
//!
//! # Story Context
//!
//! Story 1.4: Parser AST Definitions (AC #10)
//! - Defines the interface (this file)
//! - Stub implementation for development without database
//! - Real implementation deferred to Epic 3 + Epic 8

use crate::ast::{PackageSpec, Version};
use crate::error::ParserError;

// ============================================================================
// Import Resolution Types
// ============================================================================

/// Resolved package information
///
/// Returned by ImportResolver when a package is successfully resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedPackage {
    /// Package registry tier (Local, Community, Enterprise)
    pub registry: String,
    /// Package path (e.g., ["MyApp", "Example"])
    pub path: Vec<String>,
    /// Resolved version (may differ from requested if range used)
    pub version: Version,
    /// Package ID in database
    pub package_id: Option<i64>,
    /// Whether package is active (compiled and registered)
    pub active: bool,
}

/// Import resolution error
#[derive(Debug, Clone, PartialEq)]
pub enum ImportError {
    /// Package not found in registry
    PackageNotFound {
        registry: String,
        path: Vec<String>,
        version: Version,
    },
    /// Database connection not available
    DatabaseInactive,
    /// Multiple package versions match (ambiguous)
    AmbiguousVersion {
        registry: String,
        path: Vec<String>,
        available_versions: Vec<Version>,
    },
    /// Parser error (e.g., invalid version format)
    ParserError(ParserError),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::PackageNotFound {
                registry,
                path,
                version,
            } => {
                write!(
                    f,
                    "Package not found: {}@{}:{}",
                    registry,
                    path.join("."),
                    version
                )
            }
            ImportError::DatabaseInactive => {
                write!(f, "Database connection not available for import resolution")
            }
            ImportError::AmbiguousVersion {
                registry,
                path,
                available_versions,
            } => {
                write!(
                    f,
                    "Ambiguous package version: {}@{} matches {} versions",
                    registry,
                    path.join("."),
                    available_versions.len()
                )
            }
            ImportError::ParserError(e) => write!(f, "Parser error: {}", e),
        }
    }
}

impl std::error::Error for ImportError {}

// ============================================================================
// Import Resolver Trait
// ============================================================================

/// Trait for resolving package imports
///
/// This trait defines the interface for looking up packages and their contents.
/// Implementations may use database access, filesystem scanning, or other methods.
pub trait ImportResolver {
    /// Resolve a package specification to a concrete package
    ///
    /// # Arguments
    /// * `spec` - Package specification (registry@path:version)
    ///
    /// # Returns
    /// * `Ok(ResolvedPackage)` if package found
    /// * `Err(ImportError)` if package not found or resolution fails
    ///
    /// # Example Database Query (when implemented)
    /// ```sql
    /// SELECT package_id, registry, path, version, active
    /// FROM packages
    /// WHERE registry = ? AND path = ? AND version = ?
    /// LIMIT 1
    /// ```
    fn resolve_package(&self, spec: &PackageSpec) -> Result<ResolvedPackage, ImportError>;

    /// List all pipeline names available in a package
    ///
    /// # Arguments
    /// * `package` - Package name (e.g., "utils" from import alias)
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of pipeline names
    /// * `Err(ImportError)` - Package not found or DB inactive
    ///
    /// # Example Database Query (when implemented)
    /// ```sql
    /// SELECT pipeline_name
    /// FROM pipelines
    /// WHERE package_id = ?
    /// ORDER BY pipeline_name
    /// ```
    fn list_available_pipelines(&self, package: &str) -> Result<Vec<String>, ImportError>;

    /// List all enumeration names available in a package
    ///
    /// # Arguments
    /// * `package` - Package name
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of enum names
    /// * `Err(ImportError)` - Package not found or DB inactive
    ///
    /// # Example Database Query (when implemented)
    /// ```sql
    /// SELECT enum_name
    /// FROM enumerations
    /// WHERE package_id = ?
    /// ORDER BY enum_name
    /// ```
    fn list_available_enums(&self, package: &str) -> Result<Vec<String>, ImportError>;

    /// List all error type names available in a package
    ///
    /// # Arguments
    /// * `package` - Package name
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of error names
    /// * `Err(ImportError)` - Package not found or DB inactive
    ///
    /// # Example Database Query (when implemented)
    /// ```sql
    /// SELECT error_name
    /// FROM errors
    /// WHERE package_id = ?
    /// ORDER BY error_name
    /// ```
    fn list_available_errors(&self, package: &str) -> Result<Vec<String>, ImportError>;
}

// ============================================================================
// Stub Implementation (for development without database)
// ============================================================================

/// Stub import resolver that gracefully degrades when database is inactive
///
/// This implementation:
/// - Returns empty results (no packages found)
/// - Logs warnings about inactive database
/// - Never panics or crashes
/// - Allows parser development to continue without database dependency
///
/// # Usage
/// ```rust
/// use polyglot_parser::import_resolver::{StubImportResolver, ImportResolver};
///
/// let resolver = StubImportResolver::new();
/// // Returns Err(ImportError::DatabaseInactive)
/// // let result = resolver.resolve_package(&spec);
/// ```
#[derive(Debug, Clone, Default)]
pub struct StubImportResolver {
    /// Enable warning logs when DB queries would be executed
    pub warn_on_query: bool,
}

impl StubImportResolver {
    pub fn new() -> Self {
        Self {
            warn_on_query: true,
        }
    }

    /// Create stub resolver with warnings disabled
    pub fn silent() -> Self {
        Self {
            warn_on_query: false,
        }
    }

    fn log_warning(&self, message: &str) {
        if self.warn_on_query {
            eprintln!("[WARN] StubImportResolver: {}", message);
            eprintln!("[WARN] Database inactive - import resolution unavailable");
        }
    }
}

impl ImportResolver for StubImportResolver {
    fn resolve_package(&self, spec: &PackageSpec) -> Result<ResolvedPackage, ImportError> {
        self.log_warning(&format!(
            "Would query database: SELECT * FROM packages WHERE registry = '{}' AND path = '{}' AND version = '{}'",
            spec.registry,
            spec.path.join("."),
            spec.version
        ));

        Err(ImportError::DatabaseInactive)
    }

    fn list_available_pipelines(&self, package: &str) -> Result<Vec<String>, ImportError> {
        self.log_warning(&format!(
            "Would query database: SELECT pipeline_name FROM pipelines WHERE package_id = (SELECT package_id FROM packages WHERE path = '{}')",
            package
        ));

        Err(ImportError::DatabaseInactive)
    }

    fn list_available_enums(&self, package: &str) -> Result<Vec<String>, ImportError> {
        self.log_warning(&format!(
            "Would query database: SELECT enum_name FROM enumerations WHERE package_id = (SELECT package_id FROM packages WHERE path = '{}')",
            package
        ));

        Err(ImportError::DatabaseInactive)
    }

    fn list_available_errors(&self, package: &str) -> Result<Vec<String>, ImportError> {
        self.log_warning(&format!(
            "Would query database: SELECT error_name FROM errors WHERE package_id = (SELECT package_id FROM packages WHERE path = '{}')",
            package
        ));

        Err(ImportError::DatabaseInactive)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn test_stub_resolver_returns_database_inactive() {
        let resolver = StubImportResolver::silent();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1),
            span: Span::new(
                crate::span::Position::new(1, 1, 0),
                crate::span::Position::new(1, 30, 29),
            ),
        };

        let result = resolver.resolve_package(&spec);
        assert!(matches!(result, Err(ImportError::DatabaseInactive)));
    }

    #[test]
    fn test_stub_resolver_list_pipelines_inactive() {
        let resolver = StubImportResolver::silent();

        let result = resolver.list_available_pipelines("utils");
        assert!(matches!(result, Err(ImportError::DatabaseInactive)));
    }

    #[test]
    fn test_stub_resolver_list_enums_inactive() {
        let resolver = StubImportResolver::silent();

        let result = resolver.list_available_enums("utils");
        assert!(matches!(result, Err(ImportError::DatabaseInactive)));
    }

    #[test]
    fn test_stub_resolver_list_errors_inactive() {
        let resolver = StubImportResolver::silent();

        let result = resolver.list_available_errors("utils");
        assert!(matches!(result, Err(ImportError::DatabaseInactive)));
    }

    #[test]
    fn test_import_error_display() {
        let error = ImportError::DatabaseInactive;
        assert_eq!(
            error.to_string(),
            "Database connection not available for import resolution"
        );

        let error = ImportError::PackageNotFound {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1),
        };
        assert_eq!(
            error.to_string(),
            "Package not found: Community@DataHelpers:2.3.1"
        );
    }

    #[test]
    fn test_stub_resolver_no_panic_on_missing_db() {
        // Verify that stub resolver never panics, even with invalid inputs
        let resolver = StubImportResolver::silent();

        // All of these should return Err, never panic
        let _ = resolver.list_available_pipelines("");
        let _ = resolver.list_available_enums("nonexistent");
        let _ = resolver.list_available_errors("invalid@package");
    }
}
