# Brainstorming Backlog

**Maintained By**: Mai (Secretary)
**Purpose**: Track topics requiring brainstorming sessions
**Last Updated**: 2025-11-21

---

## Pending Brainstorming Sessions

_No pending items - all brainstorming sessions completed!_

---

---

## Completed Brainstorming Sessions

### ✅ 2. Polyglot Formatting Guidelines (PFG) - Style Guide & Syntax Highlighting

**Status**: COMPLETED
**Completed Date**: 2025-11-21
**Session Document**: `docs/brainstorming-session-results-2025-11-21.md`
**ITIL Ticket**: SR-2025-002

**Outcome**:
- ✅ Complete PFG specification: `docs/technical/polyglot-formatting-guidelines-v1.0.md`
- ✅ PFG-001: Code Layout & Style (3-character rule, no indentation, spacing rules)
- ✅ PFG-002: Naming Conventions (.snake_case, |CamelCase, #CamelCase, !CamelCase)
- ✅ PFG-003: Syntax Highlighting Specification (VS Code Dark+ color scheme, 9 categories)
- ✅ PFG-004: Editor Integration Standards (LSP Tier 1/2/3 roadmap, code snippets)
- ✅ PFG-005: Tooling Standards (polyglot-tools, polyglot.toml, numbered error codes)
- ✅ PFG-006: Documentation Generation (from block markers + comments)
- ✅ PFG-007: Code Scaffolding (standard templates)
- ✅ Visual color preview: `docs/polyglot-syntax-darkmode-preview.html`

**Key Decisions**:
- NO indentation philosophy - block markers have dual duty (scope + relationships)
- 3 blank lines between file-scope definitions
- 1 blank line before branch points
- Context-aware line length (comments: 79, code: 99, strings: 120)
- VS Code Dark+ Python color scheme as standard
- Unified `polyglot-tools` binary (Rust-based)
- TOML-only configuration format

**Brainstorming Techniques Used**:
1. Mind Mapping (visual design, spacing, naming)
2. First Principles Thinking (why no indentation, why 3 blank lines)
3. SCAMPER Method (alternatives exploration)
4. Morphological Analysis (comprehensive coverage)
5. Six Thinking Hats (convergent synthesis)

**Next Steps**: Implement `polyglot-tools` prototype, TextMate grammar, LSP server

---

### ✅ 1. `[s]` Serial Load Block - Complete Specification

**Status**: COMPLETED
**Completed Date**: 2025-11-19
**Session Document**: `docs/project/brainstorming-session-results-2025-11-19.md`
**ITIL Tickets**: SR-2025-001 (Documentation), SR-2025-003 (Story Creation)

**Outcome**:
- ✅ Complete syntax: `[s] .variable: type << Format"path"`
- ✅ Parallel execution with automatic join (three-step process: collect paths → load parallel → assign)
- ✅ Error-carrying variables (value OR error with details)
- ✅ Two-level error handling: variable-level (`.var.error`) and scope-level (`[s][!]`)
- ✅ Shared error scope (one handler for all [s] at same scope/level)
- ✅ Partial success model (successful loads complete even when others fail)
- ✅ Variable state: Success = data + !NoError, Failure = #None.ErrorState + specific error
- ✅ Wildcard/array loading with combination strategies (FilenameKey, Index, Merge, Concat, FlatMap)
- ✅ Chained literal pipelines (NEW FEATURE): `JSON.FilenameKey"path".ExcludeFileName"*test*"`
- ✅ Reserved enumeration validation (!Serial.ReservedEnumeration.* errors)
- ✅ Filter syntax (chained ExcludeFileName)
- ✅ Complete examples for all use cases
- ✅ MVP scope clearly defined

**Key Decisions**:
- Parallel-first architecture for file loading
- Error resilience with partial success
- Type safety at runtime for reserved enumerations
- Extensibility via chained literal pipelines
- Fail-fast philosophy (empty files = errors)

**MVP Scope**:
- Basic file loading (JSON, YAML, TOML, XML)
- Parallel execution with automatic join
- Two-level error handling
- Wildcard/array loading
- Combination strategies (5 types)
- Chained literal pipelines
- Reserved enumeration validation

**Post-MVP** (Future):
- Security (path traversal, permissions)
- Caching, streaming, remote loading
- Compression/encryption

**Related**: Resolves brainstorming backlog item #1 (HIGH priority)

---

### ✅ 5. Error Handling Philosophy - !NoError Explicit Checks

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/project/agent-sessions/carson-2025-11-18-error-handling-philosophy.md`

**Outcome**:
- ✓ **Philosophy Confirmed**: "Success is implicit and preferred; explicit error checking is allowed for special cases" (Hybrid Approach)
- ✓ `!NoError` is fully accessible (reserved enumeration with error handling responsibilities)
- ✓ Can be compared, assigned, caught, and output (but typically not needed)
- ✓ Default behavior is implicit success (continuation after error handling)
- ✓ Best practice: use implicit continuation, explicit checks only for special cases
- ✓ Error state after catching: caller has `!NoError` unless it raises new error
- ✓ Callee that raised error keeps its error state
- ✓ Error aggregation patterns documented (3 patterns provided)
- ✓ Complete examples for all patterns

**Key Decisions**:
- `[~][!] !Error.Type` is the syntax for catching errors (scoped to previous block)
- `[o] !Error.Type` raises an error
- `[o] !NoError` is valid but redundant (implicit default)
- `[~][!] !NoError` can catch success case (useful for critical operation logging)
- Prefer implicit success (90% of cases), allow explicit for edge cases (10%)

**Best Practices Established**:
1. Use implicit continuation for success path
2. Catch specific errors only
3. Use error aggregation for multiple validations (extract to array or use boolean flag)
4. Catch `!NoError` only for meaningful cases (audit logging, critical operations)
5. Avoid checking `!NoError` explicitly unless necessary

**Related**: Resolves audit concerns about `!NoError` usage philosophy

---

### ✅ 3. URL Literals - Type System & Syntax

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/project/agent-sessions/carson-2025-11-18-url-literals-spec.md`

**Outcome**:
- ✅ `pg\url` defined as distinct type (like `pg\path`)
- ✅ Four URL literal variants specified: `url`, `urlencoded`, `urlraw`, `urltemplate`
- ✅ Seven reserved fields: `.protocol`, `.domain`, `.port`, `.path`, `.query`, `.anchor`, `.full`
- ✅ Query parameter parsing: `.query` returns `pg\serial` with key-value pairs
- ✅ Runtime protocol validation (not compile-time)
- ✅ Automatic port defaults by protocol
- ✅ String interpolation support in `urltemplate`
- ✅ Complete examples for all variants
- ✅ Standard library integration specified

**Key Decisions**:
- URLs use actual URL syntax (forward slashes) inside literals
- Type safety through distinct `pg\url` type
- Explicit encoding behavior through different literal variants
- Structured access via reserved fields

---

### ✅ 4. Comparison Operators & Range Notation - Syntax Finalization

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/project/agent-sessions/carson-2025-11-18-comparison-operators-design.md`

**Outcome**:
- ✅ Comparison operators DO EXIST in Polyglot
- ✅ Complete operator family defined: `>?`, `<?`, `=>?`, `=<?`, `=?`, `=!?`
- ✅ Range operators defined: `?[a, b]`, `?(a, b)` with mathematical interval notation
- ✅ Pattern matching: `*?` wildcard, `re?` regex
- ✅ Complete boolean logic: `[&]`, `[+]`, `[-]`, `[^]`, `[.]`
- ✅ Implicit AND in trigger blocks documented
- ✅ Type compatibility matrix established
- ✅ Exhaustive matching rules documented
- ✅ Deprecated: `?>` match operator, `Default` keyword, `..` range operator
- ✅ Story 1.2 UNBLOCKED

**Related Tickets**: PRB-2025-001 (Resolved), INC-2025-001

---

### ✅ 6. Undocumented Syntax Features - Complete Specification

**Status**: COMPLETED (Merged with Item #4)
**Completed Date**: 2025-11-18
**Session Documents**:
- `docs/project/agent-sessions/carson-2025-11-18-comparison-operators-design.md`
- `docs/project/agent-sessions/carson-2025-11-18-line-continuation-spec.md`

**Outcome**:
- ✅ `[*]` line continuation block fully specified
- ✅ `+"` string concatenation operator defined
- ✅ `[^]` confirmed as XOR operator (not line continuation)
- ✅ `[?]` switch system completely documented
- ✅ Exhaustive matching rules established
- ✅ Multiline string syntax: `[*]` + `+"`
- ✅ String interpolation: `{.variable}` for variables
- ✅ Deprecated: `?>` operator, `..` range operator, `Default` keyword

**Related Tickets**: PRB-2025-002 (Resolved)

---

### ✅ 2. `[M]` Macro Block - Complete Specification

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/project/agent-sessions/carson-2025-11-18-macro-system-spec.md`

**Outcome**:
- ✅ Macros are compile-time inline code templates
- ✅ Blocks insert by TYPE, not position
- ✅ `[M]` Macro definition block specified
- ✅ `[W]` Macro unwrap (inline insertion)
- ✅ `[{]` Scope input (variables IN from caller)
- ✅ `[}]` Scope output (variables OUT to caller)
- ✅ `[=]` Constant input (replaces `Fixed` keyword)
- ✅ `Macro.include"<chars+"` declaration syntax
- ✅ Multiple macro ordering: FIFO setup, LIFO cleanup
- ✅ Type safety at unwrap site
- ✅ Variable renaming flexibility
- ✅ Complete examples (database, logging, cache, file handling)

**Key Decisions**:
- Scope flow with braces: `{` brings IN, `}` sends OUT
- No keywords: `[=]` replaces `Fixed`
- RAII-like resource management (last acquired, first released)

---

## Guidelines for Brainstorming Sessions

When conducting brainstorming session:

1. **Preparation**:
   - Review existing documentation
   - List specific questions to answer
   - Gather related examples from other languages/systems

2. **Session Structure**:
   - Define the problem/feature clearly
   - Explore multiple approaches
   - Evaluate trade-offs
   - Propose recommendation

3. **Deliverables**:
   - Comprehensive syntax specification
   - Code examples demonstrating usage
   - Edge cases and error handling
   - Integration with existing features
   - Documentation-ready output

4. **Recording**:
   - Carson records in `docs/project/agent-sessions/carson-sessions.md`
   - Mai aggregates into approved decisions
   - Update this backlog with status

---

**Process Owner**: Mai (Secretary)
**Facilitator**: Carson (Brainstorming Coach)
