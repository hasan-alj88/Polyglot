---
audience: developer
rule: "1.6"
code: PGE01006
name: Missing Pipeline Queue
severity: error
split_from: PGE01001
---

# Rule 1.6 — Missing Pipeline Queue
`PGE01006`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->

**Statement:** Every `{-}` pipeline must contain a `[Q]` queue section. A pipeline without a queue has no concurrency strategy and is a compile error.
**Related rule:** Originally part of [[PGE01001-pipeline-execution-order|PGE01001 Pipeline Execution Order]]; split out to fire a more targeted diagnostic. See sibling rules [[PGE01005-missing-trigger|PGE01005]], [[PGE01007-missing-setup-cleanup|PGE01007]].
**Rationale:** The queue defines how concurrent invocations are handled — whether they are serialized, dropped, or run in parallel. Without it, the runtime has no policy for managing simultaneous triggers.
**Detection:** The compiler checks that every `{-}` block contains exactly one `[Q]` section.

**VALID:**
```polyglot
[ ] ✓ pipeline has a queue
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] -DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE01006 — [Q] missing
{-} -Process
   [T] -T.Call
   [W] -W.Polyglot
   [ ]
   [-] -DoWork
```

**Diagnostic:** "Pipeline `-Process` has no queue — add `[Q]` after `[T]`/`(-)` section"

## See Also

- [[concepts/pipelines/queue/INDEX|Queue]] — documents mandatory queue requirement, references PGE01006
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01006
