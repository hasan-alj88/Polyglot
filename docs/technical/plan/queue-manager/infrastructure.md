---
audience: architect
type: spec
updated: 2026-04-03
---

# Infrastructure

<!-- @queue-manager/INDEX -->

The Queue Handler runs as a Rust service backed by two external services:

| Service | Role | License |
|---------|------|---------|
| **NATS JetStream** | Messaging — signals, events, inter-service communication | Apache 2.0 |
| **Redis / Valkey** | State — queue ordering, counters, sets, atomic dispatch | Valkey: BSD 3-Clause |

NATS handles communication. Redis handles runtime state. **NoSQL** stores queue definitions and job hierarchy — but the Queue Handler never queries NoSQL directly. The Trigger Monitor reads from NoSQL and relays needed data to the Queue Handler via signal payloads. The Queue Handler logic (Dispatch Coordinator) runs in Rust.

## Storage Split

| Data | Store | Reason |
|------|-------|--------|
| Queue ordering (LIST/ZSET), Executing/Suspended Sets | Redis | Fast atomic operations, microsecond latency |
| Job runtime state (status, timestamps, pid) | Redis | Frequently read/written by dispatch loop |
| Queue config cache (constraints, strategy) | Redis | Loaded at queue registration via TM signal, updated via `command.queue.update` |
| Queue definitions (`{Q}` schema fields) | NoSQL | Immutable at runtime, read by Trigger Monitor only |
| Job hierarchy (parent→children) | NoSQL | Only read by Trigger Monitor for kill propagation |

## Host-Based Dispatch

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Handler routes dispatch signals via NATS to the Runner on the target host. Offloading work to another host means switching queues (via `=Q.Reassign` or `=Q.Dispatch.Wait.TimeOut.Reassign`).

---

See also: [[redis-containers]], [[nosql-schema]], [[design-rationale]]
