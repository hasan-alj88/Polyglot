---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 14. Variable Lifecycle (S14)

### EC-14.1: Default then Final — one reassignment

<!-- @variable-lifecycle -->
**What it tests:** Default allows exactly one promotion to Final. See [[variable-lifecycle]].

```polyglot
[=] >output#string ~> "fallback"
[ ] ... later in execution ...
[r] >output << "actual value"
```

### EC-14.2: Final — no further assignment (INVALID if reassigned)

**What it tests:** Once `<<` or `>>` is used, no more assignments.

```polyglot
[r] $x#int << 42
[ ] INVALID: $x is Final, cannot reassign
[ ] [r] $x << 99   <- would be rejected
```

### EC-14.3: Leaf-only assignment

<!-- @identifiers:Serialization Rules -->
**What it tests:** Only leaf fields (no children) can be assigned. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — assign to leaf
[r] $user:name << "Alice"
[r] $user:age << 30

[ ] INVALID — assign to branch that has children
[ ] [r] $user << "Alice"
[ ]    [r] $user:name << "Alice"
```

### EC-14.4: Sibling kind homogeneity

**What it tests:** All siblings must be the same kind (all enum or all value). Assignment within value fields is individually optional. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — all value fields, all assigned
[.] .timeout#int <~ 30
[.] .retries#int <~ 3

[ ] VALID — all value fields, mixed assignment (some assigned, some declared)
[.] .timeout#int <~ 30
[.] .retries#int

[ ] VALID — all value fields, none assigned
[.] .timeout#int
[.] .retries#int

[ ] INVALID — mixed kinds (enum + value at same level)
[ ] [.] .Active
[ ] [.] .count#int <~ 0
```
