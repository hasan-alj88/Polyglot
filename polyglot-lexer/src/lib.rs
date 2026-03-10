// Polyglot Lexer - v0.0.4
// Tokenizes Polyglot source code into a stream of tokens

pub mod error;
pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests;

pub use error::LexerError;
pub use lexer::Lexer;
pub use token::{Token, TokenKind};

/// Main entry point for lexing Polyglot source code
pub fn lex(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
