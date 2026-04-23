---
audience: developer
rule: "4.17"
code: PGE04017
name: Array Dimension Access Mismatch
severity: error
---

# Rule 4.17 — Array Dimension Access Mismatch
`PGE04017`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** When accessing elements of a multidimensional array (`#array.<type>:<N>D`), the number of dot-separated indices must exactly match the declared dimension count. Too many or too few indices is a compile error. The dimension specifier is mandatory for multidimensional arrays — `#array.<type>` without `:<N>D` defaults to `:1D`. The dimension value must be a positive integer (`:0D` is invalid).
**Rationale:** Multidimensional arrays have a fixed number of dimensions declared at compile time. The compiler enforces that every element access uses exactly the right number of indices, preventing partial access (returning a row instead of an element) or over-indexing (accessing beyond declared dimensions).
**Detection:** The compiler counts the dot-separated integer indices in an element access expression and compares against the declared `:<N>D` dimension. If the counts differ, PGE04017 fires. Also fires if `:0D` is declared.

**See also:** PGE04013 (nested array type), PGE04011 (negative array index literal)

**VALID:**
```polyglot
[ ] ✓ 1D access — single index on default 1D array
(-) <items#array:string
[-] $first << $items.0
```

```polyglot
[ ] ✓ 2D access — two indices on :2D array
(-) <matrix#array:float:2D
[-] $val << $matrix.0.1
```

```polyglot
[ ] ✓ 3D access — three indices on :3D array
(-) <cube#array:int:3D
[-] $val << $cube.2.3.0
```

```polyglot
[ ] ✓ user-defined element type with :2D
(-) <grid#array:UserRecord:2D
[-] $cell << $grid.1.2
```

**INVALID:**
```polyglot
[ ] ✗ PGE04017 — too many indices for :2D array
(-) <matrix#array:float:2D
[-] $val << $matrix.0.1.2                    [ ] ✗ 3 indices on :2D — expected 2
```

```polyglot
[ ] ✗ PGE04017 — too few indices for :3D array
(-) <cube#array:int:3D
[-] $val << $cube.2                           [ ] ✗ 1 index on :3D — expected 3
```

```polyglot
[ ] ✗ PGE04017 — :0D is not a valid dimension
(-) <nothing#array:float:0D                  [ ] ✗ dimension must be positive integer
```

**Diagnostic:** `"Array dimension access mismatch on {variable} at line {N} — {variable} is declared as :{M}D but accessed with {K} indices"`

## See Also

- [[syntax/types/arrays|Array Types]] — dimension access depth enforcement
- [[technical/edge-cases/04-type-system|EC-4.18: Multidimensional array — :ND dimension specifier]] — references PGE04017
- [[technical/edge-cases/24-datatype-defs|EC-24.13: 0D array]] — references PGE04017
