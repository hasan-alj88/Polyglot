---
audience: developer
rule: "1.32"
code: PGE01032
name: Missing Trigger Boolean Output
severity: error
---

### Rule 1.32 — Missing Trigger Boolean Output
`PGE01032`

<!-- @brainstorming:marker-declarations §4 Rule G -->

**Statement:** Every `{T}` trigger definition must include an output `>IsTriggered#bool`. The output must exist and its type must be `#bool`. Additional outputs are allowed — they wire into the execution pipeline's inputs, supplying data alongside the fire signal.
**Rationale:** `>IsTriggered#bool` is the universal trigger contract. Every trigger must produce a boolean signal indicating whether the triggering condition was met. Without it, the pipeline has no way to know if it should execute. The type must be `#bool` (not `#string` or other types) to ensure unambiguous boolean semantics.
**Detection:** The compiler checks that every `{T}` block has an `(-) >IsTriggered#bool` declaration. If missing entirely, or if the type is not `#bool`, PGE01032 fires.

**VALID:**
```polyglot
[ ] ✓ — mandatory output only
{T} -T.Minimal
   (-) >IsTriggered#bool

[ ] ✓ — mandatory output + additional data outputs
{T} -T.Good.WithData
   (-) <endpoint#string
   (-) >IsTriggered#bool
   (-) >payload#serial

[ ] ✓ — trigger with body still needs >IsTriggered#bool
{T} -T.WithBody
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >IsTriggered#bool
   [-] -CheckCondition
      (-) >ready >> >IsTriggered
```

**INVALID:**
```polyglot
[ ] ✗ PGE01032 — missing >IsTriggered#bool entirely
{T} -T.Bad.NoSignal
   (-) <config#string
   (-) >payload#serial

[ ] ✗ PGE01032 — >IsTriggered must be #bool, found #string
{T} -T.Bad.WrongType
   (-) >IsTriggered#string
```

**Diagnostic:**
- Missing: "Trigger `-T.Bad.NoSignal` must include output `>IsTriggered#bool`"
- Wrong type: "Trigger `-T.Bad.WrongType` output `>IsTriggered` must be `#bool`, found `#string`"

### See Also

- [[PGE01031-forbidden-element-in-definition|PGE01031]] — element restrictions per definition type
- [[PGE01024-incompatible-operation-marker|PGE01024]] — `[T]` invocation must target a `{T}` pipeline
- [[concepts/pipelines/io-triggers|IO & Triggers]] — documents trigger output contract
- [[marker-declarations|Marker Declarations Brainstorming]] — §4 Rule G, valid/invalid examples
