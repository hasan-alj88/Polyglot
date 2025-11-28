// Polyglot Lexer - v0.0.2
// Tokenizes Polyglot source code into a stream of tokens

pub mod token;
pub mod error;
pub mod lexer;

#[cfg(test)]
mod tests;

pub use token::{Token, TokenKind};
pub use error::LexerError;
pub use lexer::Lexer;

/// Main entry point for lexing Polyglot source code
pub fn lex(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
