---
audience: developer
rule: "1.5"
code: PGE01005
name: Missing Pipeline Trigger
severity: error
split_from: PGE01001
---

# Rule 1.5 — Missing Pipeline Trigger
`PGE01005`

<!-- @u:syntax/blocks -->

**Statement:** Every `{-}` pipeline must contain a `[T]` trigger section. A pipeline without a trigger has no way to start execution and is a compile error.
**Related rule:** Originally part of [[PGE01001-pipeline-execution-order|PGE01001 Pipeline Execution Order]]; split out to fire a more targeted diagnostic. See sibling rules [[PGE01006-missing-queue|PGE01006]], [[PGE01007-missing-setup-cleanup|PGE01007]].
**Rationale:** The trigger defines what signal initiates the pipeline. Without it, the pipeline cannot be invoked — it would exist as dead code with no entry point.
**Detection:** The compiler checks that every `{-}` block contains exactly one `[T]` section.

**VALID:**
```polyglot
[ ] ✓ pipeline has a trigger
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE01005 — [T] missing
{-} -Process
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -DoWork
```

**Diagnostic:** "Pipeline `-Process` has no trigger — add `[T]` before `[Q]`"

## See Also

- [[concepts/pipelines/io-triggers|IO & Triggers]] — documents mandatory trigger requirement, references PGE01005
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01005
