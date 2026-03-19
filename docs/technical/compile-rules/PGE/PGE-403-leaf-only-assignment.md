---
rule: "4.3"
code: PGE-403
name: Leaf-Only Assignment
severity: error
---

### Rule 4.3 — Leaf-Only Assignment
`PGE-403`

**Statement:** Assignment operators (`<<`, `>>`, `<~`, `~>`) can only target **leaf nodes** in a serialized identifier path. A leaf node is a field with no children. Assigning to an intermediate branch node (a field that has sub-fields) is a compile error. Each leaf field must be assigned individually. This applies to both fixed (`.`) and flexible (`:`) field paths.
**Rationale:** Polyglot's serialized data model treats every field path as a tree. Only leaf nodes hold values — branch nodes are structural groupings. Allowing assignment to a branch would be ambiguous: does it replace the subtree or set a value alongside children? Requiring leaf-only assignment eliminates this ambiguity.
**Detection:** The compiler checks whether the target of an assignment has declared sub-fields. If it does, the target is a branch node and PGE-403 fires.

**VALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✓ assigning to leaf fields individually
[r] $user;UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30
```

```polyglot
[ ] ✓ flexible leaf fields
[r] $config;serial
   [r] $config:timeout << 30
   [r] $config:retries << 3
```

**INVALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✗ PGE-403 — assigning to branch node $user which has children .name, .age
[r] $user;UserRecord
   [r] $user << "Alice"            [ ] ✗ PGE-403 — $user is a branch, not a leaf
   [r] $user.name << "Alice"
   [r] $user.age << 30
```

```polyglot
{#} #Address
   [.] .city;string
   [.] .zip;string

{#} #Person
   [.] .name;string
   [.] .address;Address

[ ] ✗ PGE-403 — .address is a branch (has .city, .zip children)
[r] $p;Person
   [r] $p.name << "Bob"
   [r] $p.address << "123 Main St"  [ ] ✗ PGE-403 — must assign .address.city and .address.zip
```
