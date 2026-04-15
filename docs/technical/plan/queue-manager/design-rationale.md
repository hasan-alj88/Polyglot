---
audience: architect
type: spec
updated: 2026-04-15
---

# Design Rationale

<!-- @c:queue-manager/infrastructure -->

## Why Redis for State

- **Atomic Lua scripts** — Dispatch Coordinator constraint checks and state mutations execute as one atomic operation. No race conditions.
- **Data structure fit** — LIST for FIFO/LIFO, SORTED SET for Priority, SET for Executing, HASH for Suspended and metadata.
- **Dynamic priority** — `ZADD` / `ZINCRBY` allow real-time reprioritization triggered by events.
- **Lightweight** — single binary, in-memory, microsecond latency.

## Why NATS for Messaging

- **Decoupled services** — Trigger Monitor, Queue Handler, Runner, Resource Monitor communicate only via NATS subjects.
- **Subject-based routing** — `polyglot.queue.control.{jobId}.job.pause.free.cpu.wait` routes precisely.
- **JetStream persistence** — critical events can be persisted for crash recovery and replay.
- **Embeddable** — NATS server can embed in the Polyglot runtime process.
- **Lightweight** — ~20MB, no external dependencies.

## Why This Split

| Concern | Wrong Service | Right Service |
|---------|--------------|---------------|
| Queue ordering | NATS (no ordering primitives) | Redis (LIST, ZSET) |
| Atomic constraint checks | NATS (no transactions) | Redis (Lua scripts) |
| Event signals | Redis (pub/sub is fire-and-forget) | NATS (persistent, routable) |
| Cross-service communication | Redis (not designed for this) | NATS (designed for this) |

Redis handles state and ordering. NATS handles communication. Neither duplicates the other's work.

---

See also: [[infrastructure]], [[properties]], [[dispatch-coordinator]]
