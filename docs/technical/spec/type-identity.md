---
audience: developer
type: definition
name: Type Identity
updated: 2026-03-24
status: complete
---

# Type Identity in Aljam3

All Aljam3 data is serialized strings. **"Same type" means "same schema"** — same structure and field types, not same name.

## Terminology

`{#}` declarations produce **structs** — the term for any type with a defined schema. All `{#}` types (user-defined and pglib like `#path`, `#Boolean`, `#OS`, `#Queue`, `#DateTime`) are structs.

## Rules

1. **Structural matching** — two structs with identical field structures ARE the same type regardless of name.
2. **No implicit coercion** — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc. Explicit conversion (via a pipeline call) is required.
3. **Subfield matching** — comparison happens at the relevant section/subfield level. Subfields of differently-named parent types can match if their schemas match.
4. **Universal scope** — this definition applies wherever the compiler checks type compatibility: IO wiring, race collectors (PGE03006), schema completeness (PGE04002), auto-wire (PGE08001, PGE08002), conditional matching, and any future type comparison.
5. **Struct → serial** — always allowed. A struct's fixed (`.`) fields are converted to flexible (`:`) fields in the serial. The struct is always a valid subset of serial's openness.
6. **Serial → struct** — allowed only if the serial's current fields satisfy the struct's schema (field names and types). Extra fields in the serial are ignored; missing fields are an error. The compiler performs best-effort static analysis: if the match is provably correct, no handling needed; PGE04002 on provable mismatch; PGE04009 if the compiler cannot prove the match and the user does not provide `[!]` error handling or `!<` / `!>` fallback operators. See [serial-to-struct matching](../brainstorming/serial-to-struct-matching.md) for full decision rationale.

## Examples

**Same type — different names, identical schema:**
```aljam3
{#} #UserProfile
   [.] .name#string
   [.] .email#string

{#} #ContactInfo
   [.] .name#string
   [.] .email#string

[ ] ✓ same schema — name#string + email#string
```

**Different type — different field structure:**
```aljam3
{#} #UserRecord
   [.] .name#string
   [.] .age#int

{#} #OrderRecord
   [.] .orderId#string
   [.] .total#float

[ ] ✗ schemas differ — name+age ≠ orderId+total
```

**Different type — no implicit coercion:**
```aljam3
[ ] ✗ int ≠ float — no auto-promotion
[ ] ✗ string ≠ path — no coercion
[ ] ✗ array.int ≠ array.string — element types differ
```

## Referenced by

- [PGE04001 — Type Mismatch](../compile-rules/PGE/PGE04001-type-mismatch.md)
- [PGE03006 — Race Collector Type Homogeneity](../compile-rules/PGE/PGE03006-race-collector-type-homogeneity.md)
- [PGE04002 — Schema Mismatch](../compile-rules/PGE/PGE04002-schema-mismatch.md)
- [PGE08001 — Auto-Wire Type Mismatch](../compile-rules/PGE/PGE08001-auto-wire-type-mismatch.md)
- [PGE08002 — Auto-Wire Ambiguous Type](../compile-rules/PGE/PGE08002-auto-wire-ambiguous-type.md)
- [PGE04009 — Unhandled Serial→Struct Conversion](../compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md)

## Related

- [[metadata-tree/INDEX|spec/metadata-tree]] — the unified tree where all types live
- [[data-is-trees|user/concepts/data-is-trees]] — user-facing overview of structural typing
