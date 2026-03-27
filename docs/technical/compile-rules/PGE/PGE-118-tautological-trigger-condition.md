---
rule: "1.18"
code: PGE-118
name: Tautological or Contradictory Trigger Condition
severity: error
---

### Rule 1.18 — Tautological or Contradictory Trigger Condition
`PGE-118`

**Statement:** The compiler evaluates the boolean algebra of a pipeline's `[t]` trigger expression. Multiple `[t]` lines at the `{=}` level have an implicit AND; scoped empty `[t]` lines group sub-triggers with explicit `[|]` (OR) or `[^]` (XOR) operators. If the total expression evaluates to the same boolean value (all True or all False) for every possible combination of trigger states, it is a compile error. An always-True trigger means the condition is meaningless — the pipeline triggers unconditionally. An always-False trigger means the pipeline can never be triggered — dead code.
**Rationale:** Trigger conditions exist to discriminate when a pipeline should execute. A condition that produces the same result for all inputs provides no discrimination and is always a bug — either the developer wrote a contradictory expression (never triggers) or a tautological one (always triggers, making the condition useless).
**Detection:** The compiler builds the boolean truth table for the trigger's combined expression. Each `[t]` trigger is a boolean variable (fired or not). If all rows in the truth table produce the same result (all True or all False), PGE-118 fires.

**See also:** PGE-105 (missing trigger), PGE-601 (conditional exhaustiveness), PGE-605 (compound condition overlap)

**VALID:**
```polyglot
[ ] ✓ AND of compatible triggers — both can co-fire
{=} =ProcessOnScheduleAndFiles
   [=] <files#array:path
   [t] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <files
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >result#string
   [r] >result << "processed"
```

```polyglot
[ ] ✓ OR group — not exhaustive, some states produce False
{=} =MultiTrigger
   [t]
      [t] =T.Webhook"/api"
      [|] =T.Daily"3AM"
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "triggered"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-118 — trigger expression is always False (contradiction)
{=} =NeverFires
   [t] =T.Daily"3AM"
   [t] =T.Daily"5PM"                         [ ] ✗ PGE-118 — AND of mutually exclusive timers
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "unreachable"
[ ] =T.Daily"3AM" AND =T.Daily"5PM" — never co-fire, always False
```

```polyglot
[ ] ✗ PGE-118 — XOR of same trigger is always False
{=} =XorSelf
   [t]
      [t] =T.Webhook"/hook"
      [^] =T.Webhook"/hook"                  [ ] ✗ PGE-118 — A XOR A = always False
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "unreachable"
[ ] A ⊕ A = False for all values of A
```

**Diagnostic:** `"Trigger condition on pipeline ={PipelineName} is always {true|false} — {tautological condition means trigger fires unconditionally|contradictory condition means pipeline can never trigger}"`
