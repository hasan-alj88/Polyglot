---
rule: "4.14"
code: PGE-414
name: Recursive Data Definition
severity: error
---

### Rule 4.14 — Recursive Data Definition
`PGE-414`

**Statement:** A `{#}` data definition must not reference itself — directly or transitively — unless the reference passes through `array` or `serial` indirection. Cycles in the type-reference graph that contain only fixed `[.]` or flexible `[:]` fields are a compile error.
**Rationale:** Polyglot structs must span a finite tree. A direct self-reference like `.child;Node` inside `{#} #Node` creates an infinitely nested structure that cannot be instantiated, serialized, or traversed. Indirection through `array` or `serial` breaks the cycle because these are reference types — the contained data is finite even though the schema is recursive.
**Detection:** The compiler builds a directed graph of `{#}` type references (edges from each type to the types it references via field annotations). It then checks for cycles. Any cycle that does not pass through an `array` or `serial` edge is rejected.

**See also:** PGE-902 (circular package dependency — package-level cycles), PGE-401 (type mismatch — general type validation)

**VALID:**
```polyglot
[ ] ✓ self-reference through array indirection
{#} #TreeNode
   [.] .label;string
   [.] .children;array.TreeNode        [ ] ✓ array breaks the recursion
```

```polyglot
[ ] ✓ mutual reference through serial indirection
{#} #Department
   [.] .name;string
   [.] .employees;serial.Employee

{#} #Employee
   [.] .name;string
   [.] .dept;serial.Department          [ ] ✓ serial breaks the cycle
```

```polyglot
[ ] ✓ no self-reference
{#} #Address
   [.] .street;string
   [.] .city;string
```

**INVALID:**
```polyglot
[ ] ✗ PGE-414 — direct self-reference
{#} #Node
   [.] .value;string
   [.] .child;Node                      [ ] ✗ PGE-414 — infinite recursion
```

```polyglot
[ ] ✗ PGE-414 — mutual recursion without indirection
{#} #A
   [.] .name;string
   [.] .partner;B                       [ ] ✗ PGE-414 — A→B→A cycle

{#} #B
   [.] .label;string
   [.] .partner;A
```

```polyglot
[ ] ✗ PGE-414 — transitive cycle (A→B→C→A)
{#} #A
   [.] .ref;B

{#} #B
   [.] .ref;C

{#} #C
   [.] .ref;A                           [ ] ✗ PGE-414 — cycle detected
```

**Open point:** None.
