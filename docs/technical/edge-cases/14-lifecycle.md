---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 14. Variable Lifecycle (S14)

### EC-14.1: Default then Final — one reassignment

<!-- @c:variable-lifecycle -->
**What it tests:** Default allows exactly one promotion to Final. See [[variable-lifecycle]].

```aljam3
(-) >output#string ~> "fallback"
[ ] ... later in execution ...
[-] >output << "actual value"
```

### EC-14.2: Final — no further assignment (INVALID if reassigned)

**What it tests:** Once `<<` or `>>` is used, no more assignments.

```aljam3
[-] $x#int << 42
[ ] INVALID: $x is Final, cannot reassign
[ ] [-] $x << 99   <- would be rejected
```

### EC-14.3: Leaf-only assignment

<!-- @u:identifiers:Serialization Rules -->
**What it tests:** Only leaf fields (no children) can be assigned. See [[identifiers#Serialization Rules]].

```aljam3
[ ] VALID — assign to leaf
[-] $user:name << "Alice"
[-] $user:age << 30

[ ] INVALID — assign to branch that has children
[ ] [-] $user << "Alice"
[ ]    [-] $user:name << "Alice"
```

### EC-14.4: Sibling kind homogeneity

**What it tests:** All siblings must be the same kind (all enum or all value). Assignment within value fields is individually optional. See [[identifiers#Serialization Rules]].

```aljam3
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

### EC-14.5: Final then Default on same variable

**What it tests:** `$x << 5` (Final) then `$x <~ 3` (Default) — PGE02003 fires because Final is push-once. See [[variable-lifecycle]].

```aljam3
[ ] ✗ PGE02003 — $x is Final, cannot re-default
[-] $x#int << 5
[-] $x#int <~ 3
```

### EC-14.6: Writing to input parameter in execution body

**What it tests:** Input parameters are always Final state (pipeline cannot trigger until inputs are satisfied). Writing to an input is PGE02003. See [[variable-lifecycle]], [[concepts/pipelines/io-triggers|IO & Triggers]].

```aljam3
[ ] ✗ PGE02003 — <inputName is Final, cannot reassign
{-} -Process
   [T] -T.Call
   (-) <inputName#string
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] <inputName << "overwritten"
```

### EC-14.7: Data load with plain literal — schema mismatch

**What it tests:** `[#]` requires a pipeline call or data reference, not a literal. PGE02011 fires. See [[concepts/pipelines/io-triggers|IO & Triggers]].

```aljam3
[ ] ✗ PGE02011 — plain literal is not a valid [#] source
[#] $x#int << 42
[#] $name#string << "Bob"
```
