<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

---

> ⚠️ **HISTORICAL DOCUMENT - DEPRECATED SYNTAX**
>
> This document contains **v0.0.3 syntax** with significant differences from v0.0.4:
>
> **Critical Syntax Changes:**
> - `[V]` (uppercase) → `[v]` (lowercase) for join marker
> - Additional prefix system refinements
> - Reserved indication using semicolon (`;`)
>
> **For current v0.0.4 syntax, see:**
> - [Main Documentation](../../README.md)
> - [v0.0.4 Grammar](../../reference/grammar.md)
> - [Markers Reference](../../language/syntax/markers.md)

---

# Loop I/O & Mini-Pipeline Iterations

**Date:** 2025-12-11
**Status:** 🔧 DRAFT SYNTAX - User Proposal

---

## Philosophy

> **"Each loop iteration is a mini-pipeline with its own I/O."**

Loops in Polyglot are not just iteration constructs - they are **collections of mini-pipelines** that can execute:
- Sequentially (`[r]`)
- In parallel (`[p]`)
- Fire-and-forget (`[b]`)

This is the same concept as:
- Error-handling `[!]` branches (mini-pipelines for errors)
- Conditional `[y]` forks (mini-pipelines for branches)
- But with **many iterations**

---

## Loop I/O Operators

### New Operators: `l<` and `l>`

Similar to pipeline definition operators (`i<` / `o>`):

| Operator | Purpose | Marker | Usage |
|----------|---------|--------|-------|
| `l<` | Loop input | `[<]` | Data flowing INTO loop/iteration |
| `l>` | Loop output | `[>]` | Data flowing OUT of iteration |

**Key distinction:**
- `i<` / `o>` - Pipeline definition (signature)
- `<` / `>` - Pipeline call (usage)
- `l<` / `l>` - **Loop iteration definition**

---

## Execution Modes

### Sequential: `[r] ~*`

**Iterations run one after another**

```polyglot
[r] ~ForEach
   [<] l<array << $array_var           // Input: source array
   [>] l>item >> $item_var             // Output: current item to iteration

   // Mini-pipeline body (runs for each item sequentially)
   [r] $processed << |Transform <input << $item_var

   [>] l>out_loop << $processed        // Iteration output
```

**Characteristics:**
- ✅ Order guaranteed
- ✅ Can use previous iteration results
- ❌ Slower for large arrays
- ✅ Can chain (see below)

### Parallel: `[p] ~*`

**Iterations run concurrently, must join**

```polyglot
[p] ~ForEach
   [<] l<array << $array_var           // Input: source array
   [>] l>item >> $item_var             // Output: current item to iteration

   // Mini-pipeline body (runs in parallel for each item)
   [r] $processed << |ExpensiveTransform <input << $item_var

   [>] l>out_loop << $processed        // Iteration output

   [V] ~V.JoinAll                      // REQUIRED: join/sync point
      [<] <out_loop                    // Collect all iteration outputs
      [>] >array_out >> $result_array  // Final result
```

**Characteristics:**
- ❌ Order not guaranteed (unless join specifies)
- ❌ Cannot chain (parallel has no "next")
- ✅ Faster for large arrays
- ✅ Must have join/sync

### Fire-and-Forget: `[b] ~*`

**Iterations run without waiting for results**

```polyglot
[b] ~ForEach
   [<] l<array << $events              // Input: events to process
   [>] l>item >> $event                // Output: current event

   // Mini-pipeline body (fire-and-forget)
   [r] |LogEvent <event << $event
   [r] |NotifyWebhook <event << $event

   // NO iteration output - fire-and-forget!

// NO join needed - we don't wait for results
```

**Characteristics:**
- ❌ No output expected
- ✅ Non-blocking
- ✅ Useful for side effects (logging, notifications)
- ❌ Cannot collect results

---

## Loop Patterns

### Pattern 1: `~ForEach` (Standard Iteration)

```polyglot
{|} |ProcessItems
[<] i<items:array.serial

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~ForEach
   [<] l<array << $items               // Source array
   [>] l>item >> $item                 // Current item

   [r] $transformed << |Transform <input << $item

   [>] l>out_loop << $transformed      // Iteration output

[>] o>results << $out_loop:array.serial
{x}
```

**Implicit join for `[r]` sequential:**
- Sequential loops don't need explicit `[V]` join
- Outputs automatically collected in order

### Pattern 2: `~Enumerate` (With Index)

```polyglot
{|} |ProcessWithIndex
[<] i<items:array.serial

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~Enumerate
   [<] l<array << $items               // Source array
   [>] l>index >> $i                   // Output: index (0-based)
   [>] l>item >> $item                 // Output: current item

   [r] $result << #ItemWithIndex
      [.] .index << $i
      [.] .item << $item

   [>] l>out_loop << $result

[>] o>indexed << $out_loop:array.serial
{x}
```

### Pattern 3: `~ForEach.Chained` (Chain Output to Next)

```polyglot
{|} |ChainedTransform
[<] i<initial:serial

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $steps:array.pipeline << {
   |Step1,
   |Step2,
   |Step3
}

[r] ~ForEach.Chained
   [<] l<array << $steps               // Array of transformations
   [<] l<seed << $initial              // Initial seed value
   [>] l>item >> $step                 // Current transformation pipeline

   [r] $step                           // Invoke transformation
      <input << $seed                  // Use seed (previous result)
      >output >> $result

   [>] l>out_loop >> l<seed            // ⭐ Chain: output becomes next seed

[>] o>final << $seed:serial            // Final chained result
{x}
```

**⭐ Key feature:** `[>] l>out_loop >> l<seed`
- Iteration output flows back as input to next iteration
- Creates transformation chain
- Only works with `[r]` sequential (not `[p]` parallel!)

**Compile error for parallel:**
```polyglot
[p] ~ForEach.Chained                   // ❌ ERROR: Cannot chain parallel iterations
```

### Pattern 4: `~Range` (Numeric Iteration)

```polyglot
{|} |GenerateSequence
[<] i<start:int
[<] i<end:int

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~Range
   [<] l<start << $start               // Start value
   [<] l<end << $end                   // End value (exclusive)
   [>] l>index >> $i                   // Current index

   [r] $squared << $i * $i

   [>] l>out_loop << $squared

[>] o>squares << $out_loop:array.int
{x}
```

**Variants:**
- `~Range.Inclusive` - Include end value
- `~Range.Step` - Custom step size
  ```polyglot
  [r] ~Range.Step
     [<] l<start << 0
     [<] l<end << 100
     [<] l<step << 5                  // Step by 5
     [>] l>index >> $i
  ```

### Pattern 5: `~While` / `~Until` (Conditional Loops)

```polyglot
{|} |FindFirst
[<] i<items:array.serial
[<] i<predicate:pipeline

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~While
   [<] l<condition << |HasMore         // Condition pipeline
      <items << $items
      >has_more >> $continue
   [>] l>item >> $item                 // Current item

   [r] $predicate
      <input << $item
      >matches >> $matches

   [y] $matches =? #;Boolean;True
      [>] l>out_loop << $item          // Found it, output and break
      [>] l>break                      // Exit loop

[>] o>found << $out_loop:optional.serial
{x}
```

---

## Reduction Operations

### Pattern: `~Reduce` (Fold/Accumulate)

```polyglot
{|} |Sum
[<] i<numbers:array.int

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~Reduce
   [<] l<array << $numbers             // Source array
   [<] l<initial << 0                  // Initial accumulator
   [>] l>item >> $num                  // Current item
   [>] l>accumulator >> $acc           // Current accumulator value

   [r] $new_acc << $acc + $num

   [>] l>accumulator << $new_acc       // Update accumulator

[>] o>sum << $accumulator:int
{x}
```

**Characteristics:**
- Single accumulator value (not array of outputs)
- Each iteration updates accumulator
- Final accumulator value is result

### Pattern: `~Recursive.Tournament` (Tree Reduction)

**User's example: `~Recursive.Tournament.RandomPair`**

```polyglot
{|} |ParallelSum
[<] i<numbers:array.int

[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~Recursive.Tournament.RandomPair   // Parallel tree reduction
   [<] l<array << $numbers             // Initial array
   [>] l>pair >> $pair                 // Pair of items

   [r] $sum << $pair.0 + $pair.1

   [>] l>reduced << $sum               // Reduced value

   [V] ~V.RecurseUntilOne              // Join: recurse until single value
      [<] <reduced
      [>] >final >> $total

[>] o>total << $total:int
{x}
```

**How it works:**
1. Pair up items randomly: `[1,2,3,4,5,6,7,8]` → pairs: `(1,2)`, `(3,4)`, `(5,6)`, `(7,8)`
2. Process each pair in parallel: `[3, 7, 11, 15]`
3. Recursively pair again: `(3,7)`, `(11,15)` → `[10, 26]`
4. Recurse: `(10,26)` → `[36]`
5. Done when single value remains

**Other tournament variants:**
- `~Recursive.Tournament.Sequential` - Pair items sequentially (1+2, result+3, result+4, ...)
- `~Recursive.Tournament.BalancedPair` - Always pair evenly (better performance)
- `~Recursive.Tournament.Adjacent` - Pair adjacent items

---

## Join/Sync Patterns

### `[V] ~V.JoinAll` (Wait for All)

**Required for `[p]` parallel loops**

```polyglot
[p] ~ForEach
   [<] l<array << $items
   [>] l>item >> $item

   [r] $processed << |SlowOperation <input << $item
   [>] l>out_loop << $processed

   [V] ~V.JoinAll                      // Wait for ALL iterations
      [<] <out_loop                    // Collect all outputs
      [>] >results >> $all_results     // Array of all results
```

**Semantics:**
- Blocks until **all** parallel iterations complete
- Collects all `l>out_loop` outputs
- Returns as array in **unspecified order** (parallel has no guaranteed order)

### `[V] ~V.JoinFirst` (Race)

```polyglot
[p] ~ForEach
   [<] l<array << $mirrors             // Multiple mirror servers
   [>] l>item >> $mirror

   [r] $response << |HTTP.Get
      <url << $mirror
      >data >> $data

   [>] l>out_loop << $data

   [V] ~V.JoinFirst                    // Wait for FIRST to complete
      [<] <out_loop
      [>] >fastest >> $result          // Single result (first responder)
      [>] >cancel                      // Cancel other iterations
```

**Semantics:**
- Returns as soon as **first** iteration completes
- Cancels remaining iterations
- Useful for racing multiple sources

### `[V] ~V.JoinAny` (Any Success)

```polyglot
[p] ~ForEach
   [<] l<array << $endpoints
   [>] l>item >> $endpoint

   [r] |TryFetch
      <url << $endpoint
      >result >> $result
      >error >> $error

   [y] $error =? #;Optional;None
      [>] l>out_loop << $result        // Success

   [V] ~V.JoinAny                      // Wait for ANY successful result
      [<] <out_loop
      [>] >first_success >> $data
```

**Semantics:**
- Returns when **any** iteration succeeds (no error)
- Ignores failed iterations
- Useful for fallback/retry patterns

### `[V] ~V.RecurseUntilOne` (Tree Reduction)

```polyglot
[p] ~Recursive.Tournament
   [<] l<array << $values
   [>] l>pair >> $pair

   [r] $reduced << |Combine
      <left << $pair.0
      <right << $pair.1
      >result >> $result

   [>] l>reduced << $result

   [V] ~V.RecurseUntilOne              // Recurse until single value
      [<] <reduced
      [>] >final >> $final_value
```

**Semantics:**
- Recursively pairs and reduces
- Stops when single value remains
- Tree-like reduction (log N depth)

---

## Solving the Transformation Problem

**Original problem:**
```polyglot
[r] $result << "initial"

// Want to transform multiple times...
[r] $result << |Transform1 <input << $result    // ❌ Can't reassign!
```

**Solution 1: Sequential variables (functional style)**

```polyglot
[r] $step0 << "initial"
[r] $step1 << |Transform1 <input << $step0
[r] $step2 << |Transform2 <input << $step1
[r] $step3 << |Transform3 <input << $step2

[>] o>result << $step3:string
```

**Solution 2: Pipeline composition**

```polyglot
[r] |Transform1 |> |Transform2 |> |Transform3
[|] <input:pg.string << "initial"
[|] >output1:pg.string >> <input
[|] >output2:pg.string >> <input
[|] |>
[|] >final:pg.string >> $result

[>] o>result << $result:string
```

**Solution 3: Chained loop**

```polyglot
[r] $transformations:array.pipeline << {
   |Transform1,
   |Transform2,
   |Transform3
}

[r] ~ForEach.Chained
   [<] l<array << $transformations
   [<] l<seed << "initial"
   [>] l>item >> $transform

   [r] $transform
      <input << $seed
      >output >> $result

   [>] l>out_loop >> l<seed            // Chain to next

[>] o>result << $seed:string
```

**Key insight:** No mutable variables needed! Use functional composition or chained loops.

---

## Accumulator Pattern (No Mutability)

**Problem:** Need to accumulate values across iterations

**Solution: Use `~Reduce` or chained loop**

```polyglot
{|} |SumNumbers
[<] i<numbers:array.int

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~Reduce
   [<] l<array << $numbers
   [<] l<initial << 0                  // Initial accumulator
   [>] l>item >> $num
   [>] l>accumulator >> $acc           // Current accumulator

   [r] $new_acc << $acc + $num

   [>] l>accumulator << $new_acc       // New accumulator (not reassignment!)

[>] o>sum << $accumulator:int
{x}
```

**Key:** Each iteration gets **current accumulator** and produces **new accumulator**. No mutation!

---

## More Loop Patterns (Suggestions)

### `~Filter`

```polyglot
[r] ~Filter
   [<] l<array << $items
   [<] l<predicate << |IsEven
   [>] l>item >> $item

   [r] $predicate
      <input << $item
      >matches >> $matches

   [y] $matches =? #;Boolean;True
      [>] l>out_loop << $item          // Keep item

// Only matching items in output
[>] o>filtered << $out_loop:array.int
```

### `~Map`

```polyglot
[r] ~Map
   [<] l<array << $items
   [<] l<mapper << |Double
   [>] l>item >> $item

   [r] $mapper
      <input << $item
      >output >> $mapped

   [>] l>out_loop << $mapped

[>] o>mapped << $out_loop:array.int
```

### `~FlatMap`

```polyglot
[r] ~FlatMap
   [<] l<array << $nested_arrays       // Array of arrays
   [>] l>item >> $sub_array

   [r] ~ForEach
      [<] l<array << $sub_array
      [>] l>item >> $element
      [>] l>out_loop << $element       // Flatten

// Flattened array
[>] o>flat << $out_loop:array.serial
```

### `~TakeWhile`

```polyglot
[r] ~TakeWhile
   [<] l<array << $items
   [<] l<condition << |IsPositive
   [>] l>item >> $item

   [r] $condition
      <input << $item
      >matches >> $matches

   [y] $matches =? #;Boolean;True
      [>] l>out_loop << $item          // Take item
   [y] *?
      [>] l>break                      // Stop iteration

[>] o>taken << $out_loop:array.int
```

### `~DropWhile`

```polyglot
[r] ~DropWhile
   [<] l<array << $items
   [<] l<condition << |IsNegative
   [>] l>item >> $item
   [>] l>dropped >> $should_drop       // Flag: still dropping?

   [y] $should_drop =? #;Boolean;True
      [r] $condition
         <input << $item
         >matches >> $matches

      [y] $matches =? #;Boolean;False
         [>] l>dropped << #;Boolean;False   // Stop dropping
         [>] l>out_loop << $item
   [y] *?
      [>] l>out_loop << $item          // Keep item

[>] o>remaining << $out_loop:array.int
```

### `~Partition`

```polyglot
[r] ~Partition
   [<] l<array << $items
   [<] l<predicate << |IsEven
   [>] l>item >> $item

   [r] $predicate
      <input << $item
      >matches >> $matches

   [y] $matches =? #;Boolean;True
      [>] l>out_true << $item
   [y] *?
      [>] l>out_false << $item

[>] o>evens << $out_true:array.int
[>] o>odds << $out_false:array.int
{x}
```

### `~Zip`

```polyglot
[r] ~Zip
   [<] l<array1 << $first_array
   [<] l<array2 << $second_array
   [>] l>item1 >> $item1
   [>] l>item2 >> $item2

   [r] $pair << #Pair
      [.] .first << $item1
      [.] .second << $item2

   [>] l>out_loop << $pair

[>] o>zipped << $out_loop:array.Pair
{x}
```

### `~GroupBy`

```polyglot
[r] ~GroupBy
   [<] l<array << $items
   [<] l<key_fn << |GetCategory
   [>] l>item >> $item

   [r] $key_fn
      <input << $item
      >key >> $key

   [>] l>out_group << #GroupItem
      [.] .key << $key
      [.] .value << $item

// Implicit grouping by key
[>] o>grouped << $out_group:map.string.array.serial
{x}
```

---

## Complete Example: Parallel Processing with Join

```polyglot
{|} |ProcessOrdersBatch
[<] i<order_ids:array.string

[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~ForEach
   [<] l<array << $order_ids           // Input: array of order IDs
   [>] l>item >> $order_id             // Output: current order ID

   // Mini-pipeline for each order (runs in parallel)
   [r] |Database.GetOrder
      <id << $order_id
      >order >> $order
      >error >> $error

   [y] $error =? #;Optional;None
      [r] |ProcessOrder
         <order << $order
         >processed >> $processed

      [>] l>out_loop << $processed     // Success: output processed order
   [y] *?
      [>] l>out_error << $error        // Failure: output error

   [V] ~V.JoinAll                      // Wait for all iterations
      [<] <out_loop                    // Collect successful results
      [<] <out_error                   // Collect errors
      [>] >processed_orders >> $successes
      [>] >failed_orders >> $failures

[>] o>successes << $successes:array.serial
[>] o>failures << $failures:array.error
{x}
```

---

## Improvements & Questions

### Improvement 1: Loop State Output

**Current draft doesn't show loop state clearly**

Suggestion: Make loop state outputs explicit:

```polyglot
[r] ~ForEach
   [<] l<array << $items
   [>] l>item >> $item                 // Item output
   [>] l>index >> $i                   // Index output (if needed)
   [>] l>is_first >> $is_first         // Boolean: first iteration?
   [>] l>is_last >> $is_last           // Boolean: last iteration?
```

### Improvement 2: Break/Continue

**How to break or continue loops?**

Suggestion: Use special loop outputs:

```polyglot
[r] ~ForEach
   [<] l<array << $items
   [>] l>item >> $item

   [y] $item =? "stop"
      [>] l>break                      // Exit loop immediately

   [y] $item =? "skip"
      [>] l>continue                   // Skip to next iteration

   [>] l>out_loop << $item
```

### Improvement 3: Error Handling in Loops

**How to handle errors in iterations?**

Suggestion: Loop-level error output:

```polyglot
[r] ~ForEach
   [<] l<array << $items
   [>] l>item >> $item

   [r] |MayFail
      <input << $item
      >result >> $result
      >error >> $error

   [y] $error =? #;Optional;None
      [>] l>out_loop << $result        // Success
   [y] *?
      [>] l>out_error << $error        // Error (separate output)

// Two outputs: successes and errors
[>] o>results << $out_loop:array.serial
[>] o>errors << $out_error:array.error
```

### Improvement 4: Nested Loops

**How do nested loops work?**

```polyglot
[r] ~ForEach
   [<] l<array << $outer_items
   [>] l>item >> $outer

   [r] ~ForEach
      [<] l<array << $outer.children
      [>] l>item >> $inner

      [r] $combined << #Pair
         [.] .outer << $outer
         [.] .inner << $inner

      [>] l>out_loop << $combined      // Inner loop output

   [>] l>out_loop << $inner_results    // Outer loop output

[>] o>pairs << $out_loop:array.Pair
```

### Improvement 5: Loop Metadata

**Can we add metadata to loops?**

```polyglot
[p] ~ForEach
   [%] %MaxConcurrency << 10           // Limit parallel executions
   [%] %Timeout << 5000                // Timeout per iteration (ms)
   [%] %Retry << 3                     // Retry failed iterations

   [<] l<array << $items
   [>] l>item >> $item

   // ... iteration body
```

---

## Syntax Summary

### Loop Markers
- `[r] ~*` - Sequential loop
- `[p] ~*` - Parallel loop
- `[b] ~*` - Fire-and-forget loop

### Loop I/O Operators
- `l<` - Loop input (with `[<]` marker)
- `l>` - Loop output (with `[>]` marker)

### Join/Sync Marker
- `[V] ~V.*` - Join/sync pattern

### Loop Patterns
- `~ForEach` - Standard iteration
- `~Enumerate` - With index
- `~ForEach.Chained` - Chain output to input (sequential only)
- `~Range` - Numeric iteration
- `~While` / `~Until` - Conditional loops
- `~Reduce` - Accumulator/fold
- `~Recursive.Tournament.*` - Tree reduction
- `~Map`, `~Filter`, `~FlatMap` - Functional patterns
- `~TakeWhile`, `~DropWhile` - Conditional take/drop
- `~Partition` - Split by predicate
- `~Zip` - Combine two arrays
- `~GroupBy` - Group by key

### Join Patterns
- `~V.JoinAll` - Wait for all
- `~V.JoinFirst` - Race (first to complete)
- `~V.JoinAny` - Any success (ignore failures)
- `~V.RecurseUntilOne` - Tree reduction

---

**Status:** 🔧 Excellent foundation - Ready for refinement

**Key Achievement:** Loop mini-pipelines with explicit I/O solve the transformation problem without mutable variables!
