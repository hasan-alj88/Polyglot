---
rule: "6.3"
code: PGE-603
name: Numeric Range Not Exhaustive
severity: error
---

### Rule 6.3 — Numeric Range Not Exhaustive
`PGE-603`

**Statement:** When a `[?]` conditional branches on a numeric type (`int` or `float`) using comparison operators or range operators, the branches must collectively cover -∞ to +∞. If they do not and no `*?` catch-all is present, PGE-603 fires. With `*?`, it covers any gap and PGE-603 does not fire.
**Rationale:** Numeric ranges are analyzable at compile time. The compiler can verify whether the union of all branch conditions covers the entire number line. Gaps mean some values have no defined path.
**Detection:** The compiler collects all numeric comparison/range conditions in the `[?]` block, computes their union, and checks whether it equals (-∞, +∞). If not and no `*?` exists, PGE-603 fires. The error message identifies the uncovered interval(s).

**See also:** PGE-604 (overlapping ranges), PGE-601 (general exhaustiveness)

**VALID:**
```polyglot
[ ] ✓ ranges cover -∞ to +∞ — no *? needed
[?] $val
   [?] $val <? 0
      [r] =Negative
   [?] $val ?[0,100]
      [r] =Normal
   [?] $val >? 100
      [r] =High
```

```polyglot
[ ] ✓ partial ranges + *? fills the gap
[?] $val ?[0,100]
   [?] #Boolean.True
      [r] =InRange
   [?] *?
      [r] =OutOfRange
```

**INVALID:**
```polyglot
[ ] ✗ PGE-603 — gap in coverage at [0, 100]
[?] $val
   [?] $val <? 0
      [r] =Negative
   [?] $val >? 100
      [r] =High
   [ ] ✗ PGE-603 — [0, 100] not covered, no *?
```

```polyglot
[ ] ✗ PGE-603 — only one range, rest uncovered
[?] $score >=? 90
   [?] #Boolean.True
      [r] =Grade.A
   [ ] ✗ PGE-603 — (-∞, 90) not covered, no *?
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — numeric range exhaustiveness rules reference PGE-603
