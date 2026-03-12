# Story 1.9: Syntax Consistency - Operator Prefixes for All Objects

**Syntax Version:** This story implements v0.0.3 syntax. v0.0.4 syntax (with `$` variables, `{|}` blocks, `[|]` IO) will be implemented in future epics.

**Epic:** Epic 1 - Lexer & Parser
**Created:** 2025-12-04
**Created By:** Claude (Dev Agent)

---

## Status

**Draft**

---

## Story

**As a** Polyglot language designer,
**I want** all Polyglot objects to require operator prefixes for complete syntax consistency,
**so that** the language has a unified, predictable syntax where every object type is immediately recognizable by its prefix character.

---

## Background & Motivation

### Current Syntax Inconsistency

Polyglot has a **mostly consistent** prefix system, but with critical gaps:

**Current Prefixed Objects:**
- Variables: `.name` (dot prefix)
- Enumerations: `#Config` (hash prefix)
- Pipelines: `|Pipeline` (pipe prefix)
- Errors: `!Error` (exclamation prefix)
- Pipeline formatted strings: `|Pipeline"text"` (pipe prefix)

**Current Unprefixed Objects:**
- **String literals:** `"text"` (NO PREFIX - inconsistent!)
- **Pipeline arguments:** Implicit in syntax (NO EXPLICIT OPERATORS)

### The Problem

1. **String Literals Break the Pattern:**
```polyglot
[r] .name << "Alice"              // "Alice" has no prefix - inconsistent!
[r] .count << 42                  // 42 has no prefix (numeric literal)
[r] .config << #Config.Default    // #Config has prefix - consistent!
```

2. **Pipeline Arguments Are Implicit:**
```polyglot
[|] ProcessData
[i] .input: pg\string      // Input parameter - how do we know it's an input?
[o] .result: pg\int        // Output parameter - how do we know it's an output?
```

When calling the pipeline:
```polyglot
[r] |ProcessData(.input << "data")   // Are we pushing TO the pipeline or FROM it?
```

### Design Philosophy: "Everything Has a Prefix"

Polyglot's design principle is **visual clarity through operator prefixes**:
- Every object should be instantly recognizable by its leading character
- No ambiguity about what type of object you're working with
- Consistent syntax patterns across the language

**This story** fixes the remaining inconsistencies.

---

## Acceptance Criteria

### AC1: String Literal Prefix Requirement

**Given** a `.pg` file with string literals
**When** I compile the file
**Then** string literals require a prefix operator

**Proposed Syntax Options:**

**Option A: Quote is the prefix (current)**
```polyglot
[r] .message << "Hello, world!"    // " is the prefix
```
✅ **Already implemented** - the `"` character itself serves as the prefix
✅ No breaking change required

**Option B: String prefix operator**
```polyglot
[r] .message << $"Hello, world!"   // $ prefix for strings
[r] .message << s"Hello, world!"   // s prefix for strings
```
⚠️ Would be a breaking change

**Decision:** Treat `"` as the prefix operator (Option A)
- Semantically consistent: the quote character acts as the prefix
- No breaking changes needed
- Documentation update to clarify this design principle

**And** documentation clearly states: "String literals use `"` as their prefix operator, consistent with Polyglot's 'all objects have prefixes' design"

---

### AC2: Type Prefix Requirement - Colon Operator

**Given** a variable or parameter with a type declaration
**When** I specify the type
**Then** the type must use `:` prefix with `.` hierarchy separator

**Design Decision:**
All Polyglot objects require operator prefixes. Types are no exception.

**Syntax Pattern:**
```
:namespace.type.subtype
```

**Examples:**
```polyglot
[r] .count:pg.int << 42
[r] .name:pg.string << "Alice"
[r] .items:pg.array.pg.int << {1, 2, 3}
[r] .data:pg.array.pg.string
[r] .config:pg.serial
```

**Note on Generic Types:**
Polyglot only supports single-parameter generics (arrays). No nested collections or multi-parameter generics (HashMap, Result<T,E>) exist by design - complex data manipulation happens in Python/Rust/etc., not in Polyglot.

**Generic Syntax:** All dots, no braces
- Simple: `:namespace.type`
- Array: `:namespace.array.namespace.element_type`
- Serial: `:pg.serial` (flexible data with numbered keys)

**Consistency with Existing Syntax:**
- Package: `@registry::package:version` (uses `:` for version)
- Variable: `.name.hierarchy` (uses `.` for hierarchy)
- Type: `:namespace.type` (uses `:` prefix, `.` for hierarchy)

**Benefits:**
1. ✅ Consistent prefix pattern: `.` variables, `#` enums, `|` pipelines, `:` types
2. ✅ Unified hierarchy separator: `.` for both variables AND types
3. ✅ IDE autocomplete trigger: `:` prefix signals type context
4. ✅ Parser clarity: unambiguous type vs variable reference

**Whitespace Rules:**
- **Canonical:** `.variable:type` (no space before colon)
- **Accepted:** `.variable :type` (space triggers compile WARNING)
- **Parser:** Accepts both, warns on space, suggests removal
- **Formatter:** Auto-removes space to canonical form

**Compiler Behavior:**
```
Warning: Unnecessary whitespace before type declaration
  --> example.pg:5:14
   |
 5 | [r] .count :pg.int << 42
   |            ^ Remove this space
   |
   = help: Change to `.count:pg.int`
   = note: Formatter can fix this automatically with `polyglot fmt example.pg`
```

**Replacement Pattern:**
- Old: `pg\int`, `pg\array{pg\int}`, `rs\Vec` (backslash separator, no prefix, braces for generics)
- New: `:pg.int`, `:pg.array.pg.int`, `:rs.Vec` (colon prefix, dot separator throughout)

**Migration Impact:**
- ALL existing `.pg` files with type declarations need updating
- Simple find/replace: `pg\` → `:pg.`, `rs\` → `:rs.`, etc.
- All documentation examples must be updated
- All test fixtures must be updated

---

### AC3: Pipeline Argument Operators - Input/Output Clarity

**Given** a pipeline call with arguments
**When** I pass data to/from the pipeline
**Then** the direction is explicitly marked with `<` (input) or `>` (output) operators

**Current Ambiguous Syntax:**
```polyglot
[|] ProcessData
[i] .input: pg\string
[o] .result: pg\string
[X]

// When calling - direction is unclear:
[r] .output << |ProcessData(.input << "data")  // Is .input going IN or OUT?
```

**Proposed Clear Syntax:**
```polyglot
// Explicitly mark inputs with < (data flows INTO pipeline)
[r] .output << |ProcessData(< .source_data)

// Explicitly mark outputs with > (data flows OUT OF pipeline)
[r] |ProcessData(> .destination_var)

// Multiple arguments with clear direction:
[r] .result << |ProcessData(
    < .input_file,
    < .config,
    > .output_file
)
```

**Operator Semantics:**
- `<` : "Take from calling scope, push into pipeline input"
- `>` : "Pull from pipeline output, push into calling scope variable"

**Benefits:**
1. ✅ Clear data flow direction
2. ✅ Explicit about what's being passed
3. ✅ Consistent with Polyglot's operator-first design
4. ✅ Prevents confusion about parameter binding

---

### AC3: Operator Prefix Reference Documentation

**Given** language documentation
**When** developer reads syntax guide
**Then** a comprehensive "Operator Prefixes" reference page exists

**Required Content:**

| Prefix | Object Type | Example | Purpose |
|--------|-------------|---------|---------|
| `.` | Variable | `.count` | Variable reference |
| `#` | Enumeration | `#Config` | Enumeration reference |
| `\|` | Pipeline | `\|ProcessData` | Pipeline reference |
| `!` | Error | `!NetworkError` | Error reference |
| `"` | String | `"text"` | String literal |
| `<` | Pipeline Input | `< .var` | Argument flows INTO pipeline |
| `>` | Pipeline Output | `> .var` | Argument flows OUT OF pipeline |
| `~` | Unpack | `~data` | Unpack collection |
| `@` | Package | `@Package` | Package reference |

**And** examples showing all operators in context:
```polyglot
[@] Local@MyPkg:1.0.0
[X]

[#] #Config
[<] .timeout: pg\int << 30
[X]

[|] ProcessWithIO
[i] .input_data: pg\string
[i] .config: #Config
[o] .result: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .message << "Processing {.input_data}"
[r] .timeout << #Config.timeout
[o] .result: pg\string
[X]

[|] Caller
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .source << "input data"
[r] .config_enum << #Config
[r] .output << |ProcessWithIO(
    < .source,           // Input: push TO pipeline
    < .config_enum,      // Input: push TO pipeline
    > .result_var        // Output: pull FROM pipeline
)
[o] .result_var: pg\string
[X]
```

---

### AC4: Lexer Token Updates for Pipeline Argument Operators

**Given** lexer implementation
**When** parsing pipeline arguments
**Then** lexer recognizes `<` and `>` as argument direction operators

**New Tokens Required:**
```rust
// Pipeline Argument Operators (2 tokens)
OpPipelineInput,    // < (argument flows INTO pipeline)
OpPipelineOutput,   // > (argument flows OUT OF pipeline)
```

**Lexer Logic:**
- When inside pipeline argument list `(...)`, recognize `<` and `>` as argument operators
- Outside pipeline arguments, `<` and `>` remain comparison operators (`<?`, `>?`)
- Context-dependent tokenization based on parser state

**Edge Cases:**
- Distinguish `< .var` (pipeline input) from `<?` (less than comparison)
- Distinguish `> .var` (pipeline output) from `>?` (greater than comparison)
- Handle whitespace: `<.var` vs `< .var` (both valid)

---

### AC5: Parser Updates for Pipeline Argument Operators

**Given** parser implementation
**When** parsing pipeline calls with arguments
**Then** parser requires `<` or `>` prefix for each argument

**Syntax Rules:**

1. **Pipeline call with arguments:**
```bnf
pipeline_call ::= '|' pipeline_name '(' argument_list? ')'
argument_list ::= argument (',' argument)*
argument      ::= ('<' | '>') expression
```

2. **Input argument (< prefix):**
```polyglot
[r] |Pipeline(< .input_var)
```
→ Binds `.input_var` from calling scope to pipeline input parameter

3. **Output argument (> prefix):**
```polyglot
[r] |Pipeline(> .output_var)
```
→ Binds pipeline output parameter to `.output_var` in calling scope

4. **Error Cases:**
```polyglot
[r] |Pipeline(.input_var)           // ERROR: Missing < or > operator
[r] |Pipeline(< .input, .output)    // ERROR: Second arg missing operator
```

**AST Updates:**
```rust
pub struct PipelineArgument {
    pub direction: ArgumentDirection,
    pub expression: Expression,
    pub span: Span,
}

pub enum ArgumentDirection {
    Input,   // < operator
    Output,  // > operator
}
```

---

### AC6: Backward Compatibility Consideration

**Given** existing `.pg` files without pipeline argument operators
**When** compiled with new parser
**Then** provide migration path and clear error messages

**Migration Strategy:**

**Option A: Breaking change with helpful errors**
```
Error: Pipeline argument missing direction operator
  --> file.pg:15:20
   |
15 | [r] .result << |Process(.input_data)
   |                         ^^^^^^^^^^^ missing < or >
   |
Help: Specify argument direction:
  - Use < for inputs:  |Process(< .input_data)
  - Use > for outputs: |Process(> .output_var)
```

**Option B: Deprecation warning first**
```
Warning: Pipeline argument without direction operator is deprecated
  --> file.pg:15:20
   |
15 | [r] .result << |Process(.input_data)
   |                         ^^^^^^^^^^^ will require < or > in future version
   |
Help: Add direction operator: |Process(< .input_data)
```

**Decision:** Option A (breaking change) - enforce consistency immediately
- Clearer language semantics
- Better error messages now than later
- Easier to learn correct syntax from the start

---

### AC7: Documentation Updates - Design Philosophy

**Given** user documentation
**When** developer learns Polyglot
**Then** design philosophy section explains operator prefix system

**New Documentation Section:** `docs/user/design-philosophy.md`

**Content:**
```markdown
# Polyglot Design Philosophy

## Operator Prefixes: Visual Clarity Through Consistency

Polyglot enforces a strict rule: **every object must have an operator prefix**.

### Why Operator Prefixes?

1. **Instant Recognition:** See `.` → know it's a variable
2. **Reduced Ambiguity:** `name` vs `.name` - which is a variable?
3. **Consistent Syntax:** Same pattern everywhere in the language
4. **Editor Support:** Easy to implement syntax highlighting
5. **Parse Efficiency:** Lexer can tokenize by first character

### Complete Operator Prefix Table

| Prefix | Type | Example | When to Use |
|--------|------|---------|-------------|
| `.` | Variable | `.count` | Referencing variables |
| `#` | Enum | `#Config.timeout` | Referencing enum values |
| `\|` | Pipeline | `\|ProcessData` | Calling pipelines |
| `!` | Error | `!NetworkError` | Error handling |
| `"` | String | `"text"` | String literals |
| `<` | Input | `< .var` | Pipeline input arguments |
| `>` | Output | `> .var` | Pipeline output arguments |

### Design Principle: No Bare Identifiers

Polyglot intentionally has **no bare identifiers** in user code.

**Other Languages (ambiguous):**
```javascript
name = "Alice"        // Is 'name' a variable? A function? A class?
```

**Polyglot (explicit):**
```polyglot
.name << "Alice"      // . prefix → definitely a variable
```

This consistency makes code easier to read, write, and maintain.
```

---

### AC8: Comprehensive Testing

**Given** updated lexer and parser
**When** tests run
**Then** all operator prefix scenarios are tested:

**Lexer Tests:**
1. `test_lex_pipeline_input_operator()` - tokenize `<` in argument context
2. `test_lex_pipeline_output_operator()` - tokenize `>` in argument context
3. `test_lex_comparison_vs_argument_operator()` - distinguish `<?` from `<`

**Parser Tests:**
1. `test_parse_pipeline_call_with_input_args()`
2. `test_parse_pipeline_call_with_output_args()`
3. `test_parse_pipeline_call_mixed_args()`
4. `test_parse_pipeline_call_missing_operator_fails()`

**Integration Tests:**
1. Complete pipeline with all operator prefixes
2. Nested pipeline calls with argument operators
3. Error messages for missing operators

---

## Tasks / Subtasks

### Task 1: Documentation - Operator Prefix Philosophy (AC: 7, 3)

- [ ] **Create design philosophy document**
  - [ ] File: `docs/user/design-philosophy.md`
  - [ ] Section: "Operator Prefixes: Visual Clarity"
  - [ ] Explain why every object needs a prefix
  - [ ] Show comparison with other languages

- [ ] **Create operator prefix reference**
  - [ ] File: `docs/user/syntax/operator-prefixes.md`
  - [ ] Complete table of all prefixes
  - [ ] Examples for each operator
  - [ ] Common mistakes and how to fix them

- [ ] **Update existing documentation**
  - [ ] Clarify that `"` is the string prefix operator
  - [ ] Update all examples to use `<` and `>` for pipeline arguments
  - [ ] Add design rationale sections

---

### Task 2: Lexer Updates - Pipeline Argument Operators (AC: 4)

- [ ] **Add new tokens to token.rs**
  - [ ] Add `OpPipelineInput // <` to token enum
  - [ ] Add `OpPipelineOutput // >` to token enum
  - [ ] Update token count (104 → 106 tokens)
  - [ ] Add to "Pipeline Argument Operators" section

- [ ] **Implement context-aware tokenization**
  - [ ] Track parser state: inside pipeline arguments or not
  - [ ] When inside `()` after pipeline: `<` → `OpPipelineInput`
  - [ ] When inside `()` after pipeline: `>` → `OpPipelineOutput`
  - [ ] Outside pipeline args: `<` → part of `<?` comparison
  - [ ] Outside pipeline args: `>` → part of `>?` comparison

- [ ] **Add lexer tests**
  - [ ] Test: `<` in pipeline arguments → `OpPipelineInput`
  - [ ] Test: `>` in pipeline arguments → `OpPipelineOutput`
  - [ ] Test: `<?` comparison → `OpLess`
  - [ ] Test: `>?` comparison → `OpGreater`
  - [ ] Test: whitespace handling `< .var` vs `<.var`

---

### Task 3: Parser Updates - Pipeline Argument Syntax (AC: 5)

- [ ] **Update AST definitions (ast.rs)**
  - [ ] Create `PipelineArgument` struct with `direction` and `expression`
  - [ ] Create `ArgumentDirection` enum: `Input`, `Output`
  - [ ] Update `PipelineCall` to use `Vec<PipelineArgument>`

- [ ] **Update pipeline call parsing (parser.rs)**
  - [ ] Function: `parse_pipeline_call()`
  - [ ] Parse argument list: `(` arguments `)`
  - [ ] For each argument, require `<` or `>` operator
  - [ ] Parse expression after operator
  - [ ] Create `PipelineArgument` with direction

- [ ] **Add validation for missing operators**
  - [ ] If argument starts with identifier (not `<` or `>`), error
  - [ ] Error message: "Pipeline argument missing direction operator"
  - [ ] Help text: "Use < for inputs, > for outputs"

- [ ] **Add parser tests**
  - [ ] Test: `|Pipe(< .input)` → success
  - [ ] Test: `|Pipe(> .output)` → success
  - [ ] Test: `|Pipe(< .a, > .b)` → success (mixed)
  - [ ] Test: `|Pipe(.input)` → error (missing operator)
  - [ ] Test: `|Pipe(< .a, .b)` → error (second missing)

---

### Task 4: Error Messages & Migration (AC: 6)

- [ ] **Create helpful error for missing operator**
  - [ ] Error type: `PipelineArgumentMissingDirection`
  - [ ] Include span pointing to argument
  - [ ] Show both `<` and `>` options in help text
  - [ ] Include example of correct syntax

- [ ] **Create migration guide document**
  - [ ] File: `docs/user/migration/v0.0.3-pipeline-arguments.md`
  - [ ] List breaking changes
  - [ ] Show before/after examples
  - [ ] Provide sed/awk scripts for bulk updates

- [ ] **Test error message quality**
  - [ ] Verify error spans are accurate
  - [ ] Verify help text is shown
  - [ ] Verify examples are correct
  - [ ] Verify colors/formatting work in terminal

---

### Task 5: Integration Testing (AC: 8)

- [ ] **Create comprehensive integration test**
  - [ ] File: `polyglot-parser/tests/operator_prefix_consistency.rs`
  - [ ] Test complete `.pg` file with all operator prefixes
  - [ ] Include all prefix types in one example

- [ ] **Test suite for pipeline arguments**
  - [ ] File: `polyglot-parser/tests/pipeline_argument_operators.rs`
  - [ ] Test input-only arguments
  - [ ] Test output-only arguments
  - [ ] Test mixed input/output arguments
  - [ ] Test nested pipeline calls
  - [ ] Test error cases

- [ ] **Regression testing**
  - [ ] Run full existing test suite
  - [ ] Update any tests using old pipeline call syntax
  - [ ] Verify no unexpected breakages
  - [ ] Document any intentional breaking changes

---

### Task 6: Update Examples & Documentation (AC: 3, 7)

- [ ] **Update all code examples in documentation**
  - [ ] Search for `|Pipeline(` patterns
  - [ ] Add `<` or `>` operators to arguments
  - [ ] Verify updated examples compile

- [ ] **Update language syntax guide**
  - [ ] File: `docs/user/syntax/pipeline-calls.md`
  - [ ] Explain `<` and `>` operators
  - [ ] Show data flow diagrams
  - [ ] Provide multiple examples

- [ ] **Create cheat sheet**
  - [ ] File: `docs/user/quick-reference/operators.md`
  - [ ] One-page reference for all operators
  - [ ] Quick lookup table
  - [ ] Common patterns

---

### Task 7: Code Review & Finalization

- [ ] **Self-review code changes**
  - [ ] Check code style and consistency
  - [ ] Verify all error messages are helpful
  - [ ] Review test coverage
  - [ ] Check for edge cases

- [ ] **Run quality checks**
  - [ ] Run `cargo test --all` - all tests pass
  - [ ] Run `cargo clippy` - no warnings
  - [ ] Run `cargo fmt` - code formatted
  - [ ] Check documentation builds correctly

- [ ] **Update changelog**
  - [ ] File: `CHANGELOG.md`
  - [ ] Note: Breaking change - pipeline arguments require operators
  - [ ] Document new operator prefix philosophy
  - [ ] List all affected syntax

---

## Dev Notes

### Context

This story addresses a **fundamental design principle** of Polyglot: **"All objects must have operator prefixes"**.

While Polyglot has been mostly consistent with this principle, two gaps remain:

1. **String literals** appear to violate the rule (but technically don't - `"` is the prefix)
2. **Pipeline arguments** lack explicit direction operators (causing ambiguity)

This story **clarifies** the design philosophy and **enforces** it consistently across all syntax.

---

### Design Rationale: Why Operator Prefixes?

**Problem in Other Languages:**
```python
# Python - what is 'name'?
name = "Alice"          # Variable assignment
name()                  # Function call?
name.method()           # Method call?
Name()                  # Class instantiation?
```

**Solution in Polyglot:**
```polyglot
.name << "Alice"        // . → Variable (always)
|name()                 // | → Pipeline (always)
#Name                   // # → Enumeration (always)
```

**Benefits:**
1. ✅ **Instant recognition** - no need to search for declaration
2. ✅ **Reduced cognitive load** - pattern matching by first character
3. ✅ **Better tooling** - syntax highlighting is trivial
4. ✅ **Clearer errors** - "expected variable `.name`, found pipeline `|name`"
5. ✅ **Faster parsing** - lexer can categorize by prefix

---

### String Literals: `"` Is the Prefix

**Clarification:** String literals DO have a prefix - the `"` character itself.

**Consistent with design:**
- Variables start with `.`
- Enumerations start with `#`
- Pipelines start with `|`
- Strings start with `"`

**No change needed** - just documentation to make this explicit.

---

### Pipeline Argument Operators: The Missing Piece

**Current Problem:**
```polyglot
[r] .result << |Process(.input_data, .output_data)
```
Which is input? Which is output? **Ambiguous!**

**Solution:**
```polyglot
[r] .result << |Process(< .input_data, > .output_data)
```
`<` = data flows INTO pipeline
`>` = data flows OUT OF pipeline

**Visual Mnemonic:**
```
< .input    →  Data flows left-to-right INTO pipeline
> .output   ←  Data flows right-to-left OUT OF pipeline
```

---

### Lexer Challenge: Context-Dependent Tokenization

**The Problem:**
- `<` can mean "pipeline input" or "less than comparison"
- `>` can mean "pipeline output" or "greater than comparison"

**Current Tokens:**
- `OpLess` for `<?` comparison
- `OpGreater` for `>?` comparison

**New Tokens:**
- `OpPipelineInput` for `<` in pipeline arguments
- `OpPipelineOutput` for `>` in pipeline arguments

**Solution: Context Tracking**

The lexer needs to know: "Am I inside pipeline arguments?"

**Approach 1: Parser-driven lexing**
- Parser tells lexer when entering/exiting pipeline argument context
- Lexer tokenizes based on context

**Approach 2: Two-phase lexing**
- Phase 1: Tokenize with generic tokens
- Phase 2: Parser re-categorizes based on context

**Approach 3: Lookahead disambiguation**
- If `<` followed by `.` or identifier → `OpPipelineInput`
- If `<` followed by `?` → part of `<?` comparison

**Recommended: Approach 3** - simplest, no context tracking needed

**Disambiguation Rules:**
```
< .var      →  OpPipelineInput  (< followed by variable)
<?          →  OpLess           (< followed by ?)
> .var      →  OpPipelineOutput (> followed by variable)
>?          →  OpGreater        (> followed by ?)
```

---

### AST Changes Required

**Current (ambiguous):**
```rust
pub struct PipelineCall {
    pub name: String,
    pub arguments: Vec<Expression>,  // No direction info!
    pub span: Span,
}
```

**Proposed (explicit):**
```rust
pub struct PipelineCall {
    pub name: String,
    pub arguments: Vec<PipelineArgument>,
    pub span: Span,
}

pub struct PipelineArgument {
    pub direction: ArgumentDirection,
    pub expression: Expression,
    pub span: Span,
}

pub enum ArgumentDirection {
    /// Input argument (< operator) - data flows INTO pipeline
    Input,
    /// Output argument (> operator) - data flows OUT OF pipeline
    Output,
}
```

---

### Breaking Change Impact Analysis

**Affected Code:**
- All pipeline calls with arguments
- Examples: `|Pipeline(.arg)` → `|Pipeline(< .arg)`

**Migration Effort:**
- Simple find/replace pattern
- Can provide automated migration script

**Migration Script Example:**
```bash
# Find all pipeline calls with arguments
rg '\|[A-Za-z.][\w.]*\([^)]+\)' --type polyglot

# Manual review required to determine < or >
# (Automated tool could use heuristics based on parameter names)
```

**Recommended Timeline:**
1. Implement in v0.0.3
2. Provide migration guide
3. Update all documentation examples
4. Update all test fixtures

---

### Implementation Complexity

**Estimated Effort:** 12-16 hours

**Breakdown:**
- Documentation: 4-5 hours (philosophy, reference, examples)
- Lexer changes: 3-4 hours (new tokens, disambiguation)
- Parser changes: 3-4 hours (AST, parsing logic, validation)
- Testing: 2-3 hours (lexer, parser, integration tests)

**Complexity:** Medium
- Lexer context disambiguation is the trickiest part
- AST changes are straightforward
- Documentation updates are extensive but mechanical

---

### Alternative Designs Considered

#### Alternative 1: No Pipeline Argument Operators
**Syntax:** `|Pipeline(.input, .output)`
**Problems:**
- ❌ Ambiguous direction
- ❌ Must rely on parameter position
- ❌ Error-prone

**Rejected** - violates design philosophy

---

#### Alternative 2: Named Parameters
**Syntax:** `|Pipeline(input: .data, output: .result)`
**Problems:**
- ❌ More verbose
- ❌ Requires knowing parameter names
- ❌ Less visual clarity

**Rejected** - too verbose, less consistent

---

#### Alternative 3: Different Operators
**Syntax:** `|Pipeline(→ .input, ← .output)`
**Problems:**
- ❌ Unicode characters
- ❌ Hard to type
- ❌ Inconsistent with ASCII-only language

**Rejected** - Polyglot is ASCII-only

---

#### Alternative 4: Separate Input/Output Syntax
**Syntax:**
```polyglot
[r] |Pipeline
    < .input_data        // Input section
    > .output_result     // Output section
```
**Pros:**
- ✅ Very clear separation
- ✅ Explicit input/output sections

**Cons:**
- ❌ More verbose
- ❌ Can't inline in expressions
- ❌ Breaks from function call syntax

**Rejected** - too different from standard calling conventions

---

### Chosen Design: `<` and `>` Operators

**Why this design wins:**
1. ✅ Visual and intuitive (`<` points left, `>` points right)
2. ✅ Concise (single character)
3. ✅ ASCII-only
4. ✅ Consistent with operator prefix philosophy
5. ✅ Easy to type
6. ✅ Clear in complex expressions

**Example showing clarity:**
```polyglot
[r] .final_result << |Transform(
    < .raw_data,
    < #Config.settings,
    > .processed_data
)
```
Instantly clear: two inputs, one output.

---

### Testing Strategy

**Lexer Tests:**
- Tokenize `<` in different contexts
- Tokenize `>` in different contexts
- Verify disambiguation works

**Parser Tests:**
- Parse pipeline calls with input args
- Parse pipeline calls with output args
- Parse mixed input/output
- Error cases: missing operators

**Integration Tests:**
- Complete workflows using argument operators
- Nested pipeline calls
- Complex data flow patterns

**Migration Tests:**
- Old syntax produces helpful errors
- Error messages guide to correct syntax
- Examples show correct usage

---

### Definition of Done

- [ ] All acceptance criteria met ✅
- [ ] All tasks completed ✅
- [ ] Lexer recognizes `<` and `>` as argument operators ✅
- [ ] Parser requires operators in pipeline calls ✅
- [ ] AST represents argument direction ✅
- [ ] All tests passing (lexer, parser, integration) ✅
- [ ] Design philosophy documented ✅
- [ ] Operator prefix reference complete ✅
- [ ] Migration guide written ✅
- [ ] All examples updated ✅
- [ ] Error messages are helpful ✅
- [ ] `cargo clippy` produces no warnings ✅
- [ ] Code formatted with `cargo fmt` ✅
- [ ] Changelog updated ✅

---

## Change Log

| Date | Version | Description | Author |
|------|---------|-------------|--------|
| 2025-12-04 | 1.0 | Story created - Syntax consistency with operator prefixes | Claude (Dev) |

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
