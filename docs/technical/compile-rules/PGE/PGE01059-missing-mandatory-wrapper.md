---
audience: developer
rule: "1.59"
code: PGE01059
name: Missing Mandatory Wrapper
severity: error
---

# Rule 1.59 — Missing Mandatory Wrapper
`PGE01059`

**Statement:** Every pipeline block `{-}` must explicitly declare a Wrapper configuration block `[W]`. If no custom transaction/telemetry wrapping is required, `[W] -W.Polyglot` (or previously `-W.RT`) must be specified.
**Rationale:** Wrappers dictate how the pipeline integrates with the runtime environment (telemetry, security, execution context). Making this explicit guarantees that environmental behavior is never hidden behind magic defaults.
**Detection:** During pipeline validation, the compiler confirms that a `[W]` marker exists within the block before execution starts. If it is missing entirely, the pipeline fails to compile.

**VALID:**
```polyglot
[ ] ✓ Wrapper is explicitly defined
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run
```

**INVALID:**
```polyglot
[ ] ✗ PGE01059 — Missing Wrapper block
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [-] >run                                  [ ] ✗ PGE01059 — Missing [W] block
```

**Diagnostic:** "Pipeline lacks a mandatory Wrapper `[W]` block. All pipelines must define a Wrapper `[W]`. To use the default setup/cleanup, specify `[W] -W.Polyglot`."
