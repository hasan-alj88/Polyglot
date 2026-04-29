---
audience: developer
rule: "1.6"
code: PGE01006
name: Mandatory Setup/Cleanup
severity: error
---

# Rule 1.6 — Mandatory Setup/Cleanup
`PGE01006`

**Statement:** Every pipeline block `{-}` must explicitly declare its environmental integration. This can be done either by providing a Wrapper configuration block `[W]` (e.g., `[W] -W.Polyglot`) OR by explicitly providing an inline Setup block `[\]` and Teardown block `[/]`.
**Rationale:** Execution environments often require resource setup, telemetry, or security contexts. Enforcing explicit declarations guarantees that the environment behavior is never hidden behind magic defaults.
**Detection:** During pipeline validation, the compiler confirms that either a `[W]` marker exists, OR that a `[\]` and `[/]` pair exist. If neither is found, the pipeline fails to compile.

**VALID:**
```polyglot
[ ] ✓ Wrapper is explicitly defined
{-} -ProcessData
   (-) <#None
   (-) >#None
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run
```

```polyglot
[ ] ✓ Inline Setup and Cleanup are explicitly defined
{-} -ProcessData
   (-) <#None
   (-) >#None
   [T] -T.Manual
   [Q] -Q.Default
   [\]
      [-] -Setup.Init
   [-] >run
   [/]
      [-] -Setup.Teardown
```

**INVALID:**
```polyglot
[ ] ✗ PGE01006 — Missing Wrapper or Setup/Cleanup
{-} -ProcessData
   (-) <#None
   (-) >#None
   [T] -T.Manual
   [Q] -Q.Default
   [-] >run                                  [ ] ✗ PGE01006 — Missing [W] or [\]/[/] pair
```

**Diagnostic:** "Pipeline lacks environmental integration. You must define either a Wrapper `[W]` (e.g., `[W] -W.Polyglot`) or explicitly provide an inline Setup `[\]` and Teardown `[/]` block."
