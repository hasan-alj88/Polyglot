---
rule: "4.12"
code: PGE-412
name: Nested Array Type
severity: error
---

### Rule 4.12 — Nested Array Type
`PGE-412`

**Statement:** Array type annotations must not nest — `array.array.X` is a compile error. The `;array` type is strictly one-dimensional (a one-level enumerated struct). For multidimensional data, use `;tensor`.
**Rationale:** Nested arrays create ambiguous semantics — element access, iteration, and collection behavior are undefined for arrays-of-arrays. Polyglot provides `;tensor` as the explicit multidimensional data type with well-defined indexing.
**Detection:** The compiler checks type annotations. If `array` appears as the element type of another `array`, PGE-412 fires.

**See also:** PGE-401 (type mismatch), PGE-411 (negative array index literal)

**VALID:**
```polyglot
[ ] ✓ single-level array types
[=] >items;array.string
```

```polyglot
[ ] ✓ array of structs
[=] <records;array.#UserRecord
```

```polyglot
[ ] ✓ tensor for multidimensional data
[=] >matrix;tensor.float
```

**INVALID:**
```polyglot
[ ] ✗ PGE-412 — nested array type
[=] >matrix;array.array.string                [ ] ✗ PGE-412 — array is 1D only
```

```polyglot
[ ] ✗ PGE-412 — nested array type
[=] <data;array.array.int                     [ ] ✗ PGE-412 — use ;tensor instead
```

```polyglot
[ ] ✗ PGE-412 — deeply nested array type
[=] <cube;array.array.array.float             [ ] ✗ PGE-412 — array nesting not allowed
```

**Diagnostic:** `"Nested array type on {parameter} at line {N} — array is one-dimensional only, use ;tensor for multidimensional data"`

**Open point:** The `;tensor` type is backed by `#TensorItemType`:
```polyglot
{#} #TensorItemType
   [.] .index;RawString                       [ ] positive ints separated by comma
   [.] .value;Type
```
Full `;tensor` specification TBD.
