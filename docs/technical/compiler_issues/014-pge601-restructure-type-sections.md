---
issue: "014"
title: PGE-601 needs per-type sub-rules (PGE-606/607/608)
related: PGE-601 (Rule 6.1), PGE-602 (Rule 6.2), PGE-603 (Rule 6.3), PGE-604 (Rule 6.4)
priority: enhancement
status: resolved
resolved: 2026-03-19
created: 2026-03-19
---

# 014 — PGE-601 needs per-type sub-rules (PGE-606/607/608)

## Resolution

**PGE-601 restructured into dispatch table + per-type sub-rules.**

Numbering adjusted — PGE-605 was already taken by Compound Condition Overlap (Issue 006).

1. **PGE-606** created — String Exhaustiveness (`*?` always required)
2. **PGE-607** created — Flexible Field Exhaustiveness (`*?` always required)
3. **PGE-608** created — Compound Condition Exhaustiveness (`*?` required, pending Issue 006)
4. **PGE-601** rewritten as dispatch table — type-specific sections replaced with cross-references
5. Fixed fields (`.`) remain under PGE-602 (enum rules — closed set)
6. COMPILE-RULES.md updated with PGE-606/607/608

## Problem

PGE-601 (Conditional Must Be Exhaustive) currently contains seven type-specific sections, each with its own detection logic, examples, and `*?` requirements:

| Type | Has own rule? | Location |
|------|--------------|----------|
| Enum (`{#}`) | Yes — PGE-602 | Own file |
| `#Boolean` | Yes — PGE-602 | Own file |
| `int`/`float` ranges | Yes — PGE-603/604 | Own files |
| `string` | **No** | Inline in PGE-601 |
| Flexible fields (`:`) | **No** | Inline in PGE-601 |
| Fixed fields (`.`) | **No** | Inline in PGE-601 (defers to PGE-602) |
| Compound conditions | **No** | Inline in PGE-601 (defers to issue 006) |

Three type categories have no standalone rules — their exhaustiveness logic exists only inside PGE-601. This makes PGE-601 a ~200-line megafile that's hard to navigate. It also means a developer working on string exhaustiveness has no focused rule to reference.

## Affected Rules

- `compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md`

## Proposed Resolution

Create focused rules for the uncovered types:

| New Code | Name | Scope |
|----------|------|-------|
| PGE-605 | String Exhaustiveness | `*?` always required for string conditionals |
| PGE-606 | Flexible Field Exhaustiveness | `*?` always required for `:` field conditionals |
| PGE-607 | Compound Condition Exhaustiveness | `*?` always required (pending issue 006 for static analysis) |

**Fixed fields (`.`)** do not need a separate rule — they follow enum rules (PGE-602) since fixed fields form a closed set identical to enum variants.

After creating these rules:
1. PGE-601 becomes a dispatch table with the exhaustiveness-by-type matrix
2. Each type-specific section in PGE-601 is replaced with a cross-reference
3. This also resolves issue 007 (duplicate content) since PGE-601 no longer contains type-specific examples

### File creation:

- `compile-rules/PGE/PGE-605-string-exhaustiveness.md`
- `compile-rules/PGE/PGE-606-flexible-field-exhaustiveness.md`
- `compile-rules/PGE/PGE-607-compound-condition-exhaustiveness.md`

Update `COMPILE-RULES.md` error code table with PGE-605, PGE-606, PGE-607.

## See also

- [007 — PGE-601 contains duplicated type-specific content](007-pge601-duplicate-content.md) — resolved by this restructuring
- [006 — Compound Condition Exhaustiveness](006-compound-condition-exhaustiveness.md) — PGE-607 references this
- [PGE-601 — Conditional Must Be Exhaustive](../compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md)
