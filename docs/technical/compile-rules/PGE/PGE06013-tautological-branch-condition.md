---
audience: developer
rule: "6.13"
code: PGE06013
name: Tautological or Contradictory Branch Condition
severity: error
---

### Rule 6.13 — Tautological or Contradictory Branch Condition
`PGE06013`

**Statement:** The compiler evaluates the boolean algebra of each `[?]` branch's compound expression (`[&]` AND, `[|]` OR, `[^]` XOR). If a single branch's expression evaluates to the same boolean value (all True or all False) for every possible combination of its sub-conditions, it is a compile error. An always-True branch catches everything (making subsequent branches unreachable). An always-False branch is dead code (never matches any input).
**Rationale:** Compound conditions combine multiple predicates. A branch that produces the same result for all inputs is always a bug — either the developer wrote a contradictory expression (never matches) or a tautological one (matches everything). This is the conditional equivalent of PGE01018 (tautological/contradictory trigger conditions).
**Detection:** The compiler builds the truth table for the branch's compound expression. Each sub-condition is a boolean variable (true or false). If all rows produce the same result, PGE06013 fires. This check runs before the partition refinement algorithm (PGE06005/PGE06008), which assumes all branches are individually satisfiable.

**See also:**
- [PGE01018 — Tautological or Contradictory Trigger Condition](PGE01018-tautological-trigger-condition.md) — same check for triggers
- [PGE06005 — Compound Condition Overlap](PGE06005-compound-condition-overlap.md) — overlap detection (runs after PGE06013)
- [PGE06008 — Compound Condition Exhaustiveness](PGE06008-compound-condition-exhaustiveness.md) — exhaustiveness (runs after PGE06013)
- [PGE06012 — Unreachable Branch After Wildcard](PGE06012-unreachable-branch-after-wildcard.md) — unreachable due to `*?` ordering

**VALID:**
```polyglot
[ ] ✓ compound condition is satisfiable — not tautological or contradictory
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] *?
   [r] =Deny
```

```polyglot
[ ] ✓ OR is satisfiable — some inputs match, some don't
[?] $status =? .Active
   [|] $role =? .Admin
      [r] =GrantAccess
[?] *?
   [r] =DenyAccess
```

**INVALID:**
```polyglot
[ ] ✗ PGE06013 — contradictory branch (always False)
[?] $a =? .X
   [&] $a =!? .X                              [ ] ✗ PGE06013 — A AND NOT A = always False
      [r] =NeverReached
[?] *?
   [r] =Default
```

```polyglot
[ ] ✗ PGE06013 — tautological branch (always True)
[?] $a =? .X
   [|] $a =!? .X                              [ ] ✗ PGE06013 — A OR NOT A = always True
      [r] =AlwaysReached
[?] $a =? .Y
   [r] =NeverReached                           [ ] unreachable due to tautological branch above
```

```polyglot
[ ] ✗ PGE06013 — XOR of identical conditions (always False)
[?] $status =? .Active
   [^] $status =? .Active                     [ ] ✗ PGE06013 — A XOR A = always False
      [r] =NeverReached
[?] *?
   [r] =Default
```

**Diagnostic:** `"Branch condition at line {N} is always {true|false} — {tautological condition matches all inputs|contradictory condition matches no inputs}"`

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — tautological/contradictory branch rules reference PGE06013
