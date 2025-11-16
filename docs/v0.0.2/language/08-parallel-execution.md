# Parallel Execution

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot's parallel execution system allows multiple operations to run concurrently while maintaining type safety and preventing race conditions. Parallel blocks use copy semantics to ensure thread-safe execution.

**Key Features:**
- `[p]` parallel blocks as mini-pipelines
- Copy-in semantics (implicit from outer scope)
- Explicit copy-out with `[>]` and `>>`
- `[Y]` join blocks for synchronization
- Thread-safe by design
- No shared mutable state

---

## Table of Contents

1. [Parallel Block Basics](#parallel-block-basics)
2. [Copy-In Semantics](#copy-in-semantics)
3. [Copy-Out Semantics](#copy-out-semantics)
4. [Join Blocks](#join-blocks)
5. [Variable Lifetime](#variable-lifetime)
6. [Multiple Parallel Blocks](#multiple-parallel-blocks)
7. [Nested Operations](#nested-operations)
8. [Error Handling in Parallel](#error-handling-in-parallel)
9. [Best Practices](#best-practices)
10. [Common Patterns](#common-patterns)

---

## Parallel Block Basics

### The `[p]` Block Marker

**Purpose:** Executes operation as a mini-pipeline in parallel with other `[p]` blocks

**Characteristics:**
- Independent execution
- Copy-in semantics from outer scope
- Explicit copy-out required
- Thread-safe by design

---

### Mini-Pipeline Model

Each `[p]` block is a **mini-pipeline** - a lightweight pipeline that:
- Has its own scope
- Copies data IN from outer scope
- Must explicitly copy data OUT
- Runs concurrently with other parallel blocks

```polyglot
[|] ParallelExample
[i] .data: pg\string

// Mini-pipeline 1
[p] |ProcessPartA
[<] .input: pg\string << .data
[>] .output >> result_a

// Mini-pipeline 2
[p] |ProcessPartB
[<] .input: pg\string << .data
[>] .output >> result_b

[X]
```

---

### Basic Parallel Example

```polyglot
[|] ProcessInParallel
[i] .data: pg\string

// Parallel block 1
[p] |TransformA
[<] .input: pg\string << .data
[>] .result: pg\string >> result1

// Parallel block 2
[p] |TransformB
[<] .input: pg\string << .data
[>] .result: pg\string >> result2

// Join to synchronize
[Y] |Y.Join
[>] result1
[>] result2

// Use synchronized results
[r] |CombineResults
[<] .a: pg\string << result1
[<] .b: pg\string << result2
[>] .final: pg\string >> output

[X]
```

---

## Copy-In Semantics

### Implicit Copy-In

Variables from outer scope are **implicitly copied** into parallel blocks when accessed.

```polyglot
[|] CopyInExample
[r] .shared_data: pg\string << "original"

[p] |Process
[<] .input: pg\string << .shared_data  // Implicit copy FROM outer scope
[>] .result: pg\string >> output1

[X]
```

**Key Point:** `.shared_data` is **copied** into the parallel block, not shared directly.

---

### Copy Not Reference

Parallel blocks receive **copies** of data, not references:

```polyglot
[|] CopyVsReference
[r] .counter: pg\int << 10

// Parallel block 1
[p] |IncrementA
[<] .value: pg\int << .counter  // Copies 10
// Modifying .value here does NOT affect outer .counter

// Parallel block 2
[p] |IncrementB
[<] .value: pg\int << .counter  // Also copies 10
// Independent from block 1

[X]
```

**Result:** Both blocks start with `10`, modifications are independent.

---

### Why Copy Semantics?

**Thread Safety:**
- No race conditions
- No need for locks
- Predictable behavior
- Easier to reason about

**Trade-off:**
- Memory overhead for large data
- Explicit copy-out required
- Cannot directly modify outer scope

---

## Copy-Out Semantics

### Explicit Copy-Out Required

To get data OUT of a parallel block, use `[>]` with `>>` operator:

```polyglot
[p] |ProcessData
[<] .input: pg\string << input_data
// ... processing ...
[>] .output >> result  // EXPLICIT copy OUT to outer scope
```

---

### Copy-Out Syntax

**Format:**
```polyglot
[>] .field >> outer_variable
```

**Example:**
```polyglot
[|] CopyOutExample
[r] .result: pg\string << ""

[p] |Process
[<] .input: pg\string << "data"
[>] .output >> result  // Copy OUT to .result

// After join, .result contains the output
[Y] |Y.Join
[>] result

[X]
```

---

### Multiple Copy-Outs

A parallel block can copy out multiple values:

```polyglot
[p] |ProcessMultiple
[<] .input: pg\string << data
[>] .result1 >> output1
[>] .result2 >> output2
[>] .status >> status_code
```

---

### Copy-Out is Not Immediate

**Important:** Copy-out happens at join time, not immediately.

```polyglot
[|] TimingExample
[r] .result: pg\string << "initial"

[p] |SlowProcess
[<] .input: pg\string << "data"
[>] .output >> result  // Marked for copy-out

// At this point, .result is still "initial"
[r] .current: pg\string << result  // Still "initial"

// Join happens here
[Y] |Y.Join
[>] result  // NOW .result gets the value

// Now .result has the value from parallel block
[r] .updated: pg\string << result  // Has processed value

[X]
```

---

## Join Blocks

### The `[Y]` Join Marker

**Purpose:** Synchronizes variables from parallel scopes to outer scope

**Always paired with:** `|Y.Join` pipeline

**Syntax:**
```polyglot
[Y] |Y.Join
[>] variable1
[>] variable2
```

---

### Basic Join Example

```polyglot
[|] JoinExample
[i] .data: pg\string

// Initialize result variables
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

// Parallel blocks
[p] |ProcessA
[<] .input: pg\string << .data
[>] .output >> result1

[p] |ProcessB
[<] .input: pg\string << .data
[>] .output >> result2

// Synchronize
[Y] |Y.Join
[>] result1
[>] result2

// Now both results are available
[r] |UseResults
[<] .a: pg\string << result1
[<] .b: pg\string << result2

[X]
```

---

### Why `[>]` in Join Blocks?

Join uses `[>]` (not `[<]`) because we're **pulling/extracting FROM** parallel scopes:

```polyglot
[Y] |Y.Join
[>] result1  // Pull FROM parallel scope
[>] result2  // Pull FROM parallel scope
```

**Semantic Consistency:**
- `<<` = Push INTO
- `>>` = Pull FROM
- Join = Pull FROM parallel → Use `[>]`

---

### Selective Synchronization

Only variables listed in join block are synchronized:

```polyglot
[|] SelectiveJoin
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""
[r] .result3: pg\string << ""

[p] |Process
[>] .out1 >> result1
[>] .out2 >> result2
[>] .out3 >> result3

// Only synchronize result1 and result2
[Y] |Y.Join
[>] result1
[>] result2
// result3 is NOT synchronized

// result1 and result2 are available
// result3 remains unchanged (still "")

[X]
```

---

### Join is Blocking

Join waits for all parallel blocks to complete:

```polyglot
[|] BlockingJoin
// Start parallel blocks
[p] |Fast    // Completes in 1 second
[p] |Slow    // Completes in 10 seconds

// Join waits for BOTH (10 seconds total)
[Y] |Y.Join
[>] fast_result
[>] slow_result

// Continues after both complete
[r] |NextStep

[X]
```

---

## Variable Lifetime

### Variables Listed in Join

Variables listed in join block are **synchronized and accessible** after join:

```polyglot
[|] ListedVariable
[r] .result: pg\string << "initial"

[p] |Process
[>] .output >> result

[Y] |Y.Join
[>] result  // Synchronized

// .result is accessible with new value
[r] .value: pg\string << result  // ✓ OK

[X]
```

---

### Variables NOT Listed in Join

Variables not listed in join are **NOT synchronized** and retain original values:

```polyglot
[|] UnlistedVariable
[r] .result: pg\string << "initial"

[p] |Process
[>] .output >> result

[Y] |Y.Join
// result NOT listed

// .result still has "initial" value
[r] .value: pg\string << result  // Still "initial"

[X]
```

---

### Lifetime Summary

| Scenario | After Join | Value |
|----------|-----------|-------|
| Listed in join | Synchronized | From parallel block |
| Not listed in join | Not synchronized | Original value |
| Never copied out | Not synchronized | Original value |

---

## Multiple Parallel Blocks

### Independent Execution

Multiple parallel blocks execute independently:

```polyglot
[|] MultipleParallel
[i] .data: pg\array{pg\string}

// All execute concurrently
[p] |ProcessPartA
[<] .input: pg\array{pg\string} << .data
[>] .result >> result_a

[p] |ProcessPartB
[<] .input: pg\array{pg\string} << .data
[>] .result >> result_b

[p] |ProcessPartC
[<] .input: pg\array{pg\string} << .data
[>] .result >> result_c

[p] |ProcessPartD
[<] .input: pg\array{pg\string} << .data
[>] .result >> result_d

// Join all results
[Y] |Y.Join
[>] result_a
[>] result_b
[>] result_c
[>] result_d

[X]
```

---

### No Execution Order Guarantee

Parallel blocks have **no guaranteed execution order**:

```polyglot
[|] UnorderedExecution
// These may complete in ANY order:
[p] |Task1  // Might finish 3rd
[p] |Task2  // Might finish 1st
[p] |Task3  // Might finish 2nd
[p] |Task4  // Might finish 4th

[Y] |Y.Join
// Join waits for ALL, regardless of order

[X]
```

---

### Data Partitioning Pattern

Common pattern: Partition data, process in parallel, combine:

```polyglot
[|] DataPartitioning
[i] .data: pg\array{pg\int}

// Partition into chunks
[r] |PartitionData
[<] .input: pg\array{pg\int} << .data
[<] .chunk_count: pg\int << 4
[>] .chunks: pg\array{pg\array{pg\int}} >> chunks

// Process each chunk in parallel
[p] |ProcessChunk1
[<] .chunk: pg\array{pg\int} << chunks[0]
[>] .result >> result1

[p] |ProcessChunk2
[<] .chunk: pg\array{pg\int} << chunks[1]
[>] .result >> result2

[p] |ProcessChunk3
[<] .chunk: pg\array{pg\int} << chunks[2]
[>] .result >> result3

[p] |ProcessChunk4
[<] .chunk: pg\array{pg\int} << chunks[3]
[>] .result >> result4

// Join results
[Y] |Y.Join
[>] result1
[>] result2
[>] result3
[>] result4

// Combine
[r] |CombineResults
[<] .parts: pg\array{pg\int} << array{result1, result2, result3, result4}
[>] .final: pg\array{pg\int} >> output

[X]
```

---

## Nested Operations

### Operations Inside Parallel Blocks

Use `[~]` for operations nested inside parallel blocks:

```polyglot
[|] NestedInParallel
[i] .data: pg\string

[p] |ProcessWithNesting
[<] .input: pg\string << .data
[~][r] |TransformData           // [~] means: runs WITHIN parallel block
[~][<] .text: pg\string << .input
[~][>] .transformed: pg\string >> temp
[~][r] |ValidateData            // Also WITHIN parallel block
[~][<] .value: pg\string << temp
[>] .output >> result

[Y] |Y.Join
[>] result

[X]
```

---

### Why `[~]` is Needed

**Implicit vs Explicit:**
- `[<]` and `[>]` after `[p]` are implicit (direct children)
- `[r]` inside `[p]` is explicit (needs `[~]`)

```polyglot
[p] |Parallel
[<] .input << data      // Implicit - direct child of [p]
[~][r] |Operation       // Explicit - nested operation WITHIN [p]
[~][<] .param << value  // Implicit - child of [~][r]
[>] .output >> result   // Implicit - direct child of [p]
```

---

### Nested Parallel Blocks

Parallel blocks can contain other parallel blocks:

```polyglot
[|] NestedParallel
[i] .data: pg\array{pg\string}

// Outer parallel block
[p] |ProcessOuter
[<] .input: pg\array{pg\string} << .data
[~][r] |SplitData
[~][<] .items: pg\array{pg\string} << .input
[~][>] .part1: pg\array{pg\string} >> part1
[~][>] .part2: pg\array{pg\string} >> part2

// Nested parallel blocks WITHIN outer block
[~][p] |ProcessPart1
[~][<] .data: pg\array{pg\string} << part1
[~][>] .result >> nested1

[~][p] |ProcessPart2
[~][<] .data: pg\array{pg\string} << part2
[~][>] .result >> nested2

// Nested join WITHIN outer block
[~][Y] |Y.Join
[~][>] nested1
[~][>] nested2

[~][r] |CombineNested
[~][<] .a: pg\string << nested1
[~][<] .b: pg\string << nested2
[~][>] .combined: pg\string >> temp

[>] .output >> outer_result

// Outer join
[Y] |Y.Join
[>] outer_result

[X]
```

---

## Error Handling in Parallel

### Errors in Parallel Blocks

Each parallel block can handle errors independently:

```polyglot
[|] ParallelWithErrors
[i] .files: pg\array{pg\path}

[p] |ProcessFile1
[<] .file: pg\path << files[0]
[r] |ReadFile
[<] .path: pg\path << .file
[>] .content: pg\string >> content1

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err1
[r] .content1: pg\string << ""  // Default value

[>] .result >> result1

[p] |ProcessFile2
[<] .file: pg\path << files[1]
[r] |ReadFile
[<] .path: pg\path << .file
[>] .content: pg\string >> content2

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err2
[r] .content2: pg\string << ""  // Default value

[>] .result >> result2

[Y] |Y.Join
[>] result1
[>] result2

[X]
```

---

### Error Propagation

Errors in parallel blocks can be propagated to outer scope:

```polyglot
[|] ErrorPropagation
[r] .error_occurred: pg\bool << False

[p] |MightFail
[r] |RiskyOperation

[!] !SomeError
[>] .error_flag >> error_occurred

[Y] |Y.Join
[>] error_occurred

// Check if any parallel block failed
[?] .error_occurred ?> True
[~][r] |HandleError

[X]
```

---

### Fail-Fast Pattern

Stop processing if any parallel block fails:

```polyglot
[|] FailFast
[r] .all_success: pg\bool << True

[p] |Task1
[r] |Process1
[!] !Error
[r] .all_success: pg\bool << False
[>] .success >> success1

[p] |Task2
[r] |Process2
[!] !Error
[r] .all_success: pg\bool << False
[>] .success >> success2

[Y] |Y.Join
[>] all_success

[?] .all_success ?> False
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "One or more tasks failed"

[X]
```

---

## Best Practices

### 1. Initialize Variables Before Parallel

```polyglot
// ✓ CORRECT - Initialize before parallel
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

[p] |Process1
[>] .output >> result1

[p] |Process2
[>] .output >> result2
```

---

### 2. Always Use Join

```polyglot
// ✓ CORRECT - Join after parallel blocks
[p] |Task1
[>] .out >> result1

[p] |Task2
[>] .out >> result2

[Y] |Y.Join
[>] result1
[>] result2

// ✗ WRONG - Missing join
[p] |Task1
[>] .out >> result1

[p] |Task2
[>] .out >> result2

[r] |UseResults  // result1 and result2 not synchronized!
```

---

### 3. List All Variables in Join

```polyglot
// ✓ CORRECT - List all needed variables
[Y] |Y.Join
[>] result1
[>] result2
[>] status

// ✗ RISKY - Missing variables
[Y] |Y.Join
[>] result1
// result2 and status not synchronized!
```

---

### 4. Keep Parallel Blocks Independent

```polyglot
// ✓ CORRECT - Independent blocks
[p] |ProcessA
[<] .data: pg\string << input_a
[>] .result >> result_a

[p] |ProcessB
[<] .data: pg\string << input_b
[>] .result >> result_b

// ✗ AVOID - Dependent blocks (use sequential instead)
[p] |Step1
[>] .output >> temp

[p] |Step2  // Depends on temp from Step1 - RACE CONDITION!
[<] .input: pg\string << temp
```

---

### 5. Use Copy Semantics Intentionally

```polyglot
// ✓ CORRECT - Explicit copy understanding
[r] .shared: pg\string << "data"

[p] |Process
[<] .input: pg\string << .shared  // Copies value
// Modifications don't affect outer .shared
```

---

### 6. Partition Work Evenly

```polyglot
// ✓ CORRECT - Even partitioning
[r] |PartitionEvenly
[<] .data: pg\array{pg\int} << large_dataset
[<] .workers: pg\int << 4
[>] .chunks: pg\array{pg\array{pg\int}} >> even_chunks

// ✗ AVOID - Uneven workload
[p] |SmallTask   // Finishes in 1 second
[p] |HugeTask    // Takes 100 seconds
// Join waits 100 seconds - inefficient!
```

---

### 7. Handle Errors in Each Block

```polyglot
// ✓ CORRECT - Error handling per block
[p] |Task1
[r] |Process
[!] !Error
[r] |HandleError
[>] .result >> result1

[p] |Task2
[r] |Process
[!] !Error
[r] |HandleError
[>] .result >> result2
```

---

### 8. Consider Memory Overhead

```polyglot
// ✓ CORRECT - Small data, many parallel blocks
[p] |ProcessSmallData
[<] .item: pg\int << item
[>] .result >> result

// ⚠ CAUTION - Large data copied multiple times
[p] |ProcessLargeData
[<] .huge_array: pg\array{pg\string} << million_items  // Memory overhead!
```

---

## Common Patterns

### Pattern 1: Map-Reduce

```polyglot
[|] MapReduce
[i] .data: pg\array{pg\int}

// Map phase - parallel
[p] |MapChunk1
[<] .chunk: pg\array{pg\int} << data[0:1000]
[>] .mapped >> mapped1

[p] |MapChunk2
[<] .chunk: pg\array{pg\int} << data[1000:2000]
[>] .mapped >> mapped2

[Y] |Y.Join
[>] mapped1
[>] mapped2

// Reduce phase - sequential
[r] |Reduce
[<] .parts: pg\array{pg\array{pg\int}} << array{mapped1, mapped2}
[>] .result: pg\int >> final_result

[X]
```

---

### Pattern 2: Fan-Out/Fan-In

```polyglot
[|] FanOutFanIn
[i] .request: pg\serial

// Fan-out - multiple parallel requests
[p] |CallServiceA
[<] .request: pg\serial << .request
[>] .response >> response_a

[p] |CallServiceB
[<] .request: pg\serial << .request
[>] .response >> response_b

[p] |CallServiceC
[<] .request: pg\serial << .request
[>] .response >> response_c

// Fan-in - join results
[Y] |Y.Join
[>] response_a
[>] response_b
[>] response_c

// Combine responses
[r] |MergeResponses
[<] .responses: pg\array{pg\serial} << array{response_a, response_b, response_c}
[>] .combined: pg\serial >> final_response

[X]
```

---

### Pattern 3: Pipeline Parallel Stages

```polyglot
[|] ParallelStages
[i] .data: pg\string

// Stage 1 - Sequential
[r] |Preprocess
[<] .input: pg\string << .data
[>] .prepared: pg\string >> prepared_data

// Stage 2 - Parallel
[p] |TransformA
[<] .input: pg\string << prepared_data
[>] .result >> result_a

[p] |TransformB
[<] .input: pg\string << prepared_data
[>] .result >> result_b

[Y] |Y.Join
[>] result_a
[>] result_b

// Stage 3 - Sequential
[r] |Postprocess
[<] .inputs: pg\array{pg\string} << array{result_a, result_b}
[>] .final: pg\string >> output

[X]
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - `[p]` and `[Y]` syntax
- [Block Markers](06-block-markers.md) - `[p]` and `[Y]` details
- [Operators](05-operators.md) - `>>` operator semantics
- [Expansion Operator](09-expansion-operator.md) - `[~]` in parallel blocks

### Examples
- [Parallel Execution Examples](../examples/parallel-execution.md) - Complete patterns

### Planning
- [Decision Log](../decision-log.md) - Parallel execution decisions (#12, Pending #2)

---

**End of Parallel Execution Reference**