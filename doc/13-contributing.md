# Pipeline Execution Model

[← Back to README](../README.md)

## Table of Contents
- [Overview](#overview)
- [Execution Flow](#execution-flow)
- [Execution Phases](#execution-phases)
- [Parallel Execution](#parallel-execution)
- [Resource Management](#resource-management)
- [Variable Scoping](#variable-scoping)
- [Performance Considerations](#performance-considerations)

## Overview

Polyglot pipelines execute through a well-defined sequence of phases, from trigger activation to final output. Understanding this execution model is crucial for writing efficient and correct pipelines.

**Key Principles:**
- **Sequential by default**—Operations run in order unless explicitly forked
- **Explicit dependencies**-Data flow determines execution order
- **Resource-aware**-Execution respects resource limits
- **Asynchronous**CAll operations are inherently async
- **Isolated errors**—Errors in one branch don't affect others

## Execution Flow

### Complete Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ Stage 1: Trigger & Input Monitoring (Parallel)             │
├─────────────────────────────────────────────────────────────┤
│ [i] → ◇Provided (implicit trigger)                          │
│ [t] → ◇Triggered (explicit condition)                       │
│ ...more triggers monitored in parallel                      │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 2: Activation Gate                                    │
├─────────────────────────────────────────────────────────────┤
│ ALL conditions met? → [x] gate check                        │
│   No  → Pipeline does not activate                          │
│   Yes → Continue ↓                                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 3: Queue Evaluation (Parallel)                        │
├─────────────────────────────────────────────────────────────┤
│ [Q] → ◇Checked (resource conditions, priority, limits)      │
│ ...all queue conditions checked in parallel                 │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 4: Queue Decision Gate                                │
├─────────────────────────────────────────────────────────────┤
│ ALL queue conditions pass?                                  │
│   No  → Enter queue, wait for resources                     │
│   Yes → Begin execution ↓                                   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 5: Setup Phase (Sequential)                           │
├─────────────────────────────────────────────────────────────┤
│ [\] → [\] → ... → [\]                                       │
│  ↓1   ↓2         ↓n                                         │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 6: Run Phase (Sequential + Parallel)                  │
├─────────────────────────────────────────────────────────────┤
│ [r] → [r] → ... → [r]  (Sequential)                         │
│  ↓1   ↓2         ↓n                                         │
│                                                             │
│ [f] → Branch1 ┐                                             │
│ [f] → Branch2 ├─ (Parallel)                                 │
│ [f] → Branch3 ┘                                             │
│ [b] → Background (Fire-and-forget)                          │
│                                                             │
│ ↓ Kill Monitor (parallel throughout Run Phase)              │
│   Checks: CPU, Memory, Time                                 │
│   Actions: Graceful/Immediate Kill                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 7: Join Phase                                         │
├─────────────────────────────────────────────────────────────┤
│ [j] / [Y] ← (all [f] branches converge)                     │
│ Note: [b] branches do NOT join                              │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 8: Cleanup Phase (Sequential)                         │
├─────────────────────────────────────────────────────────────┤
│ [/] → [/] → ... → [/]                                       │
│  ↓1   ↓2         ↓n                                         │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 9: Output Phase (Sequential)                          │
├─────────────────────────────────────────────────────────────┤
│ [o] → [o] → ... → [o]                                       │
│  ↓1   ↓2         ↓n                                         │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 10: Performance Metrics Collection                    │
├─────────────────────────────────────────────────────────────┤
│ Capture: CPU usage, Memory peak, Duration                   │
│ Store: Database for analysis                                │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Stage 11: Pipeline Termination                              │
├─────────────────────────────────────────────────────────────┤
│ [x] (pipeline end)                                          │
└─────────────────────────────────────────────────────────────┘
```

## Execution Phases

### Phase 1: Trigger Monitoring

All triggers are monitored in parallel. The pipeline activates only when **ALL** triggers are satisfied.

```polyglot
[|] MultiTrigger
[i] data: py\dict          \\ Implicit trigger: data provided
[t] |T.FileCreated << "*.csv"  \\ Explicit: file created
[t] |T.CpuLow << 50.0      \\ Explicit: CPU below 50%
```

All three conditions must be true:
1. Data is provided as input
2. A .csv file is created
3. CPU usage is below 50%

### Phase 2: Queue Evaluation

Queue conditions are checked in parallel:

```polyglot
[Q] |Q.Priority << 2
[Q] |Q.CpuAvailable << 75.0
[Q] |Q.MemoryAvailable << 8192
```

If all pass, execution begins immediately. Otherwise, pipeline enters queue.

### Phase 3-5: Setup Phase

Setup runs **sequentially** - each operation waits for the previous one:

```polyglot
[\] |ConnectDatabase >> db_conn
[\] |LoadConfiguration >> config
[\] |InitializeResources << config >> resources
```

Execution order: ConnectDatabase → LoadConfiguration → InitializeResources

### Phase 6: Run Phase

The main execution phase supports both sequential and parallel operations.

**Sequential:**
```polyglot
[r] |Step1 >> result1
[r] |Step2 << result1 >> result2
[r] |Step3 << result2 >> result3
```

**Parallel (Forks):**
```polyglot
[f] |ProcessA >> resultA
[f] |ProcessB >> resultB
[f] |ProcessC >> resultC
[j] |JoinAll
```

**Background:**
```polyglot
[b] |LogMetrics  \\ Fire-and-forget, doesn't join
```

### Phase 7: Join

All forked branches must complete before continuing:

```polyglot
[j] |JoinAll     \\ Wait for all [f] branches
[j] |JoinFirst   \\ Continue when first completes
[j] |JoinLast    \\ Continue when last completes
[j] |JoinNth << 2  \\ Continue when 2nd completes
```

Background branches (`[b]`) never join - they run independently.

### Phase 8: Cleanup

Cleanup runs **sequentially**, even if errors occurred:

```polyglot
[/] |DisconnectDatabase << db_conn
[/] |FreeResources << resources
[/] |LogCompletion
```

### Phase 9: Output

Outputs are collected **sequentially**:

```polyglot
[o] >> result
[o] >> metadata
[o] >> timestamp
```

## Parallel Execution

### Fork Semantics

Forked branches execute in parallel, each with isolated state:

```polyglot
[r] shared_data: py\dict >> data

[f] |Branch1
[~][r] data: py\dict >> |Transform1 >> result1
[~][o] >> result1

[f] |Branch2
[~][r] data: py\dict >> |Transform2 >> result2
[~][o] >> result2

[j] |JoinAll
[r] |Combine << result1 << result2 >> final
```

**Key Points:**
- Each branch gets a **copy** of `data` (immutable)
- Branches cannot modify shared state
- Results are collected after join
- Prevents race conditions by design

### Nested Forks

Forks can be nested:

```polyglot
[f] |Branch1
[~][r] |Operation1
[~][f] |NestedBranch1A
[~][~][r] |SubOperation1
[~][f] |NestedBranch1B
[~][~][r] |SubOperation2
[~][j] |JoinAll
[~][r] |CombineNested

[f] |Branch2
[~][r] |Operation2

[j] |JoinAll
```

### Chaining Operators

The `<|<` operator creates explicit dependencies between parallel branches:

```polyglot
[r] |DataValidation >> validated_data

[f] |ProcessA <|< DataValidation
[~][r] validated_data >> |TransformA >> resultA

[f] |ProcessB <|< DataValidation
[~][r] validated_data >> |TransformB >> resultB

[j] |JoinAll
```

Both `ProcessA` and `ProcessB` wait for `DataValidation` to complete before starting.

## Resource Management

### Kill Condition Monitoring

During execution, the Kill Condition Manager continuously monitors:

```polyglot
[Q] |Q.Kill.CPULimit << 90.0
[Q] |Q.Kill.MemoryLimit << 95.0
[Q] |Q.Kill.ExecTimeout << T"30:"
[Q] |Q.Kill.Strategy << #KillStrategy.Graceful
```

**Monitoring Flow:**
```
Pipeline Start
     ↓
Resource Watcher provides metrics
     ↓
Kill Condition Manager checks thresholds
     ↓
Threshold Exceeded?
  ├─ No → Continue monitoring
  └─ Yes → Execute kill strategy
           ├─ Graceful: SIGTERM → wait → SIGKILL
           ├─ Immediate: SIGKILL
           ├─ CustomHandler: Cleanup → SIGTERM → SIGKILL
           └─ Degrade: Pause → Free resources → Resume/Kill
```

### Resource Lifecycle

```
Setup Phase:
  - Acquire database connections
  - Open file handles
  - Allocate memory
  - Initialize language runtimes

Run Phase:
  - Use resources
  - Monitor usage
  - Enforce limits

Cleanup Phase (even on error):
  - Close connections
  - Release file handles
  - Free memory
  - Shutdown runtimes
```

## Variable Scoping

### Global Pipeline Scope

Variables defined in `[i]` are available throughout the pipeline:

```polyglot
[|] Example
[i] config: py\dict  \\ Available everywhere

[\] |LoadData << config
[r] |ProcessData << config
[/] |SaveResults << config
```

### Sequential Scope

Variables flow sequentially:

```polyglot
[r] |Step1 >> result1       \\ result1 created
[r] |Step2 << result1 >> result2  \\ result1 available
[r] |Step3 << result2 >> result3  \\ result2 available, result1 still in scope
```

### Fork Scope

Forked branches receive copies of variables:

```polyglot
[r] data: py\dict << original_data

[f] |Branch1
[~][r] data >> |Modify1 >> modified1  \\ Modifies copy

[f] |Branch2
[~][r] data >> |Modify2 >> modified2  \\ Modifies different copy

[j] |JoinAll

\\ original_data unchanged
\\ modified1 and modified2 available after join
```

### Join Scope

After join, variables from all branches are available:

```polyglot
[f] |Branch1
[~][o] >> result1

[f] |Branch2
[~][o] >> result2

[j] |JoinAll

[r] |Combine << result1 << result2 >> final
```

## Performance Considerations

### Execution Overhead

| Operation | Overhead |
|-----------|----------|
| Pipeline activation | ~100ms |
| Queue dispatch | ~50ms |
| Fork creation | ~10ms per fork |
| Join synchronization | ~5ms |
| Type conversion | ~1-10ms depending on complexity |
| Error handler check | ~1ms |
| Switch evaluation | ~2ms |

### Optimization Strategies

**1. Minimize Forks:**
```polyglot
\\ ❌ Excessive forking
[f] |Task1
[f] |Task2
[f] |Task3
[j] |JoinAll

\\ ✅ Group related work
[f] |TaskGroup1And2
[~][r] |Task1
[~][r] |Task2
[f] |Task3
[j] |JoinAll
```

**2. Use Background Branches for Non-Critical Work:**
```polyglot
[r] |CriticalOperation >> result

[b] |LogMetrics  \\ Don't wait for this
[b] |UpdateCache

[o] >> result  \\ Return immediately
```

**3. Cache Type Conversions:**
```polyglot
\\ Conversion cached automatically
[r] py_data: py\dict >> |RustOperation >> rust_result
[r] rust_result >> |AnotherRustOp  \\ No reconversion
```

**4. Reuse Runtime Processes:**
Polyglot keeps language runtimes warm in process pools to avoid cold start overhead.

### Execution Metrics

Every pipeline execution captures:
- **CPU time** - Actual CPU seconds used
- **Wall clock time** - Real-world elapsed time
- **Memory peak** - Maximum memory usage
- **Conversion overhead** - Time spent on type conversions
- **Fork count** - Number of parallel branches
- **Join latency** - Time waiting for slowest branch

These metrics help identify bottlenecks and optimize pipelines.

---

[Next: Language Integration →](11-language-integration.md)