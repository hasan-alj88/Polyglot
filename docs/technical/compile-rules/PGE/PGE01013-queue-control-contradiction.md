> [!WARNING]
> **DEPRECATED:** This general rule has been deprecated and split into granular queue constraint rules (PGE01064 - PGE01070).

---
audience: developer
rule: "1.13"
code: PGE01013
name: Queue Control Contradicts Queue Default
severity: error
---

# Rule 1.13 — Queue Control Contradicts Queue Default
`PGE01013`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A pipeline's `[Q]` section must not contain IO parameters or active controls that contradict the defaults set in its queue's `{Q}` definition.
**Rationale:** `{Q}` defines queue-level defaults that apply to all pipelines on that queue. If a pipeline overrides these with contradictory values, it creates ambiguous runtime behavior. The compiler catches these contradictions statically.
**Detection:** The compiler compares each `[Q]` IO parameter and nested `[Q]` control against the referenced `{Q}` definition. A contradiction is any parameter that directly conflicts with a queue-level default.

**VALID:**
```aljam3
{Q} #Queue:BatchQueue
   [.] .strategy#QueueStrategy << #FIFO
   [.] .maxInstances#int << 5

[ ] ✓ pipeline adds controls not in {Q} — no contradiction
{-} -BatchJob
   [T] -T.Call
   [Q] -Q.Assign"BatchQueue"
      (-) <maxConcurrent#int << 10
      [Q] -Q.Pause.Soft
         (-) <CPU.MoreThan#float << 90.0
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

**INVALID:**
```aljam3
{Q} #Queue:BatchQueue
   [.] .maxInstances#int << 1

[ ] ✗ PGE01013 — maxInstances contradicts queue default
{-} -BatchJob
   [T] -T.Call
   [Q] -Q.Assign"BatchQueue"
      (-) <maxInstances#int << 5
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

**Diagnostic:** "Pipeline `-BatchJob` sets `maxInstances << 5` but queue `#Queue:BatchQueue` defines `maxInstances << 1` — remove the pipeline override or change the queue default"

## See Also

- [[concepts/pipelines/queue/INDEX|Queue]] — documents queue control contradiction rule, references PGE01013
- [[pglib/pipelines/Q|-Q.* pglib Pipelines]] — references PGE01013 in queue control context
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01013
