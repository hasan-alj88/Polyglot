---
audience: developer
rule: "6.8"
code: PGE06008
name: Compound Condition Exhaustiveness
severity: error
split_from: PGE06001
---

# Rule 6.8 — Compound Condition Exhaustiveness
`PGE06008`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/types -->

**Statement:** A `[?]` conditional that uses compound conditions (`[&]` AND, `[+]` OR, `[^]` XOR) must be exhaustive. Exhaustiveness is proven in two ways:
1. **Partition proof (closed types only):** If all tested variables have closed types (enum, bool, int, float), the [partition refinement algorithm](../algorithms/compound-exhaustiveness.md) builds a Cartesian product grid and verifies that the union of all branch regions equals the full space. If proven, `*?` is not required.
2. **`*?` catch-all (open types):** If any tested variable has an open type (string, RawString, flexible field), the entire compound is open-ended and `*?` is mandatory.

If neither partition proof succeeds nor `*?` is present, PGE06008 fires.
**Rationale:** Compound conditions combine multiple predicates across a Cartesian product space. For closed types, the compiler can build a complete grid and verify coverage. For open types, the infinite value space makes static proof impossible — `*?` is required. This enforces Polyglot's exhaustive coverage principle — the compiler must prove that every possible combination of inputs is handled, offloading what would be a runtime crash to a compile-time guarantee.
**Detection:** The compiler classifies each tested variable as closed or open. If any is open → require `*?`. If all are closed → run the partition refinement algorithm. If the union of branch regions does not cover the full grid and `*?` is absent, PGE06008 fires.

**VALID:**
```polyglot
[ ] ✓ all closed types — partition algorithm proves exhaustiveness, no *? needed
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [-] -Allow.Drive
[?] $age >? 18
   [&] $hasLicense =? #Boolean.False
      [-] -Deny.NoLicense
[?] $age <=? 18
   [-] -Deny.Underage
[ ] Grid: {≤18,T}, {≤18,F}, {>18,T}, {>18,F} — all 4 cells covered
```

```polyglot
[ ] ✓ open type present — *? covers the open domain
[?] $name =? "admin"
   [&] $role =? .Admin
      [-] -GrantSuper
[?] *?
   [-] -GrantNormal
```

**INVALID:**
```polyglot
[ ] ✗ PGE06008 — closed types, partition proof fails, no *?
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [-] -Allow.Drive
[?] $age <=? 18
   [-] -Deny.Minor
[ ] ✗ PGE06008 — cell {>18, False} not covered by any branch
```

```polyglot
[ ] ✗ PGE06008 — open type without *?
[?] $name =? "admin"
   [&] $role =? .Admin
      [-] -GrantSuper
[?] $name =? "user"
   [-] -GrantNormal
[ ] ✗ PGE06008 — $name is string (open type), *? required
```

**See also:**
- [PGE06001 — Conditional Must Be Exhaustive](PGE06001-conditional-must-be-exhaustive.md) — parent rule
- [PGE06005 — Compound Condition Overlap](PGE06005-compound-condition-overlap.md) — overlap detection for compound conditions
- [Compound Exhaustiveness Algorithm](../algorithms/compound-exhaustiveness.md) — partition refinement algorithm spec
- [PGE06013 — Tautological or Contradictory Branch Condition](PGE06013-tautological-branch-condition.md) — prerequisite check

## See Also

- [[user/concepts/conditionals|Conditionals]] — compound condition exhaustiveness rules reference PGE06008
