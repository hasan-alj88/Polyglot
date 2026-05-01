---
audience: developer
rule: "1.14"
code: PGE01014
name: Unresolved Queue Reference
severity: error
---

# Rule 1.14 — Unresolved Queue Reference
`PGE01014`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A pipeline's `[Q]` line must reference either the jm3lib `-Q.Default` or a `{Q}` definition that exists in the current package or is imported via `[@]`. Referencing a queue that does not exist is a compile error.
**Rationale:** The runtime must locate and instantiate the queue before dispatching pipelines to it. An unresolved reference means the queue cannot be created.
**Detection:** The compiler resolves the `[Q]` reference against: (1) jm3lib queues (`-Q.Default`), (2) `{Q}` definitions in the current file/package, (3) `{Q}` definitions from imported packages.

**VALID:**
```aljam3
[ ] ✓ jm3lib queue — always available
{-} -SimpleJob
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

```aljam3
[ ] ✓ user-defined queue in same file
{Q} #Queue:WorkQueue
   [.] .strategy#QueueStrategy << #FIFO

{-} -BatchJob
   [T] -T.Call
   [Q] -Q.Assign"WorkQueue"
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

**INVALID:**
```aljam3
[ ] ✗ PGE01014 — #Queue:GPUQueue not defined or imported
{-} -RenderJob
   [T] -T.Call
   [Q] -Q.Assign"GPUQueue"
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

**Diagnostic:** "Pipeline `-RenderJob` references queue `#Queue:GPUQueue` which is not defined in this package or imported — define it with `{Q}` or import the package that contains it"

## See Also

- [[concepts/pipelines/queue/INDEX|Queue]] — documents queue reference resolution, references PGE01014
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01014
