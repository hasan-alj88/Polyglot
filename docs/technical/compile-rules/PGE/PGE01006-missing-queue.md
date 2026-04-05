---
audience: developer
rule: "1.6"
code: PGE01006
name: Missing Pipeline Queue
severity: error
split_from: PGE01001
---

### Rule 1.6 — Missing Pipeline Queue
`PGE01006`

**Statement:** Every `{=}` pipeline must contain a `[Q]` queue section. A pipeline without a queue has no concurrency strategy and is a compile error.
**Rationale:** The queue defines how concurrent invocations are handled — whether they are serialized, dropped, or run in parallel. Without it, the runtime has no policy for managing simultaneous triggers.
**Detection:** The compiler checks that every `{=}` block contains exactly one `[Q]` section.

**VALID:**
```polyglot
[ ] ✓ pipeline has a queue
{=} =Process
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE01006 — [Q] missing
{=} =Process
   [T] =T.Call
   [W] =W.Polyglot
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=Process` has no queue — add `[Q]` after `[T]`/`[=]` section"

### See Also

- [[concepts/pipelines/queue|Queue]] — documents mandatory queue requirement, references PGE01006
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01006
