//! Validation Error Types for Polyglot Syntax Validation
//!
//! This module provides error types for syntax validation, designed to collect
//! and report multiple validation issues in a single pass.

use crate::error::ParserError;
use crate::span::Span;
use polyglot_lexer::error::LexerError;
use std::fmt;

/// Validation error representing a syntax or semantic issue
///
/// Designed to be user-friendly with clear error messages and precise locations
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Error severity level
    pub severity: Severity,
    /// Error message
    pub message: String,
    /// File path where error occurred
    pub file_path: Option<String>,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Optional suggestion for fixing the error
    pub suggestion: Option<String>,
    /// Error category
    pub category: ErrorCategory,
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Fatal error that prevents compilation
    Error,
    /// Warning that should be addressed
    Warning,
}

/// Error categories for grouping related errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Lexical analysis error (invalid tokens)
    Lexer,
    /// Syntax parsing error
    Parser,
    /// Semantic validation error
    Semantic,
    /// I/O error (file not found, etc.)
    Io,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(
        severity: Severity,
        category: ErrorCategory,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            severity,
            message: message.into(),
            file_path: None,
            line,
            column,
            suggestion: None,
            category,
        }
    }

    /// Set the file path for this error
    pub fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Add a suggestion for fixing this error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Convert from LexerError
    pub fn from_lexer_error(error: LexerError, file_path: Option<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: error.to_string(),
            file_path,
            line: error.line(),
            column: error.column(),
            suggestion: None,
            category: ErrorCategory::Lexer,
        }
    }

    /// Convert from ParserError
    pub fn from_parser_error(error: ParserError, file_path: Option<String>) -> Self {
        let span = error.span();
        Self {
            severity: Severity::Error,
            message: error.to_string(),
            file_path,
            line: span.start.line,
            column: span.start.column,
            suggestion: error.hint().map(|s| s.to_string()),
            category: ErrorCategory::Parser,
        }
    }

    /// Create a semantic error for duplicate pipeline definitions
    pub fn duplicate_pipeline(
        name: &str,
        first_location: Span,
        second_location: Span,
        file_path: Option<String>,
    ) -> Self {
        Self {
            severity: Severity::Error,
            message: format!(
                "Duplicate pipeline definition '{}' (first defined at line {})",
                name, first_location.start.line
            ),
            file_path,
            line: second_location.start.line,
            column: second_location.start.column,
            suggestion: Some(format!("Rename one of the '{}' pipelines", name)),
            category: ErrorCategory::Semantic,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Rust compiler-style error formatting
        let severity_str = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };

        if let Some(ref path) = self.file_path {
            write!(
                f,
                "{}: {}:{}:{}",
                severity_str, path, self.line, self.column
            )?;
        } else {
            write!(f, "{}: {}:{}", severity_str, self.line, self.column)?;
        }

        writeln!(f)?;
        writeln!(f, "  {}", self.message)?;

        if let Some(ref suggestion) = self.suggestion {
            writeln!(f, "  help: {}", suggestion)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new(
            Severity::Error,
            ErrorCategory::Semantic,
            "Test error",
            10,
            5,
        );

        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.message, "Test error");
        assert_eq!(error.line, 10);
        assert_eq!(error.column, 5);
        assert_eq!(error.category, ErrorCategory::Semantic);
    }

    #[test]
    fn test_validation_error_with_file_path() {
        let error = ValidationError::new(
            Severity::Error,
            ErrorCategory::Parser,
            "Syntax error",
            5,
            10,
        )
        .with_file_path("test.pg");

        assert_eq!(error.file_path, Some("test.pg".to_string()));
    }

    #[test]
    fn test_validation_error_with_suggestion() {
        let error = ValidationError::new(
            Severity::Warning,
            ErrorCategory::Semantic,
            "Unused variable",
            15,
            20,
        )
        .with_suggestion("Remove the unused variable");

        assert_eq!(
            error.suggestion,
            Some("Remove the unused variable".to_string())
        );
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new(
            Severity::Error,
            ErrorCategory::Parser,
            "Unexpected token",
            10,
            5,
        )
        .with_file_path("example.pg")
        .with_suggestion("Expected ';' after statement");

        let output = format!("{}", error);
        assert!(output.contains("error: example.pg:10:5"));
        assert!(output.contains("Unexpected token"));
        assert!(output.contains("help: Expected ';' after statement"));
    }
}
