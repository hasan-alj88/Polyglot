---
audience: developer
rule: "4.25"
code: PGE04025
name: Untyped Array
severity: error
---

### Rule 4.25 — Untyped Array
`PGE04025`

<!-- @u:syntax/io -->

**Statement:** `#array` type annotations must specify an element type. Untyped arrays (`#array` without element type) are a compile error — the compiler cannot infer the element type, and all elements must share the same schema.
**Rationale:** Polyglot is type-safe first. Arrays hold homogeneous elements — every element must conform to the declared element type's schema. Without a declared type, the compiler cannot enforce this constraint, leading to potential runtime type errors.
**Detection:** The compiler checks that every `#array` type annotation includes an element type via colon notation (e.g., `#array:int`, `#array:UserRecord`).

**VALID:**
```polyglot
[ ] ✓ typed array — element type specified
(-) <items#array:int

[ ] ✓ array of user type
(-) <users#array:UserRecord
```

**INVALID:**
```polyglot
[ ] ✗ PGE04025 — no element type
(-) <items#array
```

**Diagnostic:** "Array `$name` requires an element type — use `#array:type` notation"
