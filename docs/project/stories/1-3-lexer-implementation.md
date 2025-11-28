# Story 1.3: Lexer Implementation

Status: backlog

## Story

As a compiler developer,
I want a working lexer that tokenizes Polyglot source code,
so that the parser can consume tokens for AST generation.

## Acceptance Criteria

1. **Lexer module complete**
   - Implemented in `polyglot-lexer/src/lexer.rs`
   - Uses `logos` crate (per ADR-013) for token generation
   - Exposes `Lexer` struct with public API
   - Implements `Iterator<Item = Result<Token, LexerError>>`

2. **Token recognition**
   - All 60+ token types from Story 1.2 recognized correctly
   - Block markers: All 27+ markers (`[|]`, `[X]`, `[i]`, `[o]`, `[r]`, `[p]`, etc.)
   - Operators: All 25+ operators (`|`, `~`, `<<`, `>>`, `<~`, `~>`, `=?`, etc.)
   - Identifiers: snake_case pattern (alphanumeric + underscore)
   - String literals: UTF-8 with escape sequences (`\n`, `\t`, `\"`, `\\`)
   - Number literals: integers and floats
   - Comments: Single-line `//` and multi-line `/* */`
   - Whitespace handled (spaces, tabs, newlines)

3. **Source location tracking**
   - Every token has accurate line and column position
   - Line numbers start at 1, columns start at 1
   - Multi-line tokens (strings, comments) track correctly
   - Location preserved through entire tokenization

4. **Error handling and recovery**
   - Invalid characters produce `LexerError::UnexpectedCharacter`
   - Unterminated strings produce `LexerError::UnterminatedString`
   - Invalid numbers produce `LexerError::InvalidNumber`
   - Errors include location and helpful context
   - Lexer continues after errors (collects all errors, doesn't stop at first)

5. **Performance**
   - **NFR-P1:** Tokenize 1000-line file in <100ms
   - Single-pass tokenization (no backtracking)
   - Minimal allocations (reuse buffers where possible)
   - Benchmarked with `criterion` crate

6. **API design**
   - `Lexer::new(source: &str) -> Self` - Constructor
   - `Lexer::tokenize(&mut self) -> Vec<Result<Token, LexerError>>` - Consume all tokens
   - Implements `Iterator` for lazy tokenization
   - Public API is ergonomic and well-documented

7. **Unit tests**
   - Test all token types individually
   - Test multi-character operators (`<<`, `>>`, `=?`, `>=?`, etc.)
   - Test edge cases (empty input, only whitespace, only comments)
   - Test error conditions (invalid chars, unterminated strings)
   - Test location tracking accuracy
   - Test >80% code coverage (NFR-M2)

8. **Integration tests**
   - Tokenize complete `.pg` files from examples
   - Verify token stream matches expected output
   - Test error recovery on malformed input

## Tasks / Subtasks

- [ ] Set up logos dependency (AC: #1)
  - [ ] Add `logos = "0.14"` to polyglot-lexer/Cargo.toml
  - [ ] Verify workspace dependency inheritance works
  - [ ] Run `cargo build` to confirm logos compiles

- [ ] Update TokenType enum for logos (AC: #1, #2)
  - [ ] Add `#[derive(Logos, Debug, Clone, PartialEq)]` to TokenType
  - [ ] Add regex patterns for each token type using `#[token(...)]` or `#[regex(...)]`
  - [ ] Define all 27+ block markers with `#[token("[|]")]` syntax
  - [ ] Define all 25+ operators with appropriate patterns
  - [ ] Define identifier pattern: `#[regex(r"[a-z_][a-z0-9_]*")]`
  - [ ] Define string literal pattern with escape sequence support
  - [ ] Define number patterns (integers: `#[regex(r"-?[0-9]+")]`, floats: `#[regex(r"-?[0-9]+\.[0-9]+")]`)
  - [ ] Define comment patterns (single-line and multi-line)
  - [ ] Add `#[error]` variant for invalid tokens
  - [ ] Handle whitespace with `#[regex(r"[ \t\n]+", logos::skip)]`

- [ ] Create Lexer struct (AC: #1, #6)
  - [ ] Define `Lexer` struct wrapping `logos::Lexer<TokenType>`
  - [ ] Add source string reference and position tracking
  - [ ] Implement `Lexer::new(source: &str) -> Self` constructor
  - [ ] Add helper fields for line/column tracking

- [ ] Implement location tracking (AC: #3)
  - [ ] Track current line and column during tokenization
  - [ ] Update line number on newline characters
  - [ ] Reset column on newline, increment on other chars
  - [ ] Attach `Location { line, column }` to each Token
  - [ ] Handle multi-line tokens (strings, comments) correctly

- [ ] Implement tokenize method (AC: #1, #2, #6)
  - [ ] Implement `tokenize(&mut self) -> Vec<Result<Token, LexerError>>`
  - [ ] Loop through logos lexer, collecting tokens
  - [ ] Convert logos tokens to Token struct with location
  - [ ] Handle `#[error]` variant → map to LexerError
  - [ ] Continue after errors (collect all, don't stop at first)

- [ ] Implement Iterator trait (AC: #1, #6)
  - [ ] Implement `Iterator` for `Lexer`
  - [ ] `type Item = Result<Token, LexerError>`
  - [ ] Implement `next()` method for lazy tokenization
  - [ ] Add EOF token when input exhausted

- [ ] Implement error handling (AC: #4)
  - [ ] Map logos error to `LexerError::UnexpectedCharacter`
  - [ ] Detect unterminated strings → `LexerError::UnterminatedString`
  - [ ] Detect invalid numbers → `LexerError::InvalidNumber`
  - [ ] Include source location and context in all errors
  - [ ] Add helpful error messages (e.g., "Expected closing quote")

- [ ] Handle special cases (AC: #2)
  - [ ] Multi-character operators: Disambiguate `<`, `<<`, `<~`, `<=?`
  - [ ] String escape sequences: `\n`, `\t`, `\"`, `\\`
  - [ ] Comments: Skip single-line `//` and multi-line `/* */`
  - [ ] Whitespace: Skip spaces, tabs, track newlines for location
  - [ ] Reserved enumerations: Recognize `#Boolean.True` as separate tokens

- [ ] Write comprehensive unit tests (AC: #7)
  - [ ] Test each block marker tokenizes correctly
  - [ ] Test each operator tokenizes correctly
  - [ ] Test identifier patterns (valid: `user_name`, invalid: `123start`, `User-Name`)
  - [ ] Test string literals with escape sequences
  - [ ] Test number literals (integers, floats, negative numbers)
  - [ ] Test comment handling (single-line, multi-line, nested)
  - [ ] Test location tracking accuracy (line/column for each token)
  - [ ] Test error conditions:
    - [ ] Unexpected character (e.g., `$`, `%`)
    - [ ] Unterminated string (`"hello` missing close quote)
    - [ ] Invalid number (`12.34.56`)
  - [ ] Test edge cases:
    - [ ] Empty input → EOF token only
    - [ ] Only whitespace → EOF token
    - [ ] Only comments → EOF token
    - [ ] Multi-character operator disambiguation (`<` vs `<<` vs `<~`)
  - [ ] Verify >80% test coverage with `cargo tarpaulin` or `cargo llvm-cov`

- [ ] Write integration tests (AC: #8)
  - [ ] Create `tests/lexer_tests.rs` in polyglot-lexer
  - [ ] Test tokenizing complete `.pg` example files:
    - [ ] `examples/hello_world.pg` (if exists)
    - [ ] Simple pipeline with variables
    - [ ] Pipeline with conditionals and error handling
  - [ ] Verify token stream matches expected output
  - [ ] Test error recovery on malformed input

- [ ] Performance benchmarking (AC: #5)
  - [ ] Add `criterion` dev-dependency to Cargo.toml
  - [ ] Create `benches/lexer_bench.rs`
  - [ ] Benchmark tokenizing 1000-line file
  - [ ] Verify <100ms performance (NFR-P1)
  - [ ] Benchmark token-by-token iteration vs bulk tokenize
  - [ ] Document performance results in story completion notes

- [ ] API documentation (AC: #6)
  - [ ] Add rustdoc comments to `Lexer` struct
  - [ ] Document `new()` constructor with examples
  - [ ] Document `tokenize()` method with usage
  - [ ] Document `Iterator` implementation
  - [ ] Add module-level documentation to `lexer.rs`
  - [ ] Include example code in docs showing typical usage
  - [ ] Run `cargo doc --open` to verify docs render correctly

- [ ] Update module exports (AC: #1, #6)
  - [ ] Export `Lexer` from `lib.rs`
  - [ ] Ensure `Token`, `TokenType`, `LexerError` already exported (from Story 1.2)
  - [ ] Add re-export: `pub use lexer::Lexer;`
  - [ ] Verify `cargo clippy` passes with no warnings
  - [ ] Run `cargo build` to verify everything compiles

## Dev Notes

### Architecture Context

**From Architecture Document** [Source: docs/technical/architecture.md]

- **Crate**: `polyglot-lexer` (library crate)
- **Technology Decision (ADR-013)**: Use `logos` crate (version 0.14) for lexer generation
  - Declarative token definitions via proc macros
  - Compile-time DFA generation (fast!)
  - Battle-tested (used by tree-sitter, rustpython)
  - Proven to meet <100ms requirement (NFR-P1)
- **Error Handling (ADR-004)**: Use `thiserror` for `LexerError` (already defined in Story 1.2)
- **Testing**: Unit tests in `#[cfg(test)]` modules, integration tests in `tests/`, target >80% coverage
- **Performance Target (NFR-P1)**: <100ms for 1000-line files

### Learnings from Previous Stories

**From Story 1.1 (Status: done)** [Source: docs/project/stories/1-1-project-workspace-build-system-setup.md]

- Workspace structure created: `polyglot-lexer/` crate exists
- Dependencies configured via workspace inheritance
- CI/CD pipeline configured: `.github/workflows/ci.yml`
- Clippy enforces `-D warnings` (no warnings allowed)

**From Story 1.2 (Status: drafted)** [Source: docs/project/stories/1-2-lexer-token-definitions.md]

- `Token` struct defined with `type`, `location`, `lexeme` fields
- `TokenType` enum defined with 60+ variants (27+ block markers, 25+ operators, literals, etc.)
- `LexerError` enum defined using `thiserror`
- **CRITICAL UPDATE (2025-11-24)**: Added default assignment operators `<~` (default from left) and `~>` (default to right)
- All tokens implement `Debug`, `Clone`, `PartialEq` traits

### Project Structure After Implementation

```
polyglot-lexer/
├── Cargo.toml                    # Add logos = "0.14" dependency
├── src/
│   ├── lib.rs                    # Export Lexer, Token, TokenType, LexerError
│   ├── token.rs                  # ✅ From Story 1.2 (Token, TokenType with logos annotations)
│   ├── error.rs                  # ✅ From Story 1.2 (LexerError)
│   └── lexer.rs                  # NEW: Lexer struct and implementation
├── tests/
│   └── lexer_tests.rs            # NEW: Integration tests
└── benches/
    └── lexer_bench.rs            # NEW: Performance benchmarks
```

### Implementation Guidance

#### 1. Logos Integration Pattern

**Add logos attributes to TokenType enum:**

```rust
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenType {
    // Block markers
    #[token("[|]")]
    PipelineStart,

    #[token("[X]")]
    EndMarker,

    #[token("[i]")]
    InputBlock,

    // Multi-character operators (order matters!)
    #[token("<<")]
    PushInto,

    #[token("<~")]
    DefaultFromLeft,    // NEW: Added 2025-11-24

    #[token("<=?")]
    LessThanOrEqual,

    #[token("<")]
    LessThan,

    // Identifiers
    #[regex(r"[a-z_][a-z0-9_]*", priority = 1)]
    Identifier,

    // String literals (simplified - may need custom callback for escapes)
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    // Numbers
    #[regex(r"-?[0-9]+\.[0-9]+")]
    FloatLiteral,

    #[regex(r"-?[0-9]+")]
    IntLiteral,

    // Comments (skip)
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,

    // Whitespace (skip, but track newlines for location)
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,

    #[token("\n")]
    Newline,

    // Error token
    #[error]
    Error,

    // EOF (not from logos, added manually)
    Eof,
}
```

**CRITICAL:** Multi-character operators must be defined **before** their prefixes to avoid ambiguity. Logos matches longest pattern first.

---

#### 2. Lexer Structure

```rust
use logos::Logos;

pub struct Lexer<'source> {
    logos_lexer: logos::Lexer<'source, TokenType>,
    source: &'source str,
    line: usize,
    column: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            logos_lexer: TokenType::lexer(source),
            source,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Result<Token, LexerError>> {
        let mut tokens = Vec::new();

        while let Some(token_type) = self.logos_lexer.next() {
            let location = Location {
                line: self.line,
                column: self.column,
            };

            let lexeme = self.logos_lexer.slice();

            match token_type {
                Ok(TokenType::Error) => {
                    tokens.push(Err(LexerError::UnexpectedCharacter {
                        char: lexeme.chars().next().unwrap(),
                        location,
                    }));
                }
                Ok(token_type) => {
                    tokens.push(Ok(Token {
                        token_type,
                        location,
                        lexeme: lexeme.to_string(),
                    }));
                }
                Err(_) => {
                    tokens.push(Err(LexerError::UnexpectedCharacter {
                        char: lexeme.chars().next().unwrap_or('?'),
                        location,
                    }));
                }
            }

            // Update location
            self.update_location(lexeme);
        }

        // Add EOF token
        tokens.push(Ok(Token {
            token_type: TokenType::Eof,
            location: Location {
                line: self.line,
                column: self.column,
            },
            lexeme: String::new(),
        }));

        tokens
    }

    fn update_location(&mut self, lexeme: &str) {
        for ch in lexeme.chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}
```

---

#### 3. Iterator Implementation

```rust
impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.logos_lexer.next() {
            Some(Ok(TokenType::Error)) => {
                let location = Location { line: self.line, column: self.column };
                let lexeme = self.logos_lexer.slice();
                self.update_location(lexeme);
                Some(Err(LexerError::UnexpectedCharacter {
                    char: lexeme.chars().next().unwrap(),
                    location,
                }))
            }
            Some(Ok(token_type)) => {
                let location = Location { line: self.line, column: self.column };
                let lexeme = self.logos_lexer.slice().to_string();
                self.update_location(&lexeme);
                Some(Ok(Token { token_type, location, lexeme }))
            }
            Some(Err(_)) => {
                let location = Location { line: self.line, column: self.column };
                let lexeme = self.logos_lexer.slice();
                self.update_location(lexeme);
                Some(Err(LexerError::UnexpectedCharacter {
                    char: lexeme.chars().next().unwrap_or('?'),
                    location,
                }))
            }
            None => {
                // Return EOF once, then None
                if self.line > 0 {
                    self.line = 0; // Mark as exhausted
                    Some(Ok(Token {
                        token_type: TokenType::Eof,
                        location: Location { line: 1, column: 1 },
                        lexeme: String::new(),
                    }))
                } else {
                    None
                }
            }
        }
    }
}
```

---

#### 4. Testing Strategy

**Unit Tests Pattern:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_pipeline() {
        let source = "[|] MyPipeline\n[i] .input: pg\\string\n[X]";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.tokenize();

        assert_eq!(tokens.len(), 8); // [|], Identifier, Newline, [i], ...
        assert!(tokens[0].is_ok());
        assert_eq!(tokens[0].as_ref().unwrap().token_type, TokenType::PipelineStart);
    }

    #[test]
    fn test_multi_char_operator_disambiguation() {
        let source = "< << <~ <=?";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.tokenize().into_iter()
            .filter_map(|t| t.ok())
            .collect();

        assert_eq!(tokens[0].token_type, TokenType::LessThan);
        assert_eq!(tokens[1].token_type, TokenType::PushInto);
        assert_eq!(tokens[2].token_type, TokenType::DefaultFromLeft);
        assert_eq!(tokens[3].token_type, TokenType::LessThanOrEqual);
    }

    #[test]
    fn test_location_tracking() {
        let source = "line1\nline2\nline3";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.tokenize().into_iter()
            .filter_map(|t| t.ok())
            .collect();

        assert_eq!(tokens[0].location.line, 1);
        assert_eq!(tokens[2].location.line, 2);
        assert_eq!(tokens[4].location.line, 3);
    }

    #[test]
    fn test_unterminated_string_error() {
        let source = r#""hello"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(
            t,
            Err(LexerError::UnterminatedString { .. })
        )));
    }
}
```

---

#### 5. Performance Benchmarking

**Create `benches/lexer_bench.rs`:**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polyglot_lexer::Lexer;

fn benchmark_1000_line_file(c: &mut Criterion) {
    // Generate 1000-line test file
    let source = (0..1000)
        .map(|i| format!("[|] Pipeline{}\n[i] .input{}: pg\\string\n[X]\n", i, i))
        .collect::<String>();

    c.bench_function("tokenize_1000_lines", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&source));
            lexer.tokenize()
        });
    });
}

criterion_group!(benches, benchmark_1000_line_file);
criterion_main!(benches);
```

**Run with:** `cargo bench`

**Target:** <100ms for 1000-line file (NFR-P1)

---

#### 6. String Escape Sequences

Logos doesn't handle escape sequences automatically. You may need a custom callback:

```rust
#[regex(r#""([^"\\]|\\.)*""#, parse_string)]
StringLiteral,

fn parse_string(lex: &mut logos::Lexer<TokenType>) -> Option<String> {
    let slice = lex.slice();
    let inner = &slice[1..slice.len()-1]; // Remove quotes

    // Handle escape sequences
    let mut result = String::new();
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }

    Some(result)
}
```

**Note:** This adds complexity. Consider deferring string escape handling to parser if it becomes blocking.

---

#### 7. Common Pitfalls

**❌ Operator Ordering:**
```rust
#[token("<")]      // This matches first!
LessThan,

#[token("<<")]     // This NEVER matches (< already consumed)
PushInto,
```

**✅ Correct Ordering:**
```rust
#[token("<<")]     // Longest match first
PushInto,

#[token("<~")]     // Next longest
DefaultFromLeft,

#[token("<=?")]    // Next longest
LessThanOrEqual,

#[token("<")]      // Shortest last
LessThan,
```

**❌ Newline Tracking:**
```rust
#[regex(r"[ \t\n]+", logos::skip)]  // Skips newlines = lost location tracking!
```

**✅ Correct Newline Handling:**
```rust
#[regex(r"[ \t]+", logos::skip)]    // Skip spaces/tabs only
Whitespace,

#[token("\n")]                       // Track newlines separately
Newline,
```

---

### Performance Considerations

1. **Single-Pass Tokenization**: Logos generates DFA - no backtracking needed
2. **Minimize Allocations**: Use `&str` slices where possible, clone only when necessary
3. **Location Tracking**: Update line/column during iteration, not after
4. **Error Collection**: Don't stop at first error - collect all for better UX

### References

- [Source: docs/technical/architecture.md#ADR-013-Logos-Lexer-Generator]
- [Source: docs/technical/architecture.md#Technology-Stack-Details]
- [Source: docs/project/epics.md#Story-1.3]
- [Source: docs/project/stories/1-2-lexer-token-definitions.md] - Token definitions
- [Logos Documentation: https://docs.rs/logos/]

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by story-context workflow -->

### Agent Model Used

_To be filled by dev agent_

### Debug Log References

_To be filled by dev agent during implementation_

### Completion Notes List

_To be filled by dev agent upon story completion_

### File List

_To be filled by dev agent with files created/modified/deleted_
