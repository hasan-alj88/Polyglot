# Story 1.8: Serial Error Handling Safety Mechanism - Test Coverage

**Epic:** Epic 1 - Lexer & Parser
**Created:** 2025-12-04
**Created By:** Claude (Dev Agent)

---

## Status

**In Progress** - Story 1.7 complete, implementing test coverage

**Started:** 2025-12-05

**Progress:**
- ✅ Story 1.7 AC4 completed - `[s][!]` syntax now parseable
- ✅ Parser supports enumeration body with serial blocks
- ⏳ Adding unit tests for validation
- ⏳ Adding integration tests
- ⏳ Adding edge case tests

---

## Story

**As a** Polyglot developer,
**I want** comprehensive test coverage for the serial error handling safety mechanism,
**so that** I can trust that the compiler will catch missing error handlers and prevent runtime failures in production.

---

## Acceptance Criteria

### AC1: Unit Tests for Serial Error Handling Validation

**Given** the validation module has `validate_serial_error_handling()` function
**When** I run unit tests
**Then** the following test cases pass:

1. **Pipeline without streaming blocks** → validation passes (no error handlers required)
2. **Pipeline with `[s]` block and `[s][!] *` default handler** → validation passes
3. **Pipeline with `[s]` block and `[s][!]` custom handler** → validation passes
4. **Pipeline with `[s]` block but no error handler** → validation fails with clear error message
5. **Pipeline with multiple `[s]` blocks and error handler** → validation passes
6. **Nested streaming blocks with error handler** → validation passes
7. **Nested streaming blocks without error handler** → validation fails

**And** error messages include:
- Pipeline name
- Line/column location
- Helpful suggestion: "Add [s][!] * for default error handling, or [s][!] for custom handling"

---

### AC2: Integration Tests for Complete Workflows

**Given** integration test suite
**When** I run integration tests
**Then** complete `.pg` files are tested:

1. **Valid enumeration with serial load and error handling:**
```polyglot
[@] Local@TestPkg:1.0.0
[X]

[#] #Config
[<] .timeout: pg\int
[s] "config.yaml"
[s][!] *
[X]
```
→ Should compile successfully

2. **Invalid enumeration missing error handler:**
```polyglot
[@] Local@TestPkg:1.0.0
[X]

[#] #Config
[<] .timeout: pg\int
[s] "config.yaml"
// Missing [s][!] *
[X]
```
→ Should fail validation with specific error

3. **Custom error recovery pattern:**
```polyglot
[@] Local@TestPkg:1.0.0
[X]

[#] #Secrets
[<] .api_key: pg\string
[s] ".env.secrets"
[s][!]
[r] |U.Log.Error"Failed to load secrets: {!.message}"
[r] |U.Process.Exit"1"
[X]
```
→ Should compile successfully

---

### AC3: Edge Case Coverage

**Given** edge case test scenarios
**When** tests run
**Then** all edge cases are correctly handled:

1. **Multiple serial blocks with single error handler** → validation passes
2. **Streaming in nested conditionals** → properly detected
3. **Streaming in parallel blocks** → properly detected
4. **Error handler in parent block** → properly detected (recursive search)
5. **Error handler in sibling block** → validation fails (not in scope)
6. **Empty streaming block** → still requires error handler
7. **Pipeline (not enumeration) with streaming** → validated correctly

---

### AC4: Test Documentation and Maintainability

**Given** test code
**When** reviewed
**Then** tests include:
- Clear test names describing scenario
- Inline comments explaining what's being tested
- Helper functions to reduce duplication
- Assertion messages that explain failures
- Examples of valid and invalid patterns

**And** test file location is clear:
- Unit tests: `polyglot-parser/src/validation.rs` `#[cfg(test)]` module
- Integration tests: `polyglot-parser/tests/validation_tests.rs`

---

### AC5: Test Performance

**Given** test suite
**When** all validation tests run
**Then** execution completes in <500ms total
**And** individual test cases run in <50ms each

---

### AC6: Regression Prevention

**Given** implemented tests
**When** future changes are made to validation logic
**Then** tests serve as regression guards
**And** any breaking changes are immediately detected
**And** test output clearly indicates what broke

---

## Tasks / Subtasks

### Task 1: Unit Tests in validation.rs (AC: 1)

- [ ] **Add test module to validation.rs**
  - [ ] Expand existing `#[cfg(test)] mod tests` section
  - [ ] Add imports: `use super::*;`
  - [ ] Add helper function: `create_test_program(source: &str) -> Program`

- [ ] **Test: Pipeline without streaming passes validation**
  - [ ] Test name: `test_validate_serial_no_streaming_no_handler_passes()`
  - [ ] Create simple pipeline with no `[s]` blocks
  - [ ] Call `validate_serial_error_handling()`
  - [ ] Assert: `errors.is_empty()`

- [ ] **Test: Pipeline with streaming and default handler passes**
  - [ ] Test name: `test_validate_serial_with_default_handler_passes()`
  - [ ] Create pipeline with `[s]` block and `[s][!] *`
  - [ ] Assert: validation passes

- [ ] **Test: Pipeline with streaming and custom handler passes**
  - [ ] Test name: `test_validate_serial_with_custom_handler_passes()`
  - [ ] Create pipeline with `[s]` block and `[s][!]` + recovery logic
  - [ ] Assert: validation passes

- [ ] **Test: Pipeline with streaming but no handler fails**
  - [ ] Test name: `test_validate_serial_missing_handler_fails()`
  - [ ] Create pipeline with `[s]` block, no error handler
  - [ ] Assert: `errors.len() == 1`
  - [ ] Assert: error message contains "streaming" and "error handling"
  - [ ] Assert: error category is `Semantic`

- [ ] **Test: Multiple streaming blocks with handler passes**
  - [ ] Test name: `test_validate_serial_multiple_streaming_with_handler_passes()`
  - [ ] Create pipeline with 2+ `[s]` blocks and 1 error handler
  - [ ] Assert: validation passes

- [ ] **Test: Nested streaming with handler passes**
  - [ ] Test name: `test_validate_serial_nested_streaming_with_handler_passes()`
  - [ ] Create pipeline with streaming inside conditional/parallel block
  - [ ] Include error handler at appropriate level
  - [ ] Assert: validation passes

- [ ] **Test: Nested streaming without handler fails**
  - [ ] Test name: `test_validate_serial_nested_streaming_without_handler_fails()`
  - [ ] Create pipeline with nested streaming, no handler
  - [ ] Assert: validation fails

---

### Task 2: Integration Tests (AC: 2)

- [ ] **Create integration test file**
  - [ ] File: `polyglot-parser/tests/validation_serial_error_handling.rs`
  - [ ] Add imports: `use polyglot_parser::{validate_file, Parser, FileRegistryResolver};`
  - [ ] Add helper: `create_temp_file(content: &str) -> NamedTempFile`

- [ ] **Test: Valid enumeration with serial load**
  - [ ] Test name: `test_valid_enum_with_serial_and_default_handler()`
  - [ ] Create complete `.pg` file with enum, serial block, default handler
  - [ ] Use `validate_file()` to validate
  - [ ] Assert: `result.is_ok()`

- [ ] **Test: Invalid enumeration missing handler**
  - [ ] Test name: `test_invalid_enum_with_serial_no_handler()`
  - [ ] Create `.pg` file with serial block but no handler
  - [ ] Assert: `result.is_err()`
  - [ ] Assert: error message mentions "error handling"

- [ ] **Test: Custom error recovery pattern**
  - [ ] Test name: `test_valid_enum_with_custom_error_recovery()`
  - [ ] Create `.pg` file with `[s][!]` custom handler
  - [ ] Assert: validation passes

- [ ] **Test: Multiple serial blocks scenario**
  - [ ] Test name: `test_valid_enum_multiple_serial_blocks()`
  - [ ] Create enum loading multiple files
  - [ ] Include error handler
  - [ ] Assert: validation passes

---

### Task 3: Edge Case Tests (AC: 3)

- [ ] **Test: Multiple serial blocks, single handler**
  - [ ] Test name: `test_edge_multiple_serial_single_handler()`
  - [ ] Verify one handler covers all streaming blocks

- [ ] **Test: Streaming in conditional blocks**
  - [ ] Test name: `test_edge_streaming_in_conditional()`
  - [ ] Create `[?]` conditional with `[s]` inside
  - [ ] Test with and without handler

- [ ] **Test: Streaming in parallel blocks**
  - [ ] Test name: `test_edge_streaming_in_parallel()`
  - [ ] Create `[p]` parallel with `[s]` inside
  - [ ] Test with and without handler

- [ ] **Test: Error handler in parent scope**
  - [ ] Test name: `test_edge_handler_in_parent_scope()`
  - [ ] Nested structure with handler at parent level
  - [ ] Verify recursive detection works

- [ ] **Test: Error handler in sibling (should fail)**
  - [ ] Test name: `test_edge_handler_in_sibling_fails()`
  - [ ] Handler in separate branch
  - [ ] Should not count as valid handler

- [ ] **Test: Empty streaming block**
  - [ ] Test name: `test_edge_empty_streaming_block()`
  - [ ] `[s]` block with no content
  - [ ] Still requires handler

- [ ] **Test: Pipeline (not enum) with streaming**
  - [ ] Test name: `test_edge_pipeline_streaming_validation()`
  - [ ] Regular pipeline using streaming
  - [ ] Verify validation applies to pipelines too

---

### Task 4: Test Infrastructure (AC: 4)

- [ ] **Create test helper functions**
  - [ ] Helper: `parse_test_source(source: &str) -> Result<Program, ParserError>`
  - [ ] Helper: `assert_validation_error_contains(errors: &[ValidationError], substring: &str)`
  - [ ] Helper: `create_pipeline_with_streaming(has_handler: bool) -> Program`

- [ ] **Add inline documentation**
  - [ ] Document each test's purpose
  - [ ] Add comments explaining validation rules
  - [ ] Include examples of valid/invalid patterns in comments

- [ ] **Refactor duplicate test setup**
  - [ ] Extract common setup into helper functions
  - [ ] Use test fixtures where appropriate
  - [ ] Create reusable test data builders

---

### Task 5: Performance & Regression (AC: 5, 6)

- [ ] **Benchmark test execution**
  - [ ] Run full validation test suite
  - [ ] Measure total execution time
  - [ ] Verify <500ms total
  - [ ] Add `#[ignore]` tag if benchmark test is slow

- [ ] **Add regression test scenarios**
  - [ ] Document known edge cases from Story 1.7
  - [ ] Create test for each historical bug (if any)
  - [ ] Add comments linking to related issues/stories

- [ ] **Verify test isolation**
  - [ ] Each test runs independently
  - [ ] No shared mutable state
  - [ ] Tests can run in parallel

---

### Task 6: Documentation & Cleanup (AC: 4)

- [ ] **Update test documentation**
  - [ ] Add module-level doc comment explaining safety mechanism
  - [ ] Document test organization
  - [ ] Link to relevant specs: `docs/technical/variable-states-specification.md`

- [ ] **Code review checklist**
  - [ ] All tests have descriptive names
  - [ ] All assertions have failure messages
  - [ ] Test coverage is comprehensive
  - [ ] No duplicated test logic

- [ ] **Final validation**
  - [ ] Run `cargo test --package polyglot-parser` - all pass
  - [ ] Run `cargo test --all` - no regressions
  - [ ] Run `cargo clippy` - no warnings
  - [ ] Run `cargo fmt` - code formatted

---

## Dev Notes

### Context

During Story 1.7 implementation (December 2025 Syntax Updates), the serial error handling safety mechanism was implemented in code but **no tests were written**. This creates a critical gap:

**Current State:**
- ✅ Code exists: `validate_serial_error_handling()` in `validation.rs` (lines 192-216)
- ✅ Helper functions: `block_contains_streaming()`, `block_contains_error_catch()`
- ✅ Integrated into validation pipeline (line 125)
- ❌ **Zero unit tests**
- ❌ **Zero integration tests**
- ❌ **Untested edge cases**

**Risk:** The safety mechanism may not work correctly under all scenarios, but we won't discover this until production use.

**This Story:** Add comprehensive test coverage to validate the safety mechanism works as designed.

---

### Safety Mechanism Requirements

**Rule:** Pipelines or enumerations with streaming blocks `[s]` MUST have error handling `[s][!]`.

**Valid Patterns:**

1. **Default Error Handling:**
```polyglot
[#] #Config
[s] "config.yaml"
[s][!] *              // Default handler
[X]
```

2. **Custom Error Handling:**
```polyglot
[#] #Config
[s] "config.yaml"
[s][!]                // Custom handler
[r] |U.Log.Error"Failed: {!.message}"
[X]
```

**Invalid Pattern:**
```polyglot
[#] #Config
[s] "config.yaml"
// No [s][!] - MUST FAIL VALIDATION
[X]
```

**Document Reference:** `docs/project/serial-error-handling-safety-mechanism-2025-12-03.md`

---

### Implementation Details

**Current Code (validation.rs):**

```rust
fn validate_serial_error_handling(
    program: &Program,
    file_path: &str,
    errors: &mut Vec<ValidationError>,
) {
    for definition in &program.definitions {
        if let Definition::Pipeline(pipeline) = definition {
            let has_streaming = block_contains_streaming(&pipeline.body);
            let has_error_handling = block_contains_error_catch(&pipeline.body);

            if has_streaming && !has_error_handling {
                errors.push(ValidationError::new(
                    Severity::Error,
                    ErrorCategory::Semantic,
                    format!(
                        "Pipeline '{}' contains streaming [s] blocks but lacks required error handling [s][!] * or [s][!]",
                        pipeline.name
                    ),
                    pipeline.span.start.line,
                    pipeline.span.start.column,
                ).with_file_path(file_path));
            }
        }
    }
}
```

**Test Strategy:**
1. **Unit tests** - Test the validation function directly with AST structures
2. **Integration tests** - Test complete `.pg` files through `validate_file()`
3. **Edge cases** - Test nested structures, multiple blocks, conditionals, etc.

---

### Testing Approach

**Unit Test Pattern:**
```rust
#[test]
fn test_validate_serial_missing_handler_fails() {
    // Arrange: Create program with streaming but no handler
    let source = r#"
[@] Local@Test:1.0.0
[X]

[|] TestPipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[s] "data.json"
// Missing [s][!]
[o] .result: pg\string
[X]
    "#;

    let resolver = FileRegistryResolver::empty();
    let parser = Parser::new(source, resolver).unwrap();
    let program = parser.parse().unwrap();

    // Act: Validate
    let mut errors = Vec::new();
    validate_serial_error_handling(&program, "test.pg", &mut errors);

    // Assert: Should have 1 error
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].category, ErrorCategory::Semantic);
    assert!(errors[0].message.contains("streaming"));
    assert!(errors[0].message.contains("error handling"));
}
```

**Integration Test Pattern:**
```rust
#[test]
fn test_invalid_enum_with_serial_no_handler() {
    let content = r#"
[@] Local@Test:1.0.0
[X]

[#] #Config
[<] .timeout: pg\int
[s] "config.yaml"
[X]
    "#;

    let file = create_temp_file(content);
    let result = validate_file(file.path());

    assert!(result.is_err(), "Should fail validation");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("error handling"));
}
```

---

### Test Coverage Goals

**Minimum Coverage:**
- ✅ Basic valid case (streaming + handler)
- ✅ Basic invalid case (streaming, no handler)
- ✅ Both default handler patterns (`*` and custom)
- ✅ No streaming (should pass without handler)

**Comprehensive Coverage:**
- ✅ Multiple streaming blocks
- ✅ Nested streaming (in conditionals, parallels, etc.)
- ✅ Error handler scope (parent, sibling, child)
- ✅ Empty streaming blocks
- ✅ Both pipelines and enumerations

**Edge Cases:**
- ✅ Handler in deeply nested structure
- ✅ Multiple handlers (should pass)
- ✅ Handler after streaming (order independence)
- ✅ Comments between blocks
- ✅ Mixed streaming and non-streaming

---

### Expected Effort

**Estimated Implementation Time:** 4-6 hours

**Breakdown:**
- Unit tests: 2-3 hours (8-10 test cases)
- Integration tests: 1-2 hours (4-5 complete scenarios)
- Edge case tests: 1 hour (7 edge cases)
- Documentation & cleanup: 30-60 minutes

**Complexity:** Low-Medium
- Well-defined requirements
- Existing validation infrastructure
- Clear test patterns from Story 1.7

---

### Definition of Done

- [ ] All acceptance criteria met ✅
- [ ] All tasks completed ✅
- [ ] Minimum 15 test cases added ✅
- [ ] All tests passing ✅
- [ ] Test execution time <500ms ✅
- [ ] `cargo clippy` produces no warnings ✅
- [ ] Code formatted with `cargo fmt` ✅
- [ ] Test documentation complete ✅
- [ ] No regressions in existing tests ✅

---

## Change Log

| Date | Version | Description | Author |
|------|---------|-------------|--------|
| 2025-12-04 | 1.0 | Story created - Serial error handling test coverage | Claude (Dev) |

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
