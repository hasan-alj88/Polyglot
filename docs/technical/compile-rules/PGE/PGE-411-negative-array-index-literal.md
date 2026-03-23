---
rule: "4.11"
code: PGE-411
name: Negative Array Index Literal
severity: error
---

### Rule 4.11 — Negative Array Index Literal
`PGE-411`

**Statement:** Array access with a negative literal index is a compile error. Polyglot arrays are zero-indexed; negative indices are not valid. Reverse access must use explicit stdlib pipelines (e.g., `=Array.Last`, `=Array.FromEnd`).
**Rationale:** Implicit negative-means-reverse indexing (Python-style) is a source of subtle bugs — off-by-one errors, empty array edge cases, and unclear intent. Polyglot is type-safe first; reverse access should be explicit and self-documenting.
**Detection:** The compiler checks array access expressions with literal integer indices. If the index is negative, PGE-411 fires.

**See also:** PGE-401 (type mismatch), PGE-403 (leaf-only assignment)

**VALID:**
```polyglot
[ ] ✓ zero and positive literal indices
[r] =Log
   [=] <msg << $items.0
```

```polyglot
[ ] ✓ positive literal index
[r] =Process
   [=] <first << $items.0
   [=] <third << $items.2
```

```polyglot
[ ] ✓ reverse access via explicit stdlib
[r] =Array.Last
   [=] <array << $items
   [=] >item >> $last
```

**INVALID:**
```polyglot
[ ] ✗ PGE-411 — negative array index literal
[r] =Log
   [=] <msg << $items.-1                      [ ] ✗ PGE-411 — negative index -1
```

```polyglot
[ ] ✗ PGE-411 — negative array index literal
[r] =Process
   [=] <data << $matrix.-3                    [ ] ✗ PGE-411 — negative index -3
```

**Diagnostic:** `"Negative array index {index} on {variable} at line {N} — Polyglot arrays are zero-indexed, use =Array.Last or =Array.FromEnd for reverse access"`
