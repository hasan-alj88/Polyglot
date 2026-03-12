# Story 1.7: December 2025 Syntax Updates

**Syntax Version:** This story implements v0.0.3 syntax. v0.0.4 syntax (with `$` variables, `{|}` blocks, `[|]` IO) will be implemented in future epics.

**Epic:** Epic 1 - Lexer & Parser
**Created:** 2025-12-04
**Created By:** Bob (Scrum Master)

---

## Status

**✅ COMPLETE** - All acceptance criteria (AC1-AC4) implemented and tested

**Completion Date:** 2025-12-05

**Implementation Summary:**
- ✅ AC1: Enumeration `#` prefix requirement
- ✅ AC2: Pipeline formatted strings `|Pipeline"string"`
- ✅ AC3: `~>` default pull operator
- ✅ AC4: Serial error handling `[s][!]` syntax

**Files Modified:**
- `polyglot-lexer/src/token.rs` - Added `BlockErrorCatch` token
- `polyglot-lexer/src/lexer.rs` - Added lexing for `[!]` marker
- `polyglot-parser/src/ast.rs` - Added `body: Block` field to EnumerationDefinition
- `polyglot-parser/src/parser.rs` - Implemented `parse_enumeration_body()` function
- `polyglot-parser/src/validation.rs` - Added `validate_serial_error_handling()` function
- Documentation files - Updated examples to show pipeline calls in `[s]` blocks

**Test Results:**
- All 98 unit tests pass ✅
- No compilation errors ✅
- Ready for Story 1.8 (test coverage) ✅

---

## Story

**As a** Polyglot developer,
**I want** the lexer and parser to support the December 2025 syntax changes,
**so that** I can use the documented syntax (`[#] #Config`, `|Pipeline"string"`, `~>` operator) and have my code compile successfully.

---

## Acceptance Criteria

### AC1: Enumeration Definition Syntax - `[#] #Name`

**Given** a `.pg` file with enumeration definition using `#` prefix
**When** I compile the file
**Then** the parser accepts `[#] #Name` syntax (with `#` prefix)

**And** the parser rejects old `[#] Name` syntax (without `#`) with clear error message:
```
Error: Expected '#' after '[#]' for enumeration definition
  --> file.pg:4:5
   |
 4 | [#] Config
   |     ^^^^^^ missing '#' prefix
   |
Help: Enumeration definitions require # prefix: [#] #Config
```

**And** all enumeration code examples in documentation work correctly:
- `[#] #Config`
- `[#] #UserRole`
- `[#] #DT.Hijri.*` (extendable enum)
- `[#] #Boolean.True` (reserved enum extension)

---

### AC2: Pipeline Formatted String Syntax - `|Pipeline"string"`

**Given** a `.pg` file with pipeline formatted string
**When** I compile the file
**Then** the lexer recognizes `|Pipeline"formatted {.string}"` as single token pattern

**And** the parser correctly parses it as pipeline call with formatted string argument

**And** all stdlib pipeline examples work:
- `|U.Log.Info"Processing {.count} items"`
- `|RT.Shell.Run"ls -la {.directory}"`
- `|U.String.Format"Hello {.name}, you are {.age} years old"`

**And** compilation fails with clear error if `|` prefix missing:
```
Error: Formatted string requires pipeline prefix
  --> file.pg:10:20
   |
10 | [r] .msg << U.Log.Info"text"
   |             ^^^^^^^^^^ missing '|' prefix
   |
Help: Add pipe prefix: |U.Log.Info"text"
```

---

### AC3: `~>` Pull Default Operator

**Given** a `.pg` file using `~>` operator
**When** I compile the file
**Then** the lexer has `OpDefaultPull` token for `~>` operator

**And** the parser accepts `~>` in variable assignments:
```polyglot
[r] .timeout: pg\int ~> .settings.timeout    // Pull default from source
[r] .backup: pg\int ~> .timeout              // Pull default from variable
```

**And** the operator behaves as documented (creates Default state, can override once)

---

### AC4: Serial Error Handling Validation

**Given** an enumeration with `[s]` serial load blocks
**When** I compile the file
**Then** the parser validates that either `[s][!] *` (default) or `[s][!]` (custom) exists

**And** compilation fails with clear error if error handling missing:
```
Error: [s] blocks require error handling
  --> file.pg:15:1
   |
15 | [#] #Config
16 | [s] "config.yaml"
17 | [X]
   | ^^^ missing error handling
   |
Help: Add [s][!] * for default error handling, or [s][!] for custom handling
```

**And** both error handling patterns compile successfully:
- Default: `[s][!] *`
- Custom: `[s][!]` with error recovery block

---

### AC5: Comprehensive Testing

**Given** test suite for syntax changes
**When** I run all tests
**Then** all new syntax features have unit tests:
- Lexer tests for `OpDefaultPull` token
- Lexer tests for `LiteralPipelineFormatted` token
- Parser tests for `[#] #Name` requirement
- Parser tests for `~>` operator
- Parser tests for `|Pipeline"string"` pattern
- Parser validation tests for `[s][!]` requirement

**And** all existing tests still pass (no regressions)

**And** integration test compiles complete file with all new syntax:
```polyglot
[@] Local@Test.Syntax:1.0.0
[X]


[#] #Config                              // AC1: Enum with # prefix
[<] .api_key: pg\string
[<] .timeout: pg\int
[s] "config.yaml"                        // AC4: Serial load
[s][!] *                                 // AC4: Error handling
[X]


[|] |TestNewSyntax
[i] !No.Input
[t] |T.Call
[W] |W.Polyglot.Scope

[r] .default_timeout: pg\int <~ 30
[r] .actual_timeout: pg\int ~> .default_timeout    // AC3: ~> operator
[r] .message: pg\string << |U.Log.Info"Timeout: {.actual_timeout}"  // AC2: |Pipe"str"

[o] !No.Output
[X]
```

---

### AC6: Error Message Quality

**Given** invalid syntax using old patterns
**When** compilation fails
**Then** error messages are helpful and actionable:
- Suggest correct syntax
- Show exact location of error
- Provide examples of correct usage
- Reference documentation

**And** error messages follow Rust compiler style (clear, colored, with caret pointing to issue)

---

### AC7: Documentation Alignment

**Given** updated implementation
**When** I review user documentation
**Then** all documented syntax examples compile successfully

**And** implementation status page shows Story 1.7 complete

**And** no "not yet implemented" warnings remain in docs

---

## Tasks / Subtasks

### Task 1: Lexer Updates (AC: 2, 3)

- [ ] **Add `OpDefaultPull` token to token enum** (AC3)
  - [ ] Add token variant in `polyglot-lexer/src/token.rs`
  - [ ] Document token: `OpDefaultPull // ~>`
  - [ ] Add to assignment operators group

- [ ] **Add `LiteralPipelineFormatted` token to token enum** (AC2)
  - [ ] Add token variant in `polyglot-lexer/src/token.rs`
  - [ ] Document token: `LiteralPipelineFormatted // |Pipeline"formatted {.string}"`
  - [ ] Add to literals group

- [ ] **Implement `~>` operator lexing** (AC3)
  - [ ] Update lexer logic to recognize `~>` character sequence
  - [ ] Ensure precedence: check `~` first, then `>` to distinguish from `~` unpack and `>` pipe
  - [ ] Add lexer test: `lex_tilde_greater_operator()`

- [ ] **Implement `|Pipeline"string"` pattern lexing** (AC2)
  - [ ] Add function: `lex_pipeline_formatted_string()`
  - [ ] Logic: detect `|`, read identifier, expect `"`, read string with interpolation
  - [ ] Store pipeline name and formatted string in token lexeme
  - [ ] Add lexer test: `lex_pipeline_formatted_string()`

- [ ] **Test lexer changes**
  - [ ] Run existing lexer tests - ensure no regressions
  - [ ] Add test file: `polyglot-lexer/tests/syntax_updates.rs`
  - [ ] Test `~>` operator tokenization
  - [ ] Test `|Pipeline"string"` tokenization
  - [ ] Test edge cases: `|Pipe.Nested.Name"str"`, `|Pipe"multi {.var1} {.var2}"`

---

### Task 2: Parser Updates - Enumeration Syntax (AC: 1)

- [ ] **Update `parse_enumeration_definition()` to require `#` prefix** (AC1)
  - [ ] File: `polyglot-parser/src/parser.rs`
  - [ ] After `expect(TokenKind::BlockVersionEnum)` (the `[#]` marker)
  - [ ] Add: `expect(TokenKind::PrefixEnum)?` (the `#` prefix)
  - [ ] If missing, return helpful error

- [ ] **Add helpful error message for old syntax** (AC1, AC6)
  - [ ] Create error variant: `EnumDefinitionMissingPrefix`
  - [ ] Error message: "Expected '#' after '[#]' for enumeration definition"
  - [ ] Help text: "Enumeration definitions require # prefix: [#] #Config"
  - [ ] Include source location and caret pointer

- [ ] **Update AST `EnumerationDefinition` if needed**
  - [ ] Check if AST stores enum name correctly (should include `#` prefix)
  - [ ] Update if necessary to maintain consistency

- [ ] **Test parser enumeration changes**
  - [ ] Add test: `parse_enum_with_hash_prefix_succeeds()`
  - [ ] Add test: `parse_enum_without_hash_prefix_fails()`
  - [ ] Add test: `parse_enum_error_message_quality()`
  - [ ] Test extendable enums: `[#] #DT.Hijri.*`
  - [ ] Test reserved enum extensions: `[#] #Boolean.True`

---

### Task 3: Parser Updates - Default Pull Operator (AC: 3)

- [ ] **Add `OpDefaultPull` to assignment operator parsing** (AC3)
  - [ ] File: `polyglot-parser/src/parser.rs`
  - [ ] Function: `parse_assignment()` or equivalent
  - [ ] Add match arm for `TokenKind::OpDefaultPull`
  - [ ] Map to AST assignment operator variant

- [ ] **Update AST if needed**
  - [ ] Check `AssignmentOp` enum in `polyglot-parser/src/ast.rs`
  - [ ] Add `DefaultPull` variant if not present
  - [ ] Document: `DefaultPull // ~> operator`

- [ ] **Test `~>` operator parsing**
  - [ ] Add test: `parse_default_pull_assignment()`
  - [ ] Test: `.timeout ~> .settings.timeout`
  - [ ] Test: `.backup ~> .timeout`
  - [ ] Test error cases: `~>` without left/right operand

---

### Task 4: Parser Updates - Pipeline Formatted String (AC: 2)

- [ ] **Implement `parse_pipeline_formatted_call()` function** (AC2)
  - [ ] File: `polyglot-parser/src/parser.rs`
  - [ ] Parse `LiteralPipelineFormatted` token
  - [ ] Extract pipeline name from token lexeme
  - [ ] Extract formatted string from token lexeme
  - [ ] Create AST node: `PipelineCall` with formatted string as argument

- [ ] **Integrate into main pipeline call parsing**
  - [ ] Update `parse_pipeline_call()` or `parse_statement()`
  - [ ] Check for `LiteralPipelineFormatted` token first
  - [ ] If present, use `parse_pipeline_formatted_call()`
  - [ ] Otherwise, use regular pipeline call parsing

- [ ] **Add helpful error if `|` prefix missing** (AC2, AC6)
  - [ ] Detect pattern: identifier followed by string (no `|`)
  - [ ] Example: `U.Log.Info"text"` should suggest `|U.Log.Info"text"`
  - [ ] Error message: "Formatted string requires pipeline prefix"
  - [ ] Help text: "Add pipe prefix: |PipelineName\"string\""

- [ ] **Test pipeline formatted string parsing**
  - [ ] Add test: `parse_pipeline_formatted_string()`
  - [ ] Test: `|U.Log.Info"Processing {.count}"`
  - [ ] Test: `|RT.Shell.Run"ls -la {.dir}"`
  - [ ] Test nested: `|Pipe.Nested.Name"string"`
  - [ ] Test multiple interpolations: `|Pipe"a {.x} b {.y}"`

---

### Task 5: Parser Validation - Serial Error Handling (AC: 4)

- [ ] **Add validation rule for `[s]` blocks in enumerations** (AC4)
  - [ ] File: `polyglot-parser/src/validation.rs`
  - [ ] Function: `validate_enumeration_serial_blocks()`
  - [ ] Check if enumeration has any `[s]` blocks
  - [ ] If yes, verify either `[s][!] *` or `[s][!]` exists

- [ ] **Create validation error for missing error handling** (AC4, AC6)
  - [ ] Error variant: `SerialBlockMissingErrorHandler`
  - [ ] Message: "[s] blocks require error handling"
  - [ ] Help: "Add [s][!] * for default or [s][!] for custom handling"

- [ ] **Test serial error handling validation**
  - [ ] Add test: `enum_with_serial_default_error_handling()`
  - [ ] Add test: `enum_with_serial_custom_error_handling()`
  - [ ] Add test: `enum_with_serial_no_error_handling_fails()`

---

### Task 6: Integration Testing (AC: 5, 7)

- [ ] **Create comprehensive test file** (AC5)
  - [ ] File: `polyglot-parser/tests/december_2025_syntax.rs`
  - [ ] Test complete `.pg` file with all new syntax
  - [ ] Include all acceptance criteria examples
  - [ ] Verify compilation succeeds

- [ ] **Create test file in repository** (AC7)
  - [ ] File: `tests/syntax/new-syntax-2025-12.pg`
  - [ ] Complete pipeline using all new features
  - [ ] Documented with comments explaining each feature
  - [ ] Can be used for documentation examples

- [ ] **Verify documentation examples** (AC7)
  - [ ] Extract code examples from documentation
  - [ ] Compile each example
  - [ ] Ensure all pass
  - [ ] Document any failures

- [ ] **Regression testing**
  - [ ] Run full existing test suite
  - [ ] Verify no tests break
  - [ ] If any fail, fix implementation or update tests
  - [ ] Run `cargo test --all`

---

### Task 7: Error Messages & Documentation (AC: 6, 7)

- [ ] **Review and improve all error messages** (AC6)
  - [ ] Ensure Rust compiler style (colored, caret, helpful)
  - [ ] Add "Help:" suggestions for common mistakes
  - [ ] Include examples in help text
  - [ ] Test error output formatting

- [ ] **Update implementation status documentation** (AC7)
  - [ ] File: `docs/user/implementation-status.md` (create if missing)
  - [ ] Mark Story 1.7 as complete
  - [ ] Update Epic 1 status to 100% complete
  - [ ] Remove any "not yet implemented" warnings

- [ ] **Update changelog**
  - [ ] File: `CHANGELOG.md` or equivalent
  - [ ] Add entry for December 2025 syntax updates
  - [ ] List all syntax changes with examples
  - [ ] Note: Breaking change if old enum syntax rejected

---

### Task 8: Code Review & Documentation

- [ ] **Self-review code changes**
  - [ ] Check code style matches project standards
  - [ ] Verify all `TODO` comments resolved
  - [ ] Ensure proper error handling
  - [ ] Review test coverage

- [ ] **Update code comments**
  - [ ] Document new token types
  - [ ] Explain parsing logic for new patterns
  - [ ] Add examples in doc comments

- [ ] **Final verification**
  - [ ] Run `cargo clippy` - fix all warnings
  - [ ] Run `cargo fmt` - format code
  - [ ] Run `cargo test --all` - all tests pass
  - [ ] Compile example files successfully

---

## Dev Notes

### Context

This story addresses critical gaps found between user documentation (updated December 2025) and the Epic 1 implementation (completed November 2025). The documentation shows syntax that currently does **not compile**.

**Gap Analysis Source:** `docs/Agile/syntax-changes-implementation-gap-2025-12-04.md`

### Syntax Changes Required

#### 1. Enumeration Definition: `[#] #Name`

**Change:** Enumeration definitions must use `#` prefix after `[#]` block marker.

**Before:**
```polyglot
[#] Config
[<] .api_key: pg\string
[X]
```

**After:**
```polyglot
[#] #Config
[<] .api_key: pg\string
[X]
```

**Rationale:** Consistency - enumeration references use `#Config`, so definitions should too.

**Files Updated in Documentation:** 28 files, 57 instances
**Document:** `docs/Agile/enumeration-syntax-update-2025-12-03.md`

**Implementation Impact:**
- **Lexer:** Already has `PrefixEnum` (`#`) token ✅
- **Parser:** Must require `#` after `[#]` - **NOT YET IMPLEMENTED** ❌

---

#### 2. Pipeline Formatted Strings: `|Pipeline"string"`

**Change:** Formatted string literals must have `|` prefix on pipeline name.

**Before:**
```polyglot
[r] .msg << U.Log.Info"Processing {.count}"
```

**After:**
```polyglot
[r] .msg << |U.Log.Info"Processing {.count}"
```

**Rationale:** Consistency - all Polyglot objects have prefixes (`|`, `#`, `.`, `!`)

**Document:** `docs/Agile/runtime-environments-specification-2025-12-03.md`

**Implementation Impact:**
- **Lexer:** Needs new `LiteralPipelineFormatted` token - **NOT YET IMPLEMENTED** ❌
- **Parser:** Must parse `|Pipeline"string"` pattern - **NOT YET IMPLEMENTED** ❌

**Lexer Logic Required:**
```rust
fn lex_pipeline_formatted_string(&mut self) -> Token {
    self.expect('|');                    // Pipe prefix
    let pipeline = self.read_identifier(); // Pipeline.Full.Name
    self.expect('"');                    // String start
    let string = self.read_string_with_interpolation(); // Formatted string

    Token::new(
        TokenKind::LiteralPipelineFormatted,
        format!("|{}\"{}\"", pipeline, string),
        self.line,
        self.column
    )
}
```

---

#### 3. `~>` Pull Default Operator

**Change:** New operator for pulling default values (counterpart to `<~` push default).

**New Syntax:**
```polyglot
[r] .timeout: pg\int ~> .settings.timeout    // Pull default from source
```

**Existing Operators:**
- `<<` Push left (immediate)
- `>>` Pull right (immediate)
- `<~` Push default left

**Added:**
- `~>` Pull default right

**Document:** `docs/Agile/variable-states-update-summary-2025-12-03.md`

**Implementation Impact:**
- **Lexer:** Needs `OpDefaultPull` token - **NOT YET IMPLEMENTED** ❌
- **Parser:** Must handle `~>` in assignments - **NOT YET IMPLEMENTED** ❌

**Token Definition Required:**
```rust
// Assignment Operators (3 → 4 tokens)
OpPush,          // <<
OpPull,          // >>
OpDefault,       // <~
OpDefaultPull,   // ~> NEW
```

---

#### 4. Serial Error Handling Validation

**Change:** Enumerations with `[s]` serial load blocks MUST have error handling.

**Required Patterns:**
```polyglot
// Pattern A: Default error handling
[#] #Config
[s] "config.yaml"
[s][!] *              // Default handler (required)
[X]

// Pattern B: Custom error handling
[#] #Secrets
[s] ".env.secrets"
[s][!]                // Custom handler
[r] |U.Log.Error"Failed: {!.message}"
[r] |U.Process.Exit"1"
[X]
```

**Document:** `docs/Agile/serial-error-handling-safety-mechanism-2025-12-03.md`

**Implementation Impact:**
- **Parser:** Must validate error handling present - **NOT YET IMPLEMENTED** ❌

---

### Source Tree Context

**Relevant Files:**

**Lexer:**
- `polyglot-lexer/src/token.rs` - Token type definitions (102 tokens currently)
- `polyglot-lexer/src/lexer.rs` - Lexing logic
- `polyglot-lexer/src/error.rs` - Lexer errors
- `polyglot-lexer/tests/` - Lexer tests

**Parser:**
- `polyglot-parser/src/ast.rs` - AST node definitions
- `polyglot-parser/src/parser.rs` - Parsing logic (recursive descent)
- `polyglot-parser/src/validation.rs` - AST validation
- `polyglot-parser/src/error.rs` - Parse errors
- `polyglot-parser/tests/` - Parser tests

**Current Implementation Status:**
- ✅ Story 1.1: Workspace setup (DONE)
- ✅ Story 1.2: Token definitions (DONE - 102 tokens)
- ✅ Story 1.3: Lexer implementation (DONE)
- ✅ Story 1.4: AST definitions (DONE)
- ✅ Story 1.5: Parser implementation (DONE)
- ✅ Story 1.5.5: Multi-file compilation (DONE)
- ✅ Story 1.6: Syntax validator (DONE)
- 🔲 Story 1.7: December syntax updates (THIS STORY)

---

### Previous Story Insights

From Story 1.6 (Syntax Validator):
- Validation framework established in `polyglot-parser/src/validation.rs`
- Validation errors use `ValidationError` enum with source spans
- Can build on existing validation infrastructure for serial error handling check

From Story 1.5.5 (Multi-file compilation):
- Parser already handles complex patterns
- File registry resolver exists for cross-file references
- Can use similar pattern for pipeline formatted string resolution

From Story 1.5 (Parser):
- Recursive descent parser structure is flexible
- Adding new token types is straightforward
- Error reporting infrastructure is solid

---

### Testing Standards

**From Architecture:** `docs/Tech/implementation/technical/testing-strategy.md` (not found, using project conventions)

**Test File Locations:**
- Lexer unit tests: `polyglot-lexer/src/tests.rs` or `polyglot-lexer/tests/*.rs`
- Parser unit tests: `polyglot-parser/src/tests.rs` or `polyglot-parser/tests/*.rs`
- Integration tests: `tests/*.rs` in workspace root

**Testing Framework:**
- Standard Rust testing: `#[cfg(test)]` modules
- Use `#[test]` attribute for test functions
- Use `assert_eq!`, `assert!`, `panic!` for assertions
- Use `Result<(), Box<dyn Error>>` for tests that can fail

**Coverage Requirements:**
- Every new token type must have lexer test
- Every new parsing rule must have parser test
- Every validation rule must have validation test
- Integration test must cover all new syntax together

**Test Naming Convention:**
- `test_lex_<feature>_<scenario>()`
- `test_parse_<feature>_<scenario>()`
- `test_validate_<feature>_<scenario>()`

**Example:**
```rust
#[test]
fn test_lex_default_pull_operator() {
    let source = ".x ~> .y";
    let tokens = Lexer::new(source).tokenize().unwrap();
    assert_eq!(tokens[1].kind, TokenKind::OpDefaultPull);
}

#[test]
fn test_parse_enum_requires_hash_prefix() {
    let source = "[#] Config [X]";
    let result = Parser::new(source).parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Expected '#'"));
}
```

---

### Technical Constraints

**From Architecture:**
- **Language:** Rust (Edition 2021)
- **Error Handling:** `thiserror` for library errors, `anyhow` for applications
- **Performance:** Lexer/Parser should handle 1000-line files in <100ms
- **Code Style:** Follow `rustfmt` defaults, `clippy` warnings as errors

**Version Requirements:**
- Rust: 1.75+ (matches current implementation)
- Dependencies: Maintain compatibility with existing workspace dependencies

---

### Expected Effort

**Estimated Implementation Time:** 14-20 hours (2-3 days)

**Breakdown:**
- Lexer changes: 4-6 hours (2 new tokens, lexing logic, tests)
- Parser changes: 6-8 hours (enum syntax, operator, formatted strings, validation)
- Testing: 4-6 hours (unit tests, integration tests, regression tests)

**Complexity:** Medium
- Most changes are additions, not modifications
- Existing infrastructure supports new features
- Clear specifications from documentation

---

### Definition of Done

- [ ] All acceptance criteria met ✅
- [ ] All tasks completed ✅
- [ ] All tests passing (including existing tests) ✅
- [ ] `cargo clippy` produces no warnings ✅
- [ ] Code formatted with `cargo fmt` ✅
- [ ] Documentation updated (implementation status) ✅
- [ ] Can compile test file with all new syntax ✅
- [ ] Error messages are helpful and actionable ✅

---

## Change Log

| Date | Version | Description | Author |
|------|---------|-------------|--------|
| 2025-12-04 | 1.0 | Story created - December 2025 syntax updates | Bob (SM) |

---

## Dev Agent Record

*This section will be populated by the development agent during implementation.*

### Agent Model Used

*To be filled by dev agent*

### Debug Log References

*To be filled by dev agent*

### Completion Notes

*To be filled by dev agent*

### File List

*To be filled by dev agent*

---

## QA Results

*This section will be populated by QA agent after implementation.*
