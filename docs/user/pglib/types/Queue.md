---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Queue"
metadata_instance: "%#:Queue:N"
---

# #Queue Struct

<!-- @c:types -->

Queue configuration struct used in `{Q}` queue definitions. Each `{Q} #Queue:Name` creates a Dispatch Queue at metadata path `%Queue.DispatchQueue:Name`.

---

## Definition

```aljam3
{#} #Queue
   [%] .description << "Queue configuration for pipeline dispatch"
   [%] .version << "2.0.0"
   [#] ##Scalar
   [.] .strategy#QueueStrategy
   [.] .host#String
   [.] .maxInstances#UnsignedInt
   [.] .maxConcurrent#UnsignedInt
   [.] .resourceTags#array:ResourceTag
   [.] .killPropagation#KillPropagation
   [.] .maxWaitTime#String
   [.] .description#String
```

---

## Fields

| Field | Type | Purpose | Default |
|-------|------|---------|---------|
| `.strategy` | `#QueueStrategy` | FIFO, LIFO, Priority | (required) |
| `.host` | `#String` | Target host for job execution (1 queue = 1 host) | `"localhost"` |
| `.maxInstances` | `#UnsignedInt` | Max parallel instances per pipeline on this queue | unlimited |
| `.maxConcurrent` | `#UnsignedInt` | Max other pipelines alongside (queue-level default) | unlimited |
| `.resourceTags` | `#array:ResourceTag` | Resource tags for constraint checking | empty |
| `.killPropagation` | `#KillPropagation` | How kill signals propagate to sub-jobs | `#Cascade` |
| `.maxWaitTime` | `#String` | Max time in queue before escalation (e.g., "30m") | unlimited |
| `.description` | `#String` | Human-readable queue description | empty |

See [[QueueStrategy]], [[KillPropagation]], [[ResourceTag]] for enum definitions. `#RetriggerStrategy` is a queue configuration enforced by the Trigger Monitor — see [[concepts/pipelines/io-triggers#Retrigger Strategy|Triggers]].

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Queue` | Compile-time type template |
| Instance | `%#:Queue:N` | Runtime instance (N = instance number) |

---

## Related

- [[structs]] — other pglib struct types
- [[QueueStrategy]] — queue ordering strategy enum
- [[KillPropagation]] — kill signal propagation enum
- [[ResourceTag]] — resource constraint tags
- [[syntax/types/INDEX|types]] — full type system specification
