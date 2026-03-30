---
rule: "1.6"
code: PGE-106
name: Missing Pipeline Queue
severity: error
split_from: PGE-101
---

### Rule 1.6 — Missing Pipeline Queue
`PGE-106`

**Statement:** Every `{=}` pipeline must contain a `[Q]` queue section. A pipeline without a queue has no concurrency strategy and is a compile error.
**Rationale:** The queue defines how concurrent invocations are handled — whether they are serialized, dropped, or run in parallel. Without it, the runtime has no policy for managing simultaneous triggers.
**Detection:** The compiler checks that every `{=}` block contains exactly one `[Q]` section.

**VALID:**
```polyglot
[ ] ✓ pipeline has a queue
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE-106 — [Q] missing
{=} =Process
   [t] =T.Call
   [W] =W.Polyglot
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=Process` has no queue — add `[Q]` after `[t]`/`[=]` section"

### See Also

- [[concepts/pipelines/queue|Queue]] — documents mandatory queue requirement, references PGE-106
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE-106
