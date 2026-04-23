---
audience: developer
rule: "4.12"
code: PGE04013
name: Nested Array Type
severity: error
---

# Rule 4.12 — Nested Array Type
`PGE04013`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->

**Statement:** Array type annotations must not nest — `array.array.X` is a compile error. The `#array` type is one-dimensional by default. For multidimensional data, use the `:ND` dimension specifier (e.g., `#array:float:2D`).
**Rationale:** Nested arrays create ambiguous semantics — element access, iteration, and collection behavior are undefined for arrays-of-arrays. The `:ND` specifier provides explicit multidimensional arrays with well-defined indexing and compiler-enforced access depth.
**Detection:** The compiler checks type annotations. If `array` appears as the element type of another `array`, PGE04013 fires.

**See also:** PGE04001 (type mismatch), PGE04011 (negative array index literal), PGE04017 (array dimension access mismatch)

**VALID:**
```polyglot
[ ] ✓ 1D array (default — :1D implied)
(-) >items#array:string
```

```polyglot
[ ] ✓ array of structs
(-) <records#array:UserRecord
```

```polyglot
[ ] ✓ 2D matrix using :ND dimension specifier
(-) >matrix#array:float:2D
```

```polyglot
[ ] ✓ 3D cube
(-) <cube#array:int:3D
```

**INVALID:**
```polyglot
[ ] ✗ PGE04013 — nested array type
(-) >matrix#array:array.string                [ ] ✗ PGE04013 — use #array:string:2D instead
```

```polyglot
[ ] ✗ PGE04013 — nested array type
(-) <data#array:array.int                     [ ] ✗ PGE04013 — use #array:int:2D instead
```

```polyglot
[ ] ✗ PGE04013 — deeply nested array type
(-) <cube#array:array.array.float             [ ] ✗ PGE04013 — use #array:float:3D instead
```

**Diagnostic:** `"Nested array type on {parameter} at line {N} — array is one-dimensional by default, use :ND for multidimensional data (e.g., #array:float:2D)"`

## See Also

- [[syntax/types/arrays|Array Types]] — nested array ban and `:ND` alternative
- [[technical/edge-cases/04-type-system|EC-4.18: Multidimensional array — :ND dimension specifier]] — references PGE04013
