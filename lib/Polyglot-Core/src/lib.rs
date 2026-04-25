//! # polyglot-core
//!
//! Core compiler components for the Polyglot language.
//!
//! ## Modules
//!
//! - [`lexer`] — Tokenizes `.pg` source code into a token stream.
//! - [`compiler`] — Validates the token stream, enforces compile rules,
//!   and produces an AST JSON representation.

pub mod lexer;
pub mod compiler;
