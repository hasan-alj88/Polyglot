---
audience: developer
rule: "1.3"
code: PGE01003
name: Mandatory IO
severity: error
---

# Rule 1.3 — Mandatory IO
`PGE01003`

**Statement:** Every pipeline `{-}` must explicitly declare its input and output contract before defining its trigger, queue, or setup. Even if a pipeline processes no input or returns no output, it must explicitly declare `(-) <#None` and/or `(-) >#None`. 
**Rationale:** Aljam3 requires explicit operational semantics to prevent accidental omission of data flow definitions. By forcing an explicit statement of IO (even when there is none), the compiler ensures the developer consciously designed the pipeline's boundaries, making the code self-documenting and eliminating ambiguity.
**Detection:** The compiler checks the top of the pipeline block for IO markers (`(-)` or directional markers). If it reaches a configuration block (like `[T]`) or an execution block without encountering an input and output declaration, `PGE01003` is raised.

**VALID:**
```aljam3
[ ] ✓ Explicit IO defined
{-} -ProcessData
   (-) <data#string
   (-) >status#int
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] >run
```

```aljam3
[ ] ✓ Explicitly declaring NO IO
{-} -PingService
   (-) <#None
   (-) >#None
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] -Do.Ping
```

**INVALID:**
```aljam3
[ ] ✗ PGE01003 — Missing IO declarations
{-} -PingService
   [T] -T.CLI                           [ ] ✗ PGE01003 — Reached [T] without seeing IO declarations
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] -Do.Ping
```

**Diagnostic:** "Pipeline lacks mandatory IO declarations. You must declare inputs and outputs before the `[T]` trigger. If the pipeline has no input, use `(-) <#None`. If it has no output, use `(-) >#None`."
