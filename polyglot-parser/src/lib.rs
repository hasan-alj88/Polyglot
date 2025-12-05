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
pub mod ast;
pub mod error;
pub mod file_registry_resolver;
pub mod import_resolver;
pub mod parser;
pub mod span;
pub mod validation;
pub mod validation_error;
pub mod visitor;

// Convenient re-exports
pub use error::ParserError;
pub use span::{Position, Span};
pub use visitor::Visitor;

// Validation API re-exports
pub use validation::validate_file;
pub use validation_error::{ErrorCategory, Severity, ValidationError};

// Re-export commonly used AST types
pub use ast::{
    AssignmentOperator,
    BinaryOperator,
    Block,
    BlockType,
    Definition,
    DurationUnit,
    EnumField,
    EnumerationDefinition,
    ErrorDefinition,
    ErrorField,
    Expression,
    Identifier,
    ImportDeclaration,
    InputParameter,
    Literal,
    PackageDeclaration,
    PackageSpec,
    Pattern,
    // Pipeline structures
    Pipeline,
    // Top-level structures
    Program,
    QueueConfig,
    RangeExpression,
    Statement,
    TriggerConfig,
    TypeAnnotation,
    UnaryOperator,
    Version,
    WrapperConfig,
};

// Re-export import resolution types
pub use file_registry_resolver::{
    FileRegistryResolver, PackageEntry, ParameterDef, PipelineSignature, RegistryFile,
};
pub use import_resolver::{ImportError, ImportResolver, ResolvedPackage, StubImportResolver};
pub use parser::Parser;
