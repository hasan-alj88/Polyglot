---
type: definition
name: Type Identity
updated: 2026-03-24
status: complete
---

# Type Identity in Polyglot

All Polyglot data is serialized strings. **"Same type" means "same schema"** — same structure and field types, not same name.

## Terminology

`{#}` declarations produce **structs** — the term for any type with a defined schema. All `{#}` types (user-defined and stdlib like `path`, `#Boolean`, `#OS`, `#DateTime`) are structs.

## Rules

1. **Structural matching** — two structs with identical field structures ARE the same type regardless of name.
2. **No implicit coercion** — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc. Explicit conversion (via a pipeline call) is required.
3. **Subfield matching** — comparison happens at the relevant section/subfield level. Subfields of differently-named parent types can match if their schemas match.
4. **Universal scope** — this definition applies wherever the compiler checks type compatibility: IO wiring, race collectors (PGE-306), schema completeness (PGE-402), auto-wire (PGE-801, PGE-802), conditional matching, and any future type comparison.
5. **Struct → serial** — always allowed. A struct's fixed (`.`) fields are converted to flexible (`:`) fields in the serial. The struct is always a valid subset of serial's openness.
6. **Serial → struct** — allowed only if the serial's current fields satisfy the struct's schema (field names and types). Extra fields in the serial are ignored; missing fields are an error. The compiler performs best-effort static analysis: if the match is provably correct, no handling needed; PGE-402 on provable mismatch; PGE-409 if the compiler cannot prove the match and the user does not provide `[!]` error handling with `*Continue >FallBack`. See [serial-to-struct matching](../brainstorming/serial-to-struct-matching.md) for full decision rationale.

## Examples

**Same type — different names, identical schema:**
```polyglot
{#} #UserProfile
   [.] .name#string
   [.] .email#string

{#} #ContactInfo
   [.] .name#string
   [.] .email#string

[ ] ✓ same schema — name#string + email#string
```

**Different type — different field structure:**
```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

{#} #OrderRecord
   [.] .orderId#string
   [.] .total#float

[ ] ✗ schemas differ — name+age ≠ orderId+total
```

**Different type — no implicit coercion:**
```polyglot
[ ] ✗ int ≠ float — no auto-promotion
[ ] ✗ string ≠ path — no coercion
[ ] ✗ array.int ≠ array.string — element types differ
```

## Referenced by

- [PGE-401 — Type Mismatch](../compile-rules/PGE/PGE-401-type-mismatch.md)
- [PGE-306 — Race Collector Type Homogeneity](../compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md)
- [PGE-402 — Schema Mismatch](../compile-rules/PGE/PGE-402-schema-mismatch.md)
- [PGE-801 — Auto-Wire Type Mismatch](../compile-rules/PGE/PGE-801-auto-wire-type-mismatch.md)
- [PGE-802 — Auto-Wire Ambiguous Type](../compile-rules/PGE/PGE-802-auto-wire-ambiguous-type.md)
- [PGE-409 — Unhandled Serial→Struct Conversion](../compile-rules/PGE/PGE-409-unhandled-serial-struct-conversion.md)

## Related

- [[metadata-tree|spec/metadata-tree]] — the unified tree where all types live
- [[data-is-trees|user/concepts/data-is-trees]] — user-facing overview of structural typing
