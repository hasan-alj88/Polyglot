//! Polyglot Parser - AST Definitions and Error Types
//!
//! This crate defines the Abstract Syntax Tree (AST) node types for Polyglot v0.0.2,
//! along with comprehensive error types for parser error detection.
//!
//! # Architecture
//!
//! - **span**: Position and Span types for source location tracking
//! - **ast**: Core AST node definitions (Pipeline, Block, Statement, Expression, etc.)
//! - **error**: Comprehensive ParserError enum (27 error types)
//! - **visitor**: Visitor trait for AST traversal
//!
//! # Design Principles
//!
//! 1. AST nodes are simple data structures (no behavior, just data)
//! 2. Every node includes a `Span` for precise error reporting
//! 3. All errors implement `Send + Sync` for async compatibility (ADR-004)
//! 4. Visitor pattern enables multiple analysis passes without modifying nodes
//!
//! # Story Context
//!
//! Story 1.4: Parser AST Definitions (ready-for-dev)
//! - Defines AST node *types* only
//! - Parser implementation (error detection logic) is Story 1.5

// Re-export key types from sub-modules
pub mod span;
pub mod ast;
pub mod error;
pub mod visitor;
pub mod import_resolver;
pub mod file_registry_resolver;
pub mod parser;
pub mod validation;
pub mod validation_error;

// Convenient re-exports
pub use span::{Position, Span};
pub use error::ParserError;
pub use visitor::Visitor;

// Validation API re-exports
pub use validation::validate_file;
pub use validation_error::{ValidationError, Severity, ErrorCategory};

// Re-export commonly used AST types
pub use ast::{
    // Top-level structures
    Program, Definition,
    PackageDeclaration, PackageSpec, Version, ImportDeclaration,
    EnumerationDefinition, EnumField,
    ErrorDefinition, ErrorField,
    // Pipeline structures
    Pipeline, Block, BlockType, Statement, Expression,
    Identifier, Literal, BinaryOperator, UnaryOperator, AssignmentOperator,
    TypeAnnotation, InputParameter, Pattern, RangeExpression, DurationUnit,
    TriggerConfig, QueueConfig, WrapperConfig,
};

// Re-export import resolution types
pub use import_resolver::{
    ImportResolver, StubImportResolver,
    ResolvedPackage, ImportError,
};
pub use file_registry_resolver::{
    FileRegistryResolver, RegistryFile, PackageEntry,
    PipelineSignature, ParameterDef,
};
pub use parser::Parser;
