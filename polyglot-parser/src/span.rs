//! Source location tracking types
//!
//! Defines `Position` and `Span` for precise error reporting.
//!
//! # Design
//!
//! - `Position`: Represents a single point in the source (line, column, byte offset)
//! - `Span`: Represents a range of source text (start position to end position)
//! - All positions are 1-indexed for human-readable error messages
//! - Small size (6 usizes = 48 bytes on 64-bit), safe to copy
//!
//! # Example
//!
//! ```
//! use polyglot_parser::span::{Position, Span};
//!
//! let start = Position::new(1, 5, 4);  // Line 1, Column 5, Byte offset 4
//! let end = Position::new(1, 10, 9);    // Line 1, Column 10, Byte offset 9
//! let span = Span::new(start, end);
//! ```

use serde::{Deserialize, Serialize};

/// A position in the source code
///
/// Represents a single point with line, column, and byte offset.
/// All positions are 1-indexed to match typical editor conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset from start of file (0-indexed)
    pub offset: usize,
}

impl Position {
    /// Create a new position
    ///
    /// # Arguments
    ///
    /// * `line` - Line number (1-indexed)
    /// * `column` - Column number (1-indexed)
    /// * `offset` - Byte offset (0-indexed)
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    /// Create a position at the start of a file
    pub fn start() -> Self {
        Self::new(1, 1, 0)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A span of source code
///
/// Represents a range from start position to end position (inclusive).
/// Used for precise error reporting and source location tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (inclusive)
    pub end: Position,
}

impl Span {
    /// Create a new span
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a span at the start of a file (zero-length)
    pub fn start() -> Self {
        let pos = Position::start();
        Self::new(pos, pos)
    }

    /// Create a single-character span at a position
    pub fn single(pos: Position) -> Self {
        Self::new(pos, pos)
    }

    /// Merge two spans into a span covering both
    ///
    /// Returns a new span from the start of the first span to the end of the second span.
    pub fn merge(self, other: Span) -> Span {
        let start = if self.start.offset < other.start.offset {
            self.start
        } else {
            other.start
        };

        let end = if self.end.offset > other.end.offset {
            self.end
        } else {
            other.end
        };

        Span::new(start, end)
    }

    /// Check if this span contains a position
    ///
    /// Returns true if the position is within this span (inclusive).
    pub fn contains(&self, pos: Position) -> bool {
        pos.offset >= self.start.offset && pos.offset <= self.end.offset
    }

    /// Check if this span contains another span
    pub fn contains_span(&self, other: Span) -> bool {
        self.start.offset <= other.start.offset && self.end.offset >= other.end.offset
    }

    /// Get the length of this span in bytes
    pub fn len(&self) -> usize {
        self.end.offset.saturating_sub(self.start.offset)
    }

    /// Check if this is a zero-length span
    pub fn is_empty(&self) -> bool {
        self.start.offset == self.end.offset
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{}:{}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(1, 5, 4);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);
        assert_eq!(pos.offset, 4);
    }

    #[test]
    fn test_position_start() {
        let pos = Position::start();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
        assert_eq!(pos.offset, 0);
    }

    #[test]
    fn test_position_display() {
        let pos = Position::new(42, 15, 100);
        assert_eq!(format!("{}", pos), "42:15");
    }

    #[test]
    fn test_span_new() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 10, 9);
        let span = Span::new(start, end);
        assert_eq!(span.start, start);
        assert_eq!(span.end, end);
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let span2 = Span::new(Position::new(1, 8, 7), Position::new(1, 12, 11));
        let merged = span1.merge(span2);
        assert_eq!(merged.start, span1.start);
        assert_eq!(merged.end, span2.end);
    }

    #[test]
    fn test_span_contains_position() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));
        assert!(span.contains(Position::new(1, 5, 4)));
        assert!(span.contains(Position::new(1, 1, 0))); // start inclusive
        assert!(span.contains(Position::new(1, 10, 9))); // end inclusive
        assert!(!span.contains(Position::new(1, 11, 10))); // outside
    }

    #[test]
    fn test_span_len() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));
        assert_eq!(span.len(), 9);
    }

    #[test]
    fn test_span_display_same_line() {
        let span = Span::new(Position::new(5, 10, 50), Position::new(5, 20, 60));
        assert_eq!(format!("{}", span), "5:10-20");
    }

    #[test]
    fn test_span_display_different_lines() {
        let span = Span::new(Position::new(5, 10, 50), Position::new(7, 5, 100));
        assert_eq!(format!("{}", span), "5:10-7:5");
    }
}
