---
rule: "6.13"
code: PGE-613
name: Tautological or Contradictory Branch Condition
severity: error
---

### Rule 6.13 — Tautological or Contradictory Branch Condition
`PGE-613`

**Statement:** The compiler evaluates the boolean algebra of each `[?]` branch's compound expression (`[&]` AND, `[|]` OR, `[^]` XOR). If a single branch's expression evaluates to the same boolean value (all True or all False) for every possible combination of its sub-conditions, it is a compile error. An always-True branch catches everything (making subsequent branches unreachable). An always-False branch is dead code (never matches any input).
**Rationale:** Compound conditions combine multiple predicates. A branch that produces the same result for all inputs is always a bug — either the developer wrote a contradictory expression (never matches) or a tautological one (matches everything). This is the conditional equivalent of PGE-118 (tautological/contradictory trigger conditions).
**Detection:** The compiler builds the truth table for the branch's compound expression. Each sub-condition is a boolean variable (true or false). If all rows produce the same result, PGE-613 fires. This check runs before the partition refinement algorithm (PGE-605/PGE-608), which assumes all branches are individually satisfiable.

**See also:**
- [PGE-118 — Tautological or Contradictory Trigger Condition](PGE-118-tautological-trigger-condition.md) — same check for triggers
- [PGE-605 — Compound Condition Overlap](PGE-605-compound-condition-overlap.md) — overlap detection (runs after PGE-613)
- [PGE-608 — Compound Condition Exhaustiveness](PGE-608-compound-condition-exhaustiveness.md) — exhaustiveness (runs after PGE-613)
- [PGE-612 — Unreachable Branch After Wildcard](PGE-612-unreachable-branch-after-wildcard.md) — unreachable due to `*?` ordering

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
[ ] ✗ PGE-613 — contradictory branch (always False)
[?] $a =? .X
   [&] $a =!? .X                              [ ] ✗ PGE-613 — A AND NOT A = always False
      [r] =NeverReached
[?] *?
   [r] =Default
```

```polyglot
[ ] ✗ PGE-613 — tautological branch (always True)
[?] $a =? .X
   [|] $a =!? .X                              [ ] ✗ PGE-613 — A OR NOT A = always True
      [r] =AlwaysReached
[?] $a =? .Y
   [r] =NeverReached                           [ ] unreachable due to tautological branch above
```

```polyglot
[ ] ✗ PGE-613 — XOR of identical conditions (always False)
[?] $status =? .Active
   [^] $status =? .Active                     [ ] ✗ PGE-613 — A XOR A = always False
      [r] =NeverReached
[?] *?
   [r] =Default
```

**Diagnostic:** `"Branch condition at line {N} is always {true|false} — {tautological condition matches all inputs|contradictory condition matches no inputs}"`

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — tautological/contradictory branch rules reference PGE-613
