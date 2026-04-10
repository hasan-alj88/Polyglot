---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Job"
metadata_instance: "%#:Job:N"
---

# #Job Struct

<!-- @types -->

Runtime job state within a pipeline instance. Jobs are units of work created at IO boundaries — the compiler determines job start and end via pipeline IO state (inputs must be Final to start; outputs Final or Failed to complete). Sequential `[-]` jobs chain on predecessor completion; parallel `[=]` jobs fork. The Trigger Monitor generates UIDs for new jobs when triggered.

Job state is stored at `%-:Pipeline:N.jobs:UID` in the metadata tree. See [[glossary]] for the distinction between Job and Instance.

---

## Definition

```polyglot
{#} #Job
   [%] .description << "Runtime job state within a pipeline instance"
   [%] .version << "1.0.0"
   [#] ##Flat
   [.] .PID#String
   [.] .status#QueueState
   [.] .pipeline#String
   [.] .queue#String
   [.] .parent#String
   [.] .hasChildren#Boolean
   [.] .hierarchy#String
   [.] .marker#String
   [.] .killPropagation#KillPropagation
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.PID` | `#String` | OS process ID — needed for SIGTERM, SIGKILL, SIGSTOP, SIGCONT |
| `.status` | `#QueueState` | Current lifecycle state (Pending, Executing, Completed, etc.) |
| `.pipeline` | `#String` | Pipeline name this job belongs to |
| `.queue` | `#String` | Dispatch Queue this job was assigned to |
| `.parent` | `#String` | Parent job ID (empty for root job) |
| `.hasChildren` | `#Boolean` | Whether this job has spawned sub-jobs |
| `.hierarchy` | `#String` | Dot-separated path in job tree (e.g., `ProcessData/job1/job3`) |
| `.marker` | `#String` | Block element that created this job: `[-]`, `[=]`, `[b]`, `[?]` |
| `.killPropagation` | `#KillPropagation` | How kill signals propagate to sub-jobs |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Job` | Compile-time type template |
| Instance | `%#:Job:N` | Runtime instance (N = instance number) |

---

## Related

- [[structs]] — other pglib struct types
- [[QueueState]] — job lifecycle state enum
- [[KillPropagation]] — kill signal propagation enum
- [[glossary]] — Job vs Instance distinction
- [[syntax/types/INDEX|types]] — full type system specification
