//! # aljam3-core
//!
//! Core compiler components for the Aljam3 language.
//!
//! ## Modules
//!
//! - [`lexer`] — Tokenizes `.aj3` source code into a token stream.
//! - [`compiler`] — Validates the token stream, enforces compile rules,
//!   and produces an AST JSON representation.

pub mod lexer;
pub mod compiler;
