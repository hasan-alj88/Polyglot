# Change Request #001: v0.0.2 Syntax Compliance Fixes

**Date:** 2025-12-05
**Status:** DRAFT
**Priority:** CRITICAL
**Affected Stories:** 1.2, 1.3, 1.4, 1.5, 1.6
**Blocking:** Story 1.8 (Serial Error Handling & Test Coverage)
**Epic:** Epic 1 - Lexer & Parser

---

## Executive Summary

During Story 1.8 development, a comprehensive syntax verification revealed that Stories 1.2-1.6 (marked "DONE") do not fully implement the Polyglot v0.0.2 language specification. The lexer cannot tokenize critical syntax patterns documented in `docs/user/`, including argument prefixes, macro system markers, and alias definitions. This change request proposes systematic fixes to align implementation with specification.

**Impact:** 5 stories require rework, Story 1.8 blocked until completion.

---

## Issues Identified

### Critical Priority

#### Issue #1: Standalone `<` and `>` Argument Prefix Tokenization
- **Severity:** CRITICAL - Sprint Blocker
- **Location:** `polyglot-lexer/src/lexer.rs:612-679`
- **Current Behavior:** Lexer throws `UnexpectedCharacter` error
- **Expected Behavior:** Tokenize as `DelimiterInputPrefix` and `DelimiterOutputPrefix`
- **Example Failing Syntax:**
  ```polyglot
  [<] <config:pg.path << \\path\\file.py
  [>] >return_value:pg.int >> .output
  ```
- **Documentation:** `docs/user/examples/error-handling-patterns.md:32-33`
- **Root Cause:** `lex_less_operators()` and `lex_greater_operators()` only handle compound operators (`<<`, `>>`, `<?`, `>?`), not standalone delimiters
- **Affects:**
  - All runtime wrapper calls
  - All utility pipeline calls with named arguments
  - All documentation examples using argument prefixes

#### Issue #5: Missing Macro System Block Markers
- **Severity:** CRITICAL - Feature Non-Functional
- **Location:** `polyglot-lexer/src/lexer.rs:317-352`, `polyglot-lexer/src/token.rs`
- **Missing Markers:**
  - `[M]` - Macro definition (BlockMacroDefinition)
  - `[{]` - Scope input (BlockScopeInput)
  - `[}]` - Scope output (BlockScopeOutput)
- **Example Failing Syntax:**
  ```polyglot
  [M] DatabaseSetup
  [{] .db_host:pg.string  // Scope input
  [}] .db_conn:pg.db      // Scope output
  [X]
  ```
- **Documentation:** `docs/user/language/06-block-markers.md:1313-1457`
- **Root Cause:** Macro system documented but tokens not added to lexer
- **Affects:** Entire macro system non-functional

---

### High Priority

#### Issue #2: Incomplete Test Coverage
- **Severity:** HIGH - Quality Risk
- **Location:** `polyglot-lexer/src/tests.rs`
- **Current State:** 20 basic tests, missing comprehensive v0.0.2 syntax validation
- **Missing Test Categories:**
  - Package declarations with `::` separator
  - Argument prefix tokenization (`<arg`, `>arg`)
  - Type dot notation (`pg.string`)
  - Conditional block nesting (`[?]` + `[~]`)
  - Wildcard patterns (`*?`)
  - Full pipeline examples from documentation
- **Root Cause:** Tests written before full spec finalized, not updated
- **Affects:** False confidence in "DONE" story status

#### Issue #6: Missing Alias Block Marker
- **Severity:** HIGH - Feature Non-Functional
- **Location:** `polyglot-lexer/src/lexer.rs:317-352`, `polyglot-lexer/src/token.rs`
- **Missing Marker:** `[A]` - Alias definition (BlockAliasDefinition)
- **Example Failing Syntax:**
  ```polyglot
  [#] Path.Identifiers.MyApp.DataDirectory
  [A] DataDir  // Alias definition
  [X]
  ```
- **Documentation:** `docs/user/language/06-block-markers.md:1174-1202`
- **Root Cause:** Alias feature documented but token not implemented
- **Affects:** Cannot create package-scoped aliases

---

### Medium Priority

#### Issue #3: Double Colon `::` Namespace Separator
- **Severity:** MEDIUM - Needs Verification
- **Location:** Parser (specific location TBD)
- **Current Behavior:** Tokenizes as two consecutive `DelimiterColon` tokens
- **Expected Behavior:** Parser should interpret `::` as namespace separator
- **Example Syntax:**
  ```polyglot
  [@] @Local::Examples.HelloWorld:0.0.0.1
  ```
- **Status:** Likely OK - just needs explicit parser test
- **Documentation:** All package examples in `docs/user/examples/`
- **Affects:** All package declarations

#### Issue #4: Conditional Block Nesting
- **Severity:** MEDIUM - Needs Verification
- **Location:** Parser (specific location TBD)
- **Current Behavior:** `[~]` block marker exists, nesting depth tracking unclear
- **Example Syntax:**
  ```polyglot
  [?] .condition
  [~][r] |Operation
  [~][~][!] !Error  // Nested error handling
  ```
- **Status:** Block markers implemented, parser nesting needs verification
- **Documentation:** `docs/user/examples/error-handling-patterns.md:28-41`
- **Affects:** Complex conditional logic with error handling

#### Issue #7: Format Literal Syntax Verification
- **Severity:** MEDIUM - Needs Verification
- **Current Behavior:** Unknown
- **Expected Behavior:** Parse `JSON"path"` and chained methods `JSON.FilenameKey"path".ExcludeFileName"filter"`
- **Example Syntax:**
  ```polyglot
  [s] .config << JSON"\\Config\\app.json"
  [s] .secrets << JSON.FilenameKey"\\Secrets\\*.json".ExcludeFileName"*example*"
  ```
- **Status:** May work as `Identifier` + `String`, but chained methods need verification
- **Documentation:** `docs/user/language/06-block-markers.md:560-857`
- **Affects:** `[s]` serial load block advanced features

---

## Proposed Solution

### Phase 1: Token Definitions (Story 1.2)

**Changes to `polyglot-lexer/src/token.rs`:**

```rust
// Add to Delimiters section (after line 111):
DelimiterInputPrefix,   // <
DelimiterOutputPrefix,  // >

// Add to Block Markers section (after BlockLineContinuation):
BlockMacroDefinition,   // [M]
BlockScopeInput,        // [{]
BlockScopeOutput,       // [}]
BlockAliasDefinition,   // [A]
```

**Add token descriptions:**

```rust
TokenKind::DelimiterInputPrefix => "input prefix <",
TokenKind::DelimiterOutputPrefix => "output prefix >",
TokenKind::BlockMacroDefinition => "macro definition marker [M]",
TokenKind::BlockScopeInput => "scope input marker [{]",
TokenKind::BlockScopeOutput => "scope output marker [}]",
TokenKind::BlockAliasDefinition => "alias definition marker [A]",
```

**Estimated Effort:** 1 hour
**Risk:** Low - straightforward enum additions

---

### Phase 2: Lexer Implementation (Story 1.3)

**Changes to `polyglot-lexer/src/lexer.rs`:**

#### 1. Modify `lex_less_operators()` (lines 612-648):

```rust
fn lex_less_operators(&mut self) -> Result<Token, LexerError> {
    let start_line = self.line;
    let start_column = self.column;

    self.advance(); // consume '<'

    match self.current_char() {
        '~' => {
            self.advance();
            Ok(Token::new(TokenKind::OpDefault, "<~".to_string(), start_line, start_column))
        }
        '<' => {
            self.advance();
            Ok(Token::new(TokenKind::OpPush, "<<".to_string(), start_line, start_column))
        }
        '?' => {
            self.advance();
            Ok(Token::new(TokenKind::OpLess, "<?".to_string(), start_line, start_column))
        }
        // NEW: Standalone < before identifier
        'a'..='z' | 'A'..='Z' | '_' => {
            Ok(Token::new(TokenKind::DelimiterInputPrefix, "<".to_string(), start_line, start_column))
        }
        _ => Err(LexerError::UnexpectedCharacter {
            line: start_line,
            column: start_column,
            character: '<',
        })
    }
}
```

#### 2. Modify `lex_greater_operators()` (lines 651-679):

```rust
fn lex_greater_operators(&mut self) -> Result<Token, LexerError> {
    let start_line = self.line;
    let start_column = self.column;

    self.advance(); // consume '>'

    match self.current_char() {
        '>' => {
            self.advance();
            Ok(Token::new(TokenKind::OpPull, ">>".to_string(), start_line, start_column))
        }
        '?' => {
            self.advance();
            Ok(Token::new(TokenKind::OpGreater, ">?".to_string(), start_line, start_column))
        }
        // NEW: Standalone > before identifier
        'a'..='z' | 'A'..='Z' | '_' => {
            Ok(Token::new(TokenKind::DelimiterOutputPrefix, ">".to_string(), start_line, start_column))
        }
        _ => Err(LexerError::UnexpectedCharacter {
            line: start_line,
            column: start_column,
            character: '>',
        })
    }
}
```

#### 3. Add block markers to `lex_block_marker()` (lines 317-352):

```rust
let kind = match marker_char {
    // ... existing markers ...
    'M' => TokenKind::BlockMacroDefinition,
    '{' => TokenKind::BlockScopeInput,
    '}' => TokenKind::BlockScopeOutput,
    'A' => TokenKind::BlockAliasDefinition,
    // ... rest of markers ...
}
```

**Estimated Effort:** 2 hours
**Risk:** Low - pattern-matching additions to existing functions

---

### Phase 3: Comprehensive Test Suite (Story 1.4)

**Add to `polyglot-lexer/src/tests.rs`:**

#### Package Declaration Tests
```rust
#[test]
fn test_package_declaration_with_double_colon() {
    let input = "[@] @Local::Examples.HelloWorld:1.0.0.0\n[X]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::BlockPackageStart);
    assert_eq!(tokens[1].kind, TokenKind::DelimiterAt);
    assert_eq!(tokens[2].kind, TokenKind::Identifier); // Local
    assert_eq!(tokens[3].kind, TokenKind::DelimiterColon); // :
    assert_eq!(tokens[4].kind, TokenKind::DelimiterColon); // :
    assert_eq!(tokens[5].kind, TokenKind::Identifier); // Examples
    // ... continue verification
}
```

#### Argument Prefix Tests
```rust
#[test]
fn test_input_argument_prefix() {
    let input = "[<] <config:pg.path << value";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::BlockInputBinding);
    assert_eq!(tokens[1].kind, TokenKind::DelimiterInputPrefix); // <
    assert_eq!(tokens[2].kind, TokenKind::Identifier); // config
    assert_eq!(tokens[3].kind, TokenKind::DelimiterColon);
    assert_eq!(tokens[4].kind, TokenKind::TypeNamespace); // pg
    assert_eq!(tokens[5].kind, TokenKind::DelimiterDot);
    assert_eq!(tokens[6].kind, TokenKind::TypePath); // path
    assert_eq!(tokens[7].kind, TokenKind::OpPush); // <<
}

#[test]
fn test_output_argument_prefix() {
    let input = "[>] >result:pg.int >> .output";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::BlockOutputBinding);
    assert_eq!(tokens[1].kind, TokenKind::DelimiterOutputPrefix); // >
    assert_eq!(tokens[2].kind, TokenKind::Identifier); // result
    // ... continue verification
}
```

#### Macro System Tests
```rust
#[test]
fn test_macro_definition_markers() {
    let input = "[M] TestMacro\n[{] .input:pg.string\n[}] .output:pg.string\n[X]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::BlockMacroDefinition);
    // ... verify [{] and [}] tokens
}
```

#### Full Pipeline Integration Test
```rust
#[test]
fn test_complete_pipeline_from_docs() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[|] |TestPipeline
[i] .input:pg.string
[t] |T.Call
[W] |W.Python3.11

[r] |U.RT.Python.Cli
[<] <config:pg.path << \\path\\file.py
[<] <kwargs.data:pg.string << .input
[>] >return_value:pg.int >> .result

[o] .result:pg.int
[X]
"#;
    let tokens = lex(input).unwrap();
    // Comprehensive token sequence verification
}
```

**Estimated Effort:** 4 hours
**Risk:** Low - test additions, no production code changes

---

### Phase 4: Parser Updates (Story 1.5)

**Note:** Exact changes depend on current parser implementation structure.

**Requirements:**
1. Recognize `DelimiterInputPrefix` + `Identifier` as input argument declaration
2. Recognize `DelimiterOutputPrefix` + `Identifier` as output argument declaration
3. Handle macro system markers `[M]`, `[{]`, `[}]`
4. Handle alias marker `[A]`
5. Verify `::` parsing as namespace separator
6. Verify conditional block nesting with `[~]`

**Add Parser Tests:**
```rust
#[test]
fn test_parse_input_argument() {
    let input = "[<] <config:pg.path << value";
    let ast = parse(input).unwrap();
    // Verify AST structure for input argument
}

#[test]
fn test_parse_macro_definition() {
    let input = "[M] TestMacro\n[{] .input:pg.string\n[}] .output:pg.string\n[X]";
    let ast = parse(input).unwrap();
    // Verify AST structure for macro
}
```

**Estimated Effort:** 6 hours (includes investigation time)
**Risk:** Medium - depends on parser architecture

---

### Phase 5: Syntax Validator Updates (Story 1.6)

**Requirements:**
1. Accept standalone `<` and `>` before identifiers
2. Accept macro system markers
3. Accept alias marker
4. Validate against all documentation examples

**Add Validation Tests:**
```rust
#[test]
fn test_validate_all_doc_examples() {
    // Read all examples from docs/user/examples/
    // Run validator on each
    // Assert all pass
}
```

**Estimated Effort:** 3 hours
**Risk:** Low - update validation rules based on new tokens

---

## Verification Strategy

### Automated Testing
1. All new unit tests must pass (40+ new tests)
2. All existing tests must continue to pass
3. Run lexer on all documentation examples (100+ examples)
4. Run validator on all documentation examples

### Manual Testing
1. Tokenize complex real-world `.pg` file
2. Parse complete pipeline with all syntax features
3. Validate error messages are clear for invalid syntax

### Documentation Validation
1. Every syntax pattern in `docs/user/` must have corresponding test
2. Create mapping document: "Doc Example → Test Case"
3. Update any outdated documentation examples

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Breaking existing functionality | Low | High | Comprehensive test suite, incremental changes |
| Parser complexity increases | Medium | Medium | Well-documented changes, code review |
| New bugs introduced | Medium | Medium | Test-driven approach, validation tests |
| Performance degradation | Low | Low | Benchmark before/after |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Underestimated effort | Medium | High | Buffer time, phased approach allows early assessment |
| Story 1.8 remains blocked | High | High | Prioritize critical issues (#1, #5) first |
| Additional issues discovered | Medium | Medium | Document as found, triage priority |

---

## Success Criteria

**Must Have (Required for Approval):**
- ✅ All 7 identified issues addressed
- ✅ Stories 1.2-1.6 pass updated acceptance criteria
- ✅ All tests pass (existing + new)
- ✅ All documentation examples tokenize successfully
- ✅ Story 1.8 unblocked

**Should Have (Quality Goals):**
- ✅ Test coverage >90% for lexer
- ✅ Parser handles all v0.0.2 syntax
- ✅ Clear error messages for invalid syntax

**Nice to Have (Future Improvements):**
- Documentation auto-validation in CI/CD
- Syntax fuzzing tests
- Performance benchmarks

---

## Timeline Estimate

| Phase | Estimated Effort | Dependencies |
|-------|------------------|--------------|
| Phase 1: Token Definitions | 1 hour | None |
| Phase 2: Lexer Implementation | 2 hours | Phase 1 |
| Phase 3: Test Suite | 4 hours | Phase 1, 2 |
| Phase 4: Parser Updates | 6 hours | Phase 1, 2, 3 |
| Phase 5: Validator Updates | 3 hours | Phase 1-4 |
| **Total** | **16 hours** (~2 days) | Sequential |

**Buffer:** +4 hours for unexpected issues = **20 hours total**

---

## Approval Required From

- [ ] Product Manager (John) - Business impact acceptable
- [ ] Tech Lead - Architecture approach sound
- [ ] QA Lead - Testing strategy adequate
- [ ] Developer (implementation feasibility)

---

## References

### Documentation
- `docs/user/language/06-block-markers.md` - Block marker specification
- `docs/user/language/05-operators.md` - Operator specification
- `docs/user/examples/*.md` - Real-world syntax examples
- `docs/user/async-centric-paradigm.md` - Core syntax patterns

### Code Locations
- `polyglot-lexer/src/token.rs` - Token definitions
- `polyglot-lexer/src/lexer.rs` - Lexer implementation
- `polyglot-lexer/src/tests.rs` - Lexer tests
- `polyglot-parser/src/parser.rs` - Parser implementation

### Related Issues
- Sprint Status: `docs/Agile/stories/sprint-status.yaml`
- Story Files: `docs/Agile/stories/1-*.md`

---

**Document Status:** DRAFT
**Next Steps:** Review and approval, then implement Phase 1
**Last Updated:** 2025-12-05
