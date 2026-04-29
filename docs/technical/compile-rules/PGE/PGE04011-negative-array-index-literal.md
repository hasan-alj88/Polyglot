---
audience: developer
rule: "4.11"
code: PGE04011
name: Negative Array Index Literal
severity: error
---

# Rule 4.11 — Negative Array Index Literal
`PGE04011`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Array access with a negative literal index is a compile error. Aljam3 arrays are zero-indexed; negative indices are not valid. Reverse access must use explicit pglib pipelines (e.g., `-Array.Last`, `-Array.FromEnd`).
**Rationale:** Implicit negative-means-reverse indexing (Python-style) is a source of subtle bugs — off-by-one errors, empty array edge cases, and unclear intent. Aljam3 is type-safe first; reverse access should be explicit and self-documenting.
**Detection:** The compiler checks array access expressions with literal integer indices. If the index is negative, PGE04011 fires.

**See also:** PGE04001 (type mismatch), PGE04003 (leaf-only assignment)

**VALID:**
```aljam3
[ ] ✓ zero and positive literal indices
[-] -Log
   (-) <msg << $items.0
```

```aljam3
[ ] ✓ positive literal index
[-] -Process
   (-) <first << $items.0
   (-) <third << $items.2
```

```aljam3
[ ] ✓ reverse access via explicit pglib
[-] -Array.Last
   (-) <array << $items
   (-) >item >> $last
```

**INVALID:**
```aljam3
[ ] ✗ PGE04011 — negative array index literal
[-] -Log
   (-) <msg << $items.-1                      [ ] ✗ PGE04011 — negative index -1
```

```aljam3
[ ] ✗ PGE04011 — negative array index literal
[-] -Process
   (-) <data << $matrix.-3                    [ ] ✗ PGE04011 — negative index -3
```

**Diagnostic:** `"Negative array index {index} on {variable} at line {N} — Aljam3 arrays are zero-indexed, use -Array.Last or -Array.FromEnd for reverse access"`
