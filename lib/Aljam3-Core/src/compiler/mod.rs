//! # Compiler
//!
//! Validates the token stream, enforces compile rules, and produces an AST JSON.
//!
//! ## Responsibilities
//!
//! - Consume the token stream from the [`crate::lexer`]
//! - Check for compile errors (PGE rules) and warnings (PGW rules)
//! - Build a validated AST
//! - Serialize the AST to JSON output
