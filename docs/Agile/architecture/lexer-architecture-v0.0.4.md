# Lexer Architecture - Polyglot v0.0.4

**Author:** Winston (Architect)
**Date:** 2025-12-29
**Version:** v0.0.4
**Status:** Architecture Complete - Ready for Implementation

---

## Executive Summary

This document defines the complete lexer architecture for Polyglot v0.0.4 syntax, including all 124 token types, state machine design, disambiguation algorithms, and implementation strategy. This extends the v0.0.3 lexer with support for pipeline composition (`|>`), trigger I/O wiring (`>output >> <input`), and enhanced operator support.

**Key Architecture Decisions:**
- **124 Total Token Types** (121 original + 3 new for v0.0.4)
- **State Machine Lexer** with 6 states for context-sensitive tokenization
- **Maximal Munch Algorithm** with 3-character lookahead
- **Indentation Tracking** (loop bodies only - rest of language whitespace-insensitive)
- **Two-Character Lookahead** for operator disambiguation

---

## Token Design

### Complete Token Inventory (124 Types)

#### 1. Block Markers (30 tokens)
```rust
// Package & Structure
BlockPackageStart,     // {@}
BlockPipelineStart,    // {|}
BlockEnumStart,        // {#}
BlockEnd,              // {x}

// Execution Flow
MarkerRun,             // [r]
MarkerParallel,        // [p]
MarkerFork,            // [f]
MarkerLoop,            // [~] (unpack)
MarkerPack,            // [*] (pack)

// I/O & Pipeline
MarkerPipelineDef,     // [|]
ParenPipeline,         // (|) - for triggers/composition
ParenLoop,             // (~) - loop parameters
ParenPack,             // (*) - pack parameters
MarkerInput,           // [i]
MarkerOutput,          // [o]

// Data & Types
MarkerEnum,            // [#]
MarkerAlias,           // [A]
MarkerError,           // [!]
MarkerSubfield,        // [.] (for field access)
MarkerSerialLoad,      // [s] (NEW in v0.0.4)

// Services & Runtime
MarkerTrigger,         // [t]
MarkerQueue,           // [Q]
MarkerWrapper,         // [W]

// Control Flow
MarkerSwitch,          // [?]
MarkerMatch,           // [v] (NEW in v0.0.4)
```

#### 2. Operators (45 tokens)

**Assignment Operators (4):**
```rust
OpAssignPullFinal,     // <<
OpAssignPullDefault,   // <~
OpAssignPushFinal,     // >>
OpAssignPushDefault,   // ~>
```

**Comparison Operators (10):**
```rust
OpCompareEqual,        // =?
OpCompareGreater,      // >?
OpCompareLess,         // <?
OpCompareGreaterEq,    // >=?
OpCompareLessEq,       // <=?
OpCompareNotEqual,     // =!?
OpCompareNotGreater,   // >!?
OpCompareNotLess,      // <!?
OpCompareNotGreaterEq, // >=!?
OpCompareNotLessEq,    // <=!?
```

**Logical Operators (6):**
```rust
OpLogicalAnd,          // &?
OpLogicalOr,           // |?
OpLogicalNot,          // !?
OpLogicalXor,          // ^?
OpLogicalNand,         // !&?
OpLogicalNor,          // !|?
```

**Collection Operators (6):**
```rust
OpIn,                  // in?
OpNotIn,               // in!?
OpContains,            // *?
OpNotContains,         // *!?
OpRegexMatch,          // re?
OpRegexNotMatch,       // re!?
```

**Range Operators (4):**
```rust
OpRangeInclusive,      // ?[...]
OpRangeExclusive,      // ?(...)
RangeBracketStart,     // ?[
RangeParenStart,       // ?(
```

**Data Flow Operators (8):**
```rust
OpUnpack,              // ~ (standalone)
OpPipe,                // |
OpWire,                // >> (output-to-input wiring)
OpDoubleAssign,        // <<<
OpDoubleUnpack,        // >>>
OpTernaryAssign,       // <~<
OpTernaryUnpack,       // ~>~
```

**NEW v0.0.4 Operators (3):**
```rust
PipelineComposition,   // |>
Input,                 // <param (input parameter marker)
Output,                // >param (output parameter marker)
```

**Special Operators (4):**
```rust
PrefixMeta,            // % (metadata prefix)
SemicolonReserved,     // ; (reserved enum indication)
SemicolonPlain,        // ; (regular semicolon for custom extensions)
```

#### 3. Prefixes (7 tokens)
```rust
PrefixVariable,        // $
PrefixType,            // :
PrefixEnum,            // #
PrefixPipeline,        // |
PrefixError,           // !
PrefixPackage,         // @
PrefixMeta,            // %
```

#### 4. Identifiers & Literals (8 tokens)
```rust
Identifier,            // variable_name, Pipeline_Name
StringLiteral,         // "text with {$interpolation}"
NumberLiteral,         // 42, 3.14
BooleanTrue,           // #Boolean.True or #True
BooleanFalse,          // #Boolean.False or #False
PathLiteral,           // /path/to/file
EnumValue,             // #Type.Value
```

#### 5. Structural (5 tokens)
```rust
Dot,                   // .
Comma,                 // ,
Colon,                 // :
ParenOpen,             // (
ParenClose,            // )
BracketOpen,           // [
BracketClose,          // ]
CurlyOpen,             // {
CurlyClose,            // }
```

#### 6. Whitespace & Comments (4 tokens)
```rust
Whitespace,            // spaces, tabs
Newline,               // \n
Comment,               // // line or /* block */
Indent,                // Indentation increase (loop bodies only)
Dedent,                // Indentation decrease (loop bodies only)
EOF,                   // End of file
```

**Total:** 124 token types

---

## State Machine Design

### States

```rust
enum LexerState {
    Initial,           // Default state
    InString,          // Inside string literal, tracking interpolation
    InPath,            // Inside path literal
    InComment,         // Inside comment
    InBlockMarker,     // After '[', determining marker type
    InParenIO,         // After '(', determining (|), (~), or (*)
    InMultilineCode,   // Inside multiline code block (future)
}
```

### State Transitions

```
Initial
  ├─ '"' → InString
  ├─ '/' → InPath (if /path) or Comment (if //)
  ├─ '[' → InBlockMarker
  ├─ '(' → InParenIO
  ├─ '<' → check for <<, <~, <?, <=?, or < (lookahead required)
  ├─ '>' → check for >>, ~>, >?, >=?, or > (lookahead required)
  ├─ '|' → check for |>, |?, or |Pipeline (lookahead required)
  ├─ digit → NumberLiteral
  ├─ alpha → Identifier (then check for keywords)
  └─ other → Single-char token or error

InString
  ├─ '{' → Begin interpolation (nested)
  ├─ '}' → End interpolation
  ├─ '"' → End string, return to Initial
  └─ '\\' → Escape sequence

InPath
  ├─ whitespace → End path, return to Initial
  └─ continue path characters

InBlockMarker
  ├─ ']' → Determine marker type based on content
  └─ error if invalid marker

InParenIO
  ├─ '|)' → ParenPipeline
  ├─ '~)' → ParenLoop
  ├─ '*)' → ParenPack
  └─ error or regular parenthesis expression
```

---

## Critical Disambiguation Algorithms

### 1. Pipeline Composition vs. Pipeline Call

**Challenge:** Distinguish `|>` (composition) from `|Pipeline` (call)

```rust
fn lex_pipe_or_composition(&mut self) -> Result<Token, LexerError> {
    self.advance(); // Consume '|'

    match self.peek_char() {
        Some('>') => {
            self.advance();
            Ok(Token::new(TokenKind::PipelineComposition, "|>", ...))
        }
        Some('?') => {
            self.advance();
            Ok(Token::new(TokenKind::OpLogicalOr, "|?", ...))
        }
        Some(ch) if is_identifier_start(ch) => {
            let name = self.lex_identifier();
            Ok(Token::new(TokenKind::OpPipe, format!("|{}", name), ...))
        }
        _ => Err(LexerError::InvalidPipeOperator)
    }
}
```

### 2. Input/Output Parameters vs. Comparison Operators

**Challenge:** Distinguish `<param` (input) from `<?` (comparison)

```rust
fn lex_angle_bracket(&mut self) -> Result<Token, LexerError> {
    let ch = self.current_char();
    self.advance();

    let next = self.peek_char();
    let next2 = self.peek_char_at(1);

    match (ch, next, next2) {
        ('<', Some('<'), Some('<')) => {
            // <<<
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::OpDoubleAssign, "<<<", ...))
        }
        ('<', Some('<'), _) => {
            // <<
            self.advance();
            Ok(Token::new(TokenKind::OpAssignPullFinal, "<<", ...))
        }
        ('<', Some('~'), Some('<')) => {
            // <~<
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::OpTernaryAssign, "<~<", ...))
        }
        ('<', Some('~'), _) => {
            // <~
            self.advance();
            Ok(Token::new(TokenKind::OpAssignPullDefault, "<~", ...))
        }
        ('<', Some('='), Some('?')) => {
            // <=?
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::OpCompareLessEq, "<=?", ...))
        }
        ('<', Some('!'), Some('?')) => {
            // <!?
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::OpCompareNotLess, "<!?", ...))
        }
        ('<', Some('?'), _) => {
            // <?
            self.advance();
            Ok(Token::new(TokenKind::OpCompareLess, "<?", ...))
        }
        ('<', Some(ch), _) if is_identifier_start(ch) => {
            // <parameter_name (input parameter)
            let param_name = self.lex_identifier();
            Ok(Token::new(TokenKind::Input, format!("<{}", param_name), ...))
        }
        _ => Err(LexerError::InvalidAngleBracket)
    }
}
```

### 3. Block Markers vs. Parenthesis I/O

**Challenge:** Distinguish `[|]` from `(|)`, `(~)`, `(*)`

```rust
fn lex_paren_or_io(&mut self) -> Result<Token, LexerError> {
    if self.current_char() != '(' {
        return Err(LexerError::ExpectedOpenParen);
    }
    self.advance();

    match (self.current_char(), self.peek_char()) {
        ('|', Some(')')) => {
            // (|)
            self.advance(); // consume '|'
            self.advance(); // consume ')'
            Ok(Token::new(TokenKind::ParenPipeline, "(|)", ...))
        }
        ('~', Some(')')) => {
            // (~)
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::ParenLoop, "(~)", ...))
        }
        ('*', Some(')')) => {
            // (*)
            self.advance();
            self.advance();
            Ok(Token::new(TokenKind::ParenPack, "(*)", ...))
        }
        _ => {
            // Regular parenthesis - backtrack
            Ok(Token::new(TokenKind::ParenOpen, "(", ...))
        }
    }
}
```

---

## Indentation Handling

**Critical:** Polyglot is whitespace-insensitive EXCEPT in loop bodies.

### Context Tracking

```rust
struct Lexer {
    in_loop_body: bool,
    indent_stack: Vec<usize>,
    current_indent: usize,
}

impl Lexer {
    fn enter_loop_body(&mut self) {
        self.in_loop_body = true;
        self.indent_stack.clear();
        self.indent_stack.push(0); // Base indentation
    }

    fn exit_loop_body(&mut self) {
        self.in_loop_body = false;
        self.indent_stack.clear();
    }

    fn lex_indentation(&mut self) -> Vec<Token> {
        if !self.in_loop_body {
            return vec![]; // Ignore indentation outside loops
        }

        let indent_level = self.count_leading_whitespace();
        let previous_indent = *self.indent_stack.last().unwrap();

        if indent_level > previous_indent {
            // INDENT
            self.indent_stack.push(indent_level);
            vec![Token::new(TokenKind::Indent, "", ...)]
        } else if indent_level < previous_indent {
            // DEDENT(s)
            let mut tokens = vec![];
            while let Some(&level) = self.indent_stack.last() {
                if level <= indent_level {
                    break;
                }
                self.indent_stack.pop();
                tokens.push(Token::new(TokenKind::Dedent, "", ...));
            }
            tokens
        } else {
            // Same level
            vec![]
        }
    }
}
```

### Loop Body Detection

```rust
// When lexer encounters [~] or (*):
[p] ~ForEach.Array  // Start loop
    [r] |DoSomething  // Indented - IN LOOP BODY
    [r] |DoMore       // Indented - IN LOOP BODY
[*] *Into.Array     // End loop - exit loop body
```

---

## String Interpolation

### Nested Interpolation Tracking

```rust
fn lex_string(&mut self) -> Result<Token, LexerError> {
    let mut buffer = String::new();
    let mut interpolations = vec![];
    let mut brace_depth = 0;

    self.advance(); // Skip opening "

    while let Some(ch) = self.current_char() {
        match ch {
            '"' if brace_depth == 0 => {
                self.advance();
                break;
            }
            '\\' => {
                // Escape sequence
                self.advance();
                if let Some(escaped) = self.current_char() {
                    buffer.push(self.unescape(escaped));
                    self.advance();
                }
            }
            '{' if self.peek_char() == Some('$') => {
                // Start interpolation: {$variable}
                self.advance(); // {
                self.advance(); // $
                let var_name = self.lex_identifier();
                interpolations.push(Interpolation {
                    variable: var_name,
                    position: buffer.len(),
                });
                self.expect('}')?;
                brace_depth = 0;
            }
            _ => {
                buffer.push(ch);
                self.advance();
            }
        }
    }

    Ok(Token::new(
        TokenKind::StringLiteral,
        buffer,
        interpolations,
        ...
    ))
}
```

---

## Type Annotation Parsing

**Critical:** Nested types require DOUBLE `pg.` prefix

### Example Type Annotations

```polyglot
:pg.string              // Simple type
:pg.int                 // Simple type
:pg.array.pg.string     // Array of strings - DOUBLE pg. prefix
:pg.array.pg.path       // Array of paths - DOUBLE pg. prefix
:pg.serial              // Serial (JSON-like) data
```

### Lexer Strategy

Lexer treats `:` as `PrefixType`, then tokenizes:
- `pg` → Identifier
- `.` → Dot
- `array` → Identifier
- `.` → Dot
- `pg` → Identifier (SECOND pg.)
- `.` → Dot
- `string` → Identifier

Parser validates the double-prefix requirement.

---

## Error Handling Strategy

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedChar(char, usize, usize),

    #[error("Unclosed string literal at line {0}")]
    UnclosedString(usize),

    #[error("Invalid escape sequence '\\{0}' at line {1}")]
    InvalidEscape(char, usize),

    #[error("Invalid block marker '[{0}]' at line {1}")]
    InvalidBlockMarker(String, usize),

    #[error("Inconsistent indentation at line {0} (expected {1}, found {2})")]
    InconsistentIndentation(usize, usize, usize),

    #[error("Invalid pipe operator at line {0}, column {1}")]
    InvalidPipeOperator,
}
```

### Error Recovery

1. **Character Level:** Skip invalid character, continue lexing
2. **Token Level:** Emit error token, synchronize to next statement
3. **Collect Multiple Errors:** Don't stop at first error

---

## Performance Targets

- **Lexing Speed:** <100ms for 1,000-line files
- **Memory:** <10MB for 10,000-line files
- **Lookahead:** Maximum 3 characters (no backtracking)

---

## Implementation Phases

### Phase 1: Foundation (Week 1)
- Token enum definitions (124 types)
- Basic lexer structure with state machine
- Single-character tokens

### Phase 2: Multi-Character Operators (Week 2)
- Implement 3-character lookahead
- Disambiguation algorithms for `<<`, `>>`, `|>`, etc.
- Maximal munch for operators

### Phase 3: Context-Sensitive Tokenization (Week 3)
- String interpolation
- Block markers `[X]` vs. `(X)`
- Input/Output parameters `<param` vs. `>param`

### Phase 4: Indentation Handling (Week 4)
- Loop body detection
- INDENT/DEDENT token generation
- Indentation validation

### Phase 5: Error Handling & Testing (Week 5)
- Comprehensive error messages
- Error recovery strategies
- Integration tests with parser

---

## Testing Strategy

### Unit Tests (Per Token Type)
- Valid token recognition
- Invalid input rejection
- Edge cases (empty strings, max int, etc.)

### Integration Tests (Token Sequences)
- Complete pipeline tokenization
- Operator precedence scenarios
- Indentation in loops

### Performance Tests
- 1,000-line file benchmark
- 10,000-line file stress test
- Memory usage profiling

### Error Handling Tests
- Unclosed strings
- Invalid characters
- Inconsistent indentation
- Multiple errors in single file

---

## Dependencies

**Rust Crates:**
- `thiserror` 2.0.17 - Error types
- `logos` (optional) - Fast tokenization (evaluate performance)
- `unicode-xid` - Unicode identifier validation

**Internal Dependencies:**
- None (lexer is foundation layer)

---

## References

- [v0.0.4 Specification](../../User/reference/grammar.md)
- [Polly Examples](../../../bmad-polly/data/memory/)
- [Epic 13 - v0.0.4 Migration](../epics.md#epic-13-v004-syntax-migration)

---

**Status:** Architecture Complete - Ready for Implementation
**Next Step:** Create detailed implementation stories (Story 13.1-13.5)
