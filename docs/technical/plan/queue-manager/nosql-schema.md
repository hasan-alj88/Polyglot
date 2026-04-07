---
audience: architect
type: spec
updated: 2026-04-03
---

# NoSQL Schema (definitions)

<!-- @queue-manager/infrastructure -->

## Queue Definition (from `{Q} #Queue:Name`)

```polyglot
Stored at: %Queue.DispatchQueue:Name

strategy:           #QueueStrategy     — FIFO | LIFO | Priority
host:               #String            — target host (default: "localhost")
maxInstancesWithinQueue: int           — queue-level instance limit
maxConcurrentWithinQueue: int          — queue-level concurrency limit
resourceTagWithinQueue: #array:ResourceTag — queue-level resource tags
killPropagation:    #KillPropagation   — Cascade | Downgrade
maxWaitTime:        string             — max queue wait time
description:        string             — human-readable
controls:           [Q] nested         — =Q.Pause.*, =Q.Resume.*, =Q.Kill.*
```

## Job Hierarchy (runtime tree)

```polyglot
Stored as: job tree rooted at pipeline instance

{jobId}
├── pipeline:       string             — pipeline name
├── parentJobId:    string?            — parent (null for root)
├── children:       [jobId, ...]       — sub-jobs
├── marker:         string             — [r], [p], [b], [?]
└── status:         #QueueState        — current state
```

Only the Trigger Monitor reads the hierarchy — to pre-compute kill lists and manage collector logic (`*First`/`*Nth`/`*All`). The Queue Handler never queries NoSQL.

---

See also: [[infrastructure]], [[redis-containers]], [[end-to-end-flow]]
