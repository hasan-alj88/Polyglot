//! Parser error types
//!
//! Comprehensive ParserError enum with 26+ error variants for detecting
//! all syntax and semantic errors during parsing.
//!
//! # Design
//!
//! - Uses thiserror for structured error messages
//! - Every error includes precise source location (Span)
//! - Optional hints for fixing errors
//! - Send + Sync for async compatibility (ADR-004)
//!
//! # Error Categories
//!
//! 1. **Lexer Errors** - Pass-through from lexer crate
//! 2. **Token Errors** - Unexpected tokens, EOF
//! 3. **Block Hierarchy Errors** - Block order, duplicates, missing blocks
//! 4. **Nesting Errors** - Invalid parent-child relationships
//! 5. **Operator Errors** - Misused operators, type mismatches
//! 6. **Type Errors** - Invalid type syntax, unknown types
//! 7. **Identifier Errors** - Invalid format, non-ASCII, missing prefix
//! 8. **Expression Errors** - Unclosed delimiters, invalid range syntax
//! 9. **Statement Errors** - Invalid assignment targets, missing operators
//! 10. **Pipeline Call Errors** - Invalid references, missing prefixes
//! 11. **Line Continuation Errors** - Invalid or broken continuations

use thiserror::Error;
use crate::span::Span;
use polyglot_lexer::LexerError;

/// Comprehensive parser error types for Polyglot v0.0.2
///
/// All error variants include precise source location information and
/// optional hints for fixing the error.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParserError {
    // ========================================
    // Lexer Errors (Pass-Through)
    // ========================================
    /// Lexer error with source location
    ///
    /// Wraps errors from the lexer layer with span information.
    #[error("Lexer error at {span}: {source}")]
    LexerError {
        source: LexerError,
        span: Span,
    },

    // ========================================
    // Token Errors
    // ========================================
    /// Expected one token but found another
    ///
    /// Includes context about what the parser was doing when the error occurred.
    #[error("Unexpected token at {span}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
        context: String,
        span: Span,
    },

    /// Unexpected end of file
    ///
    /// File ended while parser was expecting more tokens.
    #[error("Unexpected end of file: {context}")]
    UnexpectedEof {
        context: String,
        started_at: Span,
    },

    // ========================================
    // Block Hierarchy Errors
    // ========================================
    /// Block appears in wrong order at file level
    ///
    /// File-level blocks must follow canonical order: [@] → [#] → [!] → [M] → [|]
    #[error("Block order violation at {span}: {message}")]
    BlockOrderViolation {
        block_type: String,
        expected_order: String,
        span: Span,
        message: String,
    },

    /// Duplicate file-level or optional block
    ///
    /// Certain blocks can only appear once (e.g., [@] package, [t] trigger)
    #[error("Duplicate block at {second_span}: {block_type} already defined at {first_span}")]
    DuplicateBlock {
        block_type: String,
        first_span: Span,
        second_span: Span,
    },

    /// Required block is missing
    ///
    /// Some blocks require others (e.g., [@] requires [#] version)
    #[error("Missing required block: {block_type} expected after {after_span}")]
    MissingRequiredBlock {
        block_type: String,
        after_span: Span,
        hint: String,
    },

    /// Pipeline blocks in wrong order
    ///
    /// Pipeline-level blocks must follow canonical order:
    /// [i] → [t] → [\] → [r]/[p] → [Y] → [/] → [o]
    #[error("Invalid pipeline block order at {span}: {message}")]
    InvalidPipelineBlockOrder {
        block_type: String,
        canonical_order: String,
        span: Span,
        message: String,
    },

    /// Multiple execution blocks in same pipeline
    ///
    /// Cannot mix [r], [p], [Y], [b] in same pipeline
    #[error("Multiple execution blocks: {first_block} at {first_span}, {second_block} at {second_span}")]
    MultipleExecutionBlocks {
        first_block: String,
        first_span: Span,
        second_block: String,
        second_span: Span,
    },

    /// Duplicate optional pipeline block
    ///
    /// Optional blocks like [t], [\], [/] can only appear once per pipeline
    #[error("Duplicate optional block at {second_span}: {block_type} already defined at {first_span}")]
    DuplicateOptionalBlock {
        block_type: String,
        first_span: Span,
        second_span: Span,
    },

    // ========================================
    // Nesting Errors
    // ========================================
    /// Child block cannot be nested in parent block
    ///
    /// Certain parent-child combinations are invalid (e.g., [i] cannot contain [r])
    #[error("Invalid nesting at {span}: {child_block} cannot be child of {parent_block}")]
    InvalidNesting {
        child_block: String,
        parent_block: String,
        span: Span,
        reason: String,
    },

    /// Binding block without pipeline call
    ///
    /// [<] and [>] binding blocks require parent to call a pipeline
    #[error("Binding without pipeline call at {span}: {block} requires parent to call pipeline")]
    BindingWithoutPipelineCall {
        block: String,
        parent_span: Span,
        span: Span,
    },

    /// Block appears outside required parent context
    ///
    /// Certain blocks must be inside specific parents (e.g., [i] must be in [|])
    #[error("Orphaned block at {span}: {block_type} must be inside {required_parent}")]
    OrphanedBlock {
        block_type: String,
        required_parent: String,
        span: Span,
    },

    // ========================================
    // Operator Errors
    // ========================================
    /// String concatenation operator used incorrectly
    ///
    /// The +" operator can only concatenate string literals, not variables
    #[error("Invalid string concatenation at {span}: +\" can only concatenate string literals")]
    InvalidStringConcatenation {
        found: String,
        span: Span,
        hint: String,
    },

    /// Operator used with wrong operand types
    ///
    /// Type mismatch between operator and operand types
    #[error("Operator type mismatch at {span}: {operator} cannot be used with {operand_type}")]
    OperatorTypeMismatch {
        operator: String,
        operand_type: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Type Errors
    // ========================================
    /// Type annotation has invalid syntax
    ///
    /// Common mistakes: using / instead of \, invalid characters
    #[error("Invalid type syntax at {span}: {message}")]
    InvalidTypeSyntax {
        found: String,
        span: Span,
        message: String,
    },

    /// Type name not recognized
    ///
    /// Type not in standard library or not imported
    #[error("Unknown type at {span}: {type_name}")]
    UnknownType {
        type_name: String,
        span: Span,
        valid_types: Vec<String>,
    },

    // ========================================
    // Identifier Errors
    // ========================================
    /// Identifier format is invalid
    ///
    /// Must start with ASCII letter or underscore, contain only ASCII alphanumeric + underscore
    #[error("Invalid identifier at {span}: {identifier} - {reason}")]
    InvalidIdentifier {
        identifier: String,
        span: Span,
        reason: String,
        hint: String,
    },

    /// Identifier contains non-ASCII characters
    ///
    /// Polyglot v0.0.2 only supports ASCII identifiers
    #[error("Non-ASCII identifier at {span}: {identifier}")]
    NonAsciiIdentifier {
        identifier: String,
        span: Span,
        hint: String,
    },

    /// Identifier missing required prefix
    ///
    /// Variables need ., pipelines need |, etc.
    #[error("Missing identifier prefix at {span}: {identifier}")]
    MissingIdentifierPrefix {
        identifier: String,
        expected_prefix: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Expression Errors
    // ========================================
    /// Opening delimiter not closed
    ///
    /// Missing ), ], or } to close expression
    #[error("Unclosed delimiter at {opening_span}: expected {closing}, found {found}")]
    UnclosedDelimiter {
        opening: char,
        opening_span: Span,
        closing: char,
        found: String,
    },

    /// Range syntax is invalid
    ///
    /// Valid patterns: ?[, ?(, ?], ?)
    #[error("Invalid range syntax at {span}: {message}")]
    InvalidRangeSyntax {
        found: String,
        span: Span,
        message: String,
    },

    // ========================================
    // Statement Errors
    // ========================================
    /// Assignment to non-variable identifier
    ///
    /// Only variables (.var) can be assignment targets
    #[error("Invalid assignment target at {span}: {target}")]
    InvalidAssignmentTarget {
        target: String,
        span: Span,
        hint: String,
    },

    /// Variable declaration/assignment missing operator
    ///
    /// Need <<, >>, or <~ operator for assignment
    #[error("Missing assignment operator at {span}")]
    MissingAssignmentOperator {
        variable: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Pipeline Call Errors
    // ========================================
    /// Pipeline name is invalid
    ///
    /// Pipeline names must follow identifier rules
    #[error("Invalid pipeline reference at {span}: {pipeline}")]
    InvalidPipelineReference {
        pipeline: String,
        span: Span,
        reason: String,
    },

    /// Pipeline call missing | prefix
    ///
    /// Pipeline calls require | prefix
    #[error("Missing pipeline prefix at {span}: {identifier}")]
    MissingPipelinePrefix {
        identifier: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Line Continuation Errors
    // ========================================
    /// Line continuation used incorrectly
    ///
    /// [*] found without preceding statement
    #[error("Invalid line continuation at {span}: {reason}")]
    InvalidLineContinuation {
        span: Span,
        reason: String,
        hint: String,
    },

    /// Line continuation doesn't match context
    ///
    /// [*] must follow same statement type
    #[error("Broken continuation at {span}: {reason}")]
    BrokenContinuation {
        span: Span,
        previous_context: String,
        reason: String,
    },
}

// Implement Send + Sync for async compatibility (ADR-004)
// thiserror automatically implements these traits for all variants
// that contain Send + Sync types

impl ParserError {
    /// Get the primary span associated with this error
    ///
    /// Returns the source location where the error occurred.
    pub fn span(&self) -> Span {
        match self {
            ParserError::LexerError { span, .. } => *span,
            ParserError::UnexpectedToken { span, .. } => *span,
            ParserError::UnexpectedEof { started_at, .. } => *started_at,
            ParserError::BlockOrderViolation { span, .. } => *span,
            ParserError::DuplicateBlock { second_span, .. } => *second_span,
            ParserError::MissingRequiredBlock { after_span, .. } => *after_span,
            ParserError::InvalidPipelineBlockOrder { span, .. } => *span,
            ParserError::MultipleExecutionBlocks { second_span, .. } => *second_span,
            ParserError::DuplicateOptionalBlock { second_span, .. } => *second_span,
            ParserError::InvalidNesting { span, .. } => *span,
            ParserError::BindingWithoutPipelineCall { span, .. } => *span,
            ParserError::OrphanedBlock { span, .. } => *span,
            ParserError::InvalidStringConcatenation { span, .. } => *span,
            ParserError::OperatorTypeMismatch { span, .. } => *span,
            ParserError::InvalidTypeSyntax { span, .. } => *span,
            ParserError::UnknownType { span, .. } => *span,
            ParserError::InvalidIdentifier { span, .. } => *span,
            ParserError::NonAsciiIdentifier { span, .. } => *span,
            ParserError::MissingIdentifierPrefix { span, .. } => *span,
            ParserError::UnclosedDelimiter { opening_span, .. } => *opening_span,
            ParserError::InvalidRangeSyntax { span, .. } => *span,
            ParserError::InvalidAssignmentTarget { span, .. } => *span,
            ParserError::MissingAssignmentOperator { span, .. } => *span,
            ParserError::InvalidPipelineReference { span, .. } => *span,
            ParserError::MissingPipelinePrefix { span, .. } => *span,
            ParserError::InvalidLineContinuation { span, .. } => *span,
            ParserError::BrokenContinuation { span, .. } => *span,
        }
    }

    /// Get a user-friendly hint for fixing the error
    ///
    /// Returns an optional suggestion for how to fix the error.
    pub fn hint(&self) -> Option<&str> {
        match self {
            ParserError::InvalidStringConcatenation { hint, .. } => Some(hint),
            ParserError::OperatorTypeMismatch { hint, .. } => Some(hint),
            ParserError::InvalidIdentifier { hint, .. } => Some(hint),
            ParserError::NonAsciiIdentifier { hint, .. } => Some(hint),
            ParserError::MissingIdentifierPrefix { hint, .. } => Some(hint),
            ParserError::InvalidAssignmentTarget { hint, .. } => Some(hint),
            ParserError::MissingAssignmentOperator { hint, .. } => Some(hint),
            ParserError::MissingPipelinePrefix { hint, .. } => Some(hint),
            ParserError::InvalidLineContinuation { hint, .. } => Some(hint),
            ParserError::MissingRequiredBlock { hint, .. } => Some(hint),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Position;

    // Helper to create test spans
    fn test_span() -> Span {
        Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9))
    }

    #[test]
    fn test_unexpected_token_error() {
        let span = test_span();
        let error = ParserError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: "integer".to_string(),
            context: "parsing variable declaration".to_string(),
            span,
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Unexpected token"));
    }

    #[test]
    fn test_block_order_violation() {
        let span = test_span();
        let error = ParserError::BlockOrderViolation {
            block_type: "[|]".to_string(),
            expected_order: "[@] → [#] → [!] → [M] → [|]".to_string(),
            span,
            message: "Pipeline before package declaration".to_string(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Block order violation"));
    }

    #[test]
    fn test_duplicate_block() {
        let first_span = test_span();
        let second_span = Span::new(Position::new(5, 1, 50), Position::new(5, 10, 59));
        let error = ParserError::DuplicateBlock {
            block_type: "[@]".to_string(),
            first_span,
            second_span,
        };

        assert_eq!(error.span(), second_span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Duplicate block"));
    }

    #[test]
    fn test_invalid_string_concatenation() {
        let span = test_span();
        let hint = "Use string interpolation instead: \"{.var1} {.var2}\"".to_string();
        let error = ParserError::InvalidStringConcatenation {
            found: ".var1".to_string(),
            span,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Invalid string concatenation"));
    }

    #[test]
    fn test_unknown_type() {
        let span = test_span();
        let valid_types = vec![
            "pg\\string".to_string(),
            "pg\\int".to_string(),
            "pg\\float".to_string(),
        ];
        let error = ParserError::UnknownType {
            type_name: "pg\\unknown".to_string(),
            span,
            valid_types,
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Unknown type"));
    }

    #[test]
    fn test_invalid_identifier() {
        let span = test_span();
        let reason = "Identifiers must start with ASCII letter or underscore".to_string();
        let hint = "Use .invalid_123 or .inv123 instead".to_string();
        let error = ParserError::InvalidIdentifier {
            identifier: ".123invalid".to_string(),
            span,
            reason,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Invalid identifier"));
    }

    #[test]
    fn test_non_ascii_identifier() {
        let span = test_span();
        let hint = "Use ASCII characters (a-z, A-Z, 0-9, _)".to_string();
        let error = ParserError::NonAsciiIdentifier {
            identifier: ".变量".to_string(),
            span,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Non-ASCII identifier"));
    }

    #[test]
    fn test_unclosed_delimiter() {
        let span = test_span();
        let error = ParserError::UnclosedDelimiter {
            opening: '(',
            opening_span: span,
            closing: ')',
            found: "EOF".to_string(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Unclosed delimiter"));
    }

    #[test]
    fn test_invalid_assignment_target() {
        let span = test_span();
        let hint = "Only variables (.var) can be assignment targets".to_string();
        let error = ParserError::InvalidAssignmentTarget {
            target: "|MyPipeline".to_string(),
            span,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Invalid assignment target"));
    }

    #[test]
    fn test_missing_pipeline_prefix() {
        let span = test_span();
        let hint = "Pipeline calls require | prefix: |py.Print".to_string();
        let error = ParserError::MissingPipelinePrefix {
            identifier: "py.Print".to_string(),
            span,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Missing pipeline prefix"));
    }

    #[test]
    fn test_invalid_line_continuation() {
        let span = test_span();
        let reason = "Line continuation [*] found without preceding statement".to_string();
        let hint = "Remove [*] or add statement before it".to_string();
        let error = ParserError::InvalidLineContinuation {
            span,
            reason,
            hint: hint.clone(),
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), Some(hint.as_str()));
        assert!(error.to_string().contains("Invalid line continuation"));
    }

    #[test]
    fn test_lexer_error_wrapping() {
        let lexer_error = LexerError::UnterminatedString {
            line: 5,
            column: 10,
        };
        let span = test_span();
        let error = ParserError::LexerError {
            source: lexer_error,
            span,
        };

        assert_eq!(error.span(), span);
        assert_eq!(error.hint(), None);
        assert!(error.to_string().contains("Lexer error"));
    }

    #[test]
    fn test_error_display_formatting() {
        let span = test_span();
        let error = ParserError::UnexpectedEof {
            context: "Expected [X] block end marker for pipeline".to_string(),
            started_at: span,
        };

        let error_msg = error.to_string();
        assert!(error_msg.contains("Unexpected end of file"));
        assert!(error_msg.contains("Expected [X]"));
    }
}
