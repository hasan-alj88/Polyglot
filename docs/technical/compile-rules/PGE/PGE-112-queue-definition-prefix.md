---
rule: "1.12"
code: PGE-112
name: Queue Definition Must Use #Queue: Prefix
severity: error
---

### Rule 1.12 — Queue Definition Must Use #Queue: Prefix
`PGE-112`

**Statement:** Every `{Q}` queue definition must use the `#Queue:` prefix in its identifier. This distinguishes queue definitions from data definitions (`{#}`) and ensures the runtime can identify and instantiate queues.
**Rationale:** `{Q}` both defines a data struct and instantiates a runtime queue. The `#Queue:` prefix signals to the compiler that this is a queue instantiation, not a regular struct. Without it, the runtime cannot locate and create the queue.
**Detection:** The compiler checks that the identifier following `{Q}` matches the pattern `#Queue:<name>`.

**VALID:**
```polyglot
[ ] ✓ correct prefix
{Q} #Queue:GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .maxInstances#int << 1
```

**INVALID:**
```polyglot
[ ] ✗ PGE-112 — missing #Queue: prefix
{Q} #GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
```

```polyglot
[ ] ✗ PGE-112 — wrong prefix
{Q} =Q.MyQueue
   [.] .strategy;#QueueStrategy << #LIFO
```

**Diagnostic:** "Queue definition `{Q}` must use `#Queue:` prefix — got `#GPUQueue`, expected `#Queue:GPUQueue`"
