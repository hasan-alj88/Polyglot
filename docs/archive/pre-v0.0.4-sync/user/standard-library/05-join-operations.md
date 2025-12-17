---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/standard-library/05-join-operations.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Join Operations (|Y.*)

**Version:** 0.0.2
**Status:** Fully Documented
**Block Marker:** `[Y]`

## Overview

Join operations provide synchronization and result aggregation for parallel execution. The `|Y.*` namespace works with the `[Y]` block marker to collect results from parallel `[p]` blocks, enabling safe concurrent programming patterns.

### Philosophy

- **Explicit Synchronization:** Join points are visible and clear in code
- **Type Safety:** Join operations preserve type information
- **Copy Semantics:** Results are copied out of parallel contexts safely
- **No Shared State:** Parallel blocks communicate only through join operations
- **Predictable Behavior:** Deterministic result collection

## Block Marker: [Y]

The `[Y]` block marker establishes a join/synchronization context for collecting parallel results.

```polyglot
[p] |ParallelWork
[>] .result >> parallel_result

[Y] |Y.Join
[>] parallel_result  // Synchronize here
```

**Properties:**
- **Scope:** Wraps join operations
- **Required:** For all `|Y.*` operations
- **Position:** Follows parallel `[p]` blocks
- **Purpose:** Synchronization point in pipeline execution

**Key Rule:** Use `[>]` (output marker) within `[Y]` blocks, NOT `[<]` (input marker). Results are pulled from parallel contexts, not pushed into join.

See [Block Markers](../language/06-block-markers.md) and [Parallel Execution](../language/08-parallel-execution.md) for complete details.

## Join Operations Reference

### |Y.Join

Basic synchronization operation that waits for all parallel blocks to complete and collects their results.

**Signature:**
```polyglot
[Y] |Y.Join
[>] result_variable1
[>] result_variable2
// ... more results as needed
```

**Behavior:**
- Blocks until all referenced parallel blocks complete
- Collects all specified result variables
- Results become available in containing pipeline scope
- Order-preserving: results maintain declaration order

**Use Cases:**
- Synchronizing parallel computations
- Collecting multiple parallel results
- Wait for concurrent operations to complete

**Example: Basic Join**
```polyglot
[|] ParallelProcessing
[i] .data: pg.array.pg.int

// Parallel block 1
[p] |ProcessPartA
[<] .input: pg.array.pg.int << .data
[r] |ComputeSum
[<] .values: pg.array.pg.int << .input
[>] .sum:pg.int >> sum_result

// Parallel block 2
[p] |ProcessPartB
[<] .input: pg.array.pg.int << .data
[r] |ComputeProduct
[<] .values: pg.array.pg.int << .input
[>] .product:pg.int >> product_result

// Join both results
[Y] |Y.Join
[>] sum_result
[>] product_result

// Use collected results
[r] |CombineResults
[<] .sum:pg.int << sum_result
[<] .product:pg.int << product_result
[>] .combined:pg.string >> final_result

[o] .output:pg.string << final_result
[X]
```

### |Y.JoinAny

Wait for any one parallel block to complete, returning the first available result.

**Signature:**
```polyglot
[Y] |Y.JoinAny
[>] result_variable1
[>] result_variable2
[>] .completed_index:pg.int >> which_completed
```

**Behavior:**
- Returns as soon as first parallel block completes
- Only one result becomes available
- `.completed_index` indicates which block finished first
- Other parallel blocks may continue executing or be cancelled (implementation dependent)

**Use Cases:**
- Race conditions (first result wins)
- Timeout patterns (race against timer)
- Redundant computation (fastest result)
- Fallback strategies

**Example: Race Condition**
```polyglot
[|] FastestResult
[i] .query:pg.string

// Try multiple data sources in parallel
[p] |QueryDatabaseA
[<] .query:pg.string << .query
[r] |FetchFromA
[>] .result:pg.string >> result_a

[p] |QueryDatabaseB
[<] .query:pg.string << .query
[r] |FetchFromB
[>] .result:pg.string >> result_b

[p] |QueryDatabaseC
[<] .query:pg.string << .query
[r] |FetchFromC
[>] .result:pg.string >> result_c

// Use whichever completes first
[Y] |Y.JoinAny
[>] result_a
[>] result_b
[>] result_c
[>] .completed_index:pg.int >> winner

// Log which source won
[r] |LogWinner
[<] .index:pg.int << winner

// Use the first result (index-based access conceptual)
[r] |ProcessResult
[<] .data:pg.string << result_a  // Or result_b/result_c based on winner
[>] .final:pg.string >> output

[o] .result:pg.string << output
[X]
```

### |Y.JoinTimeout

Wait for parallel blocks with a timeout; return completed results or timeout indication.

**Signature:**
```polyglot
[Y] |Y.JoinTimeout
[<] .timeout_ms:pg.int << 5000
[>] result_variable1
[>] result_variable2
[>] .timed_out:pg.bool >> timeout_flag
[>] .completed_count:pg.int >> completed
```

**Behavior:**
- Waits up to specified timeout duration
- Returns all completed results within timeout
- `.timed_out` is true if timeout occurred
- `.completed_count` indicates how many finished
- Incomplete operations may continue or be cancelled (implementation dependent)

**Use Cases:**
- Operations with time constraints
- Partial result collection
- Graceful degradation
- Service-level objectives (SLO) enforcement

**Example: Timeout Handling**
```polyglot
[|] TimeConstrainedProcessing
[i] .data1:pg.string
[i] .data2:pg.string
[i] .max_wait_ms:pg.int

[p] |SlowOperation1
[<] .input:pg.string << .data1
[r] |ProcessSlowly1
[>] .result:pg.string >> result1

[p] |SlowOperation2
[<] .input:pg.string << .data2
[r] |ProcessSlowly2
[>] .result:pg.string >> result2

// Wait with timeout
[Y] |Y.JoinTimeout
[<] .timeout_ms:pg.int << .max_wait_ms
[>] result1
[>] result2
[>] .timed_out:pg.bool >> did_timeout
[>] .completed_count:pg.int >> completed

// Handle timeout
[t] .condition:pg.bool << did_timeout
[r] |HandleTimeout
[<] .completed:pg.int << completed
[>] .fallback:pg.string >> timeout_message

// Use available results or fallback
[r] |ProcessResults
[<] .r1:pg.string << result1  // May be empty if incomplete
[<] .r2:pg.string << result2
[<] .timeout_msg:pg.string << timeout_message
[>] .final:pg.string >> output

[o] .output:pg.string << output
[X]
```

### |Y.Collect

Collect results from parallel blocks operating on array elements (map-like pattern).

**Signature:**
```polyglot
[Y] |Y.Collect
[>] result_variable
[>] .results:pg.array{T} >> collected_array
```

**Behavior:**
- Waits for all parallel operations on array elements
- Collects results into array
- Maintains order corresponding to input array
- Type-safe: output array type matches parallel result type

**Use Cases:**
- Parallel map operations
- Bulk data processing
- Element-wise transformations
- Concurrent computations on collections

**Example: Parallel Map**
```polyglot
[|] ParallelMap
[i] .numbers: pg.array.pg.int

// Process each element in parallel
[p] |ProcessElements
[<] .items: pg.array.pg.int << .numbers

[~][r] |SquareNumber
[<] .value:pg.int << .items[*]
[>] .squared:pg.int >> squared_results

// Collect all results into array
[Y] |Y.Collect
[>] squared_results
[>] .results: pg.array.pg.int >> all_squares

[o] .output: pg.array.pg.int << all_squares
[X]

// Example usage:
// Input: [1, 2, 3, 4, 5]
// Output: [1, 4, 9, 16, 25]
```

### |Y.Reduce

Collect and reduce parallel results using an aggregation operation.

**Signature:**
```polyglot
[Y] |Y.Reduce
[<] .operation:pg.string << "sum"  // or "product", "min", "max", "concat", etc.
[>] result_variable
[>] .reduced: T >> reduced_result
```

**Behavior:**
- Waits for all parallel blocks
- Applies reduction operation to results
- Returns single aggregated value
- Type-safe: reduction operation must match result types

**Use Cases:**
- Summing parallel computations
- Finding minimum/maximum
- Concatenating parallel string results
- Aggregating metrics

**Example: Parallel Sum**
```polyglot
[|] ParallelSum
[i] .data_chunks:pg.array{pg.array.pg.int}

// Sum each chunk in parallel
[p] |SumChunks
[<] .chunks:pg.array{pg.array.pg.int} << .data_chunks

[~][r] |SumChunk
[<] .chunk: pg.array.pg.int << .chunks[*]
[>] .chunk_sum:pg.int >> chunk_sums

// Reduce all chunk sums to final sum
[Y] |Y.Reduce
[<] .operation:pg.string << "sum"
[>] chunk_sums
[>] .reduced:pg.int >> total_sum

[o] .total:pg.int << total_sum
[X]
```

### |Y.Barrier

Explicit synchronization point without result collection (wait-only).

**Signature:**
```polyglot
[Y] |Y.Barrier
```

**Behavior:**
- Blocks until all previous parallel blocks complete
- Does not collect results
- Pure synchronization point
- Useful for coordinating execution without data flow

**Use Cases:**
- Synchronization without result passing
- Checkpoint in multi-stage parallel processing
- Coordinating side-effect operations
- Phase boundaries in parallel algorithms

**Example: Synchronization Barrier**
```polyglot
[|] MultiStageParallel
[i] .files: pg.array.pg.path

// Stage 1: Validate all files in parallel
[p] |ValidateFiles
[<] .file_list: pg.array.pg.path << .files

[~][r] |ValidateFile
[<] .file:pg.path << .file_list[*]
[>] .valid:pg.bool >> validation_results

// Wait for all validations (no result collection needed here)
[Y] |Y.Barrier

// Stage 2: Process files in parallel (only if we reach here)
[p] |ProcessFiles
[<] .file_list: pg.array.pg.path << .files

[~][r] |ProcessFile
[<] .file:pg.path << .file_list[*]
[>] .result:pg.string >> processing_results

// Collect final results
[Y] |Y.Collect
[>] processing_results
[>] .results: pg.array.pg.string >> final_results

[o] .output: pg.array.pg.string << final_results
[X]
```

## Complete Examples

### Example 1: Parallel Data Pipeline

Process data through multiple parallel stages with different operations.

```polyglot
[|] ParallelDataPipeline
[i] .raw_data:pg.string

// Stage 1: Parse data in parallel chunks
[p] |ParseChunk1
[<] .data:pg.string << .raw_data
[r] |ParseFirst
[>] .parsed:pg.serial >> parsed1

[p] |ParseChunk2
[<] .data:pg.string << .raw_data
[r] |ParseSecond
[>] .parsed:pg.serial >> parsed2

[Y] |Y.Join
[>] parsed1
[>] parsed2

// Stage 2: Transform each parsed result in parallel
[p] |TransformParsed1
[<] .input:pg.serial << parsed1
[r] |Transform
[>] .output:pg.serial >> transformed1

[p] |TransformParsed2
[<] .input:pg.serial << parsed2
[r] |Transform
[>] .output:pg.serial >> transformed2

[Y] |Y.Join
[>] transformed1
[>] transformed2

// Stage 3: Merge results
[r] |MergeResults
[<] .result1:pg.serial << transformed1
[<] .result2:pg.serial << transformed2
[>] .merged:pg.serial >> final_output

[o] .output:pg.serial << final_output
[X]
```

### Example 2: Redundant Computation with Timeout

Try multiple computation strategies in parallel, use fastest result or timeout.

```polyglot
[|] RedundantComputation
[i] .problem:pg.string
[i] .timeout_ms:pg.int

// Strategy 1: Optimized algorithm
[p] |OptimizedApproach
[<] .input:pg.string << .problem
[r] |FastAlgorithm
[>] .result:pg.string >> fast_result

// Strategy 2: Brute force algorithm
[p] |BruteForceApproach
[<] .input:pg.string << .problem
[r] |BruteForceAlgorithm
[>] .result:pg.string >> brute_result

// Strategy 3: Heuristic algorithm
[p] |HeuristicApproach
[<] .input:pg.string << .problem
[r] |HeuristicAlgorithm
[>] .result:pg.string >> heuristic_result

// Use first result or timeout
[Y] |Y.JoinAny
[>] fast_result
[>] brute_result
[>] heuristic_result
[>] .completed_index:pg.int >> winner_index

// Add timeout to the join (conceptual - would need JoinTimeout for real implementation)
[r] |LogStrategy
[<] .strategy_index:pg.int << winner_index
[>] .log_message:pg.string >> log

// Select winning result (conceptual - would need index-based access)
[r] |SelectResult
[<] .index:pg.int << winner_index
[<] .r1:pg.string << fast_result
[<] .r2:pg.string << brute_result
[<] .r3:pg.string << heuristic_result
[>] .selected:pg.string >> final_answer

[o] .answer:pg.string << final_answer
[X]
```

### Example 3: Parallel Array Processing with Error Handling

Process array elements in parallel, collect successful results and errors separately.

```polyglot
[|] ParallelArrayWithErrors
[i] .urls: pg.array.pg.string

// Fetch each URL in parallel
[p] |FetchURLs
[<] .url_list: pg.array.pg.string << .urls

[~][r] |FetchURL
[<] .url:pg.string << .url_list[*]
[>] .content:pg.string >> contents
[>] .error: !Error >> errors

// Collect all results
[Y] |Y.Collect
[>] contents
[>] .results: pg.array.pg.string >> all_contents

[Y] |Y.Collect
[>] errors
[>] .results:pg.array{!Error} >> all_errors

// Filter successful fetches (conceptual)
[r] |FilterSuccessful
[<] .contents: pg.array.pg.string << all_contents
[<] .errors:pg.array{!Error} << all_errors
[>] .successful: pg.array.pg.string >> successful_contents
[>] .failed_count:pg.int >> failure_count

// Report results
[r] |FormatReport
[<] .success: pg.array.pg.string << successful_contents
[<] .failures:pg.int << failure_count
[>] .report:pg.string >> summary

[o] .summary:pg.string << summary
[X]
```

### Example 4: Multi-Stage Reduce

Perform parallel computation with multiple reduction stages.

```polyglot
[|] MultiStageReduce
[i] .dataset: pg.array.pg.int

// Stage 1: Partition data into 4 chunks and sum each
[p] |SumChunks
[<] .data: pg.array.pg.int << .dataset

[~][r] |PartitionAndSum
[<] .full_data: pg.array.pg.int << .data
[<] .partition_id:pg.int << [0, 1, 2, 3][*]
[>] .partial_sum:pg.int >> partial_sums

// Reduce partial sums
[Y] |Y.Reduce
[<] .operation:pg.string << "sum"
[>] partial_sums
[>] .reduced:pg.int >> total

// Stage 2: Compute statistics in parallel
[p] |ComputeMean
[<] .sum:pg.int << total
[<] .count:pg.int << .dataset.length
[r] |Divide
[>] .mean:pg.float >> mean_result

[p] |ComputeMax
[<] .data: pg.array.pg.int << .dataset
[r] |FindMax
[>] .max:pg.int >> max_result

[p] |ComputeMin
[<] .data: pg.array.pg.int << .dataset
[r] |FindMin
[>] .min:pg.int >> min_result

// Join statistics
[Y] |Y.Join
[>] mean_result
[>] max_result
[>] min_result

// Format output
[r] |FormatStatistics
[<] .total:pg.int << total
[<] .mean:pg.float << mean_result
[<] .max:pg.int << max_result
[<] .min:pg.int << min_result
[>] .summary:pg.string >> stats

[o] .statistics:pg.string << stats
[X]
```

### Example 5: Pipeline Orchestration with Join

Orchestrate multiple pipeline instances and synchronize their results.

```polyglot
[|] OrchestratePipelines
[i] .job_configs: pg.array.pg.serial

// Launch pipeline instances for each job
[p] |LaunchJobs
[<] .configs: pg.array.pg.serial << .job_configs

[~][r] |LaunchJobPipeline
[<] .config:pg.serial << .configs[*]
[>] .instance_id:pg.string >> job_instances

// Collect all instance IDs
[Y] |Y.Collect
[>] job_instances
[>] .results: pg.array.pg.string >> all_instances

// Wait for all jobs to complete (conceptual - would use triggers)
[p] |MonitorJobs
[<] .instances: pg.array.pg.string << all_instances

[~][t] |T.Pipeline.Completed
[<] .instance_id:pg.string << .instances[*]
[>] .result:pg.serial >> job_results

// Collect all job results
[Y] |Y.Collect
[>] job_results
[>] .results: pg.array.pg.serial >> completed_jobs

// Aggregate results
[r] |AggregateJobResults
[<] .jobs: pg.array.pg.serial << completed_jobs
[>] .summary:pg.string >> final_summary

[o] .orchestration_result:pg.string << final_summary
[X]
```

## Best Practices

### 1. **Always Use [Y] with Parallel Blocks**

Every parallel block that produces results must have a corresponding join.

```polyglot
// ✓ CORRECT - join synchronizes parallel blocks
[p] |ParallelWork
[>] .result >> result1

[Y] |Y.Join
[>] result1

// ✗ INCORRECT - no join, result never synchronized
[p] |ParallelWork
[>] .result >> result1
// Missing join!
```

### 2. **Use [>] Not [<] in Join Blocks**

Join blocks pull results from parallel contexts using output markers.

```polyglot
// ✓ CORRECT - use [>] to pull results
[Y] |Y.Join
[>] parallel_result

// ✗ INCORRECT - don't use [<] in join blocks
[Y] |Y.Join
[<] parallel_result  // WRONG DIRECTION
```

### 3. **Match Join Variables to Parallel Outputs**

Variable names in join must match those set by parallel blocks.

```polyglot
[p] |Work
[>] .output >> my_result

// ✓ CORRECT - matches variable name
[Y] |Y.Join
[>] my_result

// ✗ INCORRECT - wrong variable name
[Y] |Y.Join
[>] other_result  // Doesn't exist
```

### 4. **Choose Appropriate Join Operation**

Select join operation based on synchronization needs:

- **|Y.Join** - Wait for all results (most common)
- **|Y.JoinAny** - First result wins (races, timeouts)
- **|Y.JoinTimeout** - Wait with time limit (SLOs, graceful degradation)
- **|Y.Collect** - Array result collection (map patterns)
- **|Y.Reduce** - Aggregated result (sum, max, etc.)
- **|Y.Barrier** - Synchronization only (no results)

### 5. **Handle Errors from Parallel Blocks**

Collect and process errors alongside successful results.

```polyglot
[p] |MayFail
[>] .result >> results
[>] .error >> errors

[Y] |Y.Collect
[>] results
[>] .results:pg.array{T} >> all_results

[Y] |Y.Collect
[>] errors
[>] .results:pg.array{!Error} >> all_errors

[r] |ProcessErrors
[<] .errors:pg.array{!Error} << all_errors
```

### 6. **Use Timeouts for Bounded Waiting**

Don't wait indefinitely for parallel operations; use timeouts.

```polyglot
[Y] |Y.JoinTimeout
[<] .timeout_ms:pg.int << 5000
[>] results
[>] .timed_out:pg.bool >> timeout_flag

[t] .condition:pg.bool << timeout_flag
[r] |HandleTimeout
```

### 7. **Order Multiple Joins Carefully**

Multiple joins execute sequentially; order affects performance.

```polyglot
// ✓ GOOD - independent joins can be ordered flexibly
[Y] |Y.Join
[>] result_set_1

[Y] |Y.Join
[>] result_set_2

// Consider: could result_set_1 and result_set_2 be joined together?
[Y] |Y.Join
[>] result_set_1
[>] result_set_2
```

### 8. **Document Parallel Logic**

Parallel and join operations benefit from clear comments.

```polyglot
// Fetch user data and preferences in parallel
[p] |FetchUserData
[>] .data >> user_data

[p] |FetchPreferences
[>] .prefs >> user_prefs

// Wait for both before rendering
[Y] |Y.Join
[>] user_data
[>] user_prefs
```

## Common Patterns

### Pattern 1: Fan-Out, Fan-In

Distribute work across parallel blocks, then aggregate results.

```polyglot
// Fan-out: Launch parallel operations
[p] |Worker1
[p] |Worker2
[p] |Worker3

// Fan-in: Collect results
[Y] |Y.Join
[>] result1
[>] result2
[>] result3
```

### Pattern 2: Pipeline Stages

Sequential stages where each stage has internal parallelism.

```polyglot
// Stage 1: Parallel processing
[p] |Stage1Worker1
[p] |Stage1Worker2
[Y] |Y.Join

// Stage 2: Parallel processing
[p] |Stage2Worker1
[p] |Stage2Worker2
[Y] |Y.Join
```

### Pattern 3: Conditional Join

Different join strategies based on runtime conditions.

```polyglot
[p] |Worker1
[p] |Worker2

// Choose join strategy based on priority
[t] .condition:pg.bool << high_priority
[Y] |Y.JoinAny  // First result if high priority

[t] .condition:pg.bool << (!high_priority)
[Y] |Y.Join  // All results if normal priority
```

### Pattern 4: Nested Parallelism

Parallel blocks containing their own parallel operations.

```polyglot
[p] |OuterParallel1
[~][p] |InnerParallel1
[~][Y] |Y.Join

[p] |OuterParallel2
[~][p] |InnerParallel2
[~][Y] |Y.Join

[Y] |Y.Join  // Outer join
```

### Pattern 5: Result Transformation

Join results then transform for final output.

```polyglot
[p] |ComputeA
[>] .result >> result_a

[p] |ComputeB
[>] .result >> result_b

[Y] |Y.Join
[>] result_a
[>] result_b

// Transform joined results
[r] |Combine
[<] .a: T << result_a
[<] .b: T << result_b
[>] .combined: T >> final_output
```

## Error Handling

Join operations should handle failures in parallel blocks:

```polyglot
[p] |MayFail
[>] .result >> results
[>] .error >> errors

[Y] |Y.Collect
[>] results
[>] .results:pg.array{T} >> all_results

[Y] |Y.Collect
[>] errors
[>] .results:pg.array{!Error} >> all_errors

// Check for any errors
[r] |HasErrors
[<] .errors:pg.array{!Error} << all_errors
[>] .has_errors:pg.bool >> error_flag

[t] .condition:pg.bool << error_flag
[r] |HandleErrors
[<] .errors:pg.array{!Error} << all_errors

// Process successful results
[t] .condition:pg.bool << (!error_flag)
[r] |ProcessResults
[<] .results:pg.array{T} << all_results
```

**Common Error Scenarios:**
- Parallel block throws error
- Timeout occurs before completion
- Join variable not found (programming error)
- Type mismatch in collection
- Deadlock (circular dependencies)

## Performance Considerations

### Parallelism Overhead

Parallel execution has overhead; only parallelize work that benefits:

```polyglot
// ✓ GOOD - substantial work in parallel
[p] |HeavyComputation1
[p] |HeavyComputation2
[Y] |Y.Join

// ✗ BAD - trivial operations don't benefit
[p] |AddTwoNumbers
[p] |SubtractTwoNumbers
[Y] |Y.Join  // Overhead > benefit
```

### Join Granularity

Fewer large joins typically perform better than many small joins:

```polyglot
// ✓ BETTER - single join point
[p] |Work1
[p] |Work2
[p] |Work3
[Y] |Y.Join
[>] result1
[>] result2
[>] result3

// ✗ WORSE - multiple join points
[p] |Work1
[Y] |Y.Join
[>] result1

[p] |Work2
[Y] |Y.Join
[>] result2

[p] |Work3
[Y] |Y.Join
[>] result3
```

### Balancing Work

Distribute work evenly across parallel blocks:

```polyglot
// ✓ GOOD - balanced work distribution
[p] |Process1000Items
[p] |Process1000Items
[p] |Process1000Items

// ✗ BAD - unbalanced work
[p] |Process2900Items
[p] |Process50Items
[p] |Process50Items
```

## See Also

- [Parallel Execution](../language/08-parallel-execution.md) - Complete parallel execution documentation
- [Block Markers](../language/06-block-markers.md) - `[Y]` and `[p]` block markers
- [Expansion Operator](../language/09-expansion-operator.md) - Using `[~]` with parallel blocks
- [Standard Library Overview](00-overview.md) - Complete stdlib organization

---

**Navigation:**
← [Triggers Catalog](04-triggers-catalog.md) | [Standard Library Index](00-overview.md) | [Examples →](../examples/00-index.md)