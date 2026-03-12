---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "loop-system"
shard: false

# --- Classification ---
type: spec
topic: "Polyglot v0.0.4 - Loop System Specificat"
summary: "Advanced: Polyglot v0.0.4 - Loop System Speci"
keywords:
  - advanced
  - features
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
phase: solutioning
workflow: greenfield
module: bmm
complexity: high

# --- Dependency Chain ---
prereqs:
  - language-syntax
  - type-system
  - control-flow
unlocks:
  - stdlib

# --- Relationships ---
related:
  []
parent: "language-advanced"

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#advanced"
  - "#features"
---
# Polyglot v0.0.4 - Loop System Specification

**Version:** v0.0.4
**Status:** ✅ Final
**Last Updated:** 2025-12-14

---

## Overview

Polyglot's loop system uses **unpack/pack operators** with explicit I/O markers to control data flow between the main pipeline and iteration mini-pipelines.

**Core Concept:** Each loop iteration is a mini-pipeline with explicit inputs and outputs.

---

## Loop Markers & Operators

### Unpack Operator: `[~]`
**Purpose:** Transfer data FROM main pipeline TO iteration mini-pipeline

**Syntax:**
```polyglot
[~] <label << $main_pipeline_var    // Input from main
[~] >label >> $iteration_var        // Output to iteration
```

**Flow:** Main pipeline → Iteration scope

### Pack Operator: `[*]`
**Purpose:** Transfer data FROM iteration mini-pipeline TO main pipeline or next iteration

**Syntax:**
```polyglot
[*] <label << $iteration_var        // Input from iteration
[*] >label >> $main_pipeline_var    // Output to main
```

**Flow:** Iteration scope → Main pipeline (or next iteration)

### Join Operator: `[v]`
**Purpose:** Named aggregation/collection operation (JOIN, not "vacuum")

**Syntax:**
```polyglot
[v] *Operation.Name
[*] <item << $iteration_value
[*] >array >> $collection_var
```

---

## Loop Execution Modes

| Marker | Mode | Description |
|--------|------|-------------|
| `[r] ~LoopPattern` | Sequential | Iterations run one after another |
| `[p] ~LoopPattern` | Parallel | Iterations run concurrently |
| `[b] ~LoopPattern` | Fire-and-forget | Background execution, no collection |

---

## Loop Pattern: ForEach.Array

**Sequential array iteration with collection:**

```polyglot
[p] ~ForEach.Array
[~] <array << $array_var           // Unpack: array from main
[~] >item >> $item_var             // Output: each item to iteration

   // Iteration mini-pipeline body
   [r] $processed << |Transform <input << $item_var
   [r] $validated << |Validate <data << $processed

   // Pack: collect results back to main
   [v] *Into.Array
   [*] <item << $validated         // Input: processed item
   [*] >array >> $results          // Output: collected array for main
```

**Flow:**
1. **Unpack:** `$array_var` from main → iterations get `$item_var`
2. **Process:** Each iteration transforms `$item_var`
3. **Pack:** Results collected into `$results` array for main pipeline

---

## Loop Pattern: Chained Sequential

**Each iteration's output becomes next iteration's input:**

```polyglot
[r] ~Chained
[~] <first << $initial_value       // First iteration input
[~] >current >> $current_var       // Current iteration variable

   // Iteration mini-pipeline
   [r] $transformed << |Step1 <input << $current_var
   [r] $result << |Step2 <input << $transformed

   // Pack to next iteration
   [v] *To.Next.Iteration
   [*] <current << $result
   [*] >next >> >current           // ⭐ Assign to next iteration's input

   // Collect last iteration's result
   [v] *Collect.Last               // Only executes in final iteration
   [*[*] >last >> $result            // Output to main pipeline
```

**Flow:**
1. **First iteration:** Receives `$initial_value` as `$current_var`
2. **Process:** Transforms `$current_var` → `$result`
3. **Chain:** `$result` becomes next iteration's `$current_var`
4. **Last:** Final `$result` captured in `$last` for main pipeline

**Key:** `[*] >next >> >current` creates the chain by assigning iteration output to next iteration's input

---

## Operator Semantics

### Unpack `[~]` - Main → Iteration

**Input side (`<`):**
```polyglot
[~] <label << $source_var          // Variable from MAIN pipeline
```
- `$source_var` must exist in main pipeline scope
- Provides data to iteration

**Output side (`>`):**
```polyglot
[~] >label >> $destination_var     // Variable in ITERATION scope
```
- Creates `$destination_var` in iteration mini-pipeline
- Each iteration gets its own instance

### Pack `[*]` - Iteration → Main (or Next)

**Input side (`<`):**
```polyglot
[*] <label << $iteration_value     // Variable from ITERATION scope
```
- `$iteration_value` must exist in current iteration
- Provides data for collection/chaining

**Output side (`>`):**
```polyglot
[*] >label >> $main_var            // Variable in MAIN pipeline
[*] >label >> >unpack_label        // Variable for NEXT iteration
```
- **To main:** `$main_var` collected in main pipeline
- **To next:** `>unpack_label` refers to unpack output label for chaining

---

## Join Operations

### Into.Array - Standard Collection

**Collects iteration results into array:**
```polyglot
[v] *Into.Array
[*] <item << $iteration_result
[*] >array >> $collected_results
```

**Behavior:**
- Appends each `$iteration_result` to array
- Order preserved (sequential) or unordered (parallel)
- `$collected_results` available in main pipeline after loop completes

### To.Next.Iteration - Sequential Chaining

**Passes iteration output to next iteration:**
```polyglot
[v] *To.Next.Iteration
[*] <current << $this_iteration_output
[*] >next >> >current_unpack_label
```

**Behavior:**
- Only valid in sequential loops (`[r] ~*`)
- Compiler error if used in parallel (`[p] ~*`)
- Creates data dependency chain between iterations

### Collect.Last - Final Iteration

**Executes only in last iteration:**
```polyglot
[v] *Collect.Last
[*[*] >last >> $final_result
```

**Behavior:**
- Skipped in all iterations except final
- Useful for chained loops to capture end result
- No input side (` <`), only output (`>`)

---

## Standard Loop Patterns

### ForEach.Array
**Purpose:** Iterate over array elements

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   // Process $element
   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

### ForEach.Range
**Purpose:** Numeric iteration

```polyglot
[r] ~ForEach.Range
[~] <range << "1..100"             // String range notation
[~] >index >> $i
   // Process with $i
   [v] *Into.Array
   [*] <item << $computed
   [*] >array >> $results
```

### Enumerate
**Purpose:** Iterate with index

```polyglot
[r] ~Enumerate
[~] <array << $items
[~] >index >> $idx
[~] >item >> $element
   // Process $element at $idx
```

### Map
**Purpose:** Transform each element

```polyglot
[p] ~Map
[~] <array << $input_array
[~] >item >> $item
   [r] $transformed << |Transform <input << $item
   [v] *Into.Array
   [*] <item << $transformed
   [*] >array >> $output_array
```

### Filter
**Purpose:** Keep elements matching predicate

```polyglot
[p] ~Filter
[~] <array << $all_items
[~] >item >> $item
   [f] |Predicate <input << $item >matches >> $keep
   [&] $keep =? #True
      [v] *Into.Array
      [*] <item << $item
      [*] >array >> $filtered_items
```

### Reduce
**Purpose:** Accumulate/fold over collection

```polyglot
[r] ~Reduce
[~] <array << $numbers
[~] <initial << 0
[~] >accumulator >> $acc
[~] >item >> $num
   [r] $new_acc << $acc + $num
   [v] *To.Next.Iteration
   [*] <current << $new_acc
   [*] >next >> >accumulator       // Chain to next

   [v] *Collect.Last
   [*[*] >last >> $new_acc           // Final sum
```

---

## Parallel Loops & Join Patterns

### Parallel Execution

```polyglot
[p] ~ForEach.Array                 // [p] = parallel marker
[~] <array << $items
[~] >item >> $item
   // Each iteration runs concurrently
   [r] $result << |ExpensiveOp <input << $item
   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

**Characteristics:**
- Iterations execute concurrently
- Collection order may not match input order (unless sorted)
- Requires thread-safe operations
- Performance boost for independent iterations

### Join All (Wait for All)

```polyglot
[p] ~ForEach.Array
[~] <array << $tasks
[~] >item >> $task
   [r] $result << |Process <task << $task
   [v] *Join.All                   // Wait for ALL iterations
   [*] <item << $result
   [*] >array >> $all_results      // Only available after all complete
```

### Join First (Race)

```polyglot
[p] ~ForEach.Array
[~] <array << $endpoints
[~] >item >> $endpoint
   [r] $response << |Ping <url << $endpoint
   [v] *Join.First                 // Return FIRST to complete
   [*[*] >first >> $response         // Only one result
```

---

## Advanced Patterns

### Nested Loops

```polyglot
[r] ~ForEach.Array
[~] <array << $outer_items
[~] >item >> $outer

   // Nested loop
   [p] ~ForEach.Array
   [~] <array << $inner_items
   [~] >item >> $inner
      [r] $pair << #Pair
         [.] .outer << $outer
         [.] .inner << $inner
      [v] *Into.Array
      [*] <item << $pair
      [*] >array >> $pairs
```

### Conditional Collection

**Only collect items meeting criteria:**
```polyglot
[p] ~ForEach.Array
[~] <array << $all_orders
[~] >item >> $order
   [f] $order.total >? 1000
      [v] *Into.Array
      [*] <item << $order
      [*] >array >> $large_orders
```

### Break/Continue

**Early loop exit:**
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [f] $item =? $target
      [v] *Break                   // Exit loop immediately
      [*] >found << $item

   [f] $item <? 0
      [v] *Continue                // Skip to next iteration
```

---

## Error Handling in Loops

### Error Collection

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [z] // Try block
      [r] $result << |RiskyOp <input << $item
      [v] *Into.Array
      [*] <item << $result
      [*] >array >> $successes

   [!] !Validation.Error >> $error
      [v] *Into.Errors             // Collect errors separately
      [*] <error << $error
      [*] >errors >> $failures
```

### Stop on First Error

```polyglot
[r] ~ForEach.Array                 // Sequential only!
[~] <array << $critical_items
[~] >item >> $item
   [r] $result << |Validate <input << $item
   [f] !? $result                  // If error
      [v] *Break
      [*] >error << $result
```

---

## Type Inference in Loops

**Datatype inference from context:**

```polyglot
[~] <array << $numbers:array.int
[~] >item >> $num                  // Type inferred as :int

[~] <array << $users:array.serial
[~] >item >> $user                 // Type inferred as :serial
```

**No explicit type annotation needed on loop variables.**

---

## Compilation Rules

### Sequential Loops (`[r] ~*`)
- ✅ Allow chaining: `*To.Next.Iteration`
- ✅ Allow `*Collect.Last`
- ✅ Guaranteed order
- ❌ Cannot use join patterns

### Parallel Loops (`[p] ~*`)
- ✅ Concurrent execution
- ✅ Allow join patterns: `*Join.All`, `*Join.First`
- ❌ Cannot chain: `*To.Next.Iteration` is compile error
- ⚠️  Unordered collection (unless explicit sort)

### Fire-and-Forget (`[b] ~*`)
- ✅ Background execution
- ❌ Cannot collect results
- ❌ Cannot use join operations
- ⚠️  No error handling

---

## Examples

### Complete Example: Sequential Transformation Pipeline

```polyglot
{|} |TransformOrders
[|] <orders :array.serial
[|] >enriched :array.serial

[t] |T.Call
[W] |W.Polyglot.Scope

// Chained transformation
[r] ~Chained
[~] <first << $orders
[~] >current >> $order

   // Step 1: Enrich with customer data
   [r] $with_customer << |EnrichCustomer <order << $order

   // Step 2: Calculate totals
   [r] $with_totals << |CalculateTotals <order << $with_customer

   // Step 3: Apply discounts
   [r] $with_discounts << |ApplyDiscounts <order << $with_totals

   // Chain to next
   [v] *To.Next.Iteration
   [*] <current << $with_discounts
   [*] >next >> >current

   // Collect final
   [v] *Collect.Last
   [*[*] >last >> $with_discounts

[|] >enriched << $last:array.serial
{x}
```

### Complete Example: Parallel Processing with Error Handling

```polyglot
{|} |ProcessImages
[|] <image_urls :array.string
[|] >processed :array.serial
[|] >errors :array.error

[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~ForEach.Array
[~] <array << $image_urls
[~] >item >> $url

   [z] // Try processing
      [r] $downloaded << |Download <url << $url
      [r] $resized << |Resize <image << $downloaded
      [r] $compressed << |Compress <image << $resized

      [v] *Into.Array
      [*] <item << $compressed
      [*] >array >> $success_results

   [!] !Network.Error >> $net_error
      [v] *Into.Errors
      [*] <error << $net_error
      [*] >errors >> $network_errors

   [!] !Image.Processing.Error >> $img_error
      [v] *Into.Errors
      [*] <error << $img_error
      [*] >errors >> $processing_errors

// Join all parallel iterations
[v] *Join.All
[*] <successes << $success_results
[*] <errors << $network_errors
[*] <errors << $processing_errors
[*] >processed >> $all_processed
[*] >errors >> $all_errors

[|] >processed << $all_processed
[|] >errors << $all_errors
{x}
```

---

## Implementation Notes

### Compiler Responsibilities

1. **Type Inference:** Infer iteration variable types from collection types
2. **Validation:**
   - Ensure unpack/pack labels match
   - Verify `*To.Next.Iteration` only in sequential loops
   - Check join patterns only in parallel loops
3. **Optimization:**
   - Parallel loop thread allocation
   - Collection preallocation when size known
   - Dead code elimination for unused join operations

### Runtime Behavior

1. **Sequential:** FIFO iteration order guaranteed
2. **Parallel:** Concurrency determined by runtime (thread pool size)
3. **Chained:** Output → next input happens atomically between iterations
4. **Collection:** Results aggregated after all iterations complete

---

## See Also

- [IO Operators](../User/language/syntax/io-operators.md) - I/O marker system
- [Markers Reference](../User/language/syntax/markers.md) - All markers including `[~]`, `[*]`, `[v]`
- [Error Handling](./error-handling.md) - Error propagation in iterations
- [Parallel Execution](./parallel-execution.md) - Thread safety and concurrency

---

**Status:** ✅ Specification Complete - Ready for Implementation
