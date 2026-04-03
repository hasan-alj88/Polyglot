---
audience: architect
type: spec
updated: 2026-04-03
---

# Properties

<!-- @queue-manager/dispatch-coordinator -->

| Property | How Achieved |
|----------|--------------|
| **No condition evaluation** | Queue Handler only reacts to command signals — the Trigger Monitor evaluates all conditions |
| **Deterministic** | Same command + same Redis state = same outcome, always |
| **Reactive** | Every state change is triggered by a NATS signal (except the Dispatch Coordinator loop) |
| **Autonomous dispatch** | Dispatch Coordinator is an event-driven loop — wakes on queue state changes, cycles deterministically |
| **Faithful** | Queue ordering is preserved — Dispatch Coordinator never reorders a queue's strategy |
| **Fair** | Two-tier RR: Tier 1 across Dispatch Queues, Tier 2 across Coordinator Queue + Resume + Teardown — all peers |
| **Atomic** | Each signal handler runs as one Redis Lua script |
| **Testable** | Pure function — mock state, feed signal, assert output |
| **Replayable** | Record signal log → replay → exact same state transitions |
| **Optimizable** | Pre-compute, short-circuit, batch — all possible because deterministic |

---

See also: [[dispatch-coordinator]], [[precomputation]], [[design-rationale]]
