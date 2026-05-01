//! # Lexer
//!
//! Tokenizes Aljam3 (`.jm3`) source code into a stream of tokens.
//!
//! ## Responsibilities
//!
//! - Read raw `.jm3` source text
//! - Produce a `Vec<Token>` (token stream) with position information
//! - Enforce lexical rules (3-space indentation, spacing, sigils)
