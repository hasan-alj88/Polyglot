# Meeting Minutes: Token List & String Literal Architecture

**Date**: 2025-11-18
**Time**: ~06:00-06:45 (inferred from file modification timestamp)
**Type**: Technical Design Session
**Participants**: hhj, Claude Code AI Agent
**Facilitator**: Not formally facilitated (pre-Mai)
**Scribe**: Documented in architecture.md

---

## Meeting Purpose

Finalize Polyglot token list and resolve string literal architectural design for Epic 1 (Lexer & Parser implementation).

---

## Agenda

1. Complete token type enumeration for v0.0.2 specification
2. Decide on lexer generator technology (Logos vs. manual)
3. Design compound token types (StringLiteral, RangeCheck)
4. Document architectural implications and decisions

---

## Discussion Summary

### Topic 1: Lexer Generator Selection

**Context**: Story 1.2 requires tokenizing 45+ distinct token types with <100ms performance requirement for 1000-line files.

**Discussion Points**:
- Evaluated manual lexer vs. Logos crate vs. Nom vs. LALRPOP
- Logos chosen for declarative token definitions and compile-time DFA generation
- Aligns with Polyglot's "Don't Reinvent the Wheel" philosophy
- Battle-tested in production parsers (tree-sitter, rustpython)

**Technical Considerations**:
- 45+ token types would require 500-800 LOC for manual lexer
- Logos reduces to ~200 LOC with regex annotations
- Compile time increase: 2-5 seconds (acceptable tradeoff)
- Source location tracking requires manual implementation on top of Logos byte positions

### Topic 2: Complete Token Enumeration

**Final Token Count**: ~60 base token types + 2 compound types

**Categories**:
- **Block Markers**: 27 single-char tokens in brackets (e.g., `[t]`, `[Q]`, `[|]`)
- **Operators**: ~24 total
  - Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
  - Negation: `!`, `~`
  - Range: `..`, `..=`
  - Pattern: `?`, `*`, `+`
  - Pipeline: `|`, `||`, `|>`, `<|`
- **Type Notation**: `:` (colon), `\` (backslash in type context)
- **Structural**: Comments, identifiers, literals, whitespace
- **Compound Types**: StringLiteral, RangeCheck (see below)

### Topic 3: String Literal Design (CRITICAL DECISION)

**Problem Statement**: How should Polyglot handle string literals given its pipeline-first philosophy?

**Decision**: Implement StringLiteral as compound token with optional pipeline prefix

**Structure**:
```rust
pub struct StringLiteral {
    /// Optional pipeline for string processing
    /// If None, defaults to |String.Formatted
    /// Format: String.{runtime_lang}.{type}.Format.{format_id}
    pipeline: Option<String>,

    /// UTF-8 string content (Unicode supported)
    content: String,

    location: Location,
}
```

**Rationale**:
- Literals are syntax sugar for pipeline invocations (consistent with Polyglot philosophy)
- Bare strings `"text"` default to `|String.Formatted` with `{.var:fmt}` substitution
- Pipeline-prefixed strings invoke specific formatters: `String.Python.int.Format.Hex"42"`
- Compiler validates pipeline existence and `pg\string` type yield (Epic 2 scope)

**Examples**:
- `"Hello {.name}"` → `|String.Formatted` pipeline with variable substitution
- `String.Python.int.Format.Hex"42"` → Custom pipeline invocation yielding "0x2a"

**Impact**:
- Epic 1: Lexer recognizes both bare and prefixed string literals
- Epic 2: Parser/compiler validates pipeline references
- Epic 3+: Runtime executes formatting pipelines

### Topic 4: RangeCheck Compound Token

**Problem Statement**: Range comparisons like `.var ?[start, end)` are a single logical construct

**Decision**: Implement RangeCheck as compound token

**Structure**:
```rust
pub enum RangeValue {
    Variable(String),      // .var_name placeholder
    Value(LiteralValue),   // Evaluatable literal
}

pub struct RangeCheck {
    left: RangeValue,           // Variable being checked
    start: RangeValue,          // Range start
    end: RangeValue,            // Range end
    start_inclusive: bool,      // [ = true, ( = false
    end_inclusive: bool,        // ] = true, ) = false
    location: Location,
}
```

**Rationale**:
- Single logical operation, not separate tokens
- Supports inclusive/exclusive boundaries
- Enables dynamic range checking with variables or literals

---

## Decisions Made

### Decision 1: Adopt Logos Lexer Generator ✅ APPROVED

**Decision ID**: ADR-013
**Status**: Approved
**Decided By**: hhj (with AI agent technical analysis)
**Rationale**: Performance, maintainability, and alignment with "Don't Reinvent the Wheel" philosophy
**Documented In**: `docs/technical/architecture.md` lines 1400-1487

**Consequences**:
- ✅ Add `logos = "0.14"` dependency
- ✅ Story 1.3 implementation reduced to ~200 LOC
- ⚠️ Manual source location tracking required
- ⚠️ Compile time +2-5 seconds acceptable

**Affected FRs**: FR1-2, FR6-7 (Lexer & Compilation)
**Affected Stories**: Story 1.2 (Token Definitions), Story 1.3 (Lexer Implementation)

---

### Decision 2: StringLiteral Compound Token Design ✅ APPROVED

**Decision ID**: DESIGN-2025-11-17-001
**Status**: Approved
**Decided By**: hhj
**Rationale**: Aligns with pipeline-first philosophy, enables syntax sugar for common case, supports advanced formatting

**Consequences**:
- ✅ Lexer recognizes optional pipeline prefix before string delimiter
- ✅ Default `|String.Formatted` pipeline for bare strings
- ⚠️ Compiler validation required (Epic 2): pipeline exists and yields `pg\string`
- ⚠️ Runtime must support variable substitution in formatted strings

**Affected FRs**: FR1-9 (Compilation), FR6 (Type System - string type)
**Affected Stories**: Story 1.2, Story 1.3, Epic 2 (Parser/Compiler)

---

### Decision 3: RangeCheck Compound Token Design ✅ APPROVED

**Decision ID**: DESIGN-2025-11-17-002
**Status**: Approved
**Decided By**: hhj
**Rationale**: Treat range comparison as atomic operation, cleaner AST representation

**Consequences**:
- ✅ Single token for entire range expression
- ✅ Supports inclusive/exclusive boundaries
- ✅ Enables dynamic ranges with variables

**Affected FRs**: FR6 (Type System - range operations)
**Affected Stories**: Story 1.2, Story 1.3

---

## Action Items

| # | Action | Owner | Deadline | Status | Dependencies |
|---|--------|-------|----------|--------|--------------|
| 1 | Add logos dependency to Cargo workspace | Dev Agent (Amelia) | Story 1.2 | ✅ DONE | - |
| 2 | Define TokenType enum with ~60 variants | Dev Agent (Amelia) | Story 1.2 | 🔄 IN PROGRESS | Action #1 |
| 3 | Implement StringLiteral struct | Dev Agent (Amelia) | Story 1.2 | 🔄 IN PROGRESS | - |
| 4 | Implement RangeCheck struct | Dev Agent (Amelia) | Story 1.2 | 🔄 IN PROGRESS | - |
| 5 | Document token regex patterns for Logos | Dev Agent (Amelia) | Story 1.3 | ⏳ PENDING | Action #2 |
| 6 | Implement source location tracking | Dev Agent (Amelia) | Story 1.3 | ⏳ PENDING | Action #5 |
| 7 | Write unit tests (>80% coverage) | Master Test Architect (Murat) | Story 1.3 | ⏳ PENDING | Action #6 |
| 8 | Update architecture.md with final decisions | Tech Writer (Paige) | Completed | ✅ DONE | - |

---

## Technical Decisions Deferred

None - all critical decisions for Epic 1 lexer work were resolved.

---

## Misalignments Identified

None during this session.

---

## Follow-up Required

1. **Epic 2 Scope**: Parser must validate string literal pipeline references
2. **Epic 2 Scope**: Compiler must verify pipelines yield `pg\string` type
3. **Epic 3+ Scope**: Runtime must implement `|String.Formatted` pipeline with variable substitution

---

## Next Steps

1. Complete Story 1.2 (Token Definitions) implementation
2. Move to Story 1.3 (Lexer Implementation with Logos)
3. Run lexer performance benchmarks to validate <100ms requirement

---

## Attachments

- **Architecture Document**: `docs/technical/architecture.md` (ADR-013)
- **Epic File**: `docs/project/epics.md` (Epic 1)
- **Story Files**:
  - `docs/project/stories/1-2-lexer-token-definitions.md`
  - Story 1.3 (not yet created)

---

**Minutes Recorded By**: Mai (Secretary)
**Date Documented**: 2025-11-18
**Source**: Reconstructed from `docs/technical/architecture.md` ADR-013 modifications
