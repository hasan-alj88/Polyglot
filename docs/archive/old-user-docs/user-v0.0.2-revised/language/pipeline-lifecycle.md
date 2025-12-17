# Pipeline Lifecycle

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Understanding the pipeline lifecycle is essential for working with Polyglot. Pipelines are defined once but can be instantiated many times, similar to classes and objects in object-oriented programming.

**Key Concepts:**
- Pipeline definition (template/class)
- Pipeline instance (object)
- Instance lifecycle states
- Queue system
- Execution vs running
- Multiple instances from one definition

---

## Table of Contents

1. [Pipeline Definition vs Instance](#pipeline-definition-vs-instance)
2. [Class/Object Analogy](#classobject-analogy)
3. [Instance Lifecycle States](#instance-lifecycle-states)
4. [Queue System](#queue-system)
5. [Execution vs Running](#execution-vs-running)
6. [Creating Instances](#creating-instances)
7. [Instance Independence](#instance-independence)
8. [Instance Termination](#instance-termination)
9. [Monitoring and Control](#monitoring-and-control)
10. [Best Practices](#best-practices)

---

## Pipeline Definition vs Instance

### Pipeline Definition

**Pipeline definition** is the template/blueprint defined with `[|]...[X]`:

```polyglot
[|] ProcessFile
[i] .file_path: pg\path
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data
[X]
```

**Think of it as:** The class definition (template)

**Characteristics:**
- Defined once
- Describes structure and behavior
- Specifies inputs, outputs, operations
- Does not execute by itself

---

### Pipeline Instance

**Pipeline instance** is a runtime instantiation of the definition:

```polyglot
// Create instance 1
[r] |ProcessFile
[<] .file_path: pg\path << "data1.txt"

// Create instance 2
[r] |ProcessFile
[<] .file_path: pg\path << "data2.txt"

// Create instance 3
[r] |ProcessFile
[<] .file_path: pg\path << "data3.txt"
```

**Think of it as:** Object instances from a class

**Characteristics:**
- Created at runtime
- Has unique instance ID
- Has its own state and variables
- Independent from other instances
- Goes through lifecycle

---

### Key Distinction

| Aspect | Definition | Instance |
|--------|-----------|----------|
| **When** | Compile-time | Runtime |
| **Count** | One | Many |
| **State** | Stateless | Stateful |
| **Execution** | Never executes | Executes |
| **Analogy** | Class | Object |
| **Syntax** | `[|] Name ... [X]` | `|Name` call |

---

## Class/Object Analogy

### Class Definition (Pipeline Definition)

```python
# Python class (analogous to pipeline definition)
class ProcessFile:
    def __init__(self, file_path):
        self.file_path = file_path

    def execute(self):
        content = read_file(self.file_path)
        return process(content)
```

```polyglot
// Polyglot pipeline definition
[|] ProcessFile
[i] .file_path: pg\path
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data
[X]
```

---

### Object Instances (Pipeline Instances)

```python
# Python objects (analogous to pipeline instances)
instance1 = ProcessFile("data1.txt")  # Create instance 1
instance2 = ProcessFile("data2.txt")  # Create instance 2
instance3 = ProcessFile("data3.txt")  # Create instance 3

instance1.execute()  # Run instance 1
instance2.execute()  # Run instance 2
```

```polyglot
// Polyglot pipeline instances
[r] |ProcessFile  // Create instance 1
[<] .file_path: pg\path << "data1.txt"

[r] |ProcessFile  // Create instance 2
[<] .file_path: pg\path << "data2.txt"

[r] |ProcessFile  // Create instance 3
[<] .file_path: pg\path << "data3.txt"
```

---

### Analogy Summary

**Pipeline Definition = Class**
- Template/blueprint
- Defines structure
- Defined once

**Pipeline Instance = Object**
- Actual running thing
- Has state
- Created multiple times

**Pipeline Call = Instantiation**
- `|PipelineName` creates new instance
- Like `new Class()` or `Class()`

---

## Instance Lifecycle States

### Complete Lifecycle

```
┌──────────┐
│ Created  │ Instance created
└────┬─────┘
     │
     ▼
┌──────────┐
│ Queued   │ In Pending queue
│ (Pending)│
└────┬─────┘
     │
     ▼
┌──────────┐
│ Running  │ In Dispatch queue (executing or paused)
│          │ ← "Running" is broad state
└────┬─────┘
     │
     ├─────► [Paused] ──┐
     │          ↓       │
     │      [Resume] ◄──┘
     │
     ▼
┌──────────┐
│  Exited  │ Instance completed or killed
└──────────┘
```

---

### State Definitions

**1. Created**
- Instance just created
- Not yet in any queue
- Transition: Immediately moves to Queued

**2. Queued (Pending)**
- Instance in `#Queues.Pending` queue
- Waiting to be dispatched
- Transition: Moves to Running when dispatched

**3. Running**
- **Broad state** that includes both:
  - Actively executing
  - Paused (temporarily stopped)
- Instance in `#Queues.Dispatch` queue
- Transition: Continues until exits or killed

**4. Paused (within Running)**
- Sub-state of Running
- Instance in `#Queues.Pause` queue
- Still considered "Running" (has not exited)
- Transition: Can resume back to active execution

**5. Exited**
- Instance completed or killed
- No longer in any queue
- Terminal state

---

### State Transitions

```
Created → Queued (automatic)
Queued → Running (dispatch)
Running → Paused (|Q.Pause)
Paused → Running (|Q.Resume)
Running → Exited (completion or |Q.Kill)
```

---

## Queue System

### Three System Queues

Polyglot has three built-in system queues:

| Queue | Purpose | Contains |
|-------|---------|----------|
| `#Queues.Pending` | Waiting to run | Queued instances |
| `#Queues.Dispatch` | Currently running | Running instances (executing or paused) |
| `#Queues.Pause` | Temporarily stopped | Paused instances (subset of Running) |

---

### `#Queues.Pending`

**Purpose:** Holds instances waiting to be dispatched

**Characteristics:**
- Instances queue here after creation
- FIFO (First-In-First-Out) by default
- Can have priority ordering
- Dispatch moves instances from Pending → Dispatch

```polyglot
// Instance created and queued
[r] |LongRunningTask
// Instance is now in #Queues.Pending

// Waiting for dispatch...
```

---

### `#Queues.Dispatch`

**Purpose:** Holds instances that are "running" (executing or paused)

**Characteristics:**
- Instances actively executing
- Includes paused instances (still "running")
- Instances stay here until completion or kill

```polyglot
// After dispatch, instance is in #Queues.Dispatch
// State: Running

// If paused
[Q] |Q.Pause
// Still in #Queues.Dispatch, but also tracked in #Queues.Pause
```

---

### `#Queues.Pause`

**Purpose:** Tracks paused instances (subset of Dispatch)

**Characteristics:**
- Instances temporarily stopped
- Still in Dispatch queue (still "Running")
- Can be resumed

```polyglot
[|] PausableTask
[i] .data: pg\string

[r] |Step1

// Conditionally pause
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

[r] |Step2  // Will run after resume

[X]
```

---

### Custom Queues

Users can define custom queues:

```polyglot
[#] Queues.Background
[<] .max_concurrent: pg\int << 5
[X]

[#] Queues.HighPriority
[<] .max_concurrent: pg\int << 10
[X]

// Assign instance to custom queue
[Q] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.Background
```

---

## Execution vs Running

### Critical Distinction

**Execution:**
- The act of **actively running code**
- Occurs when instance is in Dispatch and not paused
- Can be interrupted (paused)

**Running:**
- **Broad state** from dispatch until exit
- Includes both execution AND paused
- "Running" = "has not exited yet"

---

### Timeline Visualization

```
Instance Lifecycle:
├─ Created
├─ Queued (Pending)
└─ Running ────────────────────────────────► Exited
   │
   ├─ Executing ──┐
   ├─ Paused      ├─ All "Running"
   ├─ Executing ──┘
   └─ Exited
```

---

### Examples

**Example 1: Continuous Execution**
```
Created → Queued → Running (executing) → Exited
                   └──────────────────┘
                    "Running" state
```

**Example 2: With Pause**
```
Created → Queued → Running (executing) → Paused → Resumed (executing) → Exited
                   └────────────────────────────────────────────────┘
                              "Running" state (entire time)
```

---

### Code Example

```polyglot
[|] TaskWithPause
[i] .data: pg\string

// Dispatched - now "Running"
[r] |Step1  // Executing

[Q] |Q.Pause  // Still "Running" but paused

// ... pause duration ...

[Q] |Q.Resume  // Still "Running", now executing again

[r] |Step2  // Executing

[X]  // Exit - no longer "Running"
```

**Key Point:** From dispatch to exit, instance is always "Running", even when paused.

---

## Creating Instances

### Direct Pipeline Call

```polyglot
// Create and dispatch instance immediately
[r] |ProcessData
[<] .input: pg\string << "data"
```

**Lifecycle:** Created → Queued → Running (if capacity available)

---

### Trigger-Based Creation

```polyglot
[|] ScheduledTask
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

[r] |DoWork
[X]
```

**Lifecycle:** Trigger fires → Created → Queued → Running

**Result:** New instance created every day at 9:00 AM

---

### Multiple Instances

```polyglot
// Create 3 independent instances
[r] |ProcessFile
[<] .file_path: pg\path << "file1.txt"

[r] |ProcessFile
[<] .file_path: pg\path << "file2.txt"

[r] |ProcessFile
[<] .file_path: pg\path << "file3.txt"
```

**Result:** 3 independent instances, each with its own state

---

### Parallel Instance Creation

```polyglot
// Create instances in parallel
[p] |ProcessFileA
[<] .file: pg\path << "a.txt"

[p] |ProcessFileB
[<] .file: pg\path << "b.txt"

[p] |ProcessFileC
[<] .file: pg\path << "c.txt"

[Y] |Y.Join
// Wait for all instances to complete
```

---

## Instance Independence

### Independent State

Each instance has **its own state** - completely independent from other instances:

```polyglot
[|] Counter
[i] .start: pg\int
[r] .count: pg.mutable\int << .start

[r] |Increment
[<] .counter: pg.mutable\int << .count
[>] .result: pg\int >> .count

[X]

// Create 3 instances
[r] |Counter
[<] .start: pg\int << 0  // Instance 1: count = 0

[r] |Counter
[<] .start: pg\int << 10  // Instance 2: count = 10

[r] |Counter
[<] .start: pg\int << 100  // Instance 3: count = 100
```

**Result:** Each instance has independent `.count` variable.

---

### No Shared State

Instances **do not share mutable state**:

```polyglot
[|] ProcessData
[r] .local_state: pg.mutable\int << 0

[r] |IncrementState
// Modifies this instance's .local_state only
[X]

// Create 2 instances
[r] |ProcessData  // Instance 1: has own .local_state
[r] |ProcessData  // Instance 2: has own .local_state (independent)
```

**Key Point:** Thread-safe by design - no race conditions.

---

### Instance Identification

Each instance has a unique ID (implementation-specific):

```polyglot
// Hypothetical - get instance ID
[r] |Q.GetInstanceID
[>] .id: pg\string >> instance_id

// Use instance ID for control
[Q] |Q.Pause
[<] .instance_id: pg\string << instance_id
```

---

## Instance Termination

### Graceful Exit

Instance completes naturally when all operations finish:

```polyglot
[|] ShortTask
[r] |Step1
[r] |Step2
[r] |Step3
[X]  // Exits after Step3

// Instance lifecycle: Created → Queued → Running → Exited
```

---

### Forceful Termination

Use `|Q.Kill` to terminate instance immediately:

```polyglot
[|] LongTask
[r] |Step1

// Kill if condition met
[Q] |Q.KillIf.MemoryUsage.GreaterThan
[<] .mb: pg\uint << 4096

[r] |Step2  // May not execute if killed

[X]
```

---

### Self-Termination

Instance can kill itself:

```polyglot
[|] ConditionalTask
[i] .should_abort: pg\bool

[?] .should_abort =? #Boolean.True
[~][Q] |Q.Kill  // Kill self

[r] |ContinueWork  // Only runs if not aborted

[X]
```

---

### Error-Based Termination

Uncaught errors can terminate instance:

```polyglot
[|] RiskyTask
[r] |MightFail

// If error is not caught, instance terminates
// If error is caught, instance continues

[!] !SomeError
[r] |HandleError  // Instance continues

[X]
```

---

## Monitoring and Control

### Queue Status

Check queue state:

```polyglot
[r] |Q.Status
[<] .queue: #Queues << #Queues.Pending
[>] .count: pg\int >> pending_count

[r] |Q.Status
[<] .queue: #Queues << #Queues.Dispatch
[>] .count: pg\int >> running_count
```

---

### Instance Control

Control individual instances:

```polyglot
// Pause instance
[Q] |Q.Pause
[<] .instance_id: pg\string << target_instance

// Resume instance
[Q] |Q.Resume
[<] .instance_id: pg\string << target_instance

// Kill instance
[Q] |Q.Kill
[<] .instance_id: pg\string << target_instance

// Bump priority
[Q] |Q.PriorityBump
[<] .instance_id: pg\string << target_instance
```

---

### Conditional Control

Control based on system state:

```polyglot
// Pause if low memory
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

// Kill if taking too long
[Q] |Q.KillIf.Runtime.GreaterThan
[<] .seconds: pg\uint << 300

// Dispatch with priority if urgent
[Q] |Q.Dispatch.Priority.High
```

---

### Inter-Pipeline Control

One pipeline can control another:

```polyglot
[|] Worker
[i] .data: pg\string
[r] |ProcessData
[<] .input: pg\string << .data
[X]

[|] Manager
// Start worker
[r] |Worker
[<] .data: pg\string << "input"
[>] .worker_id: pg\string >> worker_instance

// Monitor worker
[r] |Q.Status
[<] .instance_id: pg\string << worker_instance
[>] .status: pg\string >> worker_status

// Control worker if needed
[?] .worker_status =? "overloaded"
[~][Q] |Q.Pause
[~][<] .instance_id: pg\string << worker_instance

[X]
```

---

## Best Practices

### 1. Design Stateless When Possible

```polyglot
// ✓ GOOD - Stateless (can run many instances safely)
[|] TransformData
[i] .input: pg\string
[r] |Transform
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]

// ⚠ CAUTION - Stateful (instances have mutable state)
[|] StatefulProcessor
[r] .state: pg.mutable\int << 0
[r] |UpdateState
[X]
```

---

### 2. Use Triggers for Recurring Tasks

```polyglot
// ✓ GOOD - Trigger creates instances automatically
[|] DailyReport
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

[r] |GenerateReport
[X]

// ✗ AVOID - Manual repeated calls
// [r] |DailyReport  // Have to call manually every day
```

---

### 3. Handle Instance Cleanup

```polyglot
// ✓ GOOD - Cleanup resources before exit
[|] ProcessWithResources
[i] .file: pg\path

[r] |OpenFile
[<] .path: pg\path << .file
[>] .handle: pg\int >> file_handle

[r] |ProcessFile
[<] .handle: pg\int << file_handle

// Cleanup (hypothetical cleanup block)
[/] |CloseFile
[/][<] .handle: pg\int << file_handle

[X]
```

---

### 4. Monitor Long-Running Instances

```polyglot
// ✓ GOOD - Timeout protection
[|] LongTask
[r] |Step1
[r] |Step2

[Q] |Q.KillIf.Runtime.GreaterThan
[<] .seconds: pg\uint << 600  // 10 minutes max

[X]
```

---

### 5. Use Queue Assignment for Priority

```polyglot
// ✓ GOOD - High priority tasks in separate queue
[|] UrgentTask
[Q] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.HighPriority

[r] |ProcessUrgent
[X]

// Normal priority uses default queue
[|] NormalTask
[r] |ProcessNormal
[X]
```

---

### 6. Understand Instance Independence

```polyglot
// ✓ CORRECT - Understanding independence
[|] IndependentWorker
[r] .count: pg\int << 0

[r] |Increment
// Only affects THIS instance's .count
[X]

// Create multiple independent instances
[r] |IndependentWorker  // Instance 1
[r] |IndependentWorker  // Instance 2
// Each has independent .count
```

---

### 7. Use Descriptive Pipeline Names

```polyglot
// ✓ GOOD - Clear purpose from name
[|] ProcessUserRegistration
[|] SendWelcomeEmail
[|] GenerateDailyReport

// ✗ AVOID - Unclear names
[|] Process1
[|] Task2
[|] Worker
```

---

### 8. Document Lifecycle Expectations

```polyglot
// ✓ GOOD - Document expected lifecycle
[|] BatchProcessor
// Lifecycle: Long-running, processes items until queue empty
// Expected runtime: 5-60 minutes
// Can be paused/resumed safely

[t] |T.Every.Hour
[<] .minute: pg\int << 0

[r] |ProcessBatch
[X]
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](syntax-complete.md) - Pipeline syntax
- [Block Markers](block-markers.md) - `[|]`, `[Q]` markers
- [Parallel Execution](parallel-execution.md) - Multiple instances

### Standard Library
- [Queue Control](../standard-library/queue-control.md) - `|Q.*` operations
- [Triggers](../standard-library/triggers.md) - Instance creation via triggers

### Examples
- [Complete Workflows](../examples/complete-workflows.md) - Multi-instance patterns

### Planning
- [Decision Log](../decision-log.md) - Lifecycle decisions (#26, #27)

---

**End of Pipeline Lifecycle Reference**