//! Core AST node definitions for Polyglot v0.0.2
//!
//! Defines all Abstract Syntax Tree node types including:
//! - Program: Complete .pg file with package declaration and definitions
//! - Pipeline: Top-level construct with metadata
//! - Enumeration: Top-level enum definition
//! - Error: Top-level error type definition
//! - Block: Container for statements with execution semantics
//! - Statement: Executable statements (assignments, pipeline calls, control flow)
//! - Expression: Evaluable expressions (literals, identifiers, operations)
//! - Identifier: Variable, pipeline, enum, error references
//! - Literal: String, integer, float, datetime, collection literals
//! - Operators: Binary and unary operators
//! - Type Annotations: For pipeline signatures and variable types
//!
//! # Design Principles
//!
//! 1. AST nodes are simple data structures (no behavior, just data)
//! 2. Every node includes a `Span` for error reporting
//! 3. Recursive structures use `Box<>` to avoid infinite size
//! 4. All nodes implement `Debug`, `Clone`, `PartialEq` for testing

use crate::span::Span;
use serde::{Deserialize, Serialize};

// ============================================================================
// Top-Level: Program (Complete .pg File)
// ============================================================================

/// Top-level compilation unit representing a complete .pg file
///
/// Every .pg file has this structure:
/// ```polyglot
/// [@] Local@MyApp.Example:1.0.0
/// [A] MyAlias
/// [<] @utils << Community@DataHelpers:2.3.1
/// [X]
///
/// [|] FirstPipeline
/// ...
/// [X]
///
/// [#] Status
/// ...
/// [X]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// Package declaration (required - every .pg file has one)
    pub package: PackageDeclaration,
    /// Top-level definitions (pipelines, enumerations, errors)
    pub definitions: Vec<Definition>,
    /// Source file path (for multi-file compilation tracking)
    pub source_file: Option<String>,
    /// Source location (entire file span)
    pub span: Span,
}

impl Program {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Top-level definition (pipeline, enumeration, or error)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Definition {
    /// Pipeline definition `[|] ... [X]`
    Pipeline(Pipeline),
    /// Enumeration definition `[#] ... [X]`
    Enumeration(EnumerationDefinition),
    /// Error type definition `[!] ... [X]`
    Error(ErrorDefinition),
}

// ============================================================================
// Package Declaration
// ============================================================================

/// Package declaration block `[@] ... [X]`
///
/// Example:
/// ```polyglot
/// [@] Local@MyApp.Example:1.0.0
/// [A] MyAlias
/// [<] @utils << Community@DataHelpers:2.3.1
/// [<] @db << Local@DatabaseLib:1.0.0
/// [X]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageDeclaration {
    /// Package specification (registry@path:version)
    pub spec: PackageSpec,
    /// Optional package alias `[A] ...`
    pub alias: Option<String>,
    /// Import declarations `[<] @ ...`
    pub imports: Vec<ImportDeclaration>,
    /// Source location
    pub span: Span,
}

/// Package specification (registry@path:version)
///
/// Example: `Local@MyApp.Example:1.0.0`
/// - Registry: "Local"
/// - Path: ["MyApp", "Example"]
/// - Version: 1.0.0
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageSpec {
    /// Registry tier (Local, Community, Enterprise)
    pub registry: String,
    /// Package path components (e.g., ["MyApp", "Example"])
    pub path: Vec<String>,
    /// Semantic version
    pub version: Version,
    /// Source location
    pub span: Span,
}

/// Semantic version (major.minor.patch)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Import declaration `[<] @alias << package:version`
///
/// Example: `[<] @utils << Community@DataHelpers:2.3.1`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    /// Import alias (e.g., "utils")
    pub alias: String,
    /// Imported package specification
    pub package: PackageSpec,
    /// Source location
    pub span: Span,
}

// ============================================================================
// Enumeration Definition
// ============================================================================

/// Top-level enumeration definition `[#] ... [X]`
///
/// Example:
/// ```polyglot
/// [#] Status
/// [<] .pending: pg\string << "PENDING"
/// [<] .active: pg\string << "ACTIVE"
/// [<] .completed: pg\string << "COMPLETED"
/// [A] St
/// [X]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumerationDefinition {
    /// Enumeration name path (e.g., ["Config", "Database"])
    pub name: Vec<String>,
    /// Field definitions
    pub fields: Vec<EnumField>,
    /// Optional alias `[A] ...`
    pub alias: Option<String>,
    /// Source location
    pub span: Span,
}

/// Enumeration field definition
///
/// Example: `[<] .pending: pg\string << "PENDING"`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumField {
    /// Field name (e.g., ".pending")
    pub name: String,
    /// Field type annotation
    pub field_type: TypeAnnotation,
    /// Field value (constant expression)
    pub value: Expression,
    /// Source location
    pub span: Span,
}

// ============================================================================
// Error Definition
// ============================================================================

/// Top-level error definition `[!] ! ... [X]`
///
/// Example:
/// ```polyglot
/// [!] !NetworkError
/// [<] .message: pg\string << "Network request failed"
/// [<] .code: pg\int << 1001
/// [<] .trace: pg\string << ""
/// [<] .retryable: pg\bool << #True
/// [X]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    /// Error name path (e.g., ["Http", "TimeoutError"])
    pub name: Vec<String>,
    /// Required message field
    pub message: ErrorField,
    /// Required code field
    pub code: ErrorField,
    /// Required trace field
    pub trace: ErrorField,
    /// Custom fields (optional)
    pub custom_fields: Vec<ErrorField>,
    /// Source location
    pub span: Span,
}

/// Error field definition (same structure as EnumField)
///
/// Example: `[<] .message: pg\string << "Network request failed"`
pub type ErrorField = EnumField;

// ============================================================================
// Top-Level: Pipeline
// ============================================================================

/// Top-level pipeline definition
///
/// Represents a complete Polyglot pipeline with inputs, outputs, trigger configuration,
/// and execution body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    /// Pipeline name (without `|` prefix)
    pub name: String,
    /// Optional input parameters with type annotations
    pub input: Option<Vec<InputParameter>>,
    /// Optional output type annotation
    pub output: Option<TypeAnnotation>,
    /// Optional trigger configuration (`[t]` block)
    pub trigger: Option<TriggerConfig>,
    /// Optional queue configuration (`[Q]` block)
    pub queue: Option<QueueConfig>,
    /// Optional wrapper configuration (`[W]` block)
    pub wrapper: Option<WrapperConfig>,
    /// Pipeline body (execution blocks)
    pub body: Block,
    /// Source location
    pub span: Span,
}

impl Pipeline {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

// ============================================================================
// Block Structure
// ============================================================================

/// A block of statements with execution semantics
///
/// Blocks define how statements are executed (sequentially, in parallel, etc.)
/// and contain a list of statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    /// Type of block (sequential, parallel, conditional, etc.)
    pub block_type: BlockType,
    /// Statements in this block
    pub statements: Vec<Statement>,
    /// Source location
    pub span: Span,
}

impl Block {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Block execution type
///
/// Maps to Polyglot block markers:
/// - `[r]` → Sequential
/// - `[p]` → Parallel
/// - `[?]` → Conditional
/// - `[Y]` → Join
/// - `[b]` → Background
/// - `[s]` → Streaming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockType {
    /// Sequential execution `[r]`
    Sequential,
    /// Parallel execution `[p]`
    Parallel,
    /// Conditional execution `[?]`
    Conditional,
    /// Join block `[Y]`
    Join,
    /// Background execution `[b]`
    Background,
    /// Streaming execution `[s]`
    Streaming,
    /// Input block `[i]`
    Input,
    /// Output block `[o]`
    Output,
    /// Setup block `[\]`
    Setup,
    /// Cleanup block `[/]`
    Cleanup,
    /// Input binding `[<]`
    InputBinding,
    /// Output binding `[>]`
    OutputBinding,
    /// Boolean OR `[+]`
    BoolOr,
    /// Boolean AND `[&]`
    BoolAnd,
    /// Boolean XOR `[-]`
    BoolXor,
    /// Boolean NAND `[^]`
    BoolNand,
    /// Boolean NOR `[.]`
    BoolNor,
    /// Body block `[~]`
    Body,
}

// ============================================================================
// Statement
// ============================================================================

/// Executable statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Variable declaration with optional initialization
    ///
    /// Example: `.var: pg\string << "value"`
    VariableDeclaration {
        name: Identifier,
        type_annotation: Option<TypeAnnotation>,
        init: Option<Expression>,
        span: Span,
    },

    /// Assignment to an existing variable
    ///
    /// Example: `.var << .other_var`
    Assignment {
        target: Identifier,
        operator: AssignmentOperator,
        value: Expression,
        span: Span,
    },

    /// Pipeline call
    ///
    /// Example: `|MyPipeline` or `|py.ProcessData`
    PipelineCall {
        pipeline: Identifier,
        args: Vec<Expression>,
        span: Span,
    },

    /// Return statement
    ///
    /// Example: Return value from pipeline
    Return {
        value: Option<Expression>,
        span: Span,
    },

    /// Nested block
    Block {
        block: Block,
        span: Span,
    },

    /// Conditional statement with optional else branch
    Conditional {
        condition: Expression,
        then_branch: Block,
        else_branch: Option<Block>,
        span: Span,
    },

    /// Error handling catch block
    ErrorCatch {
        error_type: Identifier,
        handler: Block,
        span: Span,
    },
}

impl Statement {
    pub fn span(&self) -> &Span {
        match self {
            Statement::VariableDeclaration { span, .. } => span,
            Statement::Assignment { span, .. } => span,
            Statement::PipelineCall { span, .. } => span,
            Statement::Return { span, .. } => span,
            Statement::Block { span, .. } => span,
            Statement::Conditional { span, .. } => span,
            Statement::ErrorCatch { span, .. } => span,
        }
    }
}

/// Assignment operators
///
/// Maps to Polyglot assignment syntax:
/// - `<<` → Push (assign from right)
/// - `>>` → Pull (assign to right)
/// - `<~` → Default (assign if not already set)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentOperator {
    /// Push operator `<<`
    Push,
    /// Pull operator `>>`
    Pull,
    /// Default operator `<~`
    Default,
}

// ============================================================================
// Expression
// ============================================================================

/// Evaluable expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal {
        value: Literal,
        span: Span,
    },

    /// Identifier reference
    Identifier {
        name: Identifier,
        span: Span,
    },

    /// Pipeline call expression
    PipelineCall {
        pipeline: Identifier,
        args: Vec<Expression>,
        span: Span,
    },

    /// Binary operation
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },

    /// Unary operation
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
        span: Span,
    },

    /// Range check
    ///
    /// Example: `.value ?[1, 10]` (value in closed range 1-10)
    RangeCheck {
        value: Box<Expression>,
        range: RangeExpression,
        span: Span,
    },

    /// Pattern match
    ///
    /// Example: `.str *? "pattern*"`
    PatternMatch {
        value: Box<Expression>,
        pattern: Pattern,
        span: Span,
    },

    /// Unpack operator
    ///
    /// Example: `~collection`
    Unpack {
        value: Box<Expression>,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Literal { span, .. } => span,
            Expression::Identifier { span, .. } => span,
            Expression::PipelineCall { span, .. } => span,
            Expression::BinaryOp { span, .. } => span,
            Expression::UnaryOp { span, .. } => span,
            Expression::RangeCheck { span, .. } => span,
            Expression::PatternMatch { span, .. } => span,
            Expression::Unpack { span, .. } => span,
        }
    }
}

// ============================================================================
// Identifier
// ============================================================================

/// Identifier types with prefixes
///
/// Polyglot identifiers are distinguished by their prefix:
/// - `.` → Variable
/// - `|` → Pipeline
/// - `#` → Enum
/// - `!` → Error
/// - `~Y.` → Join point
/// - `@` → Package
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Identifier {
    /// Variable identifier (`.var_name`)
    Variable(String),
    /// Pipeline identifier (`|pipeline_name` or `|Module.Pipeline`)
    Pipeline(String),
    /// Enum identifier (`#Enum.Variant`)
    Enum(String),
    /// Error identifier (`!ErrorType`)
    Error(String),
    /// Join point identifier (`~Y.join_name`)
    JoinPoint(String),
    /// Package identifier (`@package_name`)
    Package(String),
}

impl Identifier {
    /// Get the identifier string without prefix
    pub fn as_str(&self) -> &str {
        match self {
            Identifier::Variable(s) => s,
            Identifier::Pipeline(s) => s,
            Identifier::Enum(s) => s,
            Identifier::Error(s) => s,
            Identifier::JoinPoint(s) => s,
            Identifier::Package(s) => s,
        }
    }

    /// Check if this is a qualified identifier (contains `.`)
    pub fn is_qualified(&self) -> bool {
        self.as_str().contains('.')
    }

    /// Get the module path for qualified identifiers
    ///
    /// Example: `|py.ProcessData` → `Some("py")`
    pub fn module_path(&self) -> Option<&str> {
        if let Some(dot_pos) = self.as_str().find('.') {
            Some(&self.as_str()[..dot_pos])
        } else {
            None
        }
    }
}

// ============================================================================
// Literal
// ============================================================================

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// String literal with optional interpolation
    ///
    /// Example: `"Hello {.name}"` or plain `"Hello"`
    String(String),

    /// Integer literal
    ///
    /// Example: `42`, `-10`
    Integer(i64),

    /// Float literal
    ///
    /// Example: `3.14`, `-0.5`
    Float(f64),

    /// Boolean literal (represented as enum)
    ///
    /// Example: `#Boolean.True`, `#Boolean.False`
    Boolean(bool),

    /// Datetime literal
    ///
    /// Example: `DT"2024-01-15T14:30:00Z"`
    /// Stored as string, parsed during semantic analysis
    Datetime(String),

    /// Duration literal
    ///
    /// Example: `DT.Minutes"5"`, `DT.Hours"2"`
    Duration {
        value: String,
        unit: DurationUnit,
    },

    /// Collection literal
    ///
    /// Example: `{1, 2, 3}` or `{"a", "b", "c"}`
    Collection(Vec<Expression>),

    /// None literal
    ///
    /// Example: `#None`
    None,
}

/// Duration units for duration literals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurationUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

// ============================================================================
// Operators
// ============================================================================

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Comparison operators
    /// Equal `=?`
    Equal,
    /// Not equal `=!?`
    NotEqual,
    /// Greater than `>?`
    Greater,
    /// Less than `<?`
    Less,
    /// Greater or equal `=>?`
    GreaterEqual,
    /// Less or equal `=<?`
    LessEqual,

    // Logical operators
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Logical XOR
    Xor,
    /// Logical NAND
    Nand,
    /// Logical NOR
    Nor,

    // Data flow operators
    /// Push operator `<<`
    Push,
    /// Pull operator `>>`
    Pull,
    /// Default from `<~`
    DefaultFrom,
    /// Default to `~>`
    DefaultTo,

    // String operator
    /// String concatenation `+"`
    StringConcat,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// Negate (numeric negation)
    Negate,
    /// Logical NOT
    Not,
    /// Unpack operator `~`
    Unpack,
}

// ============================================================================
// Pattern and Range
// ============================================================================

/// Pattern matching types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard pattern `*?`
    Wildcard,
    /// Regex pattern `re?"pattern"`
    Regex(String),
    /// Enum variant pattern `#Enum.Variant`
    EnumVariant(Identifier),
}

/// Range expression for range checks
///
/// Supports four range types:
/// - `?[a, b]` - Closed (inclusive on both ends)
/// - `?(a, b)` - Open (exclusive on both ends)
/// - `?[a, b)` - Half-right (inclusive start, exclusive end)
/// - `?(a, b]` - Half-left (exclusive start, inclusive end)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeExpression {
    /// Range start value
    pub start: Box<Expression>,
    /// Range end value
    pub end: Box<Expression>,
    /// True if start is inclusive (`[`), false if exclusive (`(`)
    pub inclusive_start: bool,
    /// True if end is inclusive (`]`), false if exclusive (`)`)
    pub inclusive_end: bool,
    /// Source location
    pub span: Span,
}

// ============================================================================
// Type Annotations
// ============================================================================

/// Type annotations for variables and pipeline signatures
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeAnnotation {
    /// Named type
    ///
    /// Example: `pg\string`, `pg\int`, `py\dict`
    Named {
        namespace: String,
        type_name: String,
    },

    /// Pipeline type
    ///
    /// Example: Pipeline-typed value
    Pipeline(String),

    /// Enumeration type
    ///
    /// Example: `#MyEnum`
    Enum(String),

    /// Collection type
    ///
    /// Example: `{pg\int}` - collection of integers
    Collection(Box<TypeAnnotation>),

    /// Optional type
    ///
    /// Example: Type that can be None
    Optional(Box<TypeAnnotation>),
}

/// Input parameter for pipeline signature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputParameter {
    /// Parameter name (without `.` prefix)
    pub name: String,
    /// Optional type annotation
    pub type_annotation: Option<TypeAnnotation>,
    /// Optional default value
    pub default: Option<Expression>,
    /// Source location
    pub span: Span,
}

// ============================================================================
// Configuration Structures
// ============================================================================

/// Trigger configuration (`[t]` block)
///
/// Defines when a pipeline is triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerConfig {
    /// Trigger type (e.g., `|T.FileChange`, `|T.Schedule`)
    pub trigger_type: Identifier,
    /// Trigger parameters
    pub parameters: Vec<Expression>,
    /// Source location
    pub span: Span,
}

/// Queue configuration (`[Q]` block)
///
/// Defines queue priority and behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Queue priority
    pub priority: Option<Expression>,
    /// Queue parameters
    pub parameters: Vec<Expression>,
    /// Source location
    pub span: Span,
}

/// Wrapper configuration (`[W]` block)
///
/// Defines runtime wrapper for pipeline execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrapperConfig {
    /// Wrapper type (e.g., `|W.Python`, `|W.Rust.tokio`)
    pub wrapper_type: Identifier,
    /// Wrapper parameters
    pub parameters: Vec<Expression>,
    /// Source location
    pub span: Span,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Position;

    #[test]
    fn test_identifier_as_str() {
        let var = Identifier::Variable("my_var".to_string());
        assert_eq!(var.as_str(), "my_var");

        let pipeline = Identifier::Pipeline("py.Process".to_string());
        assert_eq!(pipeline.as_str(), "py.Process");
    }

    #[test]
    fn test_identifier_is_qualified() {
        let simple = Identifier::Variable("var".to_string());
        assert!(!simple.is_qualified());

        let qualified = Identifier::Pipeline("py.Process".to_string());
        assert!(qualified.is_qualified());
    }

    #[test]
    fn test_identifier_module_path() {
        let simple = Identifier::Pipeline("Process".to_string());
        assert_eq!(simple.module_path(), None);

        let qualified = Identifier::Pipeline("py.Process".to_string());
        assert_eq!(qualified.module_path(), Some("py"));
    }

    #[test]
    fn test_statement_span() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));
        let stmt = Statement::Return {
            value: None,
            span,
        };
        assert_eq!(stmt.span(), &span);
    }

    #[test]
    fn test_expression_span() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let expr = Expression::Literal {
            value: Literal::Integer(42),
            span,
        };
        assert_eq!(expr.span(), &span);
    }

    #[test]
    fn test_literal_integer() {
        let lit = Literal::Integer(42);
        assert_eq!(lit, Literal::Integer(42));
    }

    #[test]
    fn test_literal_string() {
        let lit = Literal::String("hello".to_string());
        assert_eq!(lit, Literal::String("hello".to_string()));
    }

    #[test]
    fn test_block_type_equality() {
        assert_eq!(BlockType::Sequential, BlockType::Sequential);
        assert_ne!(BlockType::Sequential, BlockType::Parallel);
    }

    // ========================================
    // Comprehensive AST Tests
    // ========================================

    #[test]
    fn test_all_block_types() {
        // Test all 18 BlockType variants exist and are distinct
        let types = vec![
            BlockType::Sequential,
            BlockType::Parallel,
            BlockType::Conditional,
            BlockType::Join,
            BlockType::Background,
            BlockType::Streaming,
            BlockType::Input,
            BlockType::Output,
            BlockType::Setup,
            BlockType::Cleanup,
            BlockType::InputBinding,
            BlockType::OutputBinding,
            BlockType::BoolOr,
            BlockType::BoolAnd,
            BlockType::BoolXor,
            BlockType::BoolNand,
            BlockType::BoolNor,
            BlockType::Body,
        ];
        assert_eq!(types.len(), 18);
    }

    #[test]
    fn test_nested_blocks() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(5, 1, 50));
        let inner_block = Block {
            block_type: BlockType::Sequential,
            statements: vec![
                Statement::Return { value: None, span },
            ],
            span,
        };

        let outer_block = Block {
            block_type: BlockType::Conditional,
            statements: vec![
                Statement::Block { block: inner_block, span },
            ],
            span,
        };

        assert_eq!(outer_block.statements.len(), 1);
        assert_eq!(outer_block.block_type, BlockType::Conditional);
    }

    #[test]
    fn test_variable_declaration_with_type() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 30, 29));
        let stmt = Statement::VariableDeclaration {
            name: Identifier::Variable("count".to_string()),
            type_annotation: Some(TypeAnnotation::Named {
                namespace: "pg".to_string(),
                type_name: "int".to_string(),
            }),
            init: Some(Expression::Literal {
                value: Literal::Integer(0),
                span,
            }),
            span,
        };

        match stmt {
            Statement::VariableDeclaration { name, type_annotation, init, .. } => {
                assert_eq!(name, Identifier::Variable("count".to_string()));
                assert!(type_annotation.is_some());
                assert!(init.is_some());
            }
            _ => panic!("Expected VariableDeclaration"),
        }
    }

    #[test]
    fn test_assignment_statement() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 20, 19));
        let stmt = Statement::Assignment {
            target: Identifier::Variable("result".to_string()),
            operator: AssignmentOperator::Push,
            value: Expression::Literal {
                value: Literal::Integer(42),
                span,
            },
            span,
        };

        assert_eq!(stmt.span(), &span);
    }

    #[test]
    fn test_pipeline_call_statement() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 25, 24));
        let stmt = Statement::PipelineCall {
            pipeline: Identifier::Pipeline("Process".to_string()),
            args: vec![
                Expression::Identifier {
                    name: Identifier::Variable("input".to_string()),
                    span,
                },
            ],
            span,
        };

        match stmt {
            Statement::PipelineCall { pipeline, args, .. } => {
                assert_eq!(pipeline, Identifier::Pipeline("Process".to_string()));
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected PipelineCall"),
        }
    }

    #[test]
    fn test_conditional_statement() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(5, 1, 50));
        let condition = Expression::BinaryOp {
            left: Box::new(Expression::Identifier {
                name: Identifier::Variable("x".to_string()),
                span,
            }),
            op: BinaryOperator::Greater,
            right: Box::new(Expression::Literal {
                value: Literal::Integer(10),
                span,
            }),
            span,
        };

        let then_block = Block {
            block_type: BlockType::Sequential,
            statements: vec![],
            span,
        };

        let stmt = Statement::Conditional {
            condition,
            then_branch: then_block,
            else_branch: None,
            span,
        };

        assert_eq!(stmt.span(), &span);
    }

    #[test]
    fn test_error_catch_statement() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(3, 1, 30));
        let handler = Block {
            block_type: BlockType::Sequential,
            statements: vec![],
            span,
        };

        let stmt = Statement::ErrorCatch {
            error_type: Identifier::Error("NetworkError".to_string()),
            handler,
            span,
        };

        match stmt {
            Statement::ErrorCatch { error_type, .. } => {
                assert_eq!(error_type, Identifier::Error("NetworkError".to_string()));
            }
            _ => panic!("Expected ErrorCatch"),
        }
    }

    #[test]
    fn test_all_binary_operators() {
        // Test all binary operators exist
        let ops = vec![
            BinaryOperator::Equal,
            BinaryOperator::NotEqual,
            BinaryOperator::Greater,
            BinaryOperator::Less,
            BinaryOperator::GreaterEqual,
            BinaryOperator::LessEqual,
            BinaryOperator::And,
            BinaryOperator::Or,
            BinaryOperator::Xor,
            BinaryOperator::Nand,
            BinaryOperator::Nor,
            BinaryOperator::Push,
            BinaryOperator::Pull,
            BinaryOperator::DefaultFrom,
            BinaryOperator::DefaultTo,
            BinaryOperator::StringConcat,
        ];
        assert_eq!(ops.len(), 16);
    }

    #[test]
    fn test_binary_expression() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Literal {
                value: Literal::Integer(5),
                span,
            }),
            op: BinaryOperator::Equal,
            right: Box::new(Expression::Literal {
                value: Literal::Integer(10),
                span,
            }),
            span,
        };

        match expr {
            Expression::BinaryOp { op, .. } => {
                assert_eq!(op, BinaryOperator::Equal);
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_unary_expression() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let expr = Expression::UnaryOp {
            op: UnaryOperator::Not,
            operand: Box::new(Expression::Literal {
                value: Literal::Boolean(true),
                span,
            }),
            span,
        };

        assert_eq!(expr.span(), &span);
    }

    #[test]
    fn test_range_check_expression() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 20, 19));
        let range = RangeExpression {
            start: Box::new(Expression::Literal {
                value: Literal::Integer(1),
                span,
            }),
            end: Box::new(Expression::Literal {
                value: Literal::Integer(10),
                span,
            }),
            inclusive_start: true,
            inclusive_end: true,
            span,
        };

        let expr = Expression::RangeCheck {
            value: Box::new(Expression::Identifier {
                name: Identifier::Variable("x".to_string()),
                span,
            }),
            range,
            span,
        };

        assert_eq!(expr.span(), &span);
    }

    #[test]
    fn test_pattern_match_expression() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 15, 14));
        let expr = Expression::PatternMatch {
            value: Box::new(Expression::Identifier {
                name: Identifier::Variable("status".to_string()),
                span,
            }),
            pattern: Pattern::Wildcard,
            span,
        };

        match expr {
            Expression::PatternMatch { pattern, .. } => {
                assert_eq!(pattern, Pattern::Wildcard);
            }
            _ => panic!("Expected PatternMatch"),
        }
    }

    #[test]
    fn test_unpack_expression() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));
        let expr = Expression::Unpack {
            value: Box::new(Expression::Identifier {
                name: Identifier::Variable("items".to_string()),
                span,
            }),
            span,
        };

        assert_eq!(expr.span(), &span);
    }

    #[test]
    fn test_all_literal_types() {
        let literals = vec![
            Literal::String("test".to_string()),
            Literal::Integer(42),
            Literal::Float(3.14),
            Literal::Boolean(true),
            Literal::Datetime("2025-11-27T10:00:00Z".to_string()),
            Literal::Duration {
                value: "5".to_string(),
                unit: DurationUnit::Minutes,
            },
            Literal::Collection(vec![]),
            Literal::None,
        ];
        assert_eq!(literals.len(), 8);
    }

    #[test]
    fn test_collection_literal() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 20, 19));
        let collection = Literal::Collection(vec![
            Expression::Literal {
                value: Literal::Integer(1),
                span,
            },
            Expression::Literal {
                value: Literal::Integer(2),
                span,
            },
            Expression::Literal {
                value: Literal::Integer(3),
                span,
            },
        ]);

        match collection {
            Literal::Collection(items) => {
                assert_eq!(items.len(), 3);
            }
            _ => panic!("Expected Collection"),
        }
    }

    #[test]
    fn test_duration_literal() {
        let duration = Literal::Duration {
            value: "30".to_string(),
            unit: DurationUnit::Seconds,
        };

        match duration {
            Literal::Duration { value, unit } => {
                assert_eq!(value, "30");
                assert_eq!(unit, DurationUnit::Seconds);
            }
            _ => panic!("Expected Duration"),
        }
    }

    #[test]
    fn test_all_identifier_types() {
        let identifiers = vec![
            Identifier::Variable("var".to_string()),
            Identifier::Pipeline("Pipeline".to_string()),
            Identifier::Enum("Status.Success".to_string()),
            Identifier::Error("NetworkError".to_string()),
            Identifier::JoinPoint("main".to_string()),
            Identifier::Package("network".to_string()),
        ];
        assert_eq!(identifiers.len(), 6);
    }

    #[test]
    fn test_type_annotation_collection() {
        let ty = TypeAnnotation::Collection(Box::new(TypeAnnotation::Named {
            namespace: "pg".to_string(),
            type_name: "int".to_string(),
        }));
        match ty {
            TypeAnnotation::Collection(inner) => {
                assert_eq!(*inner, TypeAnnotation::Named {
                    namespace: "pg".to_string(),
                    type_name: "int".to_string(),
                });
            }
            _ => panic!("Expected Collection type"),
        }
    }

    #[test]
    fn test_type_annotation_optional() {
        let ty = TypeAnnotation::Optional(Box::new(TypeAnnotation::Named {
            namespace: "pg".to_string(),
            type_name: "string".to_string(),
        }));
        match ty {
            TypeAnnotation::Optional(inner) => {
                assert_eq!(*inner, TypeAnnotation::Named {
                    namespace: "pg".to_string(),
                    type_name: "string".to_string(),
                });
            }
            _ => panic!("Expected Optional type"),
        }
    }

    #[test]
    fn test_input_parameter_with_default() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 30, 29));
        let param = InputParameter {
            name: "timeout".to_string(),
            type_annotation: Some(TypeAnnotation::Named {
                namespace: "pg".to_string(),
                type_name: "int".to_string(),
            }),
            default: Some(Expression::Literal {
                value: Literal::Integer(30),
                span,
            }),
            span,
        };

        assert!(param.type_annotation.is_some());
        assert!(param.default.is_some());
    }

    #[test]
    fn test_pipeline_with_configurations() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(10, 1, 100));
        let pipeline = Pipeline {
            name: "ProcessData".to_string(),
            input: Some(vec![
                InputParameter {
                    name: "data".to_string(),
                    type_annotation: Some(TypeAnnotation::Named {
                        namespace: "pg".to_string(),
                        type_name: "string".to_string(),
                    }),
                    default: None,
                    span,
                },
            ]),
            output: Some(TypeAnnotation::Named {
                namespace: "pg".to_string(),
                type_name: "int".to_string(),
            }),
            trigger: Some(TriggerConfig {
                trigger_type: Identifier::Pipeline("TG.FileChange".to_string()),
                parameters: vec![],
                span,
            }),
            queue: Some(QueueConfig {
                priority: None,
                parameters: vec![],
                span,
            }),
            wrapper: Some(WrapperConfig {
                wrapper_type: Identifier::Pipeline("py".to_string()),
                parameters: vec![],
                span,
            }),
            body: Block {
                block_type: BlockType::Sequential,
                statements: vec![],
                span,
            },
            span,
        };

        assert_eq!(pipeline.name, "ProcessData");
        assert!(pipeline.input.is_some());
        assert!(pipeline.output.is_some());
        assert!(pipeline.trigger.is_some());
        assert!(pipeline.queue.is_some());
        assert!(pipeline.wrapper.is_some());
    }

    #[test]
    fn test_pattern_variants() {
        let patterns = vec![
            Pattern::Wildcard,
            Pattern::Regex("\\d+".to_string()),
            Pattern::EnumVariant(Identifier::Enum("Status.Success".to_string())),
        ];
        assert_eq!(patterns.len(), 3);
    }

    #[test]
    fn test_range_expression_bounds() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 15, 14));

        // Inclusive range [1, 10]
        let inclusive_range = RangeExpression {
            start: Box::new(Expression::Literal {
                value: Literal::Integer(1),
                span,
            }),
            end: Box::new(Expression::Literal {
                value: Literal::Integer(10),
                span,
            }),
            inclusive_start: true,
            inclusive_end: true,
            span,
        };
        assert!(inclusive_range.inclusive_start);
        assert!(inclusive_range.inclusive_end);

        // Exclusive range (1, 10)
        let exclusive_range = RangeExpression {
            start: Box::new(Expression::Literal {
                value: Literal::Integer(1),
                span,
            }),
            end: Box::new(Expression::Literal {
                value: Literal::Integer(10),
                span,
            }),
            inclusive_start: false,
            inclusive_end: false,
            span,
        };
        assert!(!exclusive_range.inclusive_start);
        assert!(!exclusive_range.inclusive_end);
    }

    #[test]
    fn test_deeply_nested_expressions() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 30, 29));

        // ((a + b) * c) - d
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::Identifier {
                        name: Identifier::Variable("a".to_string()),
                        span,
                    }),
                    op: BinaryOperator::Push,
                    right: Box::new(Expression::Identifier {
                        name: Identifier::Variable("b".to_string()),
                        span,
                    }),
                    span,
                }),
                op: BinaryOperator::Push,
                right: Box::new(Expression::Identifier {
                    name: Identifier::Variable("c".to_string()),
                    span,
                }),
                span,
            }),
            op: BinaryOperator::Push,
            right: Box::new(Expression::Identifier {
                name: Identifier::Variable("d".to_string()),
                span,
            }),
            span,
        };

        assert_eq!(expr.span(), &span);
    }

    #[test]
    fn test_all_duration_units() {
        let units = vec![
            DurationUnit::Milliseconds,
            DurationUnit::Seconds,
            DurationUnit::Minutes,
            DurationUnit::Hours,
            DurationUnit::Days,
            DurationUnit::Weeks,
        ];
        assert_eq!(units.len(), 6);
    }

    // ========================================
    // Tests for new Program-level AST nodes
    // ========================================

    #[test]
    fn test_program_structure() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(20, 1, 500));

        let program = Program {
            package: PackageDeclaration {
                spec: PackageSpec {
                    registry: "Local".to_string(),
                    path: vec!["MyApp".to_string(), "Example".to_string()],
                    version: Version::new(1, 0, 0),
                    span,
                },
                alias: Some("MyAlias".to_string()),
                imports: vec![],
                span,
            },
            definitions: vec![],
            source_file: Some("example.pg".to_string()),
            span,
        };

        assert_eq!(program.package.alias, Some("MyAlias".to_string()));
        assert_eq!(program.source_file, Some("example.pg".to_string()));
        assert_eq!(program.definitions.len(), 0);
    }

    #[test]
    fn test_package_spec() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 40, 39));

        let spec = PackageSpec {
            registry: "Community".to_string(),
            path: vec!["DataHelpers".to_string()],
            version: Version::new(2, 3, 1),
            span,
        };

        assert_eq!(spec.registry, "Community");
        assert_eq!(spec.path.len(), 1);
        assert_eq!(spec.path[0], "DataHelpers");
        assert_eq!(spec.version, Version::new(2, 3, 1));
    }

    #[test]
    fn test_version_display() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");

        let version = Version::new(0, 0, 1);
        assert_eq!(version.to_string(), "0.0.1");
    }

    #[test]
    fn test_version_ordering() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);
        let v3 = Version::new(1, 1, 0);
        let v4 = Version::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert_eq!(v1, Version::new(1, 0, 0));
    }

    #[test]
    fn test_import_declaration() {
        let span = Span::new(Position::new(2, 1, 50), Position::new(2, 45, 94));

        let import = ImportDeclaration {
            alias: "utils".to_string(),
            package: PackageSpec {
                registry: "Community".to_string(),
                path: vec!["DataHelpers".to_string()],
                version: Version::new(2, 3, 1),
                span,
            },
            span,
        };

        assert_eq!(import.alias, "utils");
        assert_eq!(import.package.registry, "Community");
        assert_eq!(import.package.path[0], "DataHelpers");
    }

    #[test]
    fn test_package_declaration_with_imports() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(5, 3, 150));

        let package_decl = PackageDeclaration {
            spec: PackageSpec {
                registry: "Local".to_string(),
                path: vec!["MyApp".to_string()],
                version: Version::new(1, 0, 0),
                span,
            },
            alias: None,
            imports: vec![
                ImportDeclaration {
                    alias: "utils".to_string(),
                    package: PackageSpec {
                        registry: "Community".to_string(),
                        path: vec!["DataHelpers".to_string()],
                        version: Version::new(2, 3, 1),
                        span,
                    },
                    span,
                },
                ImportDeclaration {
                    alias: "db".to_string(),
                    package: PackageSpec {
                        registry: "Local".to_string(),
                        path: vec!["DatabaseLib".to_string()],
                        version: Version::new(1, 0, 0),
                        span,
                    },
                    span,
                },
            ],
            span,
        };

        assert_eq!(package_decl.imports.len(), 2);
        assert_eq!(package_decl.imports[0].alias, "utils");
        assert_eq!(package_decl.imports[1].alias, "db");
    }

    #[test]
    fn test_enumeration_definition() {
        let span = Span::new(Position::new(10, 1, 300), Position::new(15, 3, 450));

        let enum_def = EnumerationDefinition {
            name: vec!["Status".to_string()],
            fields: vec![
                EnumField {
                    name: ".pending".to_string(),
                    field_type: TypeAnnotation::Named {
                        namespace: "pg".to_string(),
                        type_name: "string".to_string(),
                    },
                    value: Expression::Literal {
                        value: Literal::String("PENDING".to_string()),
                        span,
                    },
                    span,
                },
                EnumField {
                    name: ".active".to_string(),
                    field_type: TypeAnnotation::Named {
                        namespace: "pg".to_string(),
                        type_name: "string".to_string(),
                    },
                    value: Expression::Literal {
                        value: Literal::String("ACTIVE".to_string()),
                        span,
                    },
                    span,
                },
            ],
            alias: Some("St".to_string()),
            span,
        };

        assert_eq!(enum_def.name, vec!["Status"]);
        assert_eq!(enum_def.fields.len(), 2);
        assert_eq!(enum_def.fields[0].name, ".pending");
        assert_eq!(enum_def.alias, Some("St".to_string()));
    }

    #[test]
    fn test_error_definition() {
        let span = Span::new(Position::new(20, 1, 600), Position::new(25, 3, 750));

        let error_def = ErrorDefinition {
            name: vec!["NetworkError".to_string()],
            message: EnumField {
                name: ".message".to_string(),
                field_type: TypeAnnotation::Named {
                    namespace: "pg".to_string(),
                    type_name: "string".to_string(),
                },
                value: Expression::Literal {
                    value: Literal::String("Network request failed".to_string()),
                    span,
                },
                span,
            },
            code: EnumField {
                name: ".code".to_string(),
                field_type: TypeAnnotation::Named {
                    namespace: "pg".to_string(),
                    type_name: "int".to_string(),
                },
                value: Expression::Literal {
                    value: Literal::Integer(1001),
                    span,
                },
                span,
            },
            trace: EnumField {
                name: ".trace".to_string(),
                field_type: TypeAnnotation::Named {
                    namespace: "pg".to_string(),
                    type_name: "string".to_string(),
                },
                value: Expression::Literal {
                    value: Literal::String("".to_string()),
                    span,
                },
                span,
            },
            custom_fields: vec![],
            span,
        };

        assert_eq!(error_def.name, vec!["NetworkError"]);
        assert_eq!(error_def.message.name, ".message");
        assert_eq!(error_def.code.name, ".code");
        assert_eq!(error_def.trace.name, ".trace");
        assert_eq!(error_def.custom_fields.len(), 0);
    }

    #[test]
    fn test_definition_enum_variants() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(10, 1, 300));

        // Create a minimal pipeline for testing
        let pipeline = Pipeline {
            name: "TestPipeline".to_string(),
            input: None,
            output: None,
            trigger: None,
            queue: None,
            wrapper: None,
            body: Block {
                block_type: BlockType::Sequential,
                statements: vec![],
                span,
            },
            span,
        };

        let def = Definition::Pipeline(pipeline);
        match def {
            Definition::Pipeline(p) => {
                assert_eq!(p.name, "TestPipeline");
            }
            _ => panic!("Expected Pipeline variant"),
        }
    }

    #[test]
    fn test_program_with_multiple_definitions() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(50, 1, 1500));

        let program = Program {
            package: PackageDeclaration {
                spec: PackageSpec {
                    registry: "Local".to_string(),
                    path: vec!["MyApp".to_string()],
                    version: Version::new(1, 0, 0),
                    span,
                },
                alias: None,
                imports: vec![],
                span,
            },
            definitions: vec![
                Definition::Pipeline(Pipeline {
                    name: "First".to_string(),
                    input: None,
                    output: None,
                    trigger: None,
                    queue: None,
                    wrapper: None,
                    body: Block {
                        block_type: BlockType::Sequential,
                        statements: vec![],
                        span,
                    },
                    span,
                }),
                Definition::Enumeration(EnumerationDefinition {
                    name: vec!["Status".to_string()],
                    fields: vec![],
                    alias: None,
                    span,
                }),
                Definition::Error(ErrorDefinition {
                    name: vec!["MyError".to_string()],
                    message: EnumField {
                        name: ".message".to_string(),
                        field_type: TypeAnnotation::Named {
                            namespace: "pg".to_string(),
                            type_name: "string".to_string(),
                        },
                        value: Expression::Literal {
                            value: Literal::String("Error".to_string()),
                            span,
                        },
                        span,
                    },
                    code: EnumField {
                        name: ".code".to_string(),
                        field_type: TypeAnnotation::Named {
                            namespace: "pg".to_string(),
                            type_name: "int".to_string(),
                        },
                        value: Expression::Literal {
                            value: Literal::Integer(1),
                            span,
                        },
                        span,
                    },
                    trace: EnumField {
                        name: ".trace".to_string(),
                        field_type: TypeAnnotation::Named {
                            namespace: "pg".to_string(),
                            type_name: "string".to_string(),
                        },
                        value: Expression::Literal {
                            value: Literal::String("".to_string()),
                            span,
                        },
                        span,
                    },
                    custom_fields: vec![],
                    span,
                }),
            ],
            source_file: Some("multi.pg".to_string()),
            span,
        };

        assert_eq!(program.definitions.len(), 3);
        assert!(matches!(program.definitions[0], Definition::Pipeline(_)));
        assert!(matches!(program.definitions[1], Definition::Enumeration(_)));
        assert!(matches!(program.definitions[2], Definition::Error(_)));
    }
}
