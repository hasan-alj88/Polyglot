---
rule: "6.8"
code: PGE-608
name: Compound Condition Exhaustiveness
severity: error
split_from: PGE-601
---

### Rule 6.8 — Compound Condition Exhaustiveness
`PGE-608`

**Statement:** A `[?]` conditional that uses compound conditions (`[&]` AND, `[+]` OR, `[^]` XOR) must include a `[?] *?` catch-all branch. The partition-refinement algorithm (Issue 006) exists for proving compound exhaustiveness, but `*?` remains mandatory — compound exhaustiveness proofs are complex enough that the safety net is warranted.
**Rationale:** Compound conditions combine multiple predicates across a Cartesian product space. While the algorithm can theoretically prove coverage, the complexity of real-world compound conditions makes `*?` a low-cost safety requirement. The algorithm is used for overlap detection (PGE-605) but not for eliminating the `*?` requirement.
**Detection:** When a `[?]` block contains compound condition operators (`[&]`, `[+]`, `[^]`), the compiler checks for a `[?] *?` branch. If absent, PGE-608 fires.

**VALID:**
```polyglot
[ ] ✓ compound condition — *? required until static analysis available
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] *?
   [r] =Deny.Drive
```

**INVALID:**
```polyglot
[ ] ✗ PGE-608 — compound condition without *?
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] $age <=? 18
   [r] =Deny.Minor
[ ] ✗ PGE-608 — compound conditions present, *? required
```

**Resolved — Static exhaustiveness proofs:** The partition-refinement algorithm (Issue 006) can prove compound exhaustiveness, but `*?` remains mandatory for all compound conditions. The algorithm is used for overlap detection (PGE-605) only. In future, the compiler may emit a PGW warning when `*?` is provably unreachable, but the `*?` requirement itself will not be relaxed.

**See also:**
- [PGE-601 — Conditional Must Be Exhaustive](PGE-601-conditional-must-be-exhaustive.md) — parent rule
- [PGE-605 — Compound Condition Overlap](PGE-605-compound-condition-overlap.md) — overlap detection for compound conditions
- [006 — Compound Condition Exhaustiveness](../../compiler_issues/006-compound-condition-exhaustiveness.md) — design issue
