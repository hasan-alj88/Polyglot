---
audience: developer
rule: "1.5"
code: PGE01005
name: Mandatory Queue Config
severity: error
---

# Rule 1.5 — Mandatory Queue Config
`PGE01005`

**Statement:** Every pipeline block `{-}` must explicitly declare a Queue configuration block `[Q]`. If standard execution behavior is desired, `[Q] -Q.Default` must be explicitly specified.
**Rationale:** Concurrency and backpressure are first-class concepts in Aljam3. Implicit queues hide critical architectural behavior. By requiring an explicit `[Q]`, the developer is forced to acknowledge the pipeline's operational execution limits and concurrency constraints.
**Detection:** During pipeline validation, the compiler confirms that a `[Q]` marker exists within the block before execution starts. If it is missing entirely, the pipeline fails to compile.

**VALID:**
```aljam3
[ ] ✓ Queue is explicitly defined
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
[ ] ✗ PGE01005 — Missing Queue block
{-} -ProcessData
   (-) <#None
   (-) >#None
   [T] -T.Manual
   [W] -W.Aljam3                            [ ] ✗ PGE01005 — Missing [Q] block
   [-] >run
```

**Diagnostic:** "Pipeline lacks a mandatory Queue Configuration `[Q]` block. All pipelines must define a Queue Config `[Q]`. To use standard behavior, specify `[Q] -Q.Default`."
