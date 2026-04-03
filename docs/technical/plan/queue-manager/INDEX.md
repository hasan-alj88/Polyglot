---
audience: architect
type: spec
updated: 2026-04-03
---

# Queue Manager Architecture

<!-- @concepts/pipelines/queue -->
<!-- @Q -->

The Queue Handler is responsible for dispatching triggered pipelines to execution. It manages multiple Dispatch Queues, lifecycle state transitions (pause, resume, kill), and cross-queue constraint enforcement via the Dispatch Coordinator. The Queue Handler is purely reactive — it never evaluates conditions or makes decisions. The Trigger Monitor is the decision-maker; the Queue Handler only executes commands.

## Sections

- [[infrastructure]] — External service dependencies (NATS, Redis) and storage split between Redis and NoSQL
- [[redis-containers]] — Redis data structures for Dispatch Queues, Executing/Suspended Sets, and supporting state
- [[nosql-schema]] — Queue definitions and job hierarchy schema stored in NoSQL
- [[dispatch-coordinator]] — Two-tier round-robin dispatch loop with atomic constraint checks
- [[reactive-signals]] — Signal table mapping every command to state transitions and output signals
- [[signal-payloads]] — Data payload schemas for command, state, control, runner, and collector signals
- [[nats-namespace]] — NATS subject naming conventions for all inter-service communication
- [[end-to-end-flow]] — Step-by-step flows for normal execution, pause/resume, kill, sub-jobs, and timeouts
- [[properties]] — Architectural properties: deterministic, reactive, atomic, testable, replayable
- [[precomputation]] — Constraint pre-evaluation and short-circuit optimizations
- [[sequence-diagrams]] — Mermaid sequence diagrams for dispatch, sub-jobs, kill propagation, timeouts, and pause/resume
- [[design-rationale]] — Why Redis for state, why NATS for messaging, and the storage split rationale
