---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
---

# Queue Control (|Q.*)

**Version:** 0.0.2
**Status:** Fully Documented
**Block Marker:** `[Q]`

## Overview

Queue control operations provide precision automation for managing pipeline instance lifecycle. The `|Q.*` namespace offers fine-grained control over pipeline execution: pausing, resuming, terminating, prioritizing, and monitoring instances across system queues.

### Philosophy

- **Precision Automation:** Queue control enables sophisticated orchestration patterns
- **Non-Invasive:** Operations work externally; pipelines don't need special awareness
- **System-Level:** Control operates at the runtime level, not within pipeline logic
- **Safe by Design:** Operations respect pipeline boundaries and lifecycle constraints

## System Queues

Polyglot manages pipeline instances across three system queues:

### #Queues.Pending
- **Purpose:** Newly created instances awaiting dispatch
- **State:** Instances in `Queued` lifecycle state
- **Behavior:** FIFO (First In, First Out) by default
- **Control:** Can be reordered with `|Q.PriorityBump`

### #Queues.Dispatch
- **Purpose:** Instances currently executing
- **State:** Instances in `Running` lifecycle state (actively executing)
- **Behavior:** Managed by runtime scheduler
- **Control:** Can be paused with `|Q.Pause`, terminated with `|Q.Kill`

### #Queues.Pause
- **Purpose:** Paused instances awaiting resume
- **State:** Instances in `Running` lifecycle state (paused, not executing)
- **Behavior:** Persist until resumed or killed
- **Control:** Can be resumed with `|Q.Resume`, terminated with `|Q.Kill`

**Note:** See [Pipeline Lifecycle](../language/10-pipeline-lifecycle.md) for detailed state information.

## Custom Queues

Define custom queues for application-specific orchestration:

```polyglot
[#] #AppQueues
[<] .HighPriority: pg\string << "high"
[<] .BatchProcessing: pg\string << "batch"
[<] .Background: pg\string << "background"
[X]
```

Custom queues enable:
- Priority-based execution
- Resource-specific scheduling
- Application-level orchestration
- Load balancing strategies

## Block Marker: [Q]

The `[Q]` block marker establishes a queue control context for operations.

```polyglot
[Q] |Q.Pause
[<] .instance_id: pg\string << target_id
```

**Properties:**
- **Scope:** Wraps queue control operations
- **Required:** For all `|Q.*` operations
- **Nesting:** Cannot nest within `[Q]` blocks
- **Context:** System-level, not pipeline-internal

## Operations Reference

### |Q.Pause

Pause a running pipeline instance, moving it from `#Queues.Dispatch` to `#Queues.Pause`.

**Signature:**
```polyglot
[Q] |Q.Pause
[<] .instance_id: pg\string << "instance-uuid"
[>] .success: pg\bool >> result
```

**Behavior:**
- Instance stops executing but maintains state
- Moves to `#Queues.Pause`
- Remains in `Running` lifecycle state (paused)
- Can be resumed later with `|Q.Resume`

**Use Cases:**
- Resource throttling
- Manual inspection during execution
- Coordinating dependent pipelines
- Emergency intervention

### |Q.Resume

Resume a paused pipeline instance, moving it from `#Queues.Pause` back to `#Queues.Dispatch`.

**Signature:**
```polyglot
[Q] |Q.Resume
[<] .instance_id: pg\string << "instance-uuid"
[>] .success: pg\bool >> result
```

**Behavior:**
- Instance continues from paused point
- Moves to `#Queues.Dispatch`
- Resumes active execution
- State preserved from pause

**Use Cases:**
- Resume after manual inspection
- Restart after resource availability
- Conditional continuation
- Recovery from throttling

### |Q.Kill

Terminate a pipeline instance, forcing it to the `Exited` lifecycle state.

**Signature:**
```polyglot
[Q] |Q.Kill
[<] .instance_id: pg\string << "instance-uuid"
[>] .success: pg\bool >> result
```

**Behavior:**
- Instance terminates immediately
- Moves to `Exited` lifecycle state
- No cleanup code runs
- Cannot be resumed

**Use Cases:**
- Canceling long-running operations
- Error recovery
- Resource cleanup
- Manual intervention

**Warning:** `|Q.Kill` is forceful termination. Prefer designing pipelines with graceful exit conditions.

### |Q.PriorityBump

Move an instance forward in `#Queues.Pending`, increasing execution priority.

**Signature:**
```polyglot
[Q] |Q.PriorityBump
[<] .instance_id: pg\string << "instance-uuid"
[<] .positions: pg\int << 5  // Move forward 5 positions
[>] .success: pg\bool >> result
```

**Behavior:**
- Reorders instance in `#Queues.Pending`
- Only affects queued instances (not running/paused)
- Cannot move past queue head
- Does not affect other queues

**Use Cases:**
- User-triggered priority
- Time-sensitive operations
- Dependency-based scheduling
- Dynamic workload management

### |Q.Queue.Assign

Assign an instance to a custom queue instead of default `#Queues.Pending`.

**Signature:**
```polyglot
[Q] |Q.Queue.Assign
[<] .instance_id: pg\string << "instance-uuid"
[<] .queue: pg\string << #AppQueues.HighPriority
[>] .success: pg\bool >> result
```

**Behavior:**
- Instance assigned to specified queue
- Must be called before instance starts executing
- Only valid for custom queues (not system queues)
- Enables custom scheduling strategies

**Use Cases:**
- Priority-based execution
- Resource-specific scheduling
- Load balancing
- Application-level orchestration

### |Q.Status

Query the current status of a pipeline instance.

**Signature:**
```polyglot
[Q] |Q.Status
[<] .instance_id: pg\string << "instance-uuid"
[>] .state: pg\string >> current_state      // "Created", "Queued", "Running", "Exited"
[>] .queue: pg\string >> current_queue      // Which queue (if any)
[>] .is_paused: pg\bool >> paused_flag      // True if in Pause queue
```

**Behavior:**
- Non-invasive query operation
- Returns current lifecycle state
- Reports queue assignment
- Indicates pause status

**Use Cases:**
- Monitoring dashboards
- Conditional orchestration
- Debugging pipeline flow
- Health checks

## Complete Examples

### Example 1: Conditional Pause and Resume

Pause a long-running analysis if system load is high, then resume when resources available.

```polyglot
[|] MonitoredAnalysis
[i] .data: pg\string
[i] .instance_id: pg\string  // Track this instance

[r] |HeavyAnalysis
[<] .input: pg\string << .data
[>] .result: pg\string >> analysis_result

[o] .result: pg\string << analysis_result
[X]

// Monitoring pipeline
[|] ResourceMonitor
[i] .target_instance: pg\string

[r] |CheckSystemLoad
[>] .load: pg\int >> current_load

// If load > 80%, pause target
[?] current_load ?> 80..
[~][r] .condition: pg\bool << True
[~][~][r] .condition: pg\bool << False

[r] |DecideAction
[<] .should_pause: pg\bool << .condition
[>] .action: pg\string >> action_result

[Q] |Q.Pause
[<] .instance_id: pg\string << .target_instance
[>] .success: pg\bool >> pause_result

// Wait for load to decrease
[r] |WaitForResources
[>] .ready: pg\bool >> resources_ready

// Resume when ready
[Q] |Q.Resume
[<] .instance_id: pg\string << .target_instance
[>] .success: pg\bool >> resume_result

[o] .status: pg\string << "Resumed"
[X]
```

### Example 2: Priority Queue Management

Implement a high-priority queue for time-sensitive operations.

```polyglot
// Define priority queues
[#] #AppQueues
[<] .HighPriority: pg\string << "high"
[<] .NormalPriority: pg\string << "normal"
[<] .LowPriority: pg\string << "low"
[X]

// Orchestrator assigns instances to queues
[|] PriorityOrchestrator
[i] .operation_type: pg\string
[i] .instance_id: pg\string

[r] |DeterminePriority
[<] .type: pg\string << .operation_type
[>] .priority: pg\string >> assigned_priority

// Assign to appropriate queue
[Q] |Q.Queue.Assign
[<] .instance_id: pg\string << .instance_id
[<] .queue: pg\string << assigned_priority
[>] .success: pg\bool >> assignment_result

[o] .assigned_to: pg\string << assigned_priority
[X]

// Usage
[|] TimesSensitiveOperation
[i] .data: pg\string

[r] |GetInstanceID
[>] .id: pg\string >> my_instance_id

// Request high priority
[r] |PriorityOrchestrator
[<] .operation_type: pg\string << "urgent"
[<] .instance_id: pg\string << my_instance_id

[r] |ProcessData
[<] .input: pg\string << .data
[>] .result: pg\string >> final_result

[o] .result: pg\string << final_result
[X]
```

### Example 3: Instance Monitoring Dashboard

Monitor multiple pipeline instances and report their status.

```polyglot
[|] InstanceMonitor
[i] .instance_ids: pg\array{pg\string}

// Check each instance
[~][r] |CheckInstances
[<] .ids: pg\array{pg\string} << .instance_ids

// For each instance
[p] |CheckSingleInstance
[<] .id: pg\string << .ids[*]

[Q] |Q.Status
[<] .instance_id: pg\string << .id
[>] .state: pg\string >> instance_state
[>] .queue: pg\string >> instance_queue
[>] .is_paused: pg\bool >> instance_paused

[r] |FormatStatus
[<] .id: pg\string << .id
[<] .state: pg\string << instance_state
[<] .queue: pg\string << instance_queue
[<] .paused: pg\bool << instance_paused
[>] .formatted: pg\string >> status_line

[>] .formatted >> status_results

[Y] |Y.Join
[>] status_results

[r] |AggregateReport
[<] .statuses: pg\array{pg\string} << status_results
[>] .report: pg\string >> final_report

[o] .dashboard: pg\string << final_report
[X]
```

### Example 4: Emergency Kill Switch

Implement a kill switch for runaway pipeline instances.

```polyglot
[|] EmergencyKillSwitch
[i] .instance_id: pg\string
[i] .reason: pg\string

// Log the kill operation
[r] |LogEmergency
[<] .id: pg\string << .instance_id
[<] .reason: pg\string << .reason
[>] .logged: pg\bool >> log_result

// Terminate instance
[Q] |Q.Kill
[<] .instance_id: pg\string << .instance_id
[>] .success: pg\bool >> kill_result

// Report outcome
[r] |FormatKillReport
[<] .success: pg\bool << kill_result
[<] .id: pg\string << .instance_id
[>] .report: pg\string >> final_report

[o] .result: pg\string << final_report
[X]

// Usage: Kill a specific instance
[|] MonitorAndKill
[i] .target: pg\string

[r] |DetectRunaway
[<] .instance: pg\string << .target
[>] .is_runaway: pg\bool >> runaway_detected

[t] .condition: pg\bool << runaway_detected

[r] |EmergencyKillSwitch
[<] .instance_id: pg\string << .target
[<] .reason: pg\string << "Runaway detection triggered"
[>] .result: pg\string >> kill_report

[o] .status: pg\string << kill_report
[X]
```

## Best Practices

### 1. **Design for Graceful Exit First**
Prefer pipeline logic that handles termination gracefully over relying on `|Q.Kill`.

```polyglot
// ✓ PREFERRED - graceful exit condition
[|] GracefulPipeline
[i] .should_continue: pg\bool

[t] .condition: pg\bool << .should_continue
[r] |ContinueProcessing
[X]

// ✗ AVOID - relying on external kill
[|] ForceKillPipeline
// No exit logic, requires |Q.Kill
[X]
```

### 2. **Track Instance IDs Explicitly**
Store instance IDs when you need to control them later.

```polyglot
[r] |GetInstanceID
[>] .id: pg\string >> my_instance_id

// Store for later queue control
[r] |StoreForMonitoring
[<] .instance: pg\string << my_instance_id
```

### 3. **Use Custom Queues for Prioritization**
Don't rely solely on `|Q.PriorityBump`; design custom queue strategies.

```polyglot
// ✓ GOOD - custom queue strategy
[Q] |Q.Queue.Assign
[<] .queue: pg\string << #AppQueues.HighPriority

// ✗ LESS OPTIMAL - manual bumping
[Q] |Q.PriorityBump
[<] .positions: pg\int << 100
```

### 4. **Handle Queue Operation Failures**
Always check `.success` output from queue operations.

```polyglot
[Q] |Q.Pause
[<] .instance_id: pg\string << target_id
[>] .success: pg\bool >> pause_success

[t] .condition: pg\bool << (!pause_success)
[r] |HandlePauseFailure
```

### 5. **Monitor Before Controlling**
Use `|Q.Status` to verify instance state before operations.

```polyglot
[Q] |Q.Status
[<] .instance_id: pg\string << target
[>] .state: pg\string >> current_state

// Only pause if running
[?] current_state ?> "Running"
[~][Q] |Q.Pause
[~][<] .instance_id: pg\string << target
```

### 6. **Pause/Resume for Inspection, Not Control Flow**
Don't use pause/resume as primary control flow mechanism.

```polyglot
// ✗ AVOID - pause/resume for control flow
[Q] |Q.Pause
// Wait for external event
[Q] |Q.Resume

// ✓ PREFERRED - use triggers [t] or conditionals
[t] .condition: pg\bool << event_occurred
[r] |ContinueProcessing
```

### 7. **Document Custom Queue Semantics**
Clearly define the meaning and dispatch rules for custom queues.

```polyglot
// Define queue semantics in documentation
[#] #AppQueues
[<] .Interactive: pg\string << "interactive"     // Immediate dispatch, <100ms latency
[<] .Batch: pg\string << "batch"                 // Scheduled dispatch, hourly
[<] .Background: pg\string << "background"       // Low priority, fills idle time
[X]
```

### 8. **Test Queue Control Scenarios**
Verify queue control behavior under different conditions.

```polyglot
[|] TestPauseResume
[r] |CreateTestInstance
[>] .id: pg\string >> test_id

[Q] |Q.Pause
[<] .instance_id: pg\string << test_id

[Q] |Q.Status
[<] .instance_id: pg\string << test_id
[>] .is_paused: pg\bool >> paused

// Verify paused state
[r] |AssertTrue
[<] .value: pg\bool << paused

[Q] |Q.Resume
[<] .instance_id: pg\string << test_id
[X]
```

## Common Patterns

### Pattern 1: Watchdog Monitor
Monitor instances and kill them if they exceed time limits.

```polyglot
[|] WatchdogMonitor
[i] .instance_id: pg\string
[i] .max_runtime_seconds: pg\int

[r] |WaitForTimeout
[<] .seconds: pg\int << .max_runtime_seconds

[Q] |Q.Status
[<] .instance_id: pg\string << .instance_id
[>] .state: pg\string >> current_state

// If still running, kill it
[?] current_state ?> "Running"
[~][Q] |Q.Kill
[~][<] .instance_id: pg\string << .instance_id
[X]
```

### Pattern 2: Resource-Aware Scheduling
Pause instances when resources are scarce, resume when available.

```polyglot
[|] ResourceAwareScheduler
[i] .monitored_instances: pg\array{pg\string}

[r] |CheckResources
[>] .cpu_available: pg\bool >> cpu_ok
[>] .memory_available: pg\bool >> mem_ok

[t] .condition: pg\bool << (!cpu_ok || !mem_ok)

// Pause all monitored instances
[~][p] |PauseInstances
[<] .ids: pg\array{pg\string} << .monitored_instances

[Q] |Q.Pause
[<] .instance_id: pg\string << .ids[*]

[Y] |Y.Join
[>] // Wait for all pauses

// Wait for resources
[r] |WaitForResources
[>] .ready: pg\bool >> resources_ready

// Resume all instances
[~][p] |ResumeInstances
[<] .ids: pg\array{pg\string} << .monitored_instances

[Q] |Q.Resume
[<] .instance_id: pg\string << .ids[*]

[Y] |Y.Join
[X]
```

### Pattern 3: Priority Escalation
Automatically bump priority for instances waiting too long.

```polyglot
[|] PriorityEscalation
[i] .instance_id: pg\string
[i] .max_wait_seconds: pg\int

[r] |WaitForEscalation
[<] .seconds: pg\int << .max_wait_seconds

[Q] |Q.Status
[<] .instance_id: pg\string << .instance_id
[>] .state: pg\string >> current_state

// If still queued, bump priority
[?] current_state ?> "Queued"
[~][Q] |Q.PriorityBump
[~][<] .instance_id: pg\string << .instance_id
[~][<] .positions: pg\int << 10
[X]
```

## Error Handling

Queue operations return `.success: pg\bool` to indicate outcome. Handle failures appropriately.

```polyglot
[Q] |Q.Pause
[<] .instance_id: pg\string << target_id
[>] .success: pg\bool >> pause_ok

[t] .condition: pg\bool << (!pause_ok)

// Handle failure
[r] |InvestigateFailure
[<] .operation: pg\string << "pause"
[<] .instance: pg\string << target_id

[Q] |Q.Status
[<] .instance_id: pg\string << target_id
[>] .state: pg\string >> actual_state

[r] |LogError
[<] .message: pg\string << "Failed to pause instance"
[<] .state: pg\string << actual_state
```

**Common Failure Causes:**
- Instance ID doesn't exist
- Instance already in target state (e.g., pausing paused instance)
- Instance has exited
- Invalid queue name for `|Q.Queue.Assign`
- Race conditions (instance state changed between status check and operation)

## See Also

- [Pipeline Lifecycle](../language/10-pipeline-lifecycle.md) - Lifecycle states and system queues
- [Standard Library Overview](00-overview.md) - Complete stdlib organization
- [Join Operations](03-join-operations.md) - Synchronization with `|Y.*`

---

**Navigation:**
← [Runtime Wrappers](01-runtime-wrappers.md) | [Standard Library Index](00-overview.md) | [Utilities Catalog](03-utilities-catalog.md) →