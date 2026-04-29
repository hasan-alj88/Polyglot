---
audience: automation-builder
type: specification
updated: 2026-04-26
---

<!-- @c:concepts/pipelines/queue/INDEX -->
<!-- @u:technical/plan/queue-manager/INDEX -->
<!-- @u:technical/ebnf/09-definition-blocks#9.6 -->

# Queue Configuration

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE01006). Aljam3 uses a multi-queue execution model with a unified dispatch layer:

- **Dispatch Queues** — one per `{Q} #Queue:Name` definition (plus `-Q.Default`). Each maintains its own ordering strategy (FIFO, LIFO, Priority). Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration.
- **Dispatch Coordinator** — the unified dispatch layer. Reads from all parallel Dispatch Queues simultaneously and dispatches Jobs to the Executing Set. The Dispatch Coordinator is faithful to every Dispatch Queue's ordering and concurrency rules — it never overrides or reorders a queue's internal strategy.

## Three `{Q}` Block Types

`{Q}` is a **multi-purpose block** — the identifier prefix disambiguates its role:

| Block | Purpose | Actions Allowed | Getters Allowed |
|-------|---------|-----------------|-----------------|
| `{Q} #Queue:Name` | Queue configuration (subtype of `{#}`) | None | None |
| `{Q} #QueueRules:Name` | Queue-level policy rules | `-Q.Queue.*` only | `-Q.Queue.*`, `-Q.Host.*`, `-Q.Job.*` (array) |
| `{Q} #JobRules:Name` | Job-level policy rules | `-Q.Job.*` only | `-Q.Job.*`, `-Q.Queue.*`, `-Q.Host.*` |

**Scope rule:** You can read anything, you can only act on your scope.

In `#QueueRules`, Job getters return an **array** of all Jobs on the queue. In `#JobRules`, queue/host getters return single values for the current context.

## Defining a Queue (`{Q} #Queue:Name`)

Custom queues define configuration and load rule sets. The identifier must use the `#Queue:` prefix (PGE01012).

```aljam3
{Q} #Queue:WorkerQueue
   [.] .strategy#QueueStrategy << #FIFO
   [.] .host#URL << "worker-host-01"
   [.] .pollPeriod << #DT"5s"
   [.] .resumeDebounce << #DT"10s"
   [.] .capacity << 50
   [.] .killPropagation#KillPropagation << #Downgrade
   [.] .maxInstancesWithinQueue#UnsignedInt << 10
   [.] .maxConcurrentWithinQueue#UnsignedInt << 4
   [ ] Load rules
   [Q] << #RAMGuard
   [Q] << #HostFailover
```

`-Q.Default` is the only pglib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE01014).

Queue data loading supports `[#]` to load and extend a base queue:

```aljam3
{Q} #Queue:BackupHost
   [#] << #Queue.Default
      [.] .host#URL << "alternative.host.com"
```

## Resource Limit Defaults

<!-- @c:concepts/permissions/enforcement#Resource Limits -->
<!-- @c:technical/spec/job-sandbox -->

Every job has resource limits. When a pipeline declares `{_}` resource permissions (`#RAM`, `#CPU`, `#GPU`, `#IO`, `#Processes`, `#Duration`), those values are used. When a pipeline omits resource permissions, the Queue Handler applies defaults from its queue configuration.

### Queue-Level Limit Configuration

Queues configure default limits and limit-exceeded behavior:

```aljam3
{Q} #Queue:WorkerQueue
   [.] .strategy#QueueStrategy << #FIFO
   [.] .host#URL << "worker-host-01"
   [ ] Resource limit defaults
   [.] .defaultRAM << "512MB"
   [.] .defaultCPU << "1.0"
   [.] .defaultProcesses << 20
   [.] .defaultDuration << #DT"300s"
   [ ] Limit-exceeded behavior
   [.] .onRAMExceed#LimitAction << #Kill
   [.] .onCPUExceed#LimitAction << #Throttle
   [.] .onIOExceed#LimitAction << #Throttle
   [.] .onDurationExceed#LimitAction << #Kill
   [.] .onProcessesExceed#LimitAction << #Kill
   [.] .limitGracePeriod#Duration << #DT"10s"
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `.defaultRAM` | `#string` | `"512MB"` | Default memory limit when pipeline omits `{_} #RAM` |
| `.defaultCPU` | `#string` | `"1.0"` | Default CPU limit (cores) when pipeline omits `{_} #CPU` |
| `.defaultProcesses` | `#int` | `20` | Default max child processes |
| `.defaultDuration` | `#Duration` | `300s` | Default max execution time |
| `.onRAMExceed` | `#LimitAction` | `#Kill` | Action when memory limit exceeded |
| `.onCPUExceed` | `#LimitAction` | `#Throttle` | Action when CPU limit exceeded |
| `.onIOExceed` | `#LimitAction` | `#Throttle` | Action when IO limit exceeded |
| `.onDurationExceed` | `#LimitAction` | `#Kill` | Action when execution time exceeded |
| `.onProcessesExceed` | `#LimitAction` | `#Kill` | Action when process limit exceeded |
| `.limitGracePeriod` | `#Duration` | `10s` | Grace period between SIGTERM and SIGKILL for `#Kill` actions |

> **Constraint:** `#Throttle` is only valid for `#CPU` and `#IO`. Using `#Throttle` with `#RAM`, `#GPU`, `#Processes`, or `#Duration` is a compile error — these resources can only be killed or retried, not throttled.

### Pipeline-Level Override

Pipelines override defaults by declaring `{_}` resource permissions:

```aljam3
{_} _RAMLimit
   [.] .intent << #Grant
   [.] .category << #RAM
   [.] .capability << #Limit
   [.] .max << "1GB"

{_} _CPULimit
   [.] .intent << #Grant
   [.] .category << #CPU
   [.] .capability << #Limit
   [.] .max << "2.0"

{-} -HeavyProcessing
   (-) _RAMLimit
   (-) _CPULimit
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] ...
```

When `{_}` resource permissions are declared, they replace the queue defaults for that category. Categories not declared fall back to queue defaults.

## Using a Queue (`[Q]`)

The `[Q]` line in a pipeline declares which queue it uses.

### Queue Assignment

Three equivalent syntaxes:

```aljam3
[Q] >> #WorkerQueue
[Q] -Q.Assign"{#WorkerQueue}"
[Q] -Q.Assign
   (-) <Queue << #WorkerQueue
```

### Rule Loading

Three equivalent syntaxes:

```aljam3
[Q] << #RAMGuard
[Q] -Q.Load.Job.Rules"{#CPUGuard}"
[Q] -Q.Load.Job.Rules
   (-) <rules << #IdleHibernation
```

### Full Pipeline Example

```aljam3
{-} ProcessData
   [T] ...
   [ ] Assign Queue
   [Q] >> #WorkerQueue
   [ ] Load job rules
   [Q] << #RAMGuard
   [Q] << #CPUGuard
   [Q] << #IdleHibernation
   [W] -W.Aljam3
   ...
```

### Pipeline with Custom Threshold Override

```aljam3
{-} HeavyJob
   [T] ...
   [Q] >> #WorkerQueue
   [Q] #RAMGuard
      (#) <value.GB << 8
      (#) >> $customRAMGuard
   [Q] << $customRAMGuard
   [W] -W.Aljam3
   ...
```

## See Also

- [[concepts/pipelines/queue/rules|Queue Rules & Triggers]] — Defining queue rules and the reactive trigger engine
- [[concepts/pipelines/queue/internals|Queue Internals]] — Dispatch coordinator, state transitions, and load balancing
- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[Q]` fits in pipeline element order
- [[pglib/pipelines/Q/INDEX|u:-Q.* Queue Pipelines]] — full pipeline catalog
