---
audience: developer
type: decision
updated: 2026-04-28
---

# Decision: Storage Architecture (Redis + PostgreSQL)

**Date:** 2026-04-28
**Status:** Decided

## Context

Aljam3 requires highly scalable storage solutions for two distinct domains:
1. **Queues & Job Orchestration**: High-throughput, sub-millisecond dispatching, tracking of active/suspended jobs, and parent-child job hierarchy for kill propagation.
2. **Metadata Data Tree (`%`)**: Central registry for all schema definitions (`%definition`), runtime variable instances (`%$`), and packages (`%@`). This tree is heavily path-based (`%-:ProcessData:0.<.filepath`) and dynamically expands at runtime.

Originally, a generic "NoSQL" database was considered for both the Job Hierarchy and Queue Definitions, while Redis was designated for Queue execution state.

## Decisions

The backend storage architecture is split between two distinct, 100% Free and Open-Source (OSI-approved) databases:

### 1. Redis (Valkey) for Queues and Job Hierarchy
Redis will be the exclusive store for all orchestration data.
- **Queue State**: Sets, counters, and ZSETs for dispatch logic.
- **Job Hierarchy**: Deep parent-child job trees are managed directly in Redis. While Redis lacks native graph capabilities, the microsecond in-memory performance offsets the cost of application-level (Rust/Lua) tree traversal.

### 2. PostgreSQL (JSONB) for the `%` Metadata Data Tree
PostgreSQL replaces the ambiguous "NoSQL" requirement. It serves as the persistent, canonical document store.
- **`%` Tree Storage**: All Aljam3 serialized objects, schemas, and live instances are saved into `JSONB` columns, leveraging Postgres's native JSON path operators.
- **Memory Safety**: Writing the massive data tree to disk entirely eliminates the RAM-exhaustion risks associated with storing millions of variable instances in an in-memory database like Redis.
- **Rust Integration**: The system will utilize `sqlx` to guarantee compile-time query verification against the Postgres schema.

## Alternatives Considered

- **ValkeyJSON (Unified Stack)**: Rejected due to the risk of server RAM exhaustion when Aljam3 processes gigabyte-sized variable instances.
- **SurrealDB**: Native graph and document store. Rejected because its BSL 1.1 license is not strictly OSI-approved Open Source, which conflicts with project philosophy.
- **MongoDB**: Elite document store. Rejected due to its restrictive SSPL license and lack of lightweight "local-first" ergonomics.

## Updated Files
- `docs/technical/plan/queue-manager/infrastructure.md`
- `docs/technical/plan/queue-manager/nosql-schema.md`
