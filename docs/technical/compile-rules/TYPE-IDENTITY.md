---
type: definition
name: Type Identity
updated: 2026-03-19
status: draft
---

# Type Identity in Polyglot

All Polyglot data is serialized strings. **"Same type" means "same schema"** — same structure and field types, not same name.

## Rules

1. **Structural matching** — two `{#}` types with identical field structures ARE the same type regardless of name.
2. **No implicit coercion** — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc. Explicit conversion (via a pipeline call) is required.
3. **Subfield matching** — comparison happens at the relevant section/subfield level. Subfields of differently-named parent types can match if their schemas match.
4. **Universal scope** — this definition applies wherever the compiler checks type compatibility: IO wiring, race collectors (PGE-306), schema completeness (PGE-402), auto-wire (PGE-801, PGE-802), conditional matching, and any future type comparison.

## Examples

**Same type — different names, identical schema:**
```polyglot
{#} #UserProfile
   [.] .name;string
   [.] .email;string

{#} #ContactInfo
   [.] .name;string
   [.] .email;string

[ ] ✓ same schema — name;string + email;string
```

**Different type — different field structure:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

{#} #OrderRecord
   [.] .orderId;string
   [.] .total;float

[ ] ✗ schemas differ — name+age ≠ orderId+total
```

**Different type — no implicit coercion:**
```polyglot
[ ] ✗ int ≠ float — no auto-promotion
[ ] ✗ string ≠ path — no coercion
[ ] ✗ array.int ≠ array.string — element types differ
```

## Referenced by

- [PGE-401 — Type Mismatch](PGE/PGE-401-type-mismatch.md)
- [PGE-306 — Race Collector Type Homogeneity](PGE/PGE-306-race-collector-type-homogeneity.md)
- [PGE-402 — Schema Mismatch](PGE/PGE-402-schema-mismatch.md)
- [PGE-801 — Auto-Wire Type Mismatch](PGE/PGE-801-auto-wire-type-mismatch.md)
- [PGE-802 — Auto-Wire Ambiguous Type](PGE/PGE-802-auto-wire-ambiguous-type.md)
