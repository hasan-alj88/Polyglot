---
rule: "6.3"
code: PGE06003
name: Numeric Range Not Exhaustive
severity: error
---

### Rule 6.3 — Numeric Range Not Exhaustive
`PGE06003`

**Statement:** When a `[?]` conditional branches on a numeric type (`int` or `float`) using comparison operators or range operators, the branches must collectively cover -∞ to +∞. If they do not and no `*?` catch-all is present, PGE06003 fires. With `*?`, it covers any gap and PGE06003 does not fire.
**Rationale:** Numeric ranges are analyzable at compile time. The compiler can verify whether the union of all branch conditions covers the entire number line. Gaps mean some values have no defined path.
**Detection:** The compiler collects all numeric comparison/range conditions in the `[?]` block, computes their union, and checks whether it equals (-∞, +∞). If not and no `*?` exists, PGE06003 fires. The error message identifies the uncovered interval(s).

**See also:** PGE06004 (overlapping ranges), PGE06001 (general exhaustiveness)

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
[ ] ✗ PGE06003 — gap in coverage at [0, 100]
[?] $val
   [?] $val <? 0
      [r] =Negative
   [?] $val >? 100
      [r] =High
   [ ] ✗ PGE06003 — [0, 100] not covered, no *?
```

```polyglot
[ ] ✗ PGE06003 — only one range, rest uncovered
[?] $score >=? 90
   [?] #Boolean.True
      [r] =Grade.A
   [ ] ✗ PGE06003 — (-∞, 90) not covered, no *?
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — numeric range exhaustiveness rules reference PGE06003
