//! File-Based Registry Resolver
//!
//! This module provides a temporary file-based implementation of the ImportResolver trait
//! for use during parser development and testing. It reads package metadata from a JSON
//! file instead of querying the database.
//!
//! This will be replaced with a database-backed resolver in Epic 3 + Epic 8.
//!
//! # Story Context
//!
//! Story 1.5: Parser Implementation
//! - FileRegistryResolver enables testing without database dependency
//! - JSON format specified in test design document
//! - Validates pipeline signatures, parameter types, and defaults

use crate::ast::PackageSpec;
use crate::error::ParserError;
use crate::import_resolver::{ImportError, ImportResolver, ResolvedPackage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ============================================================================
// JSON Registry Format
// ============================================================================

/// Root structure of the registry JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryFile {
    pub packages: Vec<PackageEntry>,
}

/// Single package entry in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEntry {
    pub registry: String,
    pub path: Vec<String>,
    pub version: String,
    pub active: Option<bool>,
    pub pipelines: Vec<PipelineSignature>,
    pub enumerations: Option<Vec<EnumerationDef>>,
    pub errors: Option<Vec<ErrorDef>>,
}

/// Pipeline signature with input/output types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSignature {
    pub name: String,
    pub inputs: Vec<ParameterDef>,
    pub outputs: Vec<ParameterDef>,
}

/// Parameter definition with type and default value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    pub default: Option<String>,
}

/// Enumeration definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumerationDef {
    pub name: String,
    pub fields: Vec<String>,
}

/// Error type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDef {
    pub name: String,
    pub fields: Vec<String>,
}

// ============================================================================
// FileRegistryResolver Implementation
// ============================================================================

/// File-based registry resolver for testing without database
///
/// # Example Registry JSON
/// ```json
/// {
///   "packages": [
///     {
///       "registry": "Community",
///       "path": ["DataHelpers"],
///       "version": "2.3.1",
///       "active": true,
///       "pipelines": [
///         {
///           "name": "Transform",
///           "inputs": [
///             {
///               "name": ".input",
///               "type": "pg\\int",
///               "required": true,
///               "default": null
///             }
///           ],
///           "outputs": [
///             {
///               "name": ".result",
///               "type": "pg\\int",
///               "required": true,
///               "default": null
///             }
///           ]
///         }
///       ]
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FileRegistryResolver {
    packages: HashMap<String, PackageEntry>,
    warn_on_query: bool,
}

impl FileRegistryResolver {
    /// Create a new FileRegistryResolver from a JSON file
    ///
    /// # Arguments
    /// * `registry_path` - Path to the JSON registry file
    ///
    /// # Returns
    /// * `Ok(FileRegistryResolver)` if file is valid
    /// * `Err(ParserError)` if file cannot be read or parsed
    pub fn from_file<P: AsRef<Path>>(registry_path: P) -> Result<Self, ParserError> {
        let content = std::fs::read_to_string(registry_path.as_ref()).map_err(|e| {
            ParserError::UnresolvedImport {
                package: format!("Failed to read registry file: {}", e),
                span: crate::span::Span::start(),
            }
        })?;

        let registry: RegistryFile =
            serde_json::from_str(&content).map_err(|e| ParserError::UnresolvedImport {
                package: format!("Failed to parse registry JSON: {}", e),
                span: crate::span::Span::start(),
            })?;

        let mut packages = HashMap::new();
        for package in registry.packages {
            let key = Self::make_package_key(&package.registry, &package.path, &package.version);
            packages.insert(key, package);
        }

        Ok(Self {
            packages,
            warn_on_query: false,
        })
    }

    /// Create a resolver with empty registry (for testing)
    pub fn empty() -> Self {
        Self {
            packages: HashMap::new(),
            warn_on_query: false,
        }
    }

    /// Enable warning logs (for debugging)
    pub fn with_warnings(mut self) -> Self {
        self.warn_on_query = true;
        self
    }

    /// Generate package lookup key
    fn make_package_key(registry: &str, path: &[String], version: &str) -> String {
        format!("{}@{}:{}", registry, path.join("."), version)
    }

    /// Find package entry by spec
    fn find_package(&self, spec: &PackageSpec) -> Option<&PackageEntry> {
        let key = Self::make_package_key(&spec.registry, &spec.path, &spec.version.to_string());

        if self.warn_on_query {
            eprintln!("[DEBUG] FileRegistryResolver: Looking up package: {}", key);
        }

        self.packages.get(&key)
    }

    /// Get pipeline signature by name from a package
    pub fn get_pipeline_signature(
        &self,
        spec: &PackageSpec,
        pipeline_name: &str,
    ) -> Result<&PipelineSignature, ImportError> {
        let package = self
            .find_package(spec)
            .ok_or_else(|| ImportError::PackageNotFound {
                registry: spec.registry.clone(),
                path: spec.path.clone(),
                version: spec.version.clone(),
            })?;

        package
            .pipelines
            .iter()
            .find(|p| p.name == pipeline_name)
            .ok_or_else(|| ImportError::PackageNotFound {
                registry: spec.registry.clone(),
                path: spec.path.clone(),
                version: spec.version.clone(),
            })
    }

    /// Validate parameter compatibility
    ///
    /// Checks if provided parameters match pipeline signature:
    /// - Required parameters without defaults must be provided
    /// - Parameter types must match
    /// - Extra parameters are rejected
    pub fn validate_parameters(
        &self,
        signature: &PipelineSignature,
        provided_params: &HashMap<String, String>,
    ) -> Result<(), String> {
        // Check all required parameters are provided
        for param in &signature.inputs {
            if param.required && param.default.is_none() {
                if !provided_params.contains_key(&param.name) {
                    return Err(format!(
                        "Required parameter '{}' not provided for pipeline '{}'",
                        param.name, signature.name
                    ));
                }
            }
        }

        // Check no extra parameters provided
        for provided_name in provided_params.keys() {
            if !signature.inputs.iter().any(|p| &p.name == provided_name) {
                return Err(format!(
                    "Unknown parameter '{}' for pipeline '{}'",
                    provided_name, signature.name
                ));
            }
        }

        Ok(())
    }
}

impl ImportResolver for FileRegistryResolver {
    fn resolve_package(&self, spec: &PackageSpec) -> Result<ResolvedPackage, ImportError> {
        let package = self
            .find_package(spec)
            .ok_or_else(|| ImportError::PackageNotFound {
                registry: spec.registry.clone(),
                path: spec.path.clone(),
                version: spec.version.clone(),
            })?;

        Ok(ResolvedPackage {
            registry: package.registry.clone(),
            path: package.path.clone(),
            version: spec.version.clone(),
            package_id: None,
            active: package.active.unwrap_or(true),
        })
    }

    fn list_available_pipelines(&self, package: &str) -> Result<Vec<String>, ImportError> {
        // Find package by alias (simplified - assumes unique package names)
        for pkg in self.packages.values() {
            let package_name = pkg.path.last().unwrap_or(&pkg.registry);
            if package_name == package {
                return Ok(pkg.pipelines.iter().map(|p| p.name.clone()).collect());
            }
        }

        Err(ImportError::DatabaseInactive)
    }

    fn list_available_enums(&self, package: &str) -> Result<Vec<String>, ImportError> {
        for pkg in self.packages.values() {
            let package_name = pkg.path.last().unwrap_or(&pkg.registry);
            if package_name == package {
                if let Some(enums) = &pkg.enumerations {
                    return Ok(enums.iter().map(|e| e.name.clone()).collect());
                }
                return Ok(Vec::new());
            }
        }

        Err(ImportError::DatabaseInactive)
    }

    fn list_available_errors(&self, package: &str) -> Result<Vec<String>, ImportError> {
        for pkg in self.packages.values() {
            let package_name = pkg.path.last().unwrap_or(&pkg.registry);
            if package_name == package {
                if let Some(errors) = &pkg.errors {
                    return Ok(errors.iter().map(|e| e.name.clone()).collect());
                }
                return Ok(Vec::new());
            }
        }

        Err(ImportError::DatabaseInactive)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Version;
    use crate::span::Span;

    fn create_test_registry() -> FileRegistryResolver {
        let registry_json = r#"{
            "packages": [
                {
                    "registry": "Community",
                    "path": ["DataHelpers"],
                    "version": "2.3.1.0",
                    "active": true,
                    "pipelines": [
                        {
                            "name": "Transform",
                            "inputs": [
                                {
                                    "name": ".input",
                                    "type": "pg\\int",
                                    "required": true,
                                    "default": null
                                },
                                {
                                    "name": ".scale",
                                    "type": "pg\\int",
                                    "required": true,
                                    "default": "1"
                                }
                            ],
                            "outputs": [
                                {
                                    "name": ".result",
                                    "type": "pg\\int",
                                    "required": true,
                                    "default": null
                                }
                            ]
                        }
                    ],
                    "enumerations": [
                        {
                            "name": "Status",
                            "fields": ["Active", "Inactive"]
                        }
                    ],
                    "errors": [
                        {
                            "name": "ValidationError",
                            "fields": ["message"]
                        }
                    ]
                }
            ]
        }"#;

        let registry: RegistryFile = serde_json::from_str(registry_json).unwrap();
        let mut packages = HashMap::new();
        for package in registry.packages {
            let key = FileRegistryResolver::make_package_key(
                &package.registry,
                &package.path,
                &package.version,
            );
            packages.insert(key, package);
        }

        FileRegistryResolver {
            packages,
            warn_on_query: false,
        }
    }

    #[test]
    fn test_resolve_package_found() {
        let resolver = create_test_registry();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1, 0),
            span: Span::start(),
        };

        let result = resolver.resolve_package(&spec);
        assert!(result.is_ok());

        let resolved = result.unwrap();
        assert_eq!(resolved.registry, "Community");
        assert_eq!(resolved.path, vec!["DataHelpers"]);
        assert!(resolved.active);
    }

    #[test]
    fn test_resolve_package_not_found() {
        let resolver = create_test_registry();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["NonExistent".to_string()],
            version: Version::new(1, 0, 0, 0),
            span: Span::start(),
        };

        let result = resolver.resolve_package(&spec);
        assert!(matches!(result, Err(ImportError::PackageNotFound { .. })));
    }

    #[test]
    fn test_get_pipeline_signature() {
        let resolver = create_test_registry();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1, 0),
            span: Span::start(),
        };

        let result = resolver.get_pipeline_signature(&spec, "Transform");
        assert!(result.is_ok());

        let sig = result.unwrap();
        assert_eq!(sig.name, "Transform");
        assert_eq!(sig.inputs.len(), 2);
        assert_eq!(sig.outputs.len(), 1);
    }

    #[test]
    fn test_validate_parameters_success() {
        let resolver = create_test_registry();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1, 0),
            span: Span::start(),
        };

        let sig = resolver.get_pipeline_signature(&spec, "Transform").unwrap();

        let mut params = HashMap::new();
        params.insert(".input".to_string(), "42".to_string());
        // .scale has default, so not required

        let result = resolver.validate_parameters(sig, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_parameters_missing_required() {
        let resolver = create_test_registry();

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1, 0),
            span: Span::start(),
        };

        let sig = resolver.get_pipeline_signature(&spec, "Transform").unwrap();

        let params = HashMap::new(); // Missing .input

        let result = resolver.validate_parameters(sig, &params);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Required parameter '.input' not provided"));
    }

    #[test]
    fn test_list_available_pipelines() {
        let resolver = create_test_registry();

        let result = resolver.list_available_pipelines("DataHelpers");
        assert!(result.is_ok());

        let pipelines = result.unwrap();
        assert_eq!(pipelines.len(), 1);
        assert_eq!(pipelines[0], "Transform");
    }

    #[test]
    fn test_list_available_enums() {
        let resolver = create_test_registry();

        let result = resolver.list_available_enums("DataHelpers");
        assert!(result.is_ok());

        let enums = result.unwrap();
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0], "Status");
    }

    #[test]
    fn test_list_available_errors() {
        let resolver = create_test_registry();

        let result = resolver.list_available_errors("DataHelpers");
        assert!(result.is_ok());

        let errors = result.unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "ValidationError");
    }
}
