---
rule: "2.3"
code: PGE-203
name: Final Is Push-Once
severity: error
---

### Rule 2.3 — Final Is Push-Once
`PGE-203` · also covers former `PGE-204`

**Statement:** Once a variable reaches Final state (via `<<` or `>>`), no further push is allowed. A variable in Default state (assigned via `<~` or `~>`) cannot receive another default assignment — it may only be promoted to Final via one push. Any attempt to push into a Final variable, or to re-default a Default variable, is a compile error. Pulls from a Final or Default variable are unlimited.
**Rationale:** Final means the value is settled — downstream consumers can depend on it never changing. Default values exist so pipelines can provide fallbacks that are overridden at most once. Allowing reassignment would break deterministic data flow and make pipeline behavior unpredictable. Polyglot has no mutable variables.
**Detection:** The compiler tracks each variable's lifecycle stage. A push into Final, or a default assignment into Default, is rejected.

**VALID:**
```polyglot
[ ] ✓ Final variable pulled multiple times — pulls are unlimited
[=] >name#string
[r] >name << "Alice"            [ ] Final
[r] =Greet
   [=] <who << >name            [ ] ✓ pull 1
[r] =Log
   [=] <msg << >name            [ ] ✓ pull 2 — no limit on pulls
```

```polyglot
[ ] ✓ Default → Final is one valid transition
[=] >label#string
[r] >label <~ "pending"         [ ] Default
[r] >label << "confirmed"       [ ] Final — exactly one more push allowed
[r] =Display
   [=] <text << >label          [ ] ✓ pulling Final is fine
```

```polyglot
[ ] ✓ >> also produces Final
[=] >result#string
[r] =Compute
   [=] >value >> >result        [ ] Final via >>
[r] =Log
   [=] <msg << >result          [ ] ✓ pulling Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE-203 — second push into a Final variable
[=] >name#string
[r] >name << "Alice"            [ ] Final
[r] >name << "Bob"              [ ] ✗ PGE-203 — >name is already Final
```

```polyglot
[ ] ✗ PGE-203 — default push into a Final variable
[=] >count#int
[r] >count << 42                [ ] Final
[r] >count <~ 0                 [ ] ✗ PGE-203 — cannot default-assign a Final variable
```

```polyglot
[ ] ✗ PGE-203 — >> into an already-Final variable
[=] >result#string
[r] =Step1
   [=] >out >> >result          [ ] Final via >>
[r] =Step2
   [=] >out >> >result          [ ] ✗ PGE-203 — >result is already Final
```

```polyglot
[ ] ✗ PGE-203 — second default push (Default cannot be re-defaulted)
[=] >tag#string
[r] >tag <~ "draft"                [ ] Default
[r] >tag <~ "review"              [ ] ✗ PGE-203 — already in Default, cannot default-assign again
```

```polyglot
[ ] ✓ Default → Final via one push
[=] >label#string
[r] >label <~ "pending"            [ ] Default
[r] >label << "confirmed"          [ ] Final — the one allowed push
```

```polyglot
[ ] ✓ Default pulled without promotion — stays Default
[=] >fallback#string
[r] >fallback <~ "N/A"             [ ] Default
[r] =Display
   [=] <text << >fallback          [ ] ✓ pulling Default is valid
```

```polyglot
[ ] ✓ Each conditional branch independently promotes Default → Final
[=] >status#string
[r] >status <~ "unknown"           [ ] Default
[?] $condition
   [?] #Yes
      [r] >status << "approved"    [ ] Final in this branch
   [?] #No
      [r] >status << "rejected"    [ ] Final in this branch
   [?] *?
      [ ] ✓ >status stays Default — not promoted in catch-all
```

**Resolved — Data fields:** Final semantics apply uniformly. `{#}` data fields declared with `<<` are schema-level constants (Final by definition, immutable at runtime). Fields declared with `<~` follow the same Default → Final promotion rule as `$` variables.

**Resolved — Conditional paths:** The compiler uses conservative (worst-case) analysis. If ANY conditional branch promotes a variable to Final, the compiler treats the variable as *possibly Final* after the block and rejects further pushes. The compiler does not perform path-sensitive analysis — it takes the union of all branch outcomes.
