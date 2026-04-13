---
audience: developer
rule: "6.5"
code: PGE06005
name: Compound Condition Overlap
severity: error
---

### Rule 6.5 — Compound Condition Overlap
`PGE06005`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/types -->

**Statement:** In a `[?]` conditional with compound conditions (`[&]` AND, `[+]` OR, `[^]` XOR), no two branches may cover the same cell in the product space. Overlapping branches are always a compile error, regardless of whether `*?` is present. The compiler must identify the overlapping branches and a concrete input from the shared region.
**Rationale:** Overlapping compound branches create ambiguity — when an input matches multiple branches, the compiler cannot determine which should execute. This is always a bug. Even with `*?`, overlaps must be resolved. Polyglot's exhaustive coverage model requires that every input maps to exactly one branch — ambiguity is a compile-time error, not a runtime coin flip.
**Detection:** The compiler builds a partition grid over all tested variables (per the [partition refinement algorithm](../algorithms/compound-exhaustiveness.md)), maps each branch to its covered cells, and checks if any cell is covered by more than one branch. If so, PGE06005 fires with the overlapping branches and a concrete counterexample from the shared cell.

**See also:** PGE06001 (general exhaustiveness), PGE06004 (numeric range overlap), PGE06008 (compound exhaustiveness), PGE06013 (tautological/contradictory branch — prerequisite check), [Partition Refinement Algorithm](../algorithms/compound-exhaustiveness.md), [Overlap Detection Algorithm](../algorithms/overlap-detection.md)

**VALID:**
```polyglot
[ ] ✓ non-overlapping compound conditions — no cell covered twice
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [-] -Allow.Drive
[?] $age >? 18
   [&] $hasLicense =? #Boolean.False
      [-] -Deny.NoLicense
[?] $age <=? 18
   [-] -Deny.Underage
[ ] Grid: {≤18,T}, {≤18,F}, {>18,T}, {>18,F} — each cell covered by exactly one branch
```

**INVALID:**
```polyglot
[ ] ✗ PGE06005 — branches overlap on cell {>18}
[?] $age >? 18
   [-] -Adult
[?] $age >? 15
   [-] -Teen                          [ ] ✗ PGE06005 — {>18} covered by both branches
[?] $age <=? 15
   [-] -Child
[ ] Partitions for $age: {≤15, 16..18, >18}
[ ] Branch 1 covers {>18}, Branch 2 covers {16..18, >18} — overlap at {>18}
```

```polyglot
[ ] ✗ PGE06005 — *? does not fix overlap
[?] $age >? 18
   [-] -Adult
[?] $age >? 15
   [-] -Teen                          [ ] ✗ PGE06005 — still overlaps at {>18}
[?] *?
   [-] -Other                         [ ] *? does not resolve overlap
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — compound condition overlap rules reference PGE06005
