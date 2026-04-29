---
audience: developer
rule: "1.55"
code: PGE01055
name: IO Marker Context Mismatch
severity: error
---

# Rule 1.55 — IO Marker Context Mismatch
`PGE01055`

**Statement:** An IO marker must strictly match the contextual block's operator prefix. Specifically, Data contexts `{#}` require `(#)`, Collector contexts `{*}` require `(*)`, Pipelines `{-}` require `(-)`, and Expanders `{\=}` require `(=)`. 
**Rationale:** Polyglot syntax is designed to be highly readable. Using an IO marker that belongs to a different block type (e.g., using a Pipeline IO marker inside a Data block) creates syntactic dissonance and breaks the language's strict visual parsing rules.
**Detection:** The compiler checks the prefix of the current block context (e.g. `{-}`, `{#}`, `{*}`) against the IO marker being parsed. If a mismatch is found, it raises an error.

**VALID:**
```polyglot
[ ] ✓ Using (-) in a Pipeline
{-} -Process
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
```

```polyglot
[ ] ✓ Using (#) in a Data block
{#} #User
   (#) .id#string
```

**INVALID:**
```polyglot
[ ] ✗ PGE01055 — (-) used in a Data block
{#} #User
   (-) .id#string                            [ ] ✗ PGE01055 — should be (#)
```

```polyglot
[ ] ✗ PGE01055 — (#) used in a Pipeline block
{-} -Process
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (#) <input#string                         [ ] ✗ PGE01055 — should be (-)
```

**Diagnostic:** "Pipeline IO marker used inside a Data context. Data contexts (like `{#}`) require Data IO markers like `(#)`."
