---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
---

# -Q — Queue Pipelines

<!-- @c:concepts/pipelines/queue/INDEX -->
Queue pipelines manage the multi-queue execution model. Every `-Q.*` pipeline is a **parameterized instruction** compiled into the behavior contract's signal map. The Trigger Monitor reads the signal map and executes at runtime. No `[@]` import needed. See [[concepts/pipelines/queue/INDEX|c:Queue]] for queue concepts.

**PRIMITIVE** — Queue pipelines are direct OS/runtime integrations implemented by the Aljam3 runtime. They cannot be reimplemented in user `.aj3` files.

## Permissions

No permissions required. All operations are pure computation (queue scheduling and resource management). See [[permissions]].

## `{Q}` Block Types

| Block | Purpose | Actions Allowed | Getters Allowed |
|-------|---------|-----------------|-----------------|
| `{Q} #Queue:Name` | Queue configuration | None | None |
| `{Q} #QueueRules:Name` | Queue-level policy | `-Q.Queue.*` only | `-Q.Queue.*`, `-Q.Host.*`, `-Q.Job.*` (array) |
| `{Q} #JobRules:Name` | Job-level policy | `-Q.Job.*` only | `-Q.Job.*`, `-Q.Queue.*`, `-Q.Host.*` |

**Scope rule:** You can read anything, you can only act on your scope.

## Three Kinds of `-Q.*` Pipelines

| Kind | Pattern | Purpose |
|------|---------|---------|
| **Getter** | `-Q.<Scope>.Get.<Resource>` | Return a resource measurement value |
| **State** | `-Q.<Scope>.Is.<State>` / `.Idle.*` / `.Active.*` | Return state or idle/active detection |
| **Action** | `-Q.<Scope>.<Action>` | Execute a control operation |

Where `<Scope>` is `Job`, `Host`, `Queue`, or `Queue.Jobs` (array context in `#QueueRules`).

## Pipeline Listing

### Queue Assignment

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Default\|-Q.Default]] | Standard FIFO queue, no constraints |
| [[pglib/pipelines/Q/Assign\|-Q.Assign]] | Assign pipeline to a named queue |

### Pause (resumable — Job state preserved)

| Pipeline | CPU | RAM | FDs/TCP/Locks | Timing |
|----------|-----|-----|---------------|--------|
| [[pglib/pipelines/Q/Job.Pause.Free.CPU\|-Q.Job.Pause.Free.CPU]] | Freed | Kept | Kept | `.Now` / `.Wait` |
| [[pglib/pipelines/Q/Job.Pause.Free.RAM\|-Q.Job.Pause.Free.RAM]] | Freed | Freed | Kept | `.Soft` / `.Hard` x `.Now` / `.Wait` |
| [[pglib/pipelines/Q/Job.Pause.Free.All\|-Q.Job.Pause.Free.All]] | Freed | Freed | Freed (to disk) | `.Now` / `.Wait` |

### Resume

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Job.Resume\|-Q.Job.Resume]] | Resume from CPU or RAM pause (state in memory) |
| `-Q.Job.Resume.From.Disk` | Restore from `Free.All` image files |

### Kill (terminal — Job state destroyed)

| Pipeline | Cleanup? | Description |
|----------|----------|-------------|
| [[pglib/pipelines/Q/Job.Kill.WithCleanup\|-Q.Job.Kill.WithCleanup]] | Yes | SIGTERM, run `[/]` cleanup, then SIGKILL timeout |
| [[pglib/pipelines/Q/Job.Kill.Now\|-Q.Job.Kill.Now]] | No | SIGKILL — instant termination |

### Resource Adjustment (Job keeps running)

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Job.Throttle\|-Q.Job.Throttle]] | Reduce CPU/RAM/IO allocation |
| `-Q.Job.Unthrottle` | Restore full allocation |
| `-Q.Job.Priority.Update` | Change scheduling priority |

### Spatial

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Job.Reassign\|-Q.Job.Reassign]] | Move to different queue (cross-host = CRIU transfer) |
| [[pglib/pipelines/Q/Job.Snapshot\|-Q.Job.Snapshot]] | Point-in-time state fork to disk |

### Queue Door Controls (Ingress Fate & Dispatch)

| Pipeline | Description |
|----------|-------------|
| `-Q.Queue.Ingress.Accept` | Open the queue door to new jobs |
| `-Q.Queue.Ingress.Kill.<Timing>` | Accept new jobs but immediately queue them for termination |
| `-Q.Queue.Ingress.Pause.<Level>.<Timing>` | Accept new jobs but start them in a suspended state |
| `-Q.Queue.Ingress.Divert"{#TargetQueue}"` | Accept new jobs but route their fate to another queue |
| `-Q.Queue.Dispatch.Pause` | Stop dispatching jobs to execution (they wait in queue) |
| `-Q.Queue.Dispatch.Resume` | Resume dispatching jobs to execution |

### Bulk Job Controls (Queue-Level)

| Pipeline | Description |
|----------|-------------|
| `-Q.Queue.Jobs.Pause.<Level>.<Timing>` | Bulk pause all jobs tied to this queue |
| `-Q.Queue.Jobs.Resume` | Bulk resume all jobs tied to this queue |
| `-Q.Queue.Jobs.Throttle` | Bulk throttle all jobs tied to this queue |
| `-Q.Queue.Jobs.Unthrottle` | Lift bulk throttle from all jobs tied to this queue |
| `-Q.Queue.Jobs.Kill.<Timing>` | Bulk kill all jobs tied to this queue |

### Observation

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Job.Inspect\|-Q.Job.Inspect]] | Read Job state without affecting it |
| [[pglib/pipelines/Q/Job.Branch\|-Q.Job.Branch]] | Name a marker subtree for external reference |

### No-Op

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/DoNothing\|-Q.DoNothing]] | Satisfies `*?` exhaustiveness. Compiler warns on unhandled states |

### Getter Pipelines

#### `-Q.Job.Get.*` (per-job metrics)

| Getter | Returns | Unit | Source |
|--------|---------|------|--------|
| `-Q.Job.Get.RAM.GB` | Job's RAM usage | Gigabytes | cgroup `memory.current` |
| `-Q.Job.Get.RAM.MB` | Job's RAM usage | Megabytes | cgroup `memory.current` |
| `-Q.Job.Get.CPU.Percent` | Job's CPU usage | Percentage (0-100+) | cgroup `cpu.stat` (`usage_usec` delta) |
| `-Q.Job.Get.IO.MBps` | Job's IO throughput | MB/s | cgroup `io.stat` (`rbytes`/`wbytes` delta) |
| `-Q.Job.Get.Time` | Job's wall-clock runtime | Duration `#DT` | Redis `job:{jobId}` `started_at` |
| `-Q.Job.Get.Status` | Job's current `#QueueState` | Enum value | Redis `job:{jobId}` `status` |
| `-Q.Job.Get.Disk.GB` | Disk used by Job's data | Gigabytes | `du` on job data directory |

#### `-Q.Host.Get.*` (host-level metrics)

| Getter | Returns | Unit | Source |
|--------|---------|------|--------|
| `-Q.Host.Get.RAM.GB` | Host available RAM | Gigabytes | `/proc/meminfo` `MemAvailable` |
| `-Q.Host.Get.RAM.Used.GB` | Host used RAM | Gigabytes | `/proc/meminfo` `MemTotal - MemAvailable` |
| `-Q.Host.Get.CPU.Percent` | Host CPU utilization | Percentage | `/proc/stat` cpu line delta |
| `-Q.Host.Get.Disk.GB` | Host available disk | Gigabytes | `statvfs()` on data partition |
| `-Q.Host.Get.Status` | Host status | Enum (`#Online`, `#Offline`, `#Draining`) | Resource Monitor aggregate |
| `-Q.Host.Get.GPU.Status` | GPU status | Enum (`#InUse`, `#Free`) | `nvidia-smi` / vendor tool |

#### `-Q.Queue.Get.*` (queue-level metrics)

| Getter | Returns | Unit | Source |
|--------|---------|------|--------|
| `-Q.Queue.Get.Length` | Number of Jobs in queue | Count | Redis `LLEN`/`ZCARD queue:dispatch:{queue}` |
| `-Q.Queue.Get.Executing` | Number of executing Jobs | Count | Redis `SCARD set:executing` filtered by queue |
| `-Q.Queue.Get.Suspended` | Number of suspended Jobs | Count | Redis `HLEN set:suspended` filtered by queue |

#### `-Q.Queue.Jobs.Get.*` (all jobs — returns array, `#QueueRules` context)

| Getter | Returns | Per-element | Source |
|--------|---------|-------------|--------|
| `-Q.Queue.Jobs.Get.RAM.GB` | Array of Job RAM values | Gigabytes | Redis SMEMBERS (queue filter) → per-job cgroup `memory.current` |
| `-Q.Queue.Jobs.Get.CPU.Percent` | Array of Job CPU values | Percentage | Redis SMEMBERS (queue filter) → per-job cgroup `cpu.stat` |
| `-Q.Queue.Jobs.Get.Idle.All` | Array of Job idle durations | Duration | Redis SMEMBERS (queue filter) → per-job cgroup stat deltas |

### Triggers (`-QT.*`)

Rules rely on triggers rather than continuous evaluation loops.

| Trigger Category | Examples | Usage |
|------------------|----------|-------|
| **Edge Triggers** | `-QT.Job.Resource.Exceeds.RAM"{val}"`, `-QT.Job.Parent.Terminated` | Wakes the rule via `[T]` |
| **State Triggers** | `-QT.Job.State.Is.Paused`, `-QT.Job.State.Is.Active` | Validates condition via `[&]` |

### Compile-Time Predicates (`?` Prefix)

Used for structural validation before the program compiles.

| Predicate | Evaluates To | Usage |
|-----------|--------------|-------|
| `?Queue.Host.IsEqual"{#TargetQueue}"` | `#Boolean` | `[?] ?Queue.Host.IsEqual"{#TargetQueue}"` |
| `?Queue.Strategy.IsEqual"{#Strategy}"` | `#Boolean` | `[?] ?Queue.Strategy.IsEqual"{#Strategy}"` |
| `?Queue.Supports.TCPRepair` | `#Boolean` | `[?] ?Queue.Supports.TCPRepair` |

## Example: Queue Definition + Rules + Pipeline

```aljam3
[ ] Queue configuration
{Q} #Queue:WorkerQueue
   [.] .tickPeriod << #DT"5s"
   [.] .resumeDebounce << #DT"10s"
   [.] .capacity << 50
   [Q] << #HostFailover

[ ] Job-level rule: pause when RAM exceeds threshold
{Q} #JobRules:RAMGuard
   (#) $value.GB#int <~ 4
   (#) $margin.GB#int <~ 1
   
   [ ] Compile-time validation: margin must be strictly less than value
   [?] -Math.IsGreater"{$margin.GB}", "{$value.GB}" =? #Boolean.True
      [!] >> !Queue.InvalidMargin
      
   [ ] Trigger when RAM exceeds threshold AND Job is Active
   [T] -QT.Job.State.Is.Active
   [&] -QT.Job.Resource.Exceeds.RAM"{$value.GB}GB"
   [&] -QT.Host.Resource.DropsBelow.RAM.Available"{$value.GB}GB+{$margin.GB}GB"
      [Q] -Q.Job.Pause.Free.RAM.Wait
      
   [ ] Trigger when EITHER the Job drops RAM OR the Host recovers
   [T] -QT.Job.State.Is.Paused
   [&] -QT.Job.Resource.DropsBelow.RAM"{$value.GB}GB"
   [+] -QT.Host.Resource.Exceeds.RAM.Available"{$value.GB}GB+{$margin.GB}GB"
      [Q] -Q.Job.Resume
[ ] Pipeline using the queue and rules
{-} ProcessData
   [T] ...
   [ ] Assign to WorkerQueue
   [Q] >> #WorkerQueue
   [ ] Load job rules
   [Q] << #RAMGuard
   [Q] << #CPUGuard
   [W] -W.Aljam3
   ...
```

## Deprecated Pipelines

The following pipelines have been replaced. See individual files for migration pointers.

| Old | Replacement | Reason |
|-----|-------------|--------|
| [[pglib/pipelines/Q/Pause.Soft\|d:-Q.Pause.Soft]] | [[pglib/pipelines/Q/Job.Pause.Free.CPU\|-Q.Job.Pause.Free.CPU]] | Scoped naming, explicit resource level |
| [[pglib/pipelines/Q/Pause.Hard\|d:-Q.Pause.Hard]] | [[pglib/pipelines/Q/Job.Pause.Free.RAM\|-Q.Job.Pause.Free.RAM]] | Five resource-freeing levels replace two |
| [[pglib/pipelines/Q/Resume\|d:-Q.Resume]] | [[pglib/pipelines/Q/Job.Resume\|-Q.Job.Resume]] | Pause reason set semantics |
| [[pglib/pipelines/Q/Kill.Graceful\|d:-Q.Kill.Graceful]] | [[pglib/pipelines/Q/Job.Kill.WithCleanup\|-Q.Job.Kill.WithCleanup]] | "WithCleanup" = `[/]` runs |
| [[pglib/pipelines/Q/Kill.Hard\|d:-Q.Kill.Hard]] | [[pglib/pipelines/Q/Job.Kill.Now\|-Q.Job.Kill.Now]] | "Now" = instant SIGKILL |
| [[pglib/pipelines/Q/Drain\|d:-Q.Drain]] | [[pglib/pipelines/Q/Queue.Drain\|-Q.Queue.Drain]] | Scoped naming |
| [[pglib/pipelines/Q/Flush\|d:-Q.Flush]] | [[pglib/pipelines/Q/Queue.Flush\|-Q.Queue.Flush]] | Scoped naming |
| [[pglib/pipelines/Q/Reassign\|d:-Q.Reassign]] | [[pglib/pipelines/Q/Job.Reassign\|-Q.Job.Reassign]] | Scoped naming |
| `.If.<Resource>.<Condition>` suffixes | `[?]`/`[&]`/`[+]` conditional blocks | Conditions belong in Aljam3's conditional system |

## Related

- [[concepts/pipelines/queue/INDEX|c:Queue]]
- [[pglib/INDEX]]
