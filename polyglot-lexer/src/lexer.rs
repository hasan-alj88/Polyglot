// Lexer implementation with state machine

use crate::error::LexerError;
use crate::token::{Token, TokenKind};

/// Lexer states for state machine
#[derive(Debug, Clone, Copy, PartialEq)]
enum LexerState {
    Initial,
    InString,
    InInterpolation,
    InComment,
    InBlockMarker,
}

/// Main lexer struct
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    state: LexerState,
    string_buffer: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            state: LexerState::Initial,
            string_buffer: String::new(),
        }
    }

    /// Main tokenization method
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.next_token()?;

            // Skip whitespace and comments
            if !token.kind.should_skip() {
                tokens.push(token);
            }
        }

        // Add EOF token
        tokens.push(Token::new(
            TokenKind::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    /// Get next token from source
    fn next_token(&mut self) -> Result<Token, LexerError> {
        match self.state {
            LexerState::Initial => self.lex_initial(),
            LexerState::InString => self.lex_string(),
            LexerState::InInterpolation => self.lex_interpolation(),
            LexerState::InComment => self.lex_comment(),
            LexerState::InBlockMarker => self.lex_block_marker(),
        }
    }

    /// Lex in INITIAL state
    fn lex_initial(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace_inline();

        if self.is_at_end() {
            return Ok(Token::new(
                TokenKind::Eof,
                String::new(),
                self.line,
                self.column,
            ));
        }

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current_char();

        match ch {
            // Newline
            '\n' => {
                self.advance();
                self.line += 1;
                self.column = 1;
                Ok(Token::new(
                    TokenKind::Newline,
                    "\n".to_string(),
                    start_line,
                    start_column,
                ))
            }

            // Block markers
            '[' => {
                self.state = LexerState::InBlockMarker;
                self.advance();
                self.lex_block_marker()
            }

            // String literal
            '"' => {
                self.state = LexerState::InString;
                self.advance();
                self.string_buffer.clear();
                Ok(Token::new(
                    TokenKind::StringStart,
                    "\"".to_string(),
                    start_line,
                    start_column,
                ))
            }

            // Comments
            '/' => {
                if self.peek_char() == Some('/') {
                    self.lex_single_line_comment()
                } else if self.peek_char() == Some('*') {
                    self.state = LexerState::InComment;
                    self.lex_comment()
                } else {
                    Err(LexerError::UnexpectedCharacter {
                        line: self.line,
                        column: self.column,
                        character: ch,
                    })
                }
            }

            // Operators (longest match)
            '<' => self.lex_less_operators(),
            '>' => self.lex_greater_operators(),
            '=' => self.lex_equal_operators(),
            '?' => self.lex_question_operators(),
            '*' => {
                self.advance();
                if self.peek_char() == Some('?') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::OpWildcard,
                        "*?".to_string(),
                        start_line,
                        start_column,
                    ))
                } else {
                    // Standalone * is also a wildcard (used in [!] * error handling)
                    Ok(Token::new(
                        TokenKind::OpWildcard,
                        "*".to_string(),
                        start_line,
                        start_column,
                    ))
                }
            }
            '+' => {
                if self.peek_char() == Some('"') {
                    self.advance(); // consume '+'
                    self.advance(); // consume '"'
                    Ok(Token::new(
                        TokenKind::OpStringConcat,
                        "+\"".to_string(),
                        start_line,
                        start_column,
                    ))
                } else {
                    Err(LexerError::UnexpectedCharacter {
                        line: self.line,
                        column: self.column,
                        character: ch,
                    })
                }
            }

            // Identifiers with prefixes
            '.' => self.lex_variable_identifier(),
            '#' => self.lex_enum_identifier(),
            '|' => self.lex_pipeline_identifier(),
            '!' => self.lex_error_identifier(),
            '~' => self.lex_unpack_or_join_identifier(),

            // Delimiters
            '{' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterBraceOpen,
                    "{".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '}' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterBraceClose,
                    "}".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '(' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterParenOpen,
                    "(".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ')' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterParenClose,
                    ")".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ']' => {
                // Standalone ] delimiter (for range operators, NOT block markers)
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterSquareBracketClose,
                    "]".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ',' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterComma,
                    ",".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ':' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterColon,
                    ":".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '@' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterAt,
                    "@".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '\\' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterBackslash,
                    "\\".to_string(),
                    start_line,
                    start_column,
                ))
            }

            // Numbers
            '0'..='9' | '-' => self.lex_number(),

            // Plain identifiers (for special cases like DT, RT, TG, type names, etc.)
            'a'..='z' | 'A'..='Z' | '_' => self.lex_plain_identifier(),

            _ => Err(LexerError::UnexpectedCharacter {
                line: self.line,
                column: self.column,
                character: ch,
            }),
        }
    }

    /// Lex block marker (in IN_BLOCK_MARKER state)
    fn lex_block_marker(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column - 1; // '[' was already consumed

        if self.is_at_end() {
            return Err(LexerError::UnterminatedBlockMarker {
                line: start_line,
                column: start_column,
                got: "EOF".to_string(),
            });
        }

        let marker_char = self.current_char();
        self.advance();

        if self.is_at_end() || self.current_char() != ']' {
            return Err(LexerError::UnterminatedBlockMarker {
                line: start_line,
                column: start_column,
                got: marker_char.to_string(),
            });
        }

        self.advance(); // consume ']'
        self.state = LexerState::Initial;

        let lexeme = format!("[{}]", marker_char);
        let kind = match marker_char {
            '@' => TokenKind::BlockPackageStart,
            '#' => TokenKind::BlockVersionEnum,
            'X' => TokenKind::BlockEnd,
            '|' => TokenKind::BlockPipelineStart,
            'i' => TokenKind::BlockInput,
            't' => TokenKind::BlockTrigger,
            'Q' => TokenKind::BlockQueue,
            'W' => TokenKind::BlockWrapper,
            '\\' => TokenKind::BlockSetup,
            '/' => TokenKind::BlockCleanup,
            'o' => TokenKind::BlockOutput,
            'r' => TokenKind::BlockSequential,
            '<' => TokenKind::BlockInputBinding,
            '>' => TokenKind::BlockOutputBinding,
            'p' => TokenKind::BlockParallel,
            'Y' => TokenKind::BlockJoin,
            'b' => TokenKind::BlockBackground,
            's' => TokenKind::BlockStreaming,
            '!' => TokenKind::BlockErrorCatch,
            '?' => TokenKind::BlockConditional,
            '~' => TokenKind::BlockBody,
            '+' => TokenKind::BlockBoolOr,
            '&' => TokenKind::BlockBoolAnd,
            '-' => TokenKind::BlockBoolXor,
            '^' => TokenKind::BlockBoolNand,
            '.' => TokenKind::BlockBoolNor,
            '*' => TokenKind::BlockLineContinuation,
            _ => {
                return Err(LexerError::UnknownBlockMarker {
                    line: start_line,
                    column: start_column,
                    marker: marker_char.to_string(),
                });
            }
        };

        // Special handling for line continuation marker [*]
        // Skip the newline that follows to treat next line as continuation
        if kind == TokenKind::BlockLineContinuation
            && !self.is_at_end()
            && self.current_char() == '\n'
        {
            self.advance(); // Skip the newline
            self.line += 1; // Update line counter for accurate tracking
            self.column = 1; // Reset column
                             // The Newline token is NOT emitted - that's the key behavior
        }

        Ok(Token::new(kind, lexeme, start_line, start_column))
    }

    /// Lex string content and interpolations (in IN_STRING state)
    fn lex_string(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        while !self.is_at_end() {
            let ch = self.current_char();

            match ch {
                '"' => {
                    // End of string
                    // Emit STRING_CONTENT if buffer has content BEFORE consuming "
                    if !self.string_buffer.is_empty() {
                        let content = self.string_buffer.clone();
                        self.string_buffer.clear();
                        return Ok(Token::new(
                            TokenKind::StringContent,
                            content,
                            start_line,
                            start_column,
                        ));
                    }

                    // Now consume " and emit STRING_END
                    let end_line = self.line;
                    let end_column = self.column;
                    self.advance();
                    self.state = LexerState::Initial;

                    return Ok(Token::new(
                        TokenKind::StringEnd,
                        "\"".to_string(),
                        end_line,
                        end_column,
                    ));
                }

                '{' => {
                    // Start of interpolation
                    // Emit STRING_CONTENT if buffer has content BEFORE consuming {
                    if !self.string_buffer.is_empty() {
                        let content = self.string_buffer.clone();
                        self.string_buffer.clear();
                        return Ok(Token::new(
                            TokenKind::StringContent,
                            content,
                            start_line,
                            start_column,
                        ));
                    }

                    // Now consume { and emit INTERPOLATION_START
                    let interp_line = self.line;
                    let interp_column = self.column;
                    self.advance();
                    self.state = LexerState::InInterpolation;

                    return Ok(Token::new(
                        TokenKind::InterpolationStart,
                        "{".to_string(),
                        interp_line,
                        interp_column,
                    ));
                }

                '\\' => {
                    // Escape sequence
                    self.advance();
                    if self.is_at_end() {
                        return Err(LexerError::UnterminatedString {
                            line: start_line,
                            column: start_column,
                        });
                    }

                    let escaped_ch = self.current_char();
                    self.advance();

                    let escaped = match escaped_ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '{' => '{',
                        '}' => '}',
                        _ => {
                            return Err(LexerError::InvalidEscapeSequence {
                                line: self.line,
                                column: self.column - 2,
                                escape: escaped_ch.to_string(),
                            });
                        }
                    };

                    self.string_buffer.push(escaped);
                }

                _ => {
                    self.string_buffer.push(ch);
                    self.advance();
                }
            }
        }

        Err(LexerError::UnterminatedString {
            line: start_line,
            column: start_column,
        })
    }

    /// Lex interpolation content (in IN_INTERPOLATION state)
    fn lex_interpolation(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace_inline();

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current_char();

        match ch {
            '}' => {
                // End of interpolation
                self.advance();
                self.state = LexerState::InString;
                Ok(Token::new(
                    TokenKind::InterpolationEnd,
                    "}".to_string(),
                    start_line,
                    start_column,
                ))
            }

            '.' => {
                // Variable identifier
                self.lex_variable_identifier()
            }

            ':' => {
                // Colon before format identifier
                self.advance();
                Ok(Token::new(
                    TokenKind::DelimiterColon,
                    ":".to_string(),
                    start_line,
                    start_column,
                ))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                // Format identifier
                let ident = self.read_identifier();
                Ok(Token::new(
                    TokenKind::FormatIdentifier,
                    ident,
                    start_line,
                    start_column,
                ))
            }

            '"' => Err(LexerError::UnterminatedInterpolation {
                line: start_line,
                column: start_column,
                got: "\"".to_string(),
            }),

            _ => Err(LexerError::UnexpectedCharacter {
                line: self.line,
                column: self.column,
                character: ch,
            }),
        }
    }

    /// Lex single-line comment
    fn lex_single_line_comment(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        // Consume //
        self.advance();
        self.advance();

        let mut comment = String::from("//");

        while !self.is_at_end() && self.current_char() != '\n' {
            comment.push(self.current_char());
            self.advance();
        }

        Ok(Token::new(
            TokenKind::CommentSingle,
            comment,
            start_line,
            start_column,
        ))
    }

    /// Lex multi-line comment
    fn lex_comment(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        // Consume /*
        self.advance();
        self.advance();

        let mut comment = String::from("/*");

        while !self.is_at_end() {
            if self.current_char() == '*' && self.peek_char() == Some('/') {
                comment.push('*');
                comment.push('/');
                self.advance();
                self.advance();
                self.state = LexerState::Initial;

                return Ok(Token::new(
                    TokenKind::CommentMulti,
                    comment,
                    start_line,
                    start_column,
                ));
            }

            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            comment.push(self.current_char());
            self.advance();
        }

        Err(LexerError::UnterminatedComment {
            line: start_line,
            column: start_column,
        })
    }

    // ========================================
    // Operator Lexing (Longest Match)
    // ========================================

    fn lex_less_operators(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '<'

        if self.current_char() == '~' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpDefault,
                "<~".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '<' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpPush,
                "<<".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '?' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpLess,
                "<?".to_string(),
                start_line,
                start_column,
            ))
        } else {
            Err(LexerError::UnexpectedCharacter {
                line: start_line,
                column: start_column,
                character: '<',
            })
        }
    }

    fn lex_greater_operators(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '>'

        if self.current_char() == '>' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpPull,
                ">>".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '?' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpGreater,
                ">?".to_string(),
                start_line,
                start_column,
            ))
        } else {
            Err(LexerError::UnexpectedCharacter {
                line: start_line,
                column: start_column,
                character: '>',
            })
        }
    }

    fn lex_equal_operators(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '='

        // Check 3-char operators first (longest match)
        if self.current_char() == '!' && self.peek_char() == Some('?') {
            self.advance();
            self.advance();
            Ok(Token::new(
                TokenKind::OpNotEqual,
                "=!?".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '>' && self.peek_char() == Some('?') {
            self.advance();
            self.advance();
            Ok(Token::new(
                TokenKind::OpGreaterEqual,
                "=>?".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '<' && self.peek_char() == Some('?') {
            self.advance();
            self.advance();
            Ok(Token::new(
                TokenKind::OpLessEqual,
                "=<?".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == '?' {
            self.advance();
            Ok(Token::new(
                TokenKind::OpEqual,
                "=?".to_string(),
                start_line,
                start_column,
            ))
        } else {
            Err(LexerError::UnexpectedCharacter {
                line: start_line,
                column: start_column,
                character: '=',
            })
        }
    }

    fn lex_question_operators(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '?'

        match self.current_char() {
            '[' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::OpRangeClosed,
                    "?[".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '(' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::OpRangeOpen,
                    "?(".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ']' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::OpRangeHalfRight,
                    "?]".to_string(),
                    start_line,
                    start_column,
                ))
            }
            ')' => {
                self.advance();
                Ok(Token::new(
                    TokenKind::OpRangeHalfLeft,
                    "?)".to_string(),
                    start_line,
                    start_column,
                ))
            }
            _ => Err(LexerError::UnexpectedCharacter {
                line: start_line,
                column: start_column,
                character: '?',
            }),
        }
    }

    // ========================================
    // Identifier Lexing
    // ========================================

    fn lex_variable_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '.'

        let ident = self.read_identifier_with_dots();
        let lexeme = format!(".{}", ident);

        Ok(Token::new(
            TokenKind::IdentifierVariable,
            lexeme,
            start_line,
            start_column,
        ))
    }

    fn lex_enum_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '#'

        let ident = self.read_identifier_with_dots();
        let lexeme = format!("#{}", ident);

        // Check for reserved enumerations
        let kind = match lexeme.as_str() {
            "#None" => TokenKind::ReservedNone,
            "#Boolean.True" => TokenKind::ReservedBooleanTrue,
            "#Boolean.False" => TokenKind::ReservedBooleanFalse,
            _ if lexeme.starts_with("#PgVar.States.") => match &lexeme[14..] {
                "Declared" => TokenKind::ReservedPgVarDeclared,
                "DefaultReady" => TokenKind::ReservedPgVarDefaultReady,
                "Pending" => TokenKind::ReservedPgVarPending,
                "Ready" => TokenKind::ReservedPgVarReady,
                "Faulted" => TokenKind::ReservedPgVarFaulted,
                _ => TokenKind::IdentifierEnum,
            },
            "#Pipeline.NoInput" => TokenKind::ReservedPipelineNoInput,
            _ => TokenKind::IdentifierEnum,
        };

        Ok(Token::new(kind, lexeme, start_line, start_column))
    }

    fn lex_pipeline_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '|'

        let ident = self.read_identifier_with_dots();

        // Check if this is a pipeline formatted string: |Pipeline"string"
        if !self.is_at_end() && self.current_char() == '"' {
            return self.lex_pipeline_formatted_string(ident, start_line, start_column);
        }

        let lexeme = format!("|{}", ident);

        // Check for special pipeline types
        let kind = if lexeme.starts_with("|T.") {
            TokenKind::SpecialTriggerType
        } else if lexeme.starts_with("|W.") {
            TokenKind::SpecialWrapper
        } else {
            TokenKind::IdentifierPipeline
        };

        Ok(Token::new(kind, lexeme, start_line, start_column))
    }

    fn lex_error_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '!'

        let ident = self.read_identifier_with_dots();
        let lexeme = format!("!{}", ident);

        let kind = if lexeme == "!NoError" {
            TokenKind::ReservedNoError
        } else {
            TokenKind::IdentifierError
        };

        Ok(Token::new(kind, lexeme, start_line, start_column))
    }

    fn lex_unpack_or_join_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        self.advance(); // consume '~'

        if self.current_char() == '>' {
            // Default pull operator ~>
            self.advance(); // consume '>'
            Ok(Token::new(
                TokenKind::OpDefaultPull,
                "~>".to_string(),
                start_line,
                start_column,
            ))
        } else if self.current_char() == 'Y' && self.peek_char() == Some('.') {
            // Join identifier
            self.advance(); // consume 'Y'
            self.advance(); // consume '.'
            let ident = self.read_identifier();
            let lexeme = format!("~Y.{}", ident);
            Ok(Token::new(
                TokenKind::IdentifierJoin,
                lexeme,
                start_line,
                start_column,
            ))
        } else {
            // Unpack identifier
            let ident = self.read_identifier();
            let lexeme = format!("~{}", ident);
            Ok(Token::new(
                TokenKind::IdentifierUnpack,
                lexeme,
                start_line,
                start_column,
            ))
        }
    }

    /// Lex pipeline formatted string: |Pipeline"formatted {.var} string"
    fn lex_pipeline_formatted_string(
        &mut self,
        pipeline_name: String,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexerError> {
        // At this point, we've already consumed '|' and read the pipeline identifier
        // Current char is '"'
        self.advance(); // consume opening '"'

        let mut content = String::new();
        let mut brace_depth = 0;

        while !self.is_at_end() {
            let ch = self.current_char();

            if ch == '"' && brace_depth == 0 {
                // End of string
                self.advance(); // consume closing '"'
                let lexeme = format!("|{}\"{}\"", pipeline_name, content);
                return Ok(Token::new(
                    TokenKind::LiteralPipelineFormatted,
                    lexeme,
                    start_line,
                    start_column,
                ));
            } else if ch == '{' {
                brace_depth += 1;
                content.push(ch);
                self.advance();
            } else if ch == '}' {
                if brace_depth > 0 {
                    brace_depth -= 1;
                }
                content.push(ch);
                self.advance();
            } else if ch == '\\' {
                // Handle escape sequences
                content.push(ch);
                self.advance();
                if !self.is_at_end() {
                    content.push(self.current_char());
                    self.advance();
                }
            } else if ch == '\n' {
                // Track line numbers for error reporting
                self.line += 1;
                self.column = 0;
                content.push(ch);
                self.advance();
            } else {
                content.push(ch);
                self.advance();
            }
        }

        // If we get here, string was not terminated
        Err(LexerError::UnterminatedString {
            line: start_line,
            column: start_column,
        })
    }

    fn lex_plain_identifier(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ident = self.read_identifier();

        // Check for special identifiers
        let kind = if ident.starts_with("DT") || ident.starts_with("dt") {
            TokenKind::SpecialDatetime
        } else if ident.starts_with("RT") || ident.starts_with("rt") {
            TokenKind::SpecialRuntime
        } else if ident.starts_with("TG") || ident.starts_with("tg") {
            TokenKind::SpecialTrigger
        } else if ident == "re" && self.current_char() == '?' {
            self.advance();
            return Ok(Token::new(
                TokenKind::OpRegex,
                "re?".to_string(),
                start_line,
                start_column,
            ));
        } else {
            // Check for type keywords
            match ident.as_str() {
                "pg" | "py" | "rs" | "go" | "js" | "node" => TokenKind::TypeNamespace,
                "string" => TokenKind::TypeString,
                "int" => TokenKind::TypeInt,
                "float" => TokenKind::TypeFloat,
                "bool" => TokenKind::TypeBool,
                "dt" => TokenKind::TypeDatetime,
                "path" => TokenKind::TypePath,
                "serial" => TokenKind::TypeSerial,
                "array" => TokenKind::TypeArray,
                "set" => TokenKind::TypeSet,
                _ => TokenKind::Identifier,
            }
        };

        Ok(Token::new(kind, ident, start_line, start_column))
    }

    // ========================================
    // Number Lexing
    // ========================================

    fn lex_number(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let mut number = String::new();

        // Handle negative sign
        if self.current_char() == '-' {
            number.push('-');
            self.advance();
        }

        // Read digits
        while !self.is_at_end() && self.current_char().is_ascii_digit() {
            number.push(self.current_char());
            self.advance();
        }

        // Check for float
        if !self.is_at_end()
            && self.current_char() == '.'
            && self.peek_char().is_some_and(|c| c.is_ascii_digit())
        {
            number.push('.');
            self.advance();

            while !self.is_at_end() && self.current_char().is_ascii_digit() {
                number.push(self.current_char());
                self.advance();
            }

            Ok(Token::new(
                TokenKind::LiteralFloat,
                number,
                start_line,
                start_column,
            ))
        } else {
            Ok(Token::new(
                TokenKind::LiteralInteger,
                number,
                start_line,
                start_column,
            ))
        }
    }

    // ========================================
    // Helper Methods
    // ========================================

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn read_identifier_with_dots(&mut self) -> String {
        let mut ident = String::new();

        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn skip_whitespace_inline(&mut self) {
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn current_char(&self) -> char {
        self.source[self.position]
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.source.len() {
            Some(self.source[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}
