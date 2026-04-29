---
audience: design
type: spec
updated: 2026-04-03
---

# Database Schema (definitions)

<!-- @c:queue-manager/infrastructure -->

## Queue Definition (Stored in PostgreSQL JSONB)

```aljam3
Stored at: %Queue.DispatchQueue:Name

strategy:           #QueueStrategy     — FIFO | LIFO | Priority
host:               #String            — target host (default: "localhost")
maxInstancesWithinQueue: int           — queue-level instance limit
maxConcurrentWithinQueue: int          — queue-level concurrency limit
resourceTagWithinQueue: #array:ResourceTag — queue-level resource tags
killPropagation:    #KillPropagation   — Cascade | Downgrade
maxWaitTime:        string             — max queue wait time
description:        string             — human-readable
controls:           [Q] nested         — -Q.Pause.*, -Q.Resume.*, -Q.Kill.*
```

## Job Hierarchy (Stored in Redis)

```aljam3
Stored as: job tree rooted at pipeline instance

{jobId}
├── pipeline:       string             — pipeline name
├── parentJobId:    string?            — parent (null for root)
├── children:       [jobId, ...]       — sub-jobs
├── marker:         string             — [-], [=], [b], [?]
└── status:         #QueueState        — current state
```

Only the Trigger Monitor reads the hierarchy — to pre-compute kill lists and manage collector logic (`*First`/`*Nth`/`*All`). The Queue Handler never queries PostgreSQL.

---

See also: [[infrastructure]], [[redis-containers]], [[end-to-end-flow]]
