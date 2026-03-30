---
rule: "1.7"
code: PGE-107
name: Missing Pipeline Setup/Cleanup
severity: error
split_from: PGE-101
---

### Rule 1.7 — Missing Pipeline Setup/Cleanup
`PGE-107`

**Statement:** Every `{=}` pipeline must have setup and cleanup — either via a `[W]` wrapper (which provides both) or via explicit `[\]` and `[/]` sections. A pipeline with neither is a compile error.
**Rationale:** Setup/cleanup ensures resources are acquired before the execution body runs and released after — preventing resource leaks and undefined state. Even pipelines that need no resources must declare `[W] =W.Polyglot` (which calls `=DoNothing` for both).
**Detection:** The compiler checks that every `{=}` block contains either `[W]` or both `[\]` and `[/]`.

**VALID:**
```polyglot
[ ] ✓ Form 1 — [W] provides setup/cleanup
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

```polyglot
[ ] ✓ Form 2 — explicit [\]/[/] inline
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [\]
      [r] =Setup.Init
   [r] =DoWork
   [/]
      [r] =Setup.Teardown
```

**INVALID:**
```polyglot
[ ] ✗ PGE-107 — no setup/cleanup (no [W] and no [\]/[/])
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=Process` has no setup/cleanup — add `[W]` or explicit `[\]/[/]`"

### See Also

- [[concepts/pipelines/wrappers|Wrappers]] — documents mandatory setup/cleanup requirement, references PGE-107
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE-107
