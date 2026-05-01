---
audience: design
type: spec
updated: 2026-04-03
---

# Infrastructure

<!-- @queue-manager/INDEX -->

The Queue Handler runs as a Rust service backed by two external services:

| Service | Role | License |
|---------|------|---------|
| **NATS JetStream** | Messaging — signals, events, inter-service communication | Apache 2.0 |
| **Redis / Valkey** | State — queue ordering, job hierarchy, active state | Valkey: BSD 3-Clause |
| **PostgreSQL** | Storage — `%` Data Tree (JSONB), definitions, packages | PostgreSQL License |

NATS handles communication. Redis handles runtime state and job hierarchy. **PostgreSQL** stores the massive `%` Metadata Data Tree (including queue definitions) using `JSONB` — but the Queue Handler never queries PostgreSQL directly. The Trigger Monitor reads from PostgreSQL and relays needed data to the Queue Handler via signal payloads. The Queue Handler logic (Dispatch Coordinator) runs in Rust.

## Storage Split

| Data | Store | Reason |
|------|-------|--------|
| Queue ordering (LIST/ZSET), Running/Suspended Sets | Redis | Fast atomic operations, microsecond latency |
| Job runtime state (status, timestamps, pid) | Redis | Frequently read/written by dispatch loop |
| Job hierarchy (parent→children) | Redis | Deep tree traversal and state propagation |
| Queue config cache (constraints, strategy) | Redis | Loaded at queue registration via TM signal |
| `%` Data Tree (Variables, Instances, Packages) | PostgreSQL (JSONB) | Scalable disk-backed storage, avoids RAM exhaustion |
| Queue definitions (`{Q}` schema fields) | PostgreSQL (JSONB) | Immutable at runtime, read by Trigger Monitor only |

## Host-Based Dispatch

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Handler routes dispatch signals via NATS to the Runner on the target host. Offloading work to another host means switching queues (via `=Q.Reassign` or `=Q.Dispatch.Wait.TimeOut.Reassign`).

---

See also: [[redis-containers]], [[nosql-schema]], [[design-rationale]]
