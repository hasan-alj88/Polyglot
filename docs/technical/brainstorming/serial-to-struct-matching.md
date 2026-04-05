---
audience: developer
type: brainstorming
updated: 2026-04-05
---

# Brainstorm: Serial → Struct Matching

**Status:** Decided (2026-03-20)

## Problem

When a `serial` value is pushed into a `struct`-typed target, the compiler must verify that the serial's fields match the struct's schema (field names and types). However, serial fields are dynamic — added on the fly at runtime. The compiler may not always know the serial's shape at the point of push.

## Decisions

### 1. Compile-time vs runtime check — best-effort static analysis

- **Statically known serial** — all fields assigned unconditionally before the push → compiler verifies schema match at compile time. Mismatch → **PGE04002** (schema mismatch).
- **Partially known** — some fields added conditionally or in loops → compiler verifies known fields match but cannot guarantee completeness. Emits warning, defers to runtime.
- **Fully dynamic** — serial built entirely from external input or opaque pipeline output → no static verification possible, deferred to runtime.

### 2. Mandatory error handling — PGE04009 (Murphy's Law)

**If it can go wrong, it will go wrong.** When the compiler cannot prove serial→struct compatibility at compile time, the user MUST handle the potential failure with `[!]` error handling and `*Continue >FallBack`. If unhandled → **PGE04009** ("Unhandled serial→struct conversion — `[!]` with `*Continue >FallBack` required").

No warnings. Either the compiler can prove it's safe (no handling needed), prove it's wrong (PGE04002), or the user must handle the uncertainty (PGE04009 if they don't).

```polyglot
[ ] ✓ Handled — *Continue provides fallback
[r] $record#UserRecord << $someSerial
   [!] !SchemaMismatch
      [*] *Continue >FallBack << $defaultRecord
```

```polyglot
[ ] ✗ PGE04009 — serial→struct push without error handling
[r] $record#UserRecord << $someSerial
```

### 3. Runtime behavior

A failed serial→struct match at runtime fires the `[!]` error block. Since PGE04009 guarantees `[!]` + `*Continue >FallBack` is always present when the match is uncertain, the pipeline always has a recovery path. No unhandled runtime crashes.

### 4. Partial matching — superset allowed, subset not

- Serial has **more** fields than struct requires → **allowed**. Extra `:` fields are ignored during conversion. The struct's schema is fully satisfied.
- Serial has **fewer** fields than struct requires → **error**. Missing fields mean the struct schema cannot be fulfilled.

Rationale:
- Structural matching (TYPE-IDENTITY rule 1) checks that the required schema is present, not that shapes are identical
- Serial is inherently open-ended — requiring exact match would defeat its purpose

### Summary

| Aspect | Decision |
|--------|----------|
| Static match (all fields known) | No handling needed |
| Static mismatch (provably wrong) | PGE04002 |
| Cannot prove match | `[!]` + `*Continue >FallBack` required — PGE04009 if absent |
| Runtime failure | Fires `[!]` block, `*Continue` provides fallback |
| Extra fields in serial | Allowed (ignored) |
| Missing fields in serial | Error |

## Related

- [TYPE-IDENTITY.md](../compile-rules/TYPE-IDENTITY.md) — rules 5 and 6
- PGE04001 — Type Mismatch
- PGE04002 — Schema Mismatch
- PGE04009 — Unhandled serial→struct conversion (new)
- PGE02007 — Continue After Error (`*Continue` rules)
