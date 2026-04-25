//! # Lexer
//!
//! Tokenizes Polyglot (`.pg`) source code into a stream of tokens.
//!
//! ## Responsibilities
//!
//! - Read raw `.pg` source text
//! - Produce a `Vec<Token>` (token stream) with position information
//! - Enforce lexical rules (3-space indentation, spacing, sigils)
