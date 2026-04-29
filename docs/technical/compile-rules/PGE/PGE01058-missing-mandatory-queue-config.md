---
audience: developer
rule: "1.58"
code: PGE01058
name: Missing Mandatory Queue Config
severity: error
---

# Rule 1.58 — Missing Mandatory Queue Config
`PGE01058`

**Statement:** Every pipeline block `{-}` must explicitly declare a Queue configuration block `[Q]`. If standard execution behavior is desired, `[Q] -Q.Default` must be explicitly specified.
**Rationale:** Concurrency and backpressure are first-class concepts in Polyglot. Implicit queues hide critical architectural behavior. By requiring an explicit `[Q]`, the developer is forced to acknowledge the pipeline's operational execution limits and concurrency constraints.
**Detection:** During pipeline validation, the compiler confirms that a `[Q]` marker exists within the block before execution starts. If it is missing entirely, the pipeline fails to compile.

**VALID:**
```polyglot
[ ] ✓ Queue is explicitly defined
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run
```

**INVALID:**
```polyglot
[ ] ✗ PGE01058 — Missing Queue block
{-} -ProcessData
   [T] -T.Manual
   [W] -W.Polyglot
   [-] >run                                  [ ] ✗ PGE01058 — Missing [Q] block
```

**Diagnostic:** "Pipeline lacks a mandatory Queue Configuration `[Q]` block. All pipelines must define a Queue Config `[Q]`. To use standard behavior, specify `[Q] -Q.Default`."
