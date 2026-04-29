---
audience: developer
rule: "1.7"
code: PGE01007
name: Missing Pipeline Setup/Cleanup
severity: error
split_from: PGE01001
---

# Rule 1.7 — Missing Pipeline Setup/Cleanup
`PGE01007`

<!-- @u:syntax/blocks -->

**Statement:** Every `{-}` pipeline must have setup and cleanup — either via a `[W]` wrapper (which provides both) or via explicit `[\]` and `[/]` sections. A pipeline with neither is a compile error.
**Related rule:** Originally part of [[PGE01001-pipeline-execution-order|PGE01001 Pipeline Execution Order]]; split out to fire a more targeted diagnostic. See sibling rules [[PGE01005-missing-trigger|PGE01005]], [[PGE01006-missing-queue|PGE01006]].
**Rationale:** Setup/cleanup ensures resources are acquired before the execution body runs and released after — preventing resource leaks and undefined state. Even pipelines that need no resources must declare `[W] -W.Polyglot` (which calls `-DoNothing` for both).
**Detection:** The compiler checks that every `{-}` block contains either `[W]` or both `[\]` and `[/]`.

**VALID:**
```polyglot
[ ] ✓ Form 1 — [W] provides setup/cleanup
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] -DoWork
```

```polyglot
[ ] ✓ Form 2 — explicit [\]/[/] inline
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [\]
      [-] -Setup.Init
   [-] -DoWork
   [/]
      [-] -Setup.Teardown
```

**INVALID:**
```polyglot
[ ] ✗ PGE01007 — no setup/cleanup (no [W] and no [\]/[/])
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [-] -DoWork
```

**Diagnostic:** "Pipeline `-Process` has no setup/cleanup — add `[W]` or explicit `[\]/[/]`"

## See Also

- [[concepts/pipelines/wrappers|Wrappers]] — documents mandatory setup/cleanup requirement, references PGE01007
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01007
