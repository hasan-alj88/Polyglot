---
audience: developer
rule: "1.18"
code: PGE01018
name: Tautological or Contradictory Trigger Condition
severity: error
---

### Rule 1.18 — Tautological or Contradictory Trigger Condition
`PGE01018`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** The compiler evaluates the boolean algebra of a pipeline's `[T]` trigger expression. Multiple `[T]` lines at the `{-}` level have an implicit AND; scoped empty `[T]` lines group sub-triggers with explicit `[+]` (OR) or `[^]` (XOR) operators. If the total expression evaluates to the same boolean value (all True or all False) for every possible combination of trigger states, it is a compile error. An always-True trigger means the condition is meaningless — the pipeline triggers unconditionally. An always-False trigger means the pipeline can never be triggered — dead code.
**Rationale:** Trigger conditions exist to discriminate when a pipeline should execute. A condition that produces the same result for all inputs provides no discrimination and is always a bug — either the developer wrote a contradictory expression (never triggers) or a tautological one (always triggers, making the condition useless).
**Detection:** The compiler builds the boolean truth table for the trigger's combined expression. Each `[T]` trigger is a boolean variable (fired or not). If all rows in the truth table produce the same result (all True or all False), PGE01018 fires.

**See also:** PGE01005 (missing trigger), PGE06001 (conditional exhaustiveness), PGE06005 (compound condition overlap)

**VALID:**
```polyglot
[ ] ✓ AND of compatible triggers — both can co-fire
{-} -ProcessOnScheduleAndFiles
   (-) <files#array:path
   [T] -T.Folder.NewFiles"/inbox/"
      (-) >NewFiles >> <files
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >result#string
   [-] >result << "processed"
```

```polyglot
[ ] ✓ OR group — not exhaustive, some states produce False
{-} -MultiTrigger
   [T]
      [T] -T.Webhook"/api"
      [+] -T.Daily"3AM"
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] >out << "triggered"
```

**INVALID:**
```polyglot
[ ] ✗ PGE01018 — trigger expression is always False (contradiction)
{-} -NeverFires
   [T] -T.Daily"3AM"
   [T] -T.Daily"5PM"                         [ ] ✗ PGE01018 — AND of mutually exclusive timers
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] >out << "unreachable"
[ ] -T.Daily"3AM" AND -T.Daily"5PM" — never co-fire, always False
```

```polyglot
[ ] ✗ PGE01018 — XOR of same trigger is always False
{-} -XorSelf
   [T]
      [T] -T.Webhook"/hook"
      [^] -T.Webhook"/hook"                  [ ] ✗ PGE01018 — A XOR A = always False
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] >out << "unreachable"
[ ] A ⊕ A = False for all values of A
```

**Diagnostic:** `"Trigger condition on pipeline ={PipelineName} is always {true|false} — {tautological condition means trigger fires unconditionally|contradictory condition means pipeline can never trigger}"`

### See Also

- [[concepts/pipelines/io-triggers|IO & Triggers]] — documents trigger tautology/contradiction, references PGE01018
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01018
