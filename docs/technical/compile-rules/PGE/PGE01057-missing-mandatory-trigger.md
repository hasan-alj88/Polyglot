---
audience: developer
rule: "1.57"
code: PGE01057
name: Missing Mandatory Trigger
severity: error
---

# Rule 1.57 — Missing Mandatory Trigger
`PGE01057`

**Statement:** Every pipeline block `{-}` must explicitly declare a Trigger configuration block `[T]`. If the pipeline should not be automatically triggered, `[T] -T.Manual` must be explicitly specified.
**Rationale:** Polyglot embraces explicit operational semantics. Omitting the trigger configuration makes it ambiguous how and when a pipeline is executed. By forcing the developer to declare `[T]`, the entry points and execution circumstances of the application remain unambiguous.
**Detection:** During pipeline validation, the compiler confirms that a `[T]` marker exists within the block before execution starts. If it is missing entirely, the pipeline fails to compile.

**VALID:**
```polyglot
[ ] ✓ Trigger is explicitly defined
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run
```

**INVALID:**
```polyglot
[ ] ✗ PGE01057 — Missing Trigger block
{-} -ProcessData
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run                                  [ ] ✗ PGE01057 — Missing [T] block
```

**Diagnostic:** "Pipeline lacks a mandatory Trigger `[T]` block. All pipelines must define a Trigger `[T]`. To disable automatic triggering, specify `[T] -T.Manual`."
