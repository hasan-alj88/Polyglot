---
audience: design
rule: "2.15"
code: PGE02015
name: Unused Background Label
severity: error
type: spec
updated: 2026-04-09
---

### Rule 2.15 — Unused Background Label
`PGE02015`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** An operation label on a `[b]` background call whose outputs are never consumed by any downstream operation in the same scope is a compile error. Labels exist to be accessed; an unused label on a fire-and-forget call is dead code.
**Rationale:** `[b]` background calls are fire-and-forget by design. Adding a `($)` label implies the caller intends to reference the operation's results, but if nothing ever reads the label, the label is pointless overhead. The compiler treats this as dead code to keep pipelines intentional.
**Detection:** The compiler scans all accessor references (`$Label>param`, `$Label<param`) in the enclosing scope. If a `($)` label on a `[b]` call has zero references, the error is raised.

**VALID:**
```polyglot
[ ] ✓ background label output consumed by a later call
[b] -Metrics.Log
   (-) $Log
   (-) <event << $event
[-] -Audit.Record
   (-) <traceId << $Log>id
```

**VALID:**
```polyglot
[ ] ✓ background block with children that consume the label internally
[b] -Batch.Process
   (-) $Batch
   [-] -Report.Status
      (-) <batchRef << $Batch>handle
```

**INVALID:**
```polyglot
[ ] ✗ PGE02015 — label "$Log" on [b] is never consumed
[b] -Metrics.Log
   (-) $Log
   (-) <event << $event
[-] -Next.Step
   (-) <data << $input
```

**Diagnostic:** "Operation label `$Log` on `[b]` background call is never consumed — remove the label or use its outputs"

**Related:** PGE02009 (Unreachable Code), PGE03005 ([b] Has No Collectible Output)
