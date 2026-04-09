---
audience: developer
rule: "3.1"
code: PGE03001
name: No Push Across Parallel Boundaries
severity: error
---

### Rule 3.1 — No Push Across Parallel Boundaries
`PGE03001`

**Statement:** A variable that originates inside a `[=]` parallel pipeline cannot be pushed into from outside that parallel scope. Cross-parallel data flow is collection-only — the parallel must produce its output, and the prime pipeline collects it via `(*)` after the parallel completes. This applies to all push operators (`<<`, `>>`, `<~`, `~>`).
**Rationale:** Parallel pipelines run concurrently. Allowing external pushes into a parallel's internal variables would create race conditions — two concurrent writers with no ordering guarantee. The collection model (`(*)`) provides a structured, deterministic way to gather parallel results.
**Detection:** At any push statement in the prime pipeline (or a sibling parallel) that targets a variable declared inside a `[=]` parallel scope.

**VALID:**
```polyglot
[ ] ✓ parallel produces output, prime collects via (*)
[=] -Fetch.Profile
   (-) <id << $userId
   (-) >profile >> $profile

[=] -Fetch.History
   (-) <id << $userId
   (-) >history >> $history

(*) *All
   (*) << $profile
   (*) << $history

[ ] ✓ $profile and $history accessible after collection
[-] -Report.Generate
   (-) <profile << $profile
   (-) <history << $history
```

```polyglot
[ ] ✓ prime pushes into parallel's INPUT (not its internal vars)
[=] -Compute
   (-) <input << $data            [ ] ✓ pushing into <input from prime is valid
   (-) >result >> $result
```

**INVALID:**
```polyglot
[ ] ✗ PGE03001 — prime pushes into a parallel's output variable
[=] -Fetch.Data
   (-) >result >> $fetchResult

[-] $fetchResult << "override"    [ ] ✗ PGE03001 — $fetchResult belongs to the parallel
```

```polyglot
[ ] ✗ PGE03001 — sibling parallel pushes into another parallel's variable
[=] -TaskA
   (-) >output >> $resultA

[=] -TaskB
   (-) <input << $resultA         [ ] ✗ PGE03001 — $resultA not yet collected
```

**Open point:** None — applies to all push operators (`<<`, `>>`, `<~`, `~>`). The boundary is the `[=]` scope; inputs to the parallel (`<` parameters) are pushed from the prime before the parallel starts, which is valid.

### See Also

- [[concepts/collections/INDEX|Collections]] — user-facing collection and parallel documentation (references PGE03001)
