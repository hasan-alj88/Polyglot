---
rule: "6.5"
code: PGE-605
name: Compound Condition Overlap
severity: error
---

### Rule 6.5 — Compound Condition Overlap
`PGE-605`

**Statement:** In a `[?]` conditional with compound conditions (`[&]` AND, `[|]` OR, `[^]` XOR), no two branches may cover the same cell in the product space. Overlapping branches are always a compile error, regardless of whether `*?` is present. The compiler must identify the overlapping branches and a concrete input from the shared region.
**Rationale:** Overlapping compound branches create ambiguity — when an input matches multiple branches, the compiler cannot determine which should execute. This is always a bug. Even with `*?`, overlaps must be resolved.
**Detection:** The compiler builds a partition grid over all tested variables (per the [partition refinement algorithm](../algorithms/compound-exhaustiveness.md)), maps each branch to its covered cells, and checks if any cell is covered by more than one branch. If so, PGE-605 fires with the overlapping branches and a concrete counterexample from the shared cell.

**See also:** PGE-601 (general exhaustiveness), PGE-604 (numeric range overlap), PGE-608 (compound exhaustiveness), PGE-613 (tautological/contradictory branch — prerequisite check), [Partition Refinement Algorithm](../algorithms/compound-exhaustiveness.md), [Overlap Detection Algorithm](../algorithms/overlap-detection.md)

**VALID:**
```polyglot
[ ] ✓ non-overlapping compound conditions — no cell covered twice
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] $age >? 18
   [&] $hasLicense =? #Boolean.False
      [r] =Deny.NoLicense
[?] $age <=? 18
   [r] =Deny.Underage
[ ] Grid: {≤18,T}, {≤18,F}, {>18,T}, {>18,F} — each cell covered by exactly one branch
```

**INVALID:**
```polyglot
[ ] ✗ PGE-605 — branches overlap on cell {>18}
[?] $age >? 18
   [r] =Adult
[?] $age >? 15
   [r] =Teen                          [ ] ✗ PGE-605 — {>18} covered by both branches
[?] $age <=? 15
   [r] =Child
[ ] Partitions for $age: {≤15, 16..18, >18}
[ ] Branch 1 covers {>18}, Branch 2 covers {16..18, >18} — overlap at {>18}
```

```polyglot
[ ] ✗ PGE-605 — *? does not fix overlap
[?] $age >? 18
   [r] =Adult
[?] $age >? 15
   [r] =Teen                          [ ] ✗ PGE-605 — still overlaps at {>18}
[?] *?
   [r] =Other                         [ ] *? does not resolve overlap
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — compound condition overlap rules reference PGE-605
