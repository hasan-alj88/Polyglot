---
audience: designer
rule: "10.7"
code: PGE10007
name: Chain Step Label Overflow
severity: error
type: spec
updated: 2026-04-09
---

### Rule 10.7 — Chain Step Label Overflow
`PGE10007`

<!-- @u:syntax/blocks -->

**Statement:** The number of `(.)` step labels in a chain label block must not exceed the number of steps in the chain. More labels than steps means at least one label has no target, which is a compile error.
**Rationale:** Each `(.)` step label maps positionally to a step in the chain. If there are more labels than steps, the excess labels cannot bind to anything — they are dangling references. The compiler rejects this to prevent silent misconfiguration.
**Detection:** The compiler counts the steps in the chain body and the `(.)` labels in the label block. If the label count exceeds the step count, the error is raised at the first excess label.

**VALID:**
```polyglot
[ ] ✓ 3-step chain with 3 step labels — exact match
[-] -Chain
   (.) $Fetch
   (.) $Parse
   (.) $Store
   [-] -Http.Get
   [-] -Serial.JSON.Parse
   [-] -DB.Insert
```

**VALID:**
```polyglot
[ ] ✓ 3-step chain with 2 step labels — fewer labels is allowed
[-] -Chain
   (.) $Fetch
   (.) $Parse
   [-] -Http.Get
   [-] -Serial.JSON.Parse
   [-] -DB.Insert
```

**INVALID:**
```polyglot
[ ] ✗ PGE10007 — 4 step labels but only 3 steps in chain
[-] -Chain
   (.) $Fetch
   (.) $Parse
   (.) $Store
   (.) $Verify
   [-] -Http.Get
   [-] -Serial.JSON.Parse
   [-] -DB.Insert
```

**Diagnostic:** "Chain has 3 steps but 4 `(.)` step labels — remove excess labels or add missing chain steps"

**Related:** PGE08005 (Unresolved Step Reference)
