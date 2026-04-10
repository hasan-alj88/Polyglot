---
audience: developer
rule: "6.4"
code: PGE06004
name: Numeric Range Overlap
severity: error
---

### Rule 6.4 — Numeric Range Overlap
`PGE06004`

<!-- @u:syntax/blocks -->

**Statement:** In a `[?]` conditional with numeric branches, no two branches may overlap. Overlapping ranges are always a compile error, regardless of whether `*?` is present. The compiler must identify the overlapping branches and the overlapping interval.
**Rationale:** Overlapping ranges create ambiguity — when a value falls in the overlap, the compiler cannot determine which branch should execute. This is always a bug. Even with `*?`, overlaps must be resolved.
**Detection:** The compiler checks all pairs of numeric branch conditions for intersection. If any pair has a non-empty intersection, PGE06004 fires. The error message identifies the two overlapping branches and the overlapping interval.

**See also:** PGE06003 (range not exhaustive), PGE06001 (general exhaustiveness), [Overlap Detection Algorithm](../algorithms/overlap-detection.md)

**VALID:**
```polyglot
[ ] ✓ mutually exclusive ranges — no overlap
[?] $val
   [?] $val <? 0
      [-] -Negative
   [?] $val ?[0,50)
      [-] -Low
   [?] $val ?[50,100]
      [-] -High
   [?] $val >? 100
      [-] -VeryHigh
```

**INVALID:**
```polyglot
[ ] ✗ PGE06004 — ranges overlap at [80, 90]
[?] $val
   [?] $val ?[0,90]
      [-] -Low
   [?] $val ?[80,100]
      [-] -High                [ ] ✗ PGE06004 — [80,90] in both branches
   [?] *?
      [-] -Other               [ ] *? does not fix overlap
```

```polyglot
[ ] ✗ PGE06004 — comparison operators overlap
[?] $score
   [?] $score >=? 80
      [-] -High
   [?] $score >=? 70
      [-] -Mid                 [ ] ✗ PGE06004 — [80, +∞) is in both branches
   [?] $score <? 70
      [-] -Low
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — numeric range overlap rules reference PGE06004
