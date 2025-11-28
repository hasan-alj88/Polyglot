// Lexer error types

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexerError {
    #[error("Unterminated string literal at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },

    #[error("Unterminated interpolation at line {line}, column {column}: expected '}}', got {got}")]
    UnterminatedInterpolation {
        line: usize,
        column: usize,
        got: String,
    },

    #[error("Unterminated multi-line comment at line {line}, column {column}")]
    UnterminatedComment { line: usize, column: usize },

    #[error("Unterminated block marker at line {line}, column {column}: expected ']', got {got}")]
    UnterminatedBlockMarker {
        line: usize,
        column: usize,
        got: String,
    },

    #[error("Unknown block marker at line {line}, column {column}: '[{marker}]'")]
    UnknownBlockMarker {
        line: usize,
        column: usize,
        marker: String,
    },

    #[error("Invalid identifier at line {line}, column {column}: {message}")]
    InvalidIdentifier {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Invalid escape sequence at line {line}, column {column}: '\\{escape}'")]
    InvalidEscapeSequence {
        line: usize,
        column: usize,
        escape: String,
    },

    #[error("Unexpected character at line {line}, column {column}: '{character}'")]
    UnexpectedCharacter {
        line: usize,
        column: usize,
        character: char,
    },

    #[error("Invalid number format at line {line}, column {column}: {message}")]
    InvalidNumberFormat {
        line: usize,
        column: usize,
        message: String,
    },
}

impl LexerError {
    pub fn line(&self) -> usize {
        match self {
            LexerError::UnterminatedString { line, .. } => *line,
            LexerError::UnterminatedInterpolation { line, .. } => *line,
            LexerError::UnterminatedComment { line, .. } => *line,
            LexerError::UnterminatedBlockMarker { line, .. } => *line,
            LexerError::UnknownBlockMarker { line, .. } => *line,
            LexerError::InvalidIdentifier { line, .. } => *line,
            LexerError::InvalidEscapeSequence { line, .. } => *line,
            LexerError::UnexpectedCharacter { line, .. } => *line,
            LexerError::InvalidNumberFormat { line, .. } => *line,
        }
    }

    pub fn column(&self) -> usize {
        match self {
            LexerError::UnterminatedString { column, .. } => *column,
            LexerError::UnterminatedInterpolation { column, .. } => *column,
            LexerError::UnterminatedComment { column, .. } => *column,
            LexerError::UnterminatedBlockMarker { column, .. } => *column,
            LexerError::UnknownBlockMarker { column, .. } => *column,
            LexerError::InvalidIdentifier { column, .. } => *column,
            LexerError::InvalidEscapeSequence { column, .. } => *column,
            LexerError::UnexpectedCharacter { column, .. } => *column,
            LexerError::InvalidNumberFormat { column, .. } => *column,
        }
    }
}
