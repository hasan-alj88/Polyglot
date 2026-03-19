---
issue: "013"
title: PGE-401 canonical "same type" definition should be shared
related: PGE-401 (Rule 4.1), PGE-306 (Rule 3.6), PGE-402 (Rule 4.2), PGE-801 (Rule 8.1)
priority: cleanup
status: resolved
resolved: 2026-03-19
created: 2026-03-19
---

# 013 — PGE-401 canonical "same type" definition should be shared

## Resolution

**Extracted to shared definition file.**

1. Created `compile-rules/TYPE-IDENTITY.md` with canonical "same type" = "same schema" definition
2. **PGE-401** — replaced inline definition with reference to TYPE-IDENTITY.md
3. **PGE-306** — updated cross-references to TYPE-IDENTITY.md
4. **PGE-402** — updated cross-reference to TYPE-IDENTITY.md
5. **PGE-801** — updated cross-reference to TYPE-IDENTITY.md
6. **PGE-802** — added explicit reference to TYPE-IDENTITY.md (was implicit)

## Problem

PGE-401 (Type Mismatch) contains the canonical definition of "same type" in Polyglot:

> All Polyglot data is serialized strings. **"Same type" means "same schema"** — same structure and field types, not same name. Two `{#}` types with identical field structures ARE the same type regardless of name.

This definition is referenced by at least four other rules:
- **PGE-306** (Race Collector Type Homogeneity) — "must match the target variable's schema (per PGE-401)"
- **PGE-402** (Schema Mismatch) — "field type correctness and schema matching are checked by PGE-401"
- **PGE-801** (Auto-Wire Type Mismatch) — "requires a 1-to-1 type match ... per PGE-401"
- **PGE-802** (Auto-Wire Ambiguous Type) — implicitly depends on PGE-401's type identity definition

The definition is buried in PGE-401's statement paragraph. A developer working on PGE-306 or PGE-801 must navigate to PGE-401 to understand what "same type" means. The cross-references are fragile — if PGE-401 is restructured, all dependent rules may silently lose their type semantics anchor.

## Affected Rules

- `compile-rules/PGE/PGE-401-type-mismatch.md` — source of definition
- `compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md` — references PGE-401
- `compile-rules/PGE/PGE-402-schema-mismatch.md` — references PGE-401
- `compile-rules/PGE/PGE-801-auto-wire-type-mismatch.md` — references PGE-401
- `compile-rules/PGE/PGE-802-auto-wire-ambiguous-type.md` — implicitly depends

## Proposed Resolution

**Option A — Shared preamble file (recommended):**

Create `compile-rules/TYPE-IDENTITY.md` as a standalone definition:
- "Same type" = same schema (structural, not nominal)
- Schema comparison rules (field names, field types, nesting)
- Examples of matching and non-matching schemas

Each rule that depends on this definition references `TYPE-IDENTITY.md` instead of PGE-401. PGE-401 keeps its error code but its statement references the shared definition rather than inlining it.

**Option B — Keep in PGE-401, improve cross-references:**

Mark the canonical definition with a stable anchor heading (e.g., `### Canonical Type Identity`) that other rules can reference by section. Less ideal because it couples the definition's location to one specific error rule.

## See also

- [PGE-401 — Type Mismatch](../compile-rules/PGE/PGE-401-type-mismatch.md)
- [PGE-306 — Race Collector Type Homogeneity](../compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md)
- [PGE-801 — Auto-Wire Type Mismatch](../compile-rules/PGE/PGE-801-auto-wire-type-mismatch.md)
