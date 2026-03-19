---
issue: "015"
title: PGE-701 chain error addressing needs explicit coverage
related: PGE-701 (Rule 7.1), PGE-804 (Rule 8.4)
priority: enhancement
status: resolved
resolved: 2026-03-19
created: 2026-03-19
---

# 015 — PGE-701 chain error addressing needs explicit coverage

## Resolution

**PGE-702 created with new `.N!Error` syntax and full chain error semantics.**

1. **New syntax**: Chain error addressing changed from `!N.Error` to `.N!Error` — step prefix (`.N`) mirrors IO convention, `!` separates step ref from error path, eliminating dotted-path ambiguity
2. **PGE-702** created — Chain Error Scoping: addressing syntax, scope rules (handler sees only its step's IO), PGE-804 for ambiguous name refs
3. **PGE-701** updated — chain example uses new `.N!Error` syntax, cross-references PGE-702
4. **PGE-804** updated — chain error INVALID example uses new syntax
5. **COMPILE-RULES.md** updated with PGE-702

## Problem

PGE-701 (Error Block Scoping) covers basic `[!]` scoping under `[r]` calls and briefly shows chain errors in one VALID example. However, chain error handling introduces several unique scenarios that are not explicitly covered:

### 1. Error addressing syntax

Chain errors use prefixed addressing — `!0.ErrorName` (numeric) or `!Read.ErrorName` (name-based). The rule shows this but doesn't define the addressing rules:
- When is numeric required vs. name-based allowed?
- What happens when a step name is ambiguous (parallels PGE-804)?

### 2. Ambiguous error references

PGE-804 handles ambiguous step references in `[=]` IO lines. The same ambiguity applies to `[!]` error references — if two steps share a leaf name, `!StepName.Error` is ambiguous. PGE-804's INVALID example #2 shows this case, but PGE-701 doesn't address it.

### 3. Error handler variable access

Inside a chain `[!]` handler, which variables are accessible? Can the handler read outputs from earlier steps in the chain? Can it write to outputs of later steps? The scoping rules for variable access within chain error handlers are undefined.

### 4. Multiple error sources

When a chain has multiple steps that can produce the same error type, the `[!]` handler needs to identify which step failed. The addressing syntax handles this, but the semantics (which handler runs, what state is visible) are not specified.

## Affected Rules

- `compile-rules/PGE/PGE-701-error-block-scoping.md`
- `compile-rules/PGE/PGE-804-ambiguous-step-reference.md` (related)

## Proposed Resolution

**Option A — Expand PGE-701 with chain sub-sections:**

Add sections to PGE-701:
- "Chain Error Addressing" — syntax rules for `!N.ErrorName` and `!StepName.ErrorName`
- "Ambiguous Chain Error References" — cross-reference PGE-804, same disambiguation rules apply
- "Variable Scope in Chain Error Handlers" — what's readable/writable inside `[!]` for chain steps

**Option B — Create PGE-702 (recommended):**

Create a dedicated `PGE-702 — Chain Error Scoping` rule covering:
- Addressing syntax (numeric and name-based)
- Ambiguous references (defer to PGE-804 rules)
- Variable scope within chain `[!]` handlers
- Multiple error sources from different chain steps

Option B is preferred because chain error handling is complex enough to warrant its own error code, and mixing it into PGE-701 would make that rule unwieldy.

## See also

- [PGE-701 — Error Block Scoping](../compile-rules/PGE/PGE-701-error-block-scoping.md)
- [PGE-804 — Ambiguous Step Reference](../compile-rules/PGE/PGE-804-ambiguous-step-reference.md)
