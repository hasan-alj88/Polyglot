# Story 1.6: Syntax Validator (Standalone)

**Syntax Version:** This story implements v0.0.3 syntax. v0.0.4 syntax (with `$` variables, `{|}` blocks, `[|]` IO) will be implemented in future epics.

Status: drafted

## Story

As a developer,
I want to validate `.pg` file syntax without full compilation,
So that I get fast feedback during development (FR2).

## Acceptance Criteria

**Given** a `.pg` file path
**When** I invoke the validation API
**Then** syntax errors are returned with line numbers and descriptions

**And** validator checks:
- Lexer succeeds (no invalid tokens)
- Parser succeeds (valid AST structure)
- Semantic basics (no duplicate pipeline names within file)

**And** validation completes in <500ms (NFR-P1)

**And** unit tests verify:
- Valid pipelines pass validation
- Syntax errors detected and reported clearly
- Multiple errors collected (don't stop at first error)

## Tasks / Subtasks

- [ ] **Task 1: Validation API Implementation** (AC: Entry point and error collection)
  - [ ] 1.1: Create public `validate_file(path: &Path) -> Result<(), Vec<ValidationError>>` function
  - [ ] 1.2: Implement file reading with proper error handling
  - [ ] 1.3: Define `ValidationError` struct with line numbers, column, message fields
  - [ ] 1.4: Collect multiple errors before returning (don't fail fast)

- [ ] **Task 2: Lexer Validation** (AC: Lexer succeeds check)
  - [ ] 2.1: Invoke lexer tokenization
  - [ ] 2.2: Catch and convert lexer errors to ValidationError format
  - [ ] 2.3: Continue validation even if lexer has non-fatal errors

- [ ] **Task 3: Parser Validation** (AC: Parser succeeds check)
  - [ ] 3.1: Invoke Parser::parse() on tokenized input
  - [ ] 3.2: Catch and convert parser errors to ValidationError format
  - [ ] 3.3: Extract multiple errors if parser supports error recovery

- [ ] **Task 4: Semantic Validation** (AC: Duplicate pipeline names check)
  - [ ] 4.1: Traverse parsed AST to collect all pipeline names
  - [ ] 4.2: Detect duplicate pipeline definitions within file
  - [ ] 4.3: Report duplicates with both occurrence locations
  - [ ] 4.4: Add span information for precise error reporting

- [ ] **Task 5: Error Formatting** (AC: Clear error messages)
  - [ ] 5.1: Format errors similar to Rust compiler style
  - [ ] 5.2: Include file path, line, column in error output
  - [ ] 5.3: Add helpful error messages with suggestions where applicable
  - [ ] 5.4: Sort errors by line number for readability

- [ ] **Task 6: Unit Tests** (AC: Test coverage)
  - [ ] 6.1: Test valid pipeline validation (should pass)
  - [ ] 6.2: Test lexer error detection (invalid tokens)
  - [ ] 6.3: Test parser error detection (syntax errors)
  - [ ] 6.4: Test duplicate pipeline detection
  - [ ] 6.5: Test multiple errors collected in single run
  - [ ] 6.6: Test error formatting and output

- [ ] **Task 7: Performance Validation** (AC: < 500ms)
  - [ ] 7.1: Add benchmarks for validation function
  - [ ] 7.2: Test with various file sizes
  - [ ] 7.3: Ensure < 500ms for typical pipeline files

## Dev Notes

### Architecture Context

**Crate:** `polyglot-parser`
**Entry Point:** New public function `validate_file()` in `lib.rs`
**Dependencies:**
- Uses existing `Lexer` from `polyglot-lexer`
- Uses existing `Parser` from `polyglot-parser::parser`
- May need new `ValidationError` type

**Validation Flow:**
```rust
pub fn validate_file(path: &Path) -> Result<(), Vec<ValidationError>> {
    // 1. Read file
    let source = fs::read_to_string(path)?;

    // 2. Lexer validation
    let mut errors = Vec::new();
    let tokens = match Lexer::new(&source).tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            errors.push(convert_lexer_error(e));
            return Err(errors);
        }
    };

    // 3. Parser validation
    let resolver = StubImportResolver::new(); // Validation mode
    let parser = Parser::new_from_tokens(tokens, resolver)?;
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            errors.push(convert_parser_error(e));
            // Could continue with partial AST if available
            if !errors.is_empty() {
                return Err(errors);
            }
        }
    };

    // 4. Semantic validation
    validate_no_duplicate_pipelines(&program, &mut errors);

    // 5. Return
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

### Learnings from Previous Story

**From Story 1-5-5-multi-file-compilation-same-package-resolution (Status: done)**

- **Parser Infrastructure Ready**: Complete parser at `polyglot-parser/src/parser.rs` (1,700+ lines)
  - Use `Parser::new()` and `Parser::parse()` for validation
  - Comprehensive error types in `polyglot-parser/src/error.rs` (26+ ParserError variants)
  - All errors include Span tracking for precise locations

- **Test Framework Established**: Integration tests at `polyglot-parser/tests/integration_tests.rs`
  - Follow established patterns for test fixtures
  - 97 tests currently passing (88 unit + 9 integration)
  - Test fixtures pattern: create `.pg` files in `tests/fixtures/`

- **Error Handling Patterns**: All errors use comprehensive span tracking
  - Each error includes line, column, and context
  - Can build on existing `ParserError` for conversion to `ValidationError`
  - Error messages follow descriptive patterns with helpful context

- **Performance Considerations**: Parser is fast
  - Multi-file compilation with caching performs well
  - Single-file validation should easily meet <500ms target
  - Use `FileRegistryResolver::empty()` for standalone validation

[Source: stories/1-5-5-multi-file-compilation-same-package-resolution.md#Completion-Notes-List]

### Project Structure Notes

**Files to Create:**
- `polyglot-parser/src/validation.rs` (new module for validation API)
- `polyglot-parser/src/validation_error.rs` (ValidationError type)

**Files to Modify:**
- `polyglot-parser/src/lib.rs` (export validation module)
- `polyglot-parser/Cargo.toml` (if new dependencies needed)

**Test Files:**
- `polyglot-parser/src/validation/tests.rs` (unit tests)
- `polyglot-parser/tests/validation_tests.rs` (integration tests)
- Test fixtures in `tests/fixtures/validation/`

### Testing Strategy

**Unit Tests:**
- ValidationError construction and formatting
- Error conversion from LexerError and ParserError
- Duplicate pipeline detection logic
- Error sorting and deduplication

**Integration Tests:**
- Valid file validation (should succeed)
- Invalid token errors (lexer failures)
- Syntax errors (parser failures)
- Duplicate pipeline names (semantic failures)
- Multiple errors in single file
- Performance benchmarks

### References

- [Epic 1 Breakdown: Story 1.6](../../project/epics.md#story-16-syntax-validator-standalone)
- [Architecture: Parser Design](../../technical/architecture.md#parser)
- [Story 1.5 Session Notes](../../project/agent-sessions/claude-2025-11-28-story-1-5-parser-implementation.md)
- [Story 1.5.5 Completion Notes](./1-5-5-multi-file-compilation-same-package-resolution.md#completion-notes-list)
- [PRD: FR2 - Validate syntax with errors](../../project/prd.md)

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

claude-sonnet-4-5-20250929

### Debug Log References

N/A - Implementation completed without debugging issues

### Completion Notes List

**Implementation Summary:**

Story 1.6 (Syntax Validator) has been successfully completed. All acceptance criteria met:

1. **Validation API** - Created `validate_file(path: &Path) -> Result<(), Vec<ValidationError>>` function
   - Three-phase validation: Lexer → Parser → Semantic
   - Comprehensive error collection (no fail-fast)
   - Rust compiler-style error formatting
   - File path: `polyglot-parser/src/validation.rs` (~335 lines)

2. **ValidationError Type** - Created user-friendly error reporting
   - Severity levels (Error, Warning)
   - Error categories (Lexer, Parser, Semantic, Io)
   - Conversion from LexerError and ParserError
   - Rust compiler-style Display implementation
   - File path: `polyglot-parser/src/validation_error.rs` (~230 lines)

3. **Parser Extension** - Added `Parser::new_from_tokens()` constructor
   - Allows validation to use pre-tokenized input
   - Avoids redundant lexing when lexer already succeeded
   - File path: `polyglot-parser/src/parser.rs` (lines 106-129)

4. **Test Coverage** - Comprehensive unit tests
   - 10 new tests added (4 ValidationError + 6 Validation)
   - All 107 tests passing (98 unit + 9 integration)
   - Tests cover: valid files, lexer errors, parser errors, duplicate pipelines, error sorting

5. **Performance Validation** - Benchmark confirms sub-500ms requirement
   - Small file (10 pipelines): 0.24ms
   - Medium file (50 pipelines): 1.14ms
   - Large file (100 pipelines): 2.19ms
   - Extra Large file (200 pipelines): 4.30ms
   - **Performance: 100x faster than requirement!**
   - Benchmark: `polyglot-parser/examples/benchmark_validation.rs`

**Key Technical Decisions:**

1. **Error Collection Strategy**: Collect all errors in single pass instead of failing fast
   - Better developer experience (see all issues at once)
   - Implemented via `Vec<ValidationError>` accumulation

2. **Lexer Error Handling**: Use `line()` and `column()` methods from `LexerError`
   - LexerError doesn't have `position()` method
   - Extract position info separately for ValidationError construction

3. **Duplicate Pipeline Detection**: HashMap-based tracking with span information
   - First occurrence stored with span
   - Subsequent occurrences reported with reference to first location
   - Clear error messages: "Duplicate pipeline definition 'X' (first defined at line Y)"

4. **Parser Token Constructor**: Added `new_from_tokens()` to avoid redundant lexing
   - Validation already runs lexer separately for error detection
   - Reusing tokens improves performance and code clarity

5. **Semantic Validation Scope**: Limited to duplicate pipeline names within file
   - Cross-file validation deferred to compilation phase
   - Keeps validation fast and focused on syntax

**Dependencies Added:**
- `tempfile = "3.8"` (dev-dependency for tests)

**All Acceptance Criteria Met:**
- ✅ Lexer validation (invalid tokens detected)
- ✅ Parser validation (syntax errors detected)
- ✅ Semantic validation (duplicate pipeline names detected)
- ✅ Multiple errors collected in single pass
- ✅ Error messages with line/column numbers
- ✅ Rust compiler-style formatting
- ✅ Performance <500ms (actually <5ms!)
- ✅ Comprehensive unit tests

### File List

**Created Files:**
- `polyglot-parser/src/validation.rs` - Main validation API (~335 lines)
- `polyglot-parser/src/validation_error.rs` - Error types (~230 lines)
- `polyglot-parser/examples/benchmark_validation.rs` - Performance benchmark

**Modified Files:**
- `polyglot-parser/src/lib.rs` - Added validation module exports
- `polyglot-parser/src/parser.rs` - Added `new_from_tokens()` constructor
- `polyglot-parser/Cargo.toml` - Added tempfile dev-dependency
