---
issue: "029"
title: PGE-401 vs PGE-402 boundary unclear
related: PGE-401 (Rule 4.1), PGE-402 (Rule 4.2)
priority: cleanup
status: resolved
created: 2026-03-19
---

# 029 — PGE-401 vs PGE-402 boundary unclear

## Problem

PGE-401 (Type Mismatch) and PGE-402 (Schema Mismatch) have overlapping scope that could confuse developers:

- **PGE-401:** "pushing a value of type A into a parameter expecting type B" — structural type compatibility
- **PGE-402:** "a data instance is missing required fields" — field completeness

The boundary question: is "missing a required field" a type mismatch (PGE-401, because the schemas differ structurally) or a schema mismatch (PGE-402, because the instance is incomplete)?

Current implicit distinction:
- PGE-401 = wrong structure (different field names, different field types)
- PGE-402 = right structure but incomplete (missing required fields)

This distinction works but isn't explicitly stated in either rule. A developer encountering a "field missing" error might look at PGE-401 first.

## Affected Rules

- `compile-rules/PGE/PGE-401-type-mismatch.md`
- `compile-rules/PGE/PGE-402-schema-mismatch.md`

## Proposed Resolution

Add a "Boundary with PGE-402/PGE-401" note to each rule:

**In PGE-401:** "PGE-401 fires when the source and target have **structurally different schemas** (different fields or field types). If the schemas match structurally but the source is **missing values** for required fields, see PGE-402."

**In PGE-402:** "PGE-402 fires when a data instance has the **correct schema** but is **incomplete** (missing required field values). If the schemas themselves are structurally different, see PGE-401."

## See also

- [PGE-401 — Type Mismatch](../compile-rules/PGE/PGE-401-type-mismatch.md)
- [PGE-402 — Schema Mismatch](../compile-rules/PGE/PGE-402-schema-mismatch.md)
- [TYPE-IDENTITY.md](../compile-rules/TYPE-IDENTITY.md)
