---
audience: developer
rule: "2.3"
code: PGE02003
name: Final Is Push-Once
severity: error
---

### Rule 2.3 — Final Is Push-Once
`PGE02003` · also covers former `PGE02004`

**Statement:** Once a variable reaches Final state (via `<<` or `>>`), no further push is allowed. A variable in Default state (assigned via `<~` or `~>`) cannot receive another default assignment — it may only be promoted to Final via one push. Any attempt to push into a Final variable, or to re-default a Default variable, is a compile error. Pulls from a Final or Default variable are unlimited.
**Rationale:** Final means the value is settled — downstream consumers can depend on it never changing. Default values exist so pipelines can provide fallbacks that are overridden at most once. Allowing reassignment would break deterministic data flow and make pipeline behavior unpredictable. Polyglot has no mutable variables.
**Detection:** The compiler tracks each variable's lifecycle stage. A push into Final, or a default assignment into Default, is rejected.

**VALID:**
```polyglot
[ ] ✓ Final variable pulled multiple times — pulls are unlimited
(-) >name#string
[-] >name << "Alice"            [ ] Final
[-] -Greet
   (-) <who << >name            [ ] ✓ pull 1
[-] -Log
   (-) <msg << >name            [ ] ✓ pull 2 — no limit on pulls
```

```polyglot
[ ] ✓ Default → Final is one valid transition
(-) >label#string
[-] >label <~ "pending"         [ ] Default
[-] >label << "confirmed"       [ ] Final — exactly one more push allowed
[-] -Display
   (-) <text << >label          [ ] ✓ pulling Final is fine
```

```polyglot
[ ] ✓ >> also produces Final
(-) >result#string
[-] -Compute
   (-) >value >> >result        [ ] Final via >>
[-] -Log
   (-) <msg << >result          [ ] ✓ pulling Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02003 — second push into a Final variable
(-) >name#string
[-] >name << "Alice"            [ ] Final
[-] >name << "Bob"              [ ] ✗ PGE02003 — >name is already Final
```

```polyglot
[ ] ✗ PGE02003 — default push into a Final variable
(-) >count#int
[-] >count << 42                [ ] Final
[-] >count <~ 0                 [ ] ✗ PGE02003 — cannot default-assign a Final variable
```

```polyglot
[ ] ✗ PGE02003 — >> into an already-Final variable
(-) >result#string
[-] -Step1
   (-) >out >> >result          [ ] Final via >>
[-] -Step2
   (-) >out >> >result          [ ] ✗ PGE02003 — >result is already Final
```

```polyglot
[ ] ✗ PGE02003 — second default push (Default cannot be re-defaulted)
(-) >tag#string
[-] >tag <~ "draft"                [ ] Default
[-] >tag <~ "review"              [ ] ✗ PGE02003 — already in Default, cannot default-assign again
```

```polyglot
[ ] ✓ Default → Final via one push
(-) >label#string
[-] >label <~ "pending"            [ ] Default
[-] >label << "confirmed"          [ ] Final — the one allowed push
```

```polyglot
[ ] ✓ Default pulled without promotion — stays Default
(-) >fallback#string
[-] >fallback <~ "N/A"             [ ] Default
[-] -Display
   (-) <text << >fallback          [ ] ✓ pulling Default is valid
```

```polyglot
[ ] ✓ Each conditional branch independently promotes Default → Final
(-) >status#string
[-] >status <~ "unknown"           [ ] Default
[?] $condition
   [?] #Yes
      [-] >status << "approved"    [ ] Final in this branch
   [?] #No
      [-] >status << "rejected"    [ ] Final in this branch
   [?] *?
      [ ] ✓ >status stays Default — not promoted in catch-all
```

**Resolved — Data fields:** Final semantics apply uniformly. `{#}` data fields declared with `<<` are schema-level constants (Final by definition, immutable at runtime). Fields declared with `<~` follow the same Default → Final promotion rule as `$` variables.

**Resolved — Conditional paths:** The compiler uses conservative (worst-case) analysis. If ANY conditional branch promotes a variable to Final, the compiler treats the variable as *possibly Final* after the block and rejects further pushes. The compiler does not perform path-sensitive analysis — it takes the union of all branch outcomes.

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Final state and references PGE02003
