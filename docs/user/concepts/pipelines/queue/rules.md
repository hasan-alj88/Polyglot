---
audience: automation-builder
type: specification
updated: 2026-04-26
---

<!-- @c:concepts/pipelines/queue/rules -->

# Queue Rules & Triggers

<!-- @c:glossary#Trigger Monitor -->

Rules are **parameterized reactive listeners** compiled into the behavior contract's signal map. The Trigger Monitor ([[glossary#Trigger Monitor|c:Trigger Monitor]]) evaluates them purely on a reactive, event-driven basis.

## Defining Rules (`{Q} #JobRules:Name` / `{Q} #QueueRules:Name`)

### Rule Parameters `(#)`

Rules accept parameters with `(#)` inputs. `<~` sets defaults (overridable), `<<` sets finals.

```aljam3
{Q} #JobRules:RAMGuard
   (#) $value.GB#int <~ 4
   (#) $margin.GB#int <~ 1
```

Parameters support validation inside the rule:

```aljam3
   [?] -Math.IsGreater"{$margin.GB}", "{$value.GB}" =? #Boolean.True
      [!] >> !Queue.InvalidMargin
```

Override parameters when loading a rule into a pipeline:

```aljam3
[Q] #RAMGuard
   (#) <value.GB << 8
   (#) >> $customRAMGuard
[Q] << $customRAMGuard
```

## Triggers (`[T]` and `[&]`)

Queue rules use Aljam3's trigger system rather than continuous procedural loops. 

| Marker | Meaning |
|--------|---------|
| `[T]` | Edge Trigger (wakes the rule when an event occurs) |
| `[&]` | State Trigger (must be true when the Edge Trigger fires) |
| `[+]` | State Trigger OR (alternative state condition) |

Because rules are purely reactive, you do not need `*?` wildcards or `-Q.DoNothing` blocks at runtime. If a trigger doesn't fire, no action is taken.

## Compile-Time Predicates (`[?] ?`)

For structural validation *before* the program runs, you can use Boolean Predicates prefixed with `?`. These are strictly compile-time assertions.

| Guard | Meaning |
|-------|---------|
| `?Queue.Host.IsEqual"{#TargetQueue}"` | Prevents routing loops by ensuring queues are on different hosts |
| `?Queue.Strategy.IsEqual"{#FIFO}"` | Asserts the queue strategy |

```aljam3
{Q} #QueueRules:Failover
   (#) <FailoverQueue#Queue <~ {#BackupHostQueue}
   [ ] Compile-time safety check
   [?] ?Queue.Host.IsEqual"{#FailoverQueue}" =? #Boolean.True
      [!] >> !Queue.InvalidFailoverTarget
```

## Examples

### Job Rules Example

Instead of evaluating Job RAM every tick, we trigger strictly when thresholds are crossed.

```aljam3
{Q} #JobRules:RAMGuard
   (#) $value.GB#int <~ 4
   (#) $margin.GB#int <~ 1
   
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
```

### Queue Rules Example

Queue rules handle bulk actions and ingress flow.

```aljam3
{Q} #QueueRules:OverloadProtection
   [ ] Assert Overload state when over threshold
   [T] -QT.Queue.Jobs.Aggregate.Exceeds.RAM"32.0GB"
      [ ] Stop dispatching new jobs into execution
      [Q] -Q.Queue.Dispatch.Pause
      [ ] Throttle the currently running jobs to free resources
      [Q] -Q.Queue.Jobs.Throttle
      
   [ ] Clear Overload state when under threshold
   [T] -QT.Queue.Jobs.Aggregate.DropsBelow.RAM"32.0GB"
      [Q] -Q.Queue.Jobs.Unthrottle
      [Q] -Q.Queue.Dispatch.Resume
```

## Temporal Behavior: Trigger Engine

The Trigger Monitor does not evaluate every rule every tick. Instead, it wires rules directly to OS and Runtime signals (`-QT.*`). 
When a threshold is crossed (e.g., `-QT.Job.Resource.Exceeds.RAM`), the TM evaluates any `[&]` state triggers attached to that rule. If satisfied, the `-Q.*` actions are pushed.

Conflicting pause levels: the **highest level wins** (Free.All > Free.RAM > Free.CPU).

### Anti-Flap Mechanisms

| Mechanism | Type | How |
|-----------|------|-----|
| Explicit OR/AND chains | Structural hysteresis | Designing explicit combinations of `[T]` and `[+]`/`[&]` for Resuming vs Pausing |
| `.resumeDebounce` | Temporal hysteresis | Global queue setting delaying resumes to ensure stability |

### Evaluation Model

Because execution relies entirely on the reactive engine, there is no generic "tick period". The runtime registers cgroup/system watches. Note that some values (like periodic redis length checks) may internally use a configurable poll rate:

```aljam3
{Q} #Queue:WorkerQueue
   [.] .pollPeriod << #DT"5s"
```

## Default Queue Behaviors

Every queue includes built-in rules unless explicitly overridden:

| Default | Condition | Action |
|---------|-----------|--------|
| RAM overflow prevention | Job RAM approaching host available | `-Q.Job.Pause.Free.RAM.Wait` |
| Disk space guard | Host disk < 1GB | `-Q.Job.Pause.Free.All.Wait` |
| Runaway CPU | Job CPU > 100% for > 5m (single-threaded) | `-Q.Job.Throttle` |
| Zombie cleanup | Job status = finished but process alive | `-Q.Job.Kill.Now` |
| Orphan detection | Job's parent pipeline terminated | `-Q.Job.Kill.WithCleanup` |
| CRIU image cleanup | Paused.All Job not resumed in 24h | Notify + optional Kill |

Override by setting the default to false:

```aljam3
{Q} #Queue:MyQueue
   [.] .defaults.ramOverflow << false
```

## See Also

- [[concepts/pipelines/queue/INDEX|Queue Configuration]] — Core queue setup and block types
- [[concepts/pipelines/queue/internals|Queue Internals]] — Dispatch coordinator, state transitions, and load balancing
- [[aj3lib/pipelines/Q/INDEX|u:-Q.* Queue Pipelines]] — full pipeline catalog
