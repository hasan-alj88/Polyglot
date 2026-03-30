---
rule: "1.1"
code: PGE-101
name: Pipeline Section Misordering
severity: error
---

### Rule 1.1 — Pipeline Section Misordering
`PGE-101`

**Statement:** A `{=}` pipeline's sections must appear in fixed order: `[t],[=]` → `[Q]` → setup → execution body → cleanup. If all required sections are present but in the wrong order, it is a compile error. Missing sections are caught by PGE-105 (trigger), PGE-106 (queue), and PGE-107 (setup/cleanup). An empty execution body produces PGW-101.
**Rationale:** Fixed section order guarantees that setup resources are always available before the execution body runs and always released after — preventing resource leaks and undefined state regardless of concurrent instance activity.
**Detection:** After confirming all required sections are present (via PGE-105/106/107), the compiler checks that they appear in the correct order.

**VALID:**
```polyglot
[ ] ✓ Form 1 — [W] provides setup/cleanup
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
      [=] <result >> >result
```

```polyglot
[ ] ✓ Form 2 — explicit [\]/[/] inline
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [\]
      [r] =Setup.Init
   [r] =DoWork
      [=] <result >> >result
   [/]
      [r] =Setup.Teardown
```

**INVALID:**
```polyglot
[ ] ✗ PGE-101 — [r] appears before [Q] and setup
{=} =Process
   [t] =T.Call
   [r] =DoWork
   [Q] =Q.Default
   [W] =W.Polyglot
```

**Diagnostic:** "Pipeline `=Process` sections are misordered — expected `[t],[=]` → `[Q]` → setup → body → cleanup"

### See Also

- [[concepts/pipelines/INDEX|Pipelines Overview]] — references PGE-101 in pipeline section ordering
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE-101
