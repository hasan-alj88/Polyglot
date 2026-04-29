---
audience: developer
rule: "1.4"
code: PGE01004
name: Mandatory Trigger
severity: error
---

# Rule 1.4 — Mandatory Trigger
`PGE01004`

**Statement:** Every pipeline block `{-}` must explicitly declare a Trigger configuration block `[T]`. If the pipeline should not be automatically triggered, `[T] -T.Manual` must be explicitly specified.
**Rationale:** Aljam3 embraces explicit operational semantics. Omitting the trigger configuration makes it ambiguous how and when a pipeline is executed. By forcing the developer to declare `[T]`, the entry points and execution circumstances of the application remain unambiguous.
**Detection:** During pipeline validation, the compiler confirms that a `[T]` marker exists within the block before execution starts. If it is missing entirely, the pipeline fails to compile.

**VALID:**
```aljam3
[ ] ✓ Trigger is explicitly defined
{-} -ProcessData
   (-) <#None
   (-) >#None
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] >run
```

**INVALID:**
```aljam3
[ ] ✗ PGE01004 — Missing Trigger block
{-} -ProcessData
   (-) <#None
   (-) >#None
   [Q] -Q.Default                            [ ] ✗ PGE01004 — Missing [T] block
   [W] -W.Aljam3
   [-] >run
```

**Diagnostic:** "Pipeline lacks a mandatory Trigger `[T]` block. All pipelines must define a Trigger `[T]`. To disable automatic triggering, specify `[T] -T.Manual`."
