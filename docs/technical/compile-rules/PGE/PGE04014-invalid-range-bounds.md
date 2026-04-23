---
audience: developer
rule: "4.12"
code: PGE04014
name: Invalid Range Bounds
severity: error
---

# Rule 4.12 — Invalid Range Bounds
`PGE04013`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** A range expression where both bounds are literals and the lower bound exceeds the upper bound is a compile error. For inclusive ranges `?[lo,hi]`, `lo > hi` is invalid. For exclusive ranges `?(lo,hi)`, `lo >= hi` is invalid. Mixed ranges follow accordingly: `?[lo,hi)` requires `lo < hi`; `?(lo,hi]` requires `lo < hi`.
**Rationale:** An inverted range creates an empty set that can never be satisfied — no value is simultaneously above the lower bound and below the upper bound. This is always a mistake and produces dead conditional branches. Catching it at compile time prevents silent logic errors.
**Detection:** The compiler evaluates range bounds when both are numeric literals. If the resulting range is empty (lower exceeds upper given the inclusivity), the range is rejected. Ranges with variable bounds are excluded from this check.

**See also:** PGE06003 (numeric range not exhaustive), PGE06004 (numeric range overlap), PGE04015 (conditional type-operator mismatch)

**VALID:**
```polyglot
[ ] ✓ valid inclusive range — lo < hi
{-} -Grade
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <score#int
   (-) >label#string
   [?] $score
      [?] ?[0,59]
         [-] >label << "fail"
      [?] ?[60,100]
         [-] >label << "pass"
      [?] *?
         [-] >label << "unknown"
```

```polyglot
[ ] ✓ single-point inclusive range — lo == hi is valid
{-} -Exact
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <count#int
   (-) >out#string
   [?] $count
      [?] ?[5,5]                        [ ] ✓ matches exactly 5
         [-] >out << "five"
      [?] *?
         [-] >out << "other"
```

```polyglot
[ ] ✓ valid exclusive range — lo < hi with room between
{-} -Between
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <temp#float
   (-) >out#string
   [?] $temp
      [?] ?(0.0,100.0)
         [-] >out << "in range"
      [?] *?
         [-] >out << "out of range"
```

**INVALID:**
```polyglot
[ ] ✗ PGE04013 — inverted inclusive range
{-} -BadInclusive
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <score#int
   (-) >out#string
   [?] $score
      [?] ?[100,0]                      [ ] ✗ PGE04013 — lo (100) > hi (0)
         [-] >out << "impossible"
      [?] *?
         [-] >out << "always here"
```

```polyglot
[ ] ✗ PGE04013 — empty exclusive range (lo == hi)
{-} -BadExclusive
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <temp#float
   (-) >out#string
   [?] $temp
      [?] ?(5.0,5.0)                   [ ] ✗ PGE04013 — lo (5.0) >= hi (5.0) for exclusive
         [-] >out << "impossible"
      [?] *?
         [-] >out << "always here"
```

```polyglot
[ ] ✗ PGE04013 — inverted mixed range
{-} -BadMixed
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <value#int
   (-) >out#string
   [?] $value
      [?] ?[10,3)                       [ ] ✗ PGE04013 — lo (10) > hi (3)
         [-] >out << "impossible"
      [?] *?
         [-] >out << "always here"
```

**Open point:** None.

## See Also

- [[syntax/operators|Operators]] — range bound ordering rules in conditionals
