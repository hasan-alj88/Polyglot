---
audience: developer
type: algorithm
updated: 2026-04-29
status: stable
consumes:
  - PGE01064
  - PGE01065
  - PGE01066
  - PGE01067
  - PGE01068
  - PGE01069
  - PGE01070
---

# Queue Semantics Validation

The Queue Semantics Validation algorithm is responsible for ensuring that pipelines adhere to the structural definitions and resource constraints of the queues they are assigned to. This strict enforcement occurs at compile time, eliminating the possibility of runtime resource contradictions or illegal strategy modifications.

## Core Mechanism: Two-Pass Contextual Scope Binding

The algorithm operates in two passes over the Abstract Syntax Tree (AST):

### Pass 1: Queue Registry Harvesting

The compiler scans the entire token stream for Queue Definition blocks (`{Q} #Queue:QueueName`).
For every queue definition found, it parses the configuration and stores it in an internal `QueueRegistry` (a hash map keyed by the queue name). The registry tracks immutable constraints such as:
- `.strategy`
- `.host`
- `.maxInstances`
- `.maxConcurrent`
- `.killPropagation`
- `.maxWaitTime`
- `.resourceTags`

### Pass 2: Pipeline Execution Scanning and Contextual Validation

The compiler then scans Pipeline blocks (`{-} -PipelineName`).
When traversing a pipeline, the algorithm looks for Queue Assignment instructions within the `[Q]` section (e.g., `-Q.Assign"QueueName"`).

#### Lexical Scope Tracking
Queue assignments can be dynamically mapped inside conditional execution branches (`[?]`). To handle this, the algorithm maintains a stack-based **active queue pointer** mapped to the current Lexical Scope level.
- If a queue is assigned globally inside `[Q]`, it binds to scope level `1`.
- If a queue is assigned inside a `[?]` block, it binds to scope level `2`.
- When the lexer encounters an out-of-scope token (`[/]`), the active queue pointer for that nested scope is immediately dropped.

#### Instruction Verification
While inside the execution body (`[-]`), the algorithm tracks all property override attempts (e.g., `<maxInstances#int << 5`).
When an override attempt is detected, the algorithm:
1. Looks up the *currently active queue* from the scope context.
2. Fetches the queue's structural constraints from the `QueueRegistry`.
3. Performs a direct comparison between the pipeline's localized request and the queue's global constraints.

### Contradiction Triggers

If a pipeline attempts to override a defined queue limit, the algorithm throws the specific granular error associated with the property:
- **`PGE01064`**: Attempting to change `.strategy`.
- **`PGE01065`**: Requesting a different `.host`.
- **`PGE01066`**: `<maxInstances` strictly exceeds queue `.maxInstances`.
- **`PGE01067`**: `<maxConcurrent` strictly exceeds queue `.maxConcurrent`.
- **`PGE01068`**: Attempting to alter `.killPropagation`.
- **`PGE01069`**: `<maxWaitTime` strictly exceeds queue `.maxWaitTime`.
- **`PGE01070`**: Requesting `<resourceTags` that the assigned queue lacks.

## Code Implementation Reference

The algorithm is fully implemented in the Rust compiler core:
- **File**: `lib/aljam3/src/compiler/rules/queue_semantics.rs`
- **Struct**: `QueueSemanticsAlgorithm`
- **Pass Structure**: Executed sequentially as "Algorithm 3.5" in the validation pipeline, ensuring it runs after baseline file and syntax validation, but before variable state validation.
