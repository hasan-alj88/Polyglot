---
audience: developer
rule: "4.14"
code: PGE05004
name: Recursive Data Definition
severity: error
---

# Rule 4.14 — Recursive Data Definition
`PGE05004`

<!-- @u:syntax/types -->

**Statement:** A `{#}` data definition must not reference itself — directly or transitively — unless the reference passes through `array` or `serial` indirection. Cycles in the type-reference graph that contain only fixed `[.]` or flexible `[:]` fields are a compile error.
**Rationale:** All Aljam3 data lives in a metadata spanning tree with RawString leaves. Data types (`{#}`) are schemas of that tree — they define finite structure. A self-referencing type without indirection yields an infinite tree that cannot span. Indirection through `array` or `serial` breaks the cycle because these are reference types — the schema is recursive but the tree remains finite.
**Detection:** The compiler builds a directed graph of `{#}` type references (edges from each type to the types it references via field annotations). It then checks for cycles. Any cycle that does not pass through an `array` or `serial` edge is rejected.

**See also:** PGE09002 (circular package dependency — package-level cycles), PGE04001 (type mismatch — general type validation)

**VALID:**
```aljam3
[ ] ✓ self-reference through array indirection
{#} #TreeNode
   [.] .label#string
   [.] .children#array:TreeNode        [ ] ✓ array breaks the recursion
```

```aljam3
[ ] ✓ mutual reference through serial indirection
{#} #Department
   [.] .name#string
   [.] .employees#serial.Employee

{#} #Employee
   [.] .name#string
   [.] .dept#serial.Department          [ ] ✓ serial breaks the cycle
```

```aljam3
[ ] ✓ no self-reference
{#} #Address
   [.] .street#string
   [.] .city#string
```

**INVALID:**
```aljam3
[ ] ✗ PGE05004 — direct self-reference
{#} #Node
   [.] .value#string
   [.] .child#Node                      [ ] ✗ PGE05004 — infinite recursion
```

```aljam3
[ ] ✗ PGE05004 — mutual recursion without indirection
{#} #A
   [.] .name#string
   [.] .partner#B                       [ ] ✗ PGE05004 — A→B→A cycle

{#} #B
   [.] .label#string
   [.] .partner#A
```

```aljam3
[ ] ✗ PGE05004 — transitive cycle (A→B→C→A)
{#} #A
   [.] .ref#B

{#} #B
   [.] .ref#C

{#} #C
   [.] .ref#A                           [ ] ✗ PGE05004 — cycle detected
```

```aljam3
[ ] ✗ PGE05004 — direct self-reference via flexible field
{#} #Category
   [:] :name#string
   [:] :sub#Category                    [ ] ✗ PGE05004 — infinite recursion via flexible field
```

**Open point:** None.
