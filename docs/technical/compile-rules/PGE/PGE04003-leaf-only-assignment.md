---
audience: developer
rule: "4.3"
code: PGE04003
name: Leaf-Only Assignment
severity: error
---

# Rule 4.3 — Leaf-Only Assignment
`PGE04003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Assignment operators (`<<`, `>>`, `<~`, `~>`) can only target **leaf nodes** in a serialized identifier path. A leaf node is a field with no children. Assigning to an intermediate branch node (a field that has sub-fields) is a compile error. Each leaf field must be assigned individually. This applies to both fixed (`.`) and flexible (`:`) field paths.
**Rationale:** Aljam3's serialized data model treats every field path as a tree. Only leaf nodes hold values — branch nodes are structural groupings. Allowing assignment to a branch would be ambiguous: does it replace the subtree or set a value alongside children? Requiring leaf-only assignment eliminates this ambiguity.
**Detection:** The compiler checks whether the target of an assignment has declared sub-fields. If it does, the target is a branch node and PGE04003 fires.

**VALID:**
```aljam3
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✓ assigning to leaf fields individually
[-] $user#UserRecord
   [-] $user.name << "Alice"
   [-] $user.age << 30
```

```aljam3
[ ] ✓ flexible leaf fields
[-] $config#serial
   [-] $config:timeout << 30
   [-] $config:retries << 3
```

**INVALID:**
```aljam3
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✗ PGE04003 — assigning to branch node $user which has children .name, .age
[-] $user#UserRecord
   [-] $user << "Alice"            [ ] ✗ PGE04003 — $user is a branch, not a leaf
   [-] $user.name << "Alice"
   [-] $user.age << 30
```

```aljam3
{#} #Address
   [.] .city#string
   [.] .zip#string

{#} #Person
   [.] .name#string
   [.] .address#Address

[ ] ✗ PGE04003 — .address is a branch (has .city, .zip children)
[-] $p#Person
   [-] $p.name << "Bob"
   [-] $p.address << "123 Main St"  [ ] ✗ PGE04003 — must assign .address.city and .address.zip
```
