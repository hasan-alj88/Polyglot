---
audience: developer
rule: "1.1"
code: PGE01001
name: Pipeline Section Misordering
severity: error
---

# Rule 1.1 — Pipeline Section Misordering
`PGE01001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A `{-}` pipeline's sections must appear in fixed order: `(-),[T]` → `[Q]` → setup → execution body → cleanup. If all required sections are present but in the wrong order, it is a compile error. Missing sections are caught by PGE01005 (trigger), PGE01006 (queue), and PGE01007 (setup/cleanup). An empty execution body produces PGW01001.
**Rationale:** Fixed section order guarantees that setup resources are always available before the execution body runs and always released after — preventing resource leaks and undefined state regardless of concurrent instance activity.
**Detection:** After confirming all required sections are present (via PGE01005/106/107), the compiler checks that they appear in the correct order.

**VALID:**
```polyglot
[ ] ✓ Form 1 — [W] provides setup/cleanup
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] -DoWork
      (-) <result >> >result
```

```polyglot
[ ] ✓ Form 2 — explicit [\]/[/] inline
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [\]
      [-] -Setup.Init
   [-] -DoWork
      (-) <result >> >result
   [/]
      [-] -Setup.Teardown
```

**INVALID:**
```polyglot
[ ] ✗ PGE01001 — [-] appears before [Q] and setup
{-} -Process
   [T] -T.Call
   [-] -DoWork
   [Q] -Q.Default
   [W] -W.Polyglot
```

**Diagnostic:** "Pipeline `-Process` sections are misordered — expected `(-),[T]` → `[Q]` → setup → body → cleanup"

## See Also

- [[concepts/pipelines/INDEX|Pipelines Overview]] — references PGE01001 in pipeline section ordering
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01001
