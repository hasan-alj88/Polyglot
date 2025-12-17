# Pre-Implementation Readiness Meeting - Epic 1 (Lexer)

**Date:** 2025-11-25
**Meeting Type:** Technical Readiness Review
**Attendees:** John (PM), Winston (Architect), Amelia (Dev), Bob (SM), Murat (Tea), Mary (Analyst)
**Objective:** Resolve all blocking questions before Epic 1 (Lexer) implementation

---

## Meeting Context

**Status Before Meeting:**
- ✅ Story 1-1: Project Workspace Setup (DONE)
- ⚠️ Story 1-2: Token Definitions (drafted, not ready)
- ⚠️ Story 1-3: Lexer Implementation (just created, not ready)

**Concerns Raised:**
- Ambiguity in token definitions
- Missing operators in Story 1-2
- Unclear error handling strategy
- Testing approach undefined
- Location tracking approach unclear

---

## Decisions Made

### 1. Polyglot Architecture Clarification

**Line Structure (Corrected):**
```
<One or More Block Elements (3 chars each)> <Expression>

Examples:
[|] my_pipeline                    # 1 block element + expression
[|][#] UserType                    # 2 block elements + expression
[i][<] .user_id pg\int << 0       # 2 block elements + expression
[r] |FetchUser                     # 1 block element + expression
[X]                                # 1 block element, no expression
```

**Two-Pass Compilation Model:**

**Pass 1: Lexer (Expression-Level)**
- Input: Polyglot source code (`.pg` file)
- Output: `Vec<Token>` + `Vec<LexerError>`
- Validates: Expression syntax only
- Errors: Invalid operators, unterminated strings, unexpected characters
- Independence: Each line is independent (error on line N doesn't affect line N+1)

**Pass 2: Parser (Structural + Semantic)**
- Input: `Vec<Token>` from lexer
- Output: AST + semantic errors
- Validates:
  - Block hierarchy (matching `[|]`...`[X]`, proper nesting)
  - Variable declarations (`.variable_name`, types, states)
  - Pipeline declarations and references (`|PipelineName`)
  - Unpack operators (`~` usage)
  - Enumerations (user-defined + reserved)

**Pass 3: IDE Language Server (Future, Out of Scope)**
- Real-time error highlighting
- Auto-completion
- Error recovery suggestions
- Quick fixes

**Separation of Concerns:**
- Lexer: Tokenization ONLY (no semantic validation)
- Parser: Structure + semantics
- IDE: Developer UX features

---

### 2. Q1: Enumeration Tokenization Strategy

**Question:** How to tokenize hierarchical enumeration references like `#Boolean.True` or `#Variables.States.Ready`?

**Decision:** Tokenize as separate atomic tokens

**Implementation:**
```rust
// #Boolean.True tokenizes as:
[
    EnumMarker("#"),
    Identifier("Boolean"),
    Dot("."),
    Identifier("True")
]
```

**Rationale:**
- Lexer handles syntax, parser handles semantics
- Hierarchical paths assembled during AST construction
- Works for all hierarchical identifiers (enumerations, pipelines, modules)

**Reserved Enumeration Handling:**

**Storage Strategy:** Hybrid approach (ADR to be added to architecture.md)
- Hard-code core reserved enums in Rust code (`#Boolean.*`)
- Load extendable enums from `reserved-enums.toml`
- Validation happens in Parser (Epic 2), not Lexer

**Starter List (v0.0.2):**
```
#Boolean.True           (Not extendable)
#Boolean.False          (Not extendable)
#Queues.Default         (Extendable)
#DT.Business.Week.TwoDayWeekend.SatSun  (Not extendable)
#DT.Business.Week.TwoDayWeekend.FriSat  (Not extendable)
#DT.Business.Week.TwoDayWeekend.ThuFri  (Not extendable)
... (all two-day combinations)
```

**Two Types:**
1. **Full Reserved:** Complete hierarchy un-editable (e.g., `#Boolean.*`)
2. **Extendable Reserved:** Pattern like `#Some.path.*.continue` where `*` can be user-defined, but reserved fields MUST be defined in `[#]` scope via `[<]`

**Backlog Item Created:**
- 📋 Catalog complete reserved enumeration set for v0.0.2 (deferred to Epic 2)

**Story Impact:**
- Story 1-2: ✅ No changes needed (existing tokens cover this)
- Story 1-3: ✅ No changes needed

---

### 3. Q2: Error Recovery Behavior

**Question:** Should lexer stop on first error or collect all errors?

**Decision:** Collect ALL errors, insert `TokenType::Error` placeholders

**Rationale:**
1. Developer productivity - fix all errors in one pass
2. IDE integration - language servers need all diagnostics
3. Polyglot's line independence - error on line N doesn't affect line N+1

**Implementation Strategy:**

```rust
pub struct LexResult {
    pub tokens: Vec<Token>,      // Includes Error tokens as placeholders
    pub errors: Vec<LexerError>,  // Detailed error information
}

// On error:
Err(_) => {
    // 1. Record detailed error
    errors.push(LexerError::UnexpectedCharacter { ... });

    // 2. Insert placeholder token (DON'T skip)
    tokens.push(Token {
        token_type: TokenType::Error,
        lexeme: bad_lexeme.to_string(),
        location: current_location,
    });

    // 3. Continue lexing
    self.advance();
}
```

**Why Placeholders?**
- Parser needs complete token stream to validate block hierarchy
- Error tokens preserve line structure for Pass 2
- Context-rich error messages (parser knows which expression failed)

**Unterminated String Handling:**
- Flag as `LexerError::UnterminatedString`
- Insert `TokenType::Error` placeholder
- **Don't recover, don't guess** - let user fix it properly
- Note: IDE language server (future) should provide real-time help

**Story Impact:**
- Story 1-2: Update AC #3 to specify error collection + placeholders
- Story 1-3: Update AC #4 to specify `LexResult` API

**Test Cases to Add:**
```rust
#[test]
fn test_multiple_errors_collected() {
    let source = "pipeline @@ $$ invalid";
    let result = Lexer::new(source).tokenize();
    assert_eq!(result.errors.len(), 3);
    assert!(result.tokens.iter().any(|t| t.token_type == TokenType::Error));
}

#[test]
fn test_lexer_never_panics() {
    let source = "\x00\xFF\xFE\xFD invalid UTF-8 @@##$$";
    let result = Lexer::new(source).tokenize();
    // Must complete without panic
    assert!(!result.errors.is_empty());
}
```

---

### 4. Q3: Coverage Tool Selection

**Question:** cargo-llvm-cov or tarpaulin?

**Decision:** Use `cargo-llvm-cov`

**Rationale:**

| Feature | tarpaulin | cargo-llvm-cov |
|---------|-----------|----------------|
| Platform Support | Linux only | ✅ Linux, macOS, Windows |
| Accuracy | ~85-90% | ✅ ~98-99% |
| Speed | 2-3x slower | ✅ 1.05-1.1x slower |
| Macro Coverage | ❌ Often missed | ✅ Fully captured |
| Used by Rust Core | ❌ No | ✅ Yes |

**Critical for Polyglot:**
- Logos macros generate 30-40% of lexer code
- tarpaulin would report false low coverage
- llvm-cov captures macro-generated code accurately

**Implementation:**

```bash
# Install
cargo install cargo-llvm-cov

# Local development
cargo llvm-cov test
cargo llvm-cov --html --open

# CI/CD enforcement
cargo llvm-cov --fail-under-lines 80
```

**CI/CD Integration:**
```yaml
# .github/workflows/ci.yml
- name: Install cargo-llvm-cov
  uses: taiki-e/install-action@cargo-llvm-cov

- name: Run tests with coverage
  run: cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

- name: Enforce coverage threshold
  run: cargo llvm-cov --fail-under-lines 80
```

**Story Impact:**
- Story 1-2: Update AC #4 to specify cargo-llvm-cov
- Story 1-2: Add DoD requirement: `cargo llvm-cov --fail-under-lines 80`

---

### 5. Q4: Location Tracking Strategy

**Question:** Use Logos built-in spans or manual tracking?

**Decision:** Manual tracking (incremental approach)

**Options Evaluated:**

| Approach | Complexity | Performance | Suitability |
|----------|-----------|-------------|-------------|
| Logos Spans | Medium | O(n²) - too slow | ❌ Fails NFR-P1 |
| Manual Tracking | Low | O(1) - fast | ✅ Meets NFR-P1 |
| Hybrid (Line Map) | High | O(log n) - overkill | ⚠️ Future optimization |

**Implementation:**

```rust
pub struct Lexer<'source> {
    logos_lexer: logos::Lexer<'source, TokenType>,
    source: &'source str,
    line: usize,      // Manual tracking
    column: usize,    // Manual tracking
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
```

**Rationale:**
- Meets NFR-P1 (<100ms for 1000-line files)
- Simple, maintainable, easy to test
- Polyglot pipelines are typically small files (100-500 lines)
- Can refactor to hybrid approach later if needed (no API changes)

**Common Pitfalls to Avoid:**
1. Handle UTF-8 multi-byte characters correctly (use `chars()` not `bytes()`)
2. Handle tabs if needed (tab stops at multiples of 8)
3. Always call `update_location()` after EVERY token (including errors)

**Test Cases to Add:**
```rust
#[test]
fn test_location_tracking_accuracy() {
    let source = r#"[|] my_pipeline
[i] .user_id pg\int
[r] |FetchUser"#;

    let result = Lexer::new(source).tokenize();
    assert_eq!(result.tokens[0].location, Location { line: 1, column: 1 });
    assert_eq!(result.tokens[2].location, Location { line: 2, column: 1 });
}

#[test]
fn test_error_location_tracking() {
    let source = "[|] pipeline_name $invalid";
    let result = Lexer::new(source).tokenize();
    assert_eq!(result.errors[0].location().column, 19);
}
```

**Story Impact:**
- Story 1-3: ✅ No changes needed (example already uses manual tracking)
- Story 1-3: Add dev note about UTF-8 handling and always calling `update_location()`

---

## Backlog Items Created

1. **📋 Complete Reserved Enumeration Catalog** (Deferred to Epic 2)
   - Document all reserved enumerations for v0.0.2
   - Implement `reserved-enums.toml` structure
   - Add validation in parser

2. **📋 IDE Language Server** (Deferred post-Epic 2)
   - Real-time error highlighting
   - Auto-completion for pipelines, enumerations, variables
   - Error recovery suggestions
   - Quick fixes

---

## Story Updates Required

### Story 1-2: Lexer Token Definitions

**Dev Notes to Add:**
- Line structure: One or more 3-char block elements + expression
- Hierarchical identifiers tokenized as separate tokens (`#`, `Identifier`, `.`)
- Lexer validates expression syntax only (no semantic validation)

**AC #3 Update:**
- Specify error collection (not early termination)
- Specify `TokenType::Error` placeholder insertion

**AC #4 Update:**
- Specify `cargo-llvm-cov` for coverage measurement
- Add DoD: `cargo llvm-cov --fail-under-lines 80`

**Test Cases to Add:**
- Multiple errors collected in single pass
- Error tokens inserted at correct positions
- Lexer never panics on invalid input (fuzz testing)

---

### Story 1-3: Lexer Implementation

**AC #4 Update:**
- Specify `LexResult { tokens, errors }` return type
- Clarify that `tokens` includes `TokenType::Error` placeholders

**Tasks to Add:**
- Implement error token insertion (no skip, no recovery)
- Always call `update_location()` after every token
- Handle UTF-8 characters correctly in location tracking

**Dev Notes to Add:**
- Manual location tracking is O(1) and sufficient for NFR-P1
- Polyglot's line independence enables simple error handling
- Parser (Epic 2) handles semantic validation

**Test Cases to Add:**
- Location tracking accuracy tests
- Error location accuracy tests
- Performance validation (NFR-P1: <100ms for 1000 lines)

---

## Architecture Document Updates

**New Section to Add: Reserved Enumeration Storage**

**Decision:** Hybrid approach
- Hard-code core reserved enums in `polyglot-parser/src/reserved_enums.rs`
- Load extendable enums from `polyglot-parser/reserved-enums.toml`
- Parser validates enumeration references during AST construction

**TOML Structure:**
```toml
[reserved.boolean]
path = "Boolean"
extendable = false
values = ["True", "False"]

[reserved.queues]
path = "Queues"
extendable = true
values = ["Default"]

[reserved.dt_business_week]
path = "DT.Business.Week"
extendable = true
# ... etc
```

---

## Risk Assessment

**Before Meeting:** HIGH
- 7 open architectural questions
- 3 stories with "not implementable" risk
- Ambiguous requirements

**After Meeting:** LOW
- ✅ All architectural decisions documented
- ✅ Implementation path is clear
- ✅ Test strategy defined
- ✅ Performance approach validated

---

## Next Actions

1. **Immediate (Bob):**
   - ✅ Record meeting notes (this document)
   - Generate sample Polyglot code for validation
   - Update Stories 1-2 and 1-3 with decisions
   - Update architecture.md with reserved enum storage decision

2. **Before Implementation (Bob):**
   - Run `/bmad:bmm:workflows:story-context` for Story 1-2
   - Mark Story 1-2 as "ready" using `/bmad:bmm:workflows:story-ready`

3. **Implementation (Amelia):**
   - Implement Story 1-2 (Token Definitions)
   - Implement Story 1-3 (Lexer Implementation)
   - Ensure >80% test coverage with cargo-llvm-cov

4. **Epic 2 Preparation:**
   - Write IR Specification document (scheduled post-Epic 1)
   - Complete reserved enumeration catalog

---

## Meeting Outcome

**✅ APPROVED TO PROCEED WITH EPIC 1 IMPLEMENTATION**

All blocking questions resolved. Stories ready for update and implementation.

---

**Document Status:** Final
**Next Review:** After Epic 1 completion (Stories 1-2, 1-3 done)
