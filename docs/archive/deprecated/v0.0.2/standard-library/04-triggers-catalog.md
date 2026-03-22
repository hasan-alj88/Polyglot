---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
---

# Triggers Catalog (|T.*)

**Version:** 0.0.2
**Status:** Catalog Only - APIs To Be Determined
**Block Marker:** `[t]` (trigger/conditional blocks)

## Overview

The `|T.*` namespace provides event-driven and conditional execution utilities that work with the `[t]` trigger block marker. Triggers enable reactive programming patterns, event handling, and sophisticated conditional logic beyond simple boolean conditions.

### Philosophy

- **Event-Driven:** React to events rather than poll for changes
- **Declarative:** Express what conditions matter, not how to check them
- **Composable:** Combine triggers for complex conditional logic
- **Non-Blocking:** Triggers should not block pipeline execution unnecessarily
- **Type-Safe:** All trigger conditions respect Polyglot's type system

### Documentation Status

**Current Version (0.0.2):** This is a catalog-only reference. Each trigger category is listed with its intended purpose and scope. Specific function signatures, parameters, and detailed examples will be provided in future documentation versions.

**Future Versions:** Complete API specifications, parameter details, error handling, timing semantics, and comprehensive examples.

## Trigger Block Marker: [t]

The `[t]` block marker establishes conditional execution contexts. Code within or after a `[t]` block executes only when the condition is satisfied.

**Basic Usage:**
```polyglot
[t] .condition: pg\bool << (value > 10)
[r] |ExecuteWhenTrue
// Executes only if condition is true
```

**With Trigger Operations:**
```polyglot
[t] |T.Timer.After
[<] .milliseconds: pg\int << 5000
[>] .triggered: pg\bool >> timer_fired

// Following code executes after 5000ms
[r] |DelayedOperation
```

See [Block Markers](../language/06-block-markers.md) for complete `[t]` documentation.

## Trigger Categories

### |T.Timer.*

**Purpose:** Time-based trigger conditions

**Intended Scope:**
- Delay execution for specified duration
- Execute at specific time
- Periodic/recurring execution
- Timeout detection
- Debouncing and throttling
- Interval-based triggers

**Example Operations (APIs TBD):**
- `|T.Timer.After` - Trigger after delay
- `|T.Timer.At` - Trigger at specific time
- `|T.Timer.Every` - Trigger periodically
- `|T.Timer.Timeout` - Trigger if operation exceeds time
- `|T.Timer.Debounce` - Trigger after quiet period
- `|T.Timer.Throttle` - Trigger at maximum rate

**Conceptual Example:**
```polyglot
// Execute after 5 second delay
[t] |T.Timer.After
[<] .milliseconds: pg\int << 5000
[>] .triggered: pg\bool >> timer_done

[r] |DelayedOperation
```

### |T.Event.*

**Purpose:** Event-based triggers from external sources

**Intended Scope:**
- File system events (file created, modified, deleted)
- Network events (connection, data received)
- User input events
- System events (signal, interrupt)
- Custom application events
- Event pattern matching

**Example Operations (APIs TBD):**
- `|T.Event.FileCreated` - Trigger on file creation
- `|T.Event.FileModified` - Trigger on file modification
- `|T.Event.Signal` - Trigger on system signal
- `|T.Event.Custom` - Trigger on custom event
- `|T.Event.Pattern` - Trigger on event pattern match

**Conceptual Example:**
```polyglot
// Execute when file is created
[t] |T.Event.FileCreated
[<] .path: pg\path << \\Path\\.unix << "/watch/folder"
[>] .triggered: pg\bool >> file_created
[>] .file_path: pg\path >> created_file

[r] |ProcessNewFile
[<] .path: pg\path << created_file
```

### |T.Condition.*

**Purpose:** Advanced conditional logic beyond simple boolean checks

**Intended Scope:**
- Value range checks
- Pattern matching conditions
- Collection conditions (any, all, none)
- Type-based conditions
- Comparison chains
- Null/empty checks
- Composite conditions (AND, OR, XOR)

**Example Operations (APIs TBD):**
- `|T.Condition.InRange` - Value within range
- `|T.Condition.Matches` - Pattern matching
- `|T.Condition.Any` - Any element satisfies condition
- `|T.Condition.All` - All elements satisfy condition
- `|T.Condition.None` - No elements satisfy condition
- `|T.Condition.IsEmpty` - Collection is empty
- `|T.Condition.And` - Logical AND of conditions
- `|T.Condition.Or` - Logical OR of conditions

**Conceptual Example:**
```polyglot
// Trigger if value in range
[t] |T.Condition.InRange
[<] .value: pg\int << user_input
[<] .min: pg\int << 0
[<] .max: pg\int << 100
[>] .satisfied: pg\bool >> in_range

[r] |ProcessValidInput
```

### |T.State.*

**Purpose:** State change detection and monitoring

**Intended Scope:**
- Detect when value changes
- Trigger on state transitions
- Edge detection (rising/falling)
- Threshold crossing detection
- Change rate monitoring
- State history tracking

**Example Operations (APIs TBD):**
- `|T.State.Changed` - Value changed from previous
- `|T.State.Transition` - State transitioned between values
- `|T.State.RisingEdge` - Value went from false to true
- `|T.State.FallingEdge` - Value went from true to false
- `|T.State.Threshold` - Value crossed threshold
- `|T.State.RateOfChange` - Change rate exceeds limit

**Conceptual Example:**
```polyglot
// Trigger when value crosses threshold
[t] |T.State.Threshold
[<] .value: pg\float << current_temperature
[<] .threshold: pg\float << 100.0
[<] .direction: pg\string << "rising"
[>] .triggered: pg\bool >> threshold_crossed

[r] |HandleOverheat
```

### |T.Data.*

**Purpose:** Data availability and readiness triggers

**Intended Scope:**
- Wait for data availability
- Trigger when queue has items
- Trigger when resource is ready
- Data validation triggers
- Buffer full/empty detection
- Stream readiness

**Example Operations (APIs TBD):**
- `|T.Data.Available` - Data is available
- `|T.Data.QueueNotEmpty` - Queue has items
- `|T.Data.Ready` - Resource is ready
- `|T.Data.Valid` - Data passes validation
- `|T.Data.BufferFull` - Buffer reached capacity
- `|T.Data.StreamReady` - Stream has data

**Conceptual Example:**
```polyglot
// Wait for queue to have data
[t] |T.Data.QueueNotEmpty
[<] .queue_name: pg\string << #AppQueues.Processing
[>] .has_data: pg\bool >> queue_ready

[r] |ProcessNextItem
```

### |T.Resource.*

**Purpose:** System resource availability triggers

**Intended Scope:**
- CPU availability
- Memory availability
- Disk space availability
- Network bandwidth availability
- Lock acquisition
- Resource quota checks

**Example Operations (APIs TBD):**
- `|T.Resource.CPUAvailable` - CPU below threshold
- `|T.Resource.MemoryAvailable` - Memory available
- `|T.Resource.DiskSpace` - Disk space available
- `|T.Resource.LockAcquired` - Lock obtained
- `|T.Resource.NetworkReady` - Network accessible

**Conceptual Example:**
```polyglot
// Wait for CPU availability
[t] |T.Resource.CPUAvailable
[<] .max_usage_percent: pg\int << 80
[>] .available: pg\bool >> cpu_ready

[r] |StartIntensiveOperation
```

### |T.Pipeline.*

**Purpose:** Pipeline instance state triggers

**Intended Scope:**
- Trigger when pipeline completes
- Trigger on pipeline state change
- Trigger when pipeline count reaches threshold
- Pipeline error detection
- Pipeline timeout detection

**Example Operations (APIs TBD):**
- `|T.Pipeline.Completed` - Pipeline instance finished
- `|T.Pipeline.StateChanged` - Pipeline changed state
- `|T.Pipeline.CountReached` - Number of instances threshold
- `|T.Pipeline.Failed` - Pipeline encountered error
- `|T.Pipeline.Timeout` - Pipeline exceeded time limit

**Conceptual Example:**
```polyglot
// Wait for pipeline completion
[t] |T.Pipeline.Completed
[<] .instance_id: pg\string << processing_instance
[>] .completed: pg\bool >> is_done
[>] .result: pg\serial >> pipeline_result

[r] |HandleResults
[<] .data: pg\serial << pipeline_result
```

### |T.Network.*

**Purpose:** Network-related triggers

**Intended Scope:**
- HTTP request received
- WebSocket message received
- TCP/UDP data received
- Connection established/closed
- Network error detection
- API endpoint availability

**Example Operations (APIs TBD):**
- `|T.Network.HTTPRequest` - HTTP request received
- `|T.Network.WebSocketMessage` - WebSocket message
- `|T.Network.TCPData` - TCP data received
- `|T.Network.Connected` - Connection established
- `|T.Network.Disconnected` - Connection closed
- `|T.Network.Reachable` - Endpoint reachable

**Conceptual Example:**
```polyglot
// Trigger on HTTP request
[t] |T.Network.HTTPRequest
[<] .endpoint: pg\string << "/api/webhook"
[>] .received: pg\bool >> request_received
[>] .body: pg\string >> request_body

[r] |HandleWebhook
[<] .payload: pg\string << request_body
```

### |T.Combine.*

**Purpose:** Composite triggers combining multiple conditions

**Intended Scope:**
- Logical AND/OR/XOR combinations
- Sequential trigger chains
- Parallel trigger synchronization
- Trigger counting (N of M)
- Priority-based triggers
- Mutual exclusion triggers

**Example Operations (APIs TBD):**
- `|T.Combine.And` - All triggers must fire
- `|T.Combine.Or` - Any trigger fires
- `|T.Combine.Sequence` - Triggers fire in order
- `|T.Combine.Parallel` - All triggers fire concurrently
- `|T.Combine.NofM` - N out of M triggers fire
- `|T.Combine.First` - First trigger wins

**Conceptual Example:**
```polyglot
// Trigger when both conditions met
[t] |T.Combine.And
[<] .conditions: pg\array{pg\bool} << [condition1, condition2]
[>] .all_true: pg\bool >> both_satisfied

[r] |ExecuteWhenBothTrue
```

## Usage Patterns

### Pattern 1: Simple Time Delay

```polyglot
// Conceptual example (API TBD)
[|] DelayedAlert
[i] .message: pg\string

// Wait 5 seconds
[t] |T.Timer.After
[<] .milliseconds: pg\int << 5000

// Execute after delay
[r] |SendAlert
[<] .text: pg\string << .message

[X]
```

### Pattern 2: File System Watcher

```polyglot
// Conceptual example (API TBD)
[|] FileWatcher
[i] .watch_path: pg\path

// Trigger on new file
[t] |T.Event.FileCreated
[<] .path: pg\path << .watch_path
[>] .new_file: pg\path >> created_file

// Process the file
[r] |ProcessFile
[<] .file: pg\path << created_file
[>] .result: pg\string >> processing_result

[o] .result: pg\string << processing_result
[X]
```

### Pattern 3: Conditional Execution with Validation

```polyglot
// Conceptual example (API TBD)
[|] ValidatedOperation
[i] .input: pg\int

// Check if input in valid range
[t] |T.Condition.InRange
[<] .value: pg\int << .input
[<] .min: pg\int << 0
[<] .max: pg\int << 1000
[>] .valid: pg\bool >> is_valid

// Only execute if valid
[r] |ProcessValidInput
[<] .value: pg\int << .input
[>] .result: pg\int >> processed

[o] .output: pg\int << processed
[X]
```

### Pattern 4: Resource-Aware Execution

```polyglot
// Conceptual example (API TBD)
[|] ResourceAwareTask
[i] .data: pg\string

// Wait for available resources
[t] |T.Resource.CPUAvailable
[<] .max_usage_percent: pg\int << 70

[t] |T.Resource.MemoryAvailable
[<] .min_available_mb: pg\int << 512

// Execute intensive operation
[r] |IntensiveProcessing
[<] .input: pg\string << .data
[>] .result: pg\string >> final_result

[o] .result: pg\string << final_result
[X]
```

### Pattern 5: Composite Trigger

```polyglot
// Conceptual example (API TBD)
[|] MultiConditionTask
[i] .value: pg\int
[i] .timeout_ms: pg\int

// Combine time and value conditions
[t] |T.Combine.Or

[~][t] |T.Timer.After
[<] .milliseconds: pg\int << .timeout_ms

[~][t] |T.Condition.InRange
[<] .value: pg\int << .value
[<] .min: pg\int << 100
[<] .max: pg\int << 200

// Execute if either condition met
[r] |HandleCondition
[>] .result: pg\string >> outcome

[o] .result: pg\string << outcome
[X]
```

### Pattern 6: Pipeline Completion Synchronization

```polyglot
// Conceptual example (API TBD)
[|] SynchronizedWorkflow
[i] .worker_instance: pg\string

// Start dependent operation
[r] |StartBackgroundWork
[>] .instance_id: pg\string >> background_id

// Wait for completion
[t] |T.Pipeline.Completed
[<] .instance_id: pg\string << background_id
[>] .result: pg\serial >> background_result

// Process results
[r] |ProcessResults
[<] .data: pg\serial << background_result
[>] .final: pg\string >> final_output

[o] .output: pg\string << final_output
[X]
```

## Integration with Standard Library

### With Queue Control

```polyglot
// Conceptual: Resume pipeline when resources available
[t] |T.Resource.MemoryAvailable
[<] .min_available_mb: pg\int << 1024

[Q] |Q.Resume
[<] .instance_id: pg\string << paused_instance
```

### With Runtime Wrappers

```polyglot
// Conceptual: Execute Python when data ready
[t] |T.Data.Available
[<] .source: pg\string << data_source

[w] |W.Python3.11
[r] |PythonProcessor
[<] .input: pg\string << data_source
```

### With Error Handling

```polyglot
// Conceptual: Timeout detection with error
[t] |T.Timer.Timeout
[<] .milliseconds: pg\int << 30000
[<] .operation_id: pg\string << op_id
[>] .timed_out: pg\bool >> did_timeout

[!] !TimeoutError
[<] .message: pg\string << "Operation exceeded 30s"
[<] .code: pg\int << 5001
```

### With Parallel Execution

```polyglot
// Conceptual: Parallel triggers
[p] |WaitForCondition1
[~][t] |T.Timer.After
[<] .milliseconds: pg\int << 1000
[>] .result >> result1

[p] |WaitForCondition2
[~][t] |T.Event.FileCreated
[<] .path: pg\path << watch_path
[>] .result >> result2

[Y] |Y.Join
[>] result1
[>] result2
```

## Design Principles

### 1. **Declarative Conditions**
Express what conditions matter, not how to check them.

### 2. **Non-Blocking**
Triggers should not busy-wait or block unnecessarily.

### 3. **Composable**
Triggers combine naturally for complex conditions.

### 4. **Explicit State**
Trigger state and results are explicit, not hidden.

### 5. **Resource Efficient**
Triggers use minimal resources while waiting.

### 6. **Type Safe**
All trigger inputs/outputs are strongly typed.

### 7. **Debuggable**
Trigger state should be observable and inspectable.

## Trigger vs. [t] Block

**[t] Block with Boolean:**
- Simple conditional execution
- Evaluate boolean expression
- No waiting or event handling

```polyglot
[t] .condition: pg\bool << (value > 10)
[r] |Execute  // Executes immediately if true
```

**|T.* Trigger Operations:**
- Event-driven execution
- Time-based delays
- Resource waiting
- State monitoring

```polyglot
[t] |T.Timer.After
[<] .milliseconds: pg\int << 5000
// Waits 5 seconds before continuing
[r] |Execute
```

**Guideline:** Use simple `[t]` blocks for immediate boolean checks. Use `|T.*` for events, delays, or monitoring.

## Future Documentation

Upcoming documentation versions will provide:

1. **Complete API Specifications:**
   - Input parameters with types
   - Output values with types
   - Timing semantics
   - Resource usage characteristics
   - Error conditions

2. **Comprehensive Examples:**
   - Real-world trigger scenarios
   - Composite trigger patterns
   - Performance considerations
   - Error handling

3. **Best Practices:**
   - When to use triggers vs. polling
   - Trigger composition strategies
   - Resource management
   - Testing trigger-based code

4. **Performance Characteristics:**
   - Trigger overhead
   - Resource consumption
   - Scalability considerations
   - Optimization techniques

## Common Use Cases

### Use Case 1: Scheduled Tasks
Execute operations at specific times or intervals.

### Use Case 2: Event Handlers
React to file system, network, or system events.

### Use Case 3: Resource Management
Wait for system resources before intensive operations.

### Use Case 4: Timeout Handling
Detect and handle operations that exceed time limits.

### Use Case 5: State Machines
Trigger transitions based on state changes.

### Use Case 6: Workflow Orchestration
Coordinate multiple pipelines based on completion events.

### Use Case 7: Rate Limiting
Control execution frequency with throttling/debouncing.

### Use Case 8: Condition Synchronization
Wait for multiple conditions before proceeding.

## Quick Reference: Trigger Categories

| Namespace | Purpose | Status |
|-----------|---------|--------|
| `|T.Timer.*` | Time-based triggers | APIs TBD |
| `|T.Event.*` | Event-based triggers | APIs TBD |
| `|T.Condition.*` | Advanced conditional logic | APIs TBD |
| `|T.State.*` | State change detection | APIs TBD |
| `|T.Data.*` | Data availability triggers | APIs TBD |
| `|T.Resource.*` | Resource availability | APIs TBD |
| `|T.Pipeline.*` | Pipeline state triggers | APIs TBD |
| `|T.Network.*` | Network-related triggers | APIs TBD |
| `|T.Combine.*` | Composite triggers | APIs TBD |

## See Also

- [Block Markers](../language/06-block-markers.md) - Complete `[t]` block documentation
- [Standard Library Overview](00-overview.md) - Complete stdlib organization
- [Queue Control](02-queue-control.md) - Pipeline instance management
- [Pipeline Lifecycle](../language/10-pipeline-lifecycle.md) - Instance states

---

**Navigation:**
← [Utilities Catalog](03-utilities-catalog.md) | [Standard Library Index](00-overview.md) | [Join Operations](05-join-operations.md) →