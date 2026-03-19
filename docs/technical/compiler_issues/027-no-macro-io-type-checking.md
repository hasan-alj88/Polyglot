---
issue: "027"
title: No type checking for [W] wrapper wiring to {M} macro
related: PGE-104 (Rule 1.4), PGE-401 (Rule 4.1)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 027 — No type checking for [W] wrapper wiring to {M} macro

## Problem

When a `[W]` wrapper references a `{M}` macro, IO is wired via `[=]` lines:

```polyglot
[W] =MyWrapper
   [=] $connStr << $connStr
   [=] $timeout << $timeout
```

No compiler rule validates that:

### 1. Wrapper references a valid macro
`[W]` must reference a `{M}` macro, not a `{=}` pipeline. No rule enforces this.

### 2. IO type matching at call site
The `[=]` wiring types must match the macro's `[{]` input and `[}]` output types. No rule checks schema compatibility between wrapper wiring and macro IO.

### 3. Required inputs provided
If the macro declares required `[{]` inputs, the wrapper `[=]` must provide them all. No completeness check exists.

## Affected Rules

- `compile-rules/PGE/PGE-104-macro-structural-constraints.md` (macro structure, not call-site validation)
- `compile-rules/PGE/PGE-401-type-mismatch.md` (type checking, but doesn't cover macro wiring)

## Proposed Resolution

**Option A — Extend PGE-104:**

Add call-site validation to PGE-104 (Macro Structural Constraints). However, PGE-104 is about the macro definition, not usage.

**Option B — New rules (recommended):**

| Code | Rule | Name |
|------|------|------|
| PGE-108 | 1.8 | Wrapper Must Reference Macro |
| PGE-109 | 1.9 | Wrapper IO Mismatch |

Separate structural validity (must be a macro) from type checking (IO must match).

## See also

- [PGE-104 — Macro Structural Constraints](../compile-rules/PGE/PGE-104-macro-structural-constraints.md)
