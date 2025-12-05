# Story 1.2: Lexer Token Definitions

Status: drafted

## Story

As a developer,
I want token types defined for all Polyglot operators and constructs,
so that the lexer can tokenize `.pg` files correctly.

## Acceptance Criteria

1. **Token enum complete**
   - Defined in `polyglot-lexer/src/token.rs`
   - **Block Markers** (square-bracket syntax):
     - Core: `[|]` (Pipeline def), `[X]` (End), `[i]` (Input), `[o]` (Output)
     - Execution: `[r]` (Run sequential), `[p]` (Parallel), `[~]` (Expansion prefix)
     - Data flow: `[<]` (Pass input/Define field), `[>]` (Pass output)
     - Synchronization: `[Y]` (Join)
     - Services: `[t]` (Trigger), `[Q]` (Queue), `[W]` (Wrapper)
     - Types: `[#]` (Enumeration def), `[!]` (Error def/catch), `[A]` (Alias)
     - Control: `[?]` (Switch/conditional)
     - New in v0.0.2: `[=]` (Fixed/immutable), `[*]` (Line continuation), `[+]` (OR), `[&]` (AND), `[^]` (XOR), `[.]` (Group), `[{]` (Scope input), `[}]` (Scope output)
   - **Operators** (symbolic syntax):
     - Pipeline: `|` (call pipeline)
     - Data: `~` (unpack), `<<` (push into), `>>` (pull from), `<~` (default from left), `~>` (default to right)
     - Reference: `@` (package access), `#` (enumeration marker), `!` (error marker)
     - Comparison: `=?`, `>?`, `<?`, `>=?`, `<=?` (comparison operators)
     - Negation: `=!?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `<!?<` (negation operators)
     - Range: `?[`, `?(`, `]`, `)` (range operators)
     - Pattern: `?*` (wildcard), `?re` (regex pattern)
   - **Identifiers and Literals**:
     - Identifiers (snake_case, alphanumeric + underscore)
     - Strings (with escape sequences: `\n`, `\t`, `\"`, `\\`)
     - Numbers (integers, floats)
     - Booleans: `#Boolean.True`, `#Boolean.False` (or `#True`, `#False`)
   - **Structural Tokens**:
     - Comments (single-line `//`, multi-line `/* */`)
     - Whitespace, newline
     - EOF marker

2. **Token structure**
   - Token type enum with all operator variants
   - Source location tracking (line, column)
   - Lexeme field (original text)
   - Implements `Debug`, `Clone`, `PartialEq` traits

3. **Error handling**
   - `LexerError` type defined using `thiserror`
   - Error types: UnexpectedCharacter, UnterminatedString, InvalidNumber
   - Errors include source location and context

4. **Unit tests**
   - Each operator tokenizes correctly
   - Edge cases handled (nested brackets, escaped characters)
   - Invalid characters rejected with appropriate errors
   - Test coverage >80% for token.rs module

## Tasks / Subtasks

- [ ] Define Token enum in polyglot-lexer/src/token.rs (AC: #1, #2)
  - [ ] Create Token struct with type, location, and lexeme fields
  - [ ] Define TokenType enum with all operator variants
  - [ ] Add Location struct (line: usize, column: usize)
  - [ ] Implement Debug, Clone, PartialEq traits for Token

- [ ] Define all block marker token types (AC: #1)
  - [ ] Core structure: `[|]` (Pipeline def), `[X]` (End), `[i]` (Input), `[o]` (Output)
  - [ ] Execution control: `[r]` (Run), `[p]` (Parallel), `[~]` (Expansion), `[Y]` (Join)
  - [ ] Data flow: `[<]` (Pass input/Field), `[>]` (Pass output)
  - [ ] Services: `[t]` (Trigger), `[Q]` (Queue), `[W]` (Wrapper)
  - [ ] Type definitions: `[#]` (Enumeration), `[!]` (Error), `[A]` (Alias)
  - [ ] Control flow: `[?]` (Switch/conditional)
  - [ ] New v0.0.2 markers: `[=]`, `[*]`, `[+]`, `[&]`, `[^]`, `[.]`, `[{]`, `[}]`

- [ ] Define all operator token types (AC: #1)
  - [ ] Pipeline operators: `|` (call)
  - [ ] Data operators: `~` (unpack), `<<` (push), `>>` (pull), `<~` (default from left), `~>` (default to right)
  - [ ] Reference operators: `@` (package), `#` (enum marker), `!` (error marker)
  - [ ] Comparison operators: `=?`, `>?`, `<?`, `>=?`, `<=?`
  - [ ] Negation operators: `=!?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `<!?<`
  - [ ] Range operators: `?[`, `?(`, `]`, `)`
  - [ ] Pattern operators: `?*` (wildcard), `?re` (regex)

- [ ] Define identifier and literal token types (AC: #1)
  - [ ] Identifiers: snake_case, alphanumeric + underscore
  - [ ] Strings: with escape sequences `\n`, `\t`, `\"`, `\\`
  - [ ] Numbers: integers and floats
  - [ ] Booleans: `#Boolean.True`, `#Boolean.False` (enumeration-based, NO keywords)

- [ ] Define structural token types (AC: #1)
  - [ ] Comments: `//` (single-line), `/* */` (multi-line)
  - [ ] Whitespace, Newline, EOF

- [ ] Define LexerError type (AC: #3)
  - [ ] Create error.rs module in polyglot-lexer
  - [ ] Use thiserror to define LexerError enum
  - [ ] Add variants: UnexpectedCharacter, UnterminatedString, InvalidNumber
  - [ ] Include location and context in each error variant
  - [ ] Implement Display with helpful error messages
  - [ ] Ensure errors are Send + Sync for async compatibility

- [ ] Implement helper methods and traits (AC: #2)
  - [ ] Implement Display trait for Token (for debugging)
  - [ ] Add is_operator(), is_literal(), is_whitespace() helper methods
  - [ ] Add token_name() method returning &str for error messages
  - [ ] Implement From<&str> for TokenType for common operators

- [ ] Write comprehensive unit tests (AC: #4)
  - [ ] Test token creation for each operator type
  - [ ] Test location tracking accuracy
  - [ ] Test lexeme storage for all token types
  - [ ] Test error variants with appropriate messages
  - [ ] Test edge cases: empty strings, max numbers, special characters
  - [ ] Test Display implementation output
  - [ ] Verify >80% code coverage with cargo-tarpaulin or similar

- [ ] Update module exports (AC: #1, #2, #3)
  - [ ] Export Token, TokenType, Location from lib.rs
  - [ ] Export LexerError and error types
  - [ ] Add module-level documentation
  - [ ] Verify cargo clippy passes with no warnings

## Dev Notes

### Architecture Context

**From Architecture Document** [Source: docs/technical/architecture.md]

- **Crate**: `polyglot-lexer` (library crate, already created in Story 1.1)
- **Error Handling**: Use `thiserror` for structured error types per ADR-004
- **Critical Requirement**: All errors must be `Send + Sync` for async compatibility
- **Technology**: Rust 2021 Edition with workspace dependency inheritance
- **Testing**: Unit tests in `#[cfg(test)]` modules, target >80% coverage (NFR-M2)

### Learnings from Previous Story

**From Story 1.1 (Status: done)** [Source: docs/project/stories/1-1-project-workspace-build-system-setup.md]

- **Workspace Structure**: `polyglot-lexer` crate already created at `/polyglot-lexer/`
- **Existing Files**:
  - `polyglot-lexer/Cargo.toml` - Workspace dependencies configured (thiserror, serde available)
  - `polyglot-lexer/src/lib.rs` - Entry point with default test scaffolding
- **Error Handling Setup**: `thiserror 2.0.17` available via workspace inheritance
- **Testing Framework**: Cargo test infrastructure configured in CI/CD (.github/workflows/ci.yml)
- **Code Quality**: Clippy configured with `-D warnings`, enforces Rust best practices
- **Build Verification**: Use `cargo build`, `cargo test`, `cargo clippy` to verify changes

**Important**: Do NOT recreate the polyglot-lexer crate - it already exists. Add new modules (token.rs, error.rs) to the existing crate structure.

### Project Structure Notes

```
polyglot-lexer/
├── Cargo.toml                    # Already configured with workspace deps
├── src/
│   ├── lib.rs                    # Update: export Token, TokenType, LexerError
│   ├── token.rs                  # NEW: Token and TokenType definitions
│   └── error.rs                  # NEW: LexerError definition
└── tests/                        # Optional: integration tests
```

### Implementation Guidance

1. **Token Design Principles**:
   - Keep Token struct lightweight (it will be created millions of times)
   - Use `&str` for lexeme where possible (consider lifetime implications)
   - Source location is critical for error reporting - always track accurately

2. **v0.0.2 Syntax Reference**:
   - **CRITICAL DISTINCTION**: Block markers use square brackets `[X]`, operators use symbols `|`, `~`, `@`, etc.
   - Block markers define code structure (pipelines, blocks, execution modes)
   - Operators perform operations (call pipelines, access packages, transform data)
   - Qualified references use pipe notation: `|Module.Component`
   - Identifiers follow Rust naming: snake_case, alphanumeric + underscore
   - String literals support escape sequences: `\n`, `\t`, `\"`, `\\`
   - **NO KEYWORDS**: v0.0.2 eliminated all 5 keywords (True, False, Fixed, Default, Exposed)
     - True/False → `#Boolean.True` / `#Boolean.False` (reserved enumeration)
     - Fixed → `[=]` block marker (immutable assignment)
     - Default → inline `<<` in `[i]` declaration
     - Exposed → `[}]` scope output (macro-exported variables)

3. **Error Message Quality**:
   - Include source location in all errors
   - Provide helpful context (e.g., "Expected closing bracket")
   - Follow Rust error message conventions (lowercase, no trailing punctuation)

4. **Testing Strategy**:
   - Unit tests in `token.rs` and `error.rs` using `#[cfg(test)]` modules
   - Test each token type individually
   - Test error conditions thoroughly
   - Consider property-based testing with `proptest` for edge cases

5. **Performance Considerations**:
   - Token creation is on critical path - keep it fast
   - Avoid unnecessary allocations (use `&str` where lifetime permits)
   - Consider using `Cow<str>` for lexeme if needed
   - Performance target: Token creation should be O(1)

### References

- [Source: docs/project/epics.md#Story-1.2]
- [Source: docs/technical/architecture.md#Technology-Stack-Details]
- [Source: docs/technical/architecture.md#ADR-004-Error-Handling]
- [Source: docs/user/language/quick-start.md] - Syntax reference
- [Source: docs/project/stories/1-1-project-workspace-build-system-setup.md#File-List]

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
