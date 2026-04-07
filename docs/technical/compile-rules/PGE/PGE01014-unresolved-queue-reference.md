---
audience: developer
rule: "1.14"
code: PGE01014
name: Unresolved Queue Reference
severity: error
---

### Rule 1.14 — Unresolved Queue Reference
`PGE01014`

**Statement:** A pipeline's `[Q]` line must reference either the pglib `=Q.Default` or a `{Q}` definition that exists in the current package or is imported via `[@]`. Referencing a queue that does not exist is a compile error.
**Rationale:** The runtime must locate and instantiate the queue before dispatching pipelines to it. An unresolved reference means the queue cannot be created.
**Detection:** The compiler resolves the `[Q]` reference against: (1) pglib queues (`=Q.Default`), (2) `{Q}` definitions in the current file/package, (3) `{Q}` definitions from imported packages.

**VALID:**
```polyglot
[ ] ✓ pglib queue — always available
{=} =SimpleJob
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

```polyglot
[ ] ✓ user-defined queue in same file
{Q} #Queue:WorkQueue
   [.] .strategy#QueueStrategy << #FIFO

{=} =BatchJob
   [T] =T.Call
   [Q] =Q.Assign"WorkQueue"
   [W] =W.Polyglot
   [r] =DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE01014 — #Queue:GPUQueue not defined or imported
{=} =RenderJob
   [T] =T.Call
   [Q] =Q.Assign"GPUQueue"
   [W] =W.Polyglot
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=RenderJob` references queue `#Queue:GPUQueue` which is not defined in this package or imported — define it with `{Q}` or import the package that contains it"

### See Also

- [[concepts/pipelines/queue|Queue]] — documents queue reference resolution, references PGE01014
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01014
