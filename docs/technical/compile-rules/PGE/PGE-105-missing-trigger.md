---
rule: "1.5"
code: PGE-105
name: Missing Pipeline Trigger
severity: error
split_from: PGE-101
---

### Rule 1.5 — Missing Pipeline Trigger
`PGE-105`

**Statement:** Every `{=}` pipeline must contain a `[t]` trigger section. A pipeline without a trigger has no way to start execution and is a compile error.
**Rationale:** The trigger defines what event initiates the pipeline. Without it, the pipeline cannot be invoked — it would exist as dead code with no entry point.
**Detection:** The compiler checks that every `{=}` block contains exactly one `[t]` section.

**VALID:**
```polyglot
[ ] ✓ pipeline has a trigger
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE-105 — [t] missing
{=} =Process
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=Process` has no trigger — add `[t]` before `[Q]`"

### See Also

- [[concepts/pipelines/io-triggers|IO & Triggers]] — documents mandatory trigger requirement, references PGE-105
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE-105
