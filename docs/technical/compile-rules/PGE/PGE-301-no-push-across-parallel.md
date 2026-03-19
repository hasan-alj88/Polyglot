---
rule: "3.1"
code: PGE-301
name: No Push Across Parallel Boundaries
severity: error
---

### Rule 3.1 — No Push Across Parallel Boundaries
`PGE-301`

**Statement:** A variable that originates inside a `[p]` parallel pipeline cannot be pushed into from outside that parallel scope. Cross-parallel data flow is collection-only — the parallel must produce its output, and the prime pipeline collects it via `[*]` after the parallel completes. This applies to all push operators (`<<`, `>>`, `<~`, `~>`).
**Rationale:** Parallel pipelines run concurrently. Allowing external pushes into a parallel's internal variables would create race conditions — two concurrent writers with no ordering guarantee. The collection model (`[*]`) provides a structured, deterministic way to gather parallel results.
**Detection:** At any push statement in the prime pipeline (or a sibling parallel) that targets a variable declared inside a `[p]` parallel scope.

**VALID:**
```polyglot
[ ] ✓ parallel produces output, prime collects via [*]
[p] =Fetch.Profile
   [=] <id << $userId
   [=] >profile >> $profile

[p] =Fetch.History
   [=] <id << $userId
   [=] >history >> $history

[*] *All
   [*] << $profile
   [*] << $history

[ ] ✓ $profile and $history accessible after collection
[r] =Report.Generate
   [=] <profile << $profile
   [=] <history << $history
```

```polyglot
[ ] ✓ prime pushes into parallel's INPUT (not its internal vars)
[p] =Compute
   [=] <input << $data            [ ] ✓ pushing into <input from prime is valid
   [=] >result >> $result
```

**INVALID:**
```polyglot
[ ] ✗ PGE-301 — prime pushes into a parallel's output variable
[p] =Fetch.Data
   [=] >result >> $fetchResult

[r] $fetchResult << "override"    [ ] ✗ PGE-301 — $fetchResult belongs to the parallel
```

```polyglot
[ ] ✗ PGE-301 — sibling parallel pushes into another parallel's variable
[p] =TaskA
   [=] >output >> $resultA

[p] =TaskB
   [=] <input << $resultA         [ ] ✗ PGE-301 — $resultA not yet collected
```

**Open point:** None — applies to all push operators (`<<`, `>>`, `<~`, `~>`). The boundary is the `[p]` scope; inputs to the parallel (`<` parameters) are pushed from the prime before the parallel starts, which is valid.
