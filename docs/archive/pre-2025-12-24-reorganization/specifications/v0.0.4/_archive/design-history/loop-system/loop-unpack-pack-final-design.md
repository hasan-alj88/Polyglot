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

# Loop Unpack/Pack Operators - Final Design

**Date:** 2025-12-11
**Status:** ✅ FINALIZED DESIGN

---

## Core Philosophy

> **"Each iteration is a mini-pipeline with named I/O."**

**Key concept:** Iterations are NOT just loop bodies - they are **mini-pipelines** that:
- Have their own scope
- Have named inputs and outputs
- Can be composed and chained
- Bridge between main pipeline and iteration execution

---

## Operator System

### Three Operator Types

| Marker | Operator | Purpose | Mnemonic |
|--------|----------|---------|----------|
| `[|]` | `<` / `>` | Pipeline I/O | Pipe = pipeline flow |
| `[~]` | `<` / `>` | Unpack (spread) | Tilde = expand/spread |
| `[*]` | `<` / `>` | Pack (collect) | Asterisk = gather/glob |

**Consistency:** All use `<` for input, `>` for output, but with different markers

### Usage Patterns

```polyglot
// Pipeline I/O
[r] |Pipeline
[|] <param << $var                    // Pipeline input
[|] >result >> $var                   // Pipeline output

// Unpack (collection → iterations)
[r] ~ForEach
[~] <array << $items                  // Unpack input: source collection
[~] >item >> $item                    // Unpack output: iteration variable

// Pack (iterations → collection)
[V] *Join.All
[*] <item << $result                  // Pack input: collect from iterations
[*] >array >> $results                // Pack output: main pipeline variable
```

---

## Unpack Semantics: `[~]`

### Unpack Binds: Main Pipeline → Iteration Mini-Pipelines

**`[~] <array << $items`**
- Takes `$items` from main pipeline scope
- Unpacks it (iterates over elements)
- Feeds into iteration mini-pipelines

**`[~] >item >> $item`**
- For **each** iteration mini-pipeline
- Output current item as `$item` input
- `$item` is visible in iteration scope only

### Example: Standard ForEach

```polyglot
[r] ~ForEach
[~] <array << $items                  // Main scope: $items → unpack
[~] >item >> $item                    // Iteration scope: create $item

   // Iteration mini-pipeline scope (indented 3 spaces)
   [r] |Transform
   [|] <input << $item                // Use $item in iteration
   [|] >result >> $processed

   [V] *Join.All
   [*] <item << $processed            // Collect $processed from each iteration
   [*] >array >> $results             // Main scope: create $results
```

### Multiple Unpack Outputs

Some unpack operators provide multiple iteration variables:

```polyglot
[r] ~Enumerate
[~] <array << $items                  // Unpack: source array
[~] >index >> $i                      // Iteration: index (0, 1, 2, ...)
[~] >item >> $item                    // Iteration: current item

   [r] $indexed << #ItemWithIndex
      [.] .index << $i
      [.] .item << $item

   [V] *Join.All
   [*] <result << $indexed
   [*] >array >> $indexed_items
```

**Note:** Multiple outputs provided by Polyglot standard library, not user-defined.

---

## Pack Semantics: `[*]`

### Pack Binds: Iteration Mini-Pipelines → Main Pipeline

**`[*] <item << $result`**
- Collects variable `$result` from **each** iteration
- Aggregates across all iterations
- Based on pack operator behavior

**`[*] >array >> $results`**
- Outputs collected results into main pipeline scope
- Creates `$results` variable in main scope
- Type depends on pack operator (array, map, single value, etc.)

### Example: Parallel with Join

```polyglot
[p] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] |ExpensiveOperation
   [|] <input << $item
   [|] >result >> $processed

   [V] *Join.All                      // Pack operator: wait for all
   [*] <item << $processed            // Collect: $processed from each
   [*] >array >> $all_results         // Output: array in main scope
```

---

## Scope Boundaries

### Iteration Mini-Pipeline Scope

```polyglot
[r] ~ForEach
[~] <array << $items                  // ← Main pipeline scope
[~] >item >> $item                    // ← Iteration input (boundary)

   // ↓ Iteration mini-pipeline scope (3-space indent)
   [r] $processed << |Transform <input << $item
   [r] $doubled << $processed * 2     // ← Only visible in iteration

   [V] *Join.All                      // ← End of iteration scope
   [*] <item << $doubled              // ← Bridge: iteration → main
   [*] >array >> $results             // ← Main pipeline scope
```

**Rules:**
- Iteration scope starts after `[~]` markers (indentation = 3 spaces)
- Iteration scope ends at `[V]` pack marker
- Variables in iteration scope NOT visible in main scope
- Variables in main scope NOT visible in iteration scope (must pass via `[~]`)
- Pack operators `[*]` bridge between scopes

---

## Pack Operators (Join Patterns)

### `[V] *Join.All` (Collect All)

**Wait for all iterations, collect all results**

```polyglot
[p] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] $result << |Process <input << $item

   [V] *Join.All
   [*] <item << $result               // Collect all $result values
   [*] >array >> $results             // Output: array of all results
```

**Behavior:**
- Blocks until all iterations complete
- Collects all `<item` values into array
- Order: unspecified for `[p]` parallel, sequential for `[r]`

### `[V] *Join.First` (Race)

**Return first iteration to complete, cancel others**

```polyglot
[p] ~ForEach
[~] <array << $mirrors                // Multiple mirror servers
[~] >item >> $url

   [r] $data << |HTTP.Get <url << $url

   [V] *Join.First
   [*] <item << $data                 // Collect first responder
   [*] >value >> $fastest             // Output: single value (first)
```

**Behavior:**
- Returns as soon as first iteration completes
- Cancels remaining iterations
- Output: single value (not array)

### `[V] *Join.All.Success` (Only Successful)

**Collect only successful iterations, ignore failures**

```polyglot
[p] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] |MayFail
   [|] <input << $item
   [|] >result >> $processed

   [V] *Join.All.Success              // Only collect successful
   [*] <item << $processed            // Collects only if $processed not failed
   [*] >array >> $successes           // Output: array of successes only
```

**Behavior:**
- Waits for all iterations
- Only collects iterations where variable is in `Ready` state (not `Failed`)
- Ignores failed iterations

### `[V] *Chain` (Chain to Next Iteration)

**Feed iteration output as input to next iteration**

```polyglot
[r] ~ForEach.Chained
[~] <array << $steps                  // Array of transformation pipelines
[~] >item >> $transform               // Current transformation

   [r] $transform
   [|] <input << $current_value
   [|] >result >> $next_value

   [V] *Chain
   [*] <current << $next_value        // From this iteration
   [*] >next >> >item                 // To next iteration (feeds back to [~])

   [V] *CollectLast
   [*] <last << $next_value           // Only last iteration
   [*] >main >> $final_result         // Output to main pipeline
```

**Key insight:** `[*] >next >> >item`
- `>next` = pack output name
- `>> >item` = feed to unpack output `>item` for next iteration
- Creates transformation chain

**Behavior:**
- Each iteration's output becomes next iteration's input
- Only works with `[r]` sequential (not `[p]` parallel)
- Final iteration collected via separate pack operator

### `[V] *CollectLast` (Only Last Iteration)

**Collect only from the last iteration**

```polyglot
[r] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] $accumulated << $previous + $item

   [V] *CollectLast
   [*] <last << $accumulated
   [*] >value >> $final_sum           // Single value: last iteration only
```

**Behavior:**
- Only collects from final iteration
- Output: single value (not array)
- Useful for accumulation patterns

### `[V] *Reduce` (Accumulator Pattern)

**Maintain accumulator across iterations**

```polyglot
[r] ~Reduce
[~] <array << $numbers
[~] <initial << 0                     // Unpack: initial accumulator value
[~] >item >> $num                     // Iteration: current item
[~] >accumulator >> $acc              // Iteration: current accumulator

   [r] $new_acc << $acc + $num

   [V] *Reduce
   [*] <accumulator << $new_acc       // Update accumulator for next iteration
   [*] >value >> $sum                 // Output: final accumulated value
```

**Behavior:**
- Maintains single accumulator value across iterations
- Each iteration updates accumulator
- Final accumulator value output to main scope

---

## Variable States and Error Handling

### Variable States

Variables in Polyglot have states:
- `Ready` - Has valid value
- `Failed` - Has error information
- `Pending` - Waiting for value
- `Default` - Using default value

### Checking State

```polyglot
[r] |MayFail
[|] <input << $item
[|] >result >> $processed             // May be Ready or Failed

[f] $processed;state =? #;pg;state;failed
   [r] $final << "N/A"                // Handle error
[f] *?
   [r] $final << $processed           // Use successful value
```

### Error Handling in Loops

```polyglot
[p] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] |MayFail
   [|] <input << $item
   [|] >result >> $processed          // Can be Failed state

   [f] $processed;state =? #;pg;state;failed
      [r] $safe_value << "ERROR"
   [f] *?
      [r] $safe_value << $processed

   [V] *Join.All
   [*] <item << $safe_value            // Always collect (handled errors)
   [*] >array >> $results
```

**Alternative: Separate success/failure collections**

```polyglot
[p] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] |MayFail
   [|] <input << $item
   [|] >result >> $processed

   [V] *Join.All.Success              // Only successes
   [*] <success << $processed
   [*] >array >> $successes

   [V] *Join.All.Failures             // Only failures
   [*] <failure << $processed         // Collects error info
   [*] >array >> $failures

// Use $successes and $failures in main pipeline
```

---

## Named Parameters for Multiple Pack Operations

### Problem: Ambiguous Mapping

```polyglot
[V] *Partition
[*] <item << $result                  // ❓ Which output?
[*] <item << $error                   // ❓ Which output?
[*] >array >> $results
[*] >array >> $errors
```

**Ambiguous:** Which `<item` goes to which `>array`?

### Solution: Named Pack Parameters

**Proposal A: Named pack operations**

```polyglot
[V] *Partition
[*] <success << $result               // Named: success
[*] <failure << $error                // Named: failure
[*] >successes >> $results            // Matches: success
[*] >failures >> $errors              // Matches: failure
```

**Proposal B: Grouped pack operations**

```polyglot
[V] *Partition.Success                // Pack operation for successes
[*] <item << $result
[*] >array >> $results

[V] *Partition.Failure                // Pack operation for failures
[*] <item << $error
[*] >array >> $errors
```

**Proposal C: Explicit mapping**

```polyglot
[V] *Partition
[*] <success << $result               // Input name: success
   [*] >array >> $results             // Output for this input

[*] <failure << $error                // Input name: failure
   [*] >array >> $errors              // Output for this input
```

**Recommendation:** Proposal A (named parameters) - clearest and most concise

---

## Complete Examples

### Example 1: Parallel Processing with Error Handling

```polyglot
{|} |ProcessOrders
[|] <orders:array.Order << $order_list

[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~ForEach
[~] <array << $order_list
[~] >item >> $order

   [r] |ValidateOrder
   [|] <order << $order
   [|] >valid >> $validated
   [|] >error >> $validation_error

   [f] $validation_error;state =? #;pg;state;failed
      [r] $final_result << $validation_error
   [f] *?
      [r] |ProcessOrder
      [|] <order << $validated
      [|] >processed >> $final_result

   [V] *Join.All.Success
   [*] <success << $final_result
   [*] >array >> $processed_orders

   [V] *Join.All.Failures
   [*] <failure << $final_result
   [*] >array >> $failed_orders

[>] o>successes << $processed_orders:array.Order
[>] o>failures << $failed_orders:array.error
{x}
```

### Example 2: Chained Transformations

```polyglot
{|} |ApplyTransformChain
[|] <initial:serial << $data
[|] <transforms:array.pipeline << $transform_steps

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~ForEach.Chained
[~] <array << $transform_steps
[~] <seed << $initial                 // Initial value
[~] >item >> $transform               // Current transformation
[~] >current >> $value                // Current chained value

   [r] $transform
   [|] <input << $value
   [|] >output >> $next_value

   [V] *Chain
   [*] <current << $next_value
   [*] >next >> >current              // Feed to next iteration

   [V] *CollectLast
   [*] <last << $next_value
   [*] >main >> $final_result

[>] o>result << $final_result:serial
{x}
```

### Example 3: Reduce Pattern (Sum)

```polyglot
{|} |SumNumbers
[|] <numbers:array.int << $number_list

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~Reduce
[~] <array << $number_list
[~] <initial << 0
[~] >item >> $num
[~] >accumulator >> $acc

   [r] $new_acc << $acc + $num

   [V] *Reduce
   [*] <accumulator << $new_acc
   [*] >value >> $total_sum

[>] o>sum << $total_sum:int
{x}
```

### Example 4: Map with Filter

```polyglot
{|} |MapAndFilter
[|] <items:array.int << $numbers

[t] |T.Call
[W] |W.Polyglot.Scope

[r] ~ForEach
[~] <array << $numbers
[~] >item >> $num

   [r] $doubled << $num * 2

   [f] $doubled >? 10                 // Only keep if > 10
      [r] $keep << $doubled
   [f] *?
      [r] $keep << #;Optional;None    // Filter out

   [V] *Join.All.Some                 // Only collect non-None values
   [*] <item << $keep
   [*] >array >> $filtered_results

[>] o>results << $filtered_results:array.int
{x}
```

### Example 5: Fire-and-Forget

```polyglot
{|} |LogEvents
[|] <events:array.Event << $event_list

[t] |T.Call
[W] |W.Polyglot.Scope

[b] ~ForEach                          // Fire-and-forget
[~] <array << $event_list
[~] >item >> $event

   [r] |LogToDatabase
   [|] <event << $event

   [r] |SendWebhook
   [|] <event << $event

// No [V] or [*] - fire-and-forget has no collection
{x}
```

---

## Standard Unpack Operators

| Operator | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `~ForEach` | `<array` | `>item` | Standard iteration |
| `~Enumerate` | `<array` | `>index`, `>item` | With index |
| `~Range` | `<start`, `<end`, `<step` | `>index` | Numeric range |
| `~Zip` | `<array1`, `<array2` | `>item1`, `>item2` | Parallel arrays |
| `~Window` | `<array`, `<size`, `<step` | `>window` | Sliding window |
| `~Chunk` | `<array`, `<size` | `>chunk` | Fixed-size chunks |
| `~ForEach.Chained` | `<array`, `<seed` | `>item`, `>current` | Chained transformations |
| `~Reduce` | `<array`, `<initial` | `>item`, `>accumulator` | Accumulator pattern |
| `~While` | `<condition` | `>item` | Conditional loop |
| `~Until` | `<condition` | `>item` | Conditional loop |

---

## Standard Pack Operators

| Operator | Behavior | Output Type |
|----------|----------|-------------|
| `*Join.All` | Wait for all, collect all | Array |
| `*Join.First` | First to complete, cancel others | Single value |
| `*Join.Any` | First success, ignore failures | Single value |
| `*Join.All.Success` | Only successful iterations | Array |
| `*Join.All.Failures` | Only failed iterations | Array |
| `*Chain` | Feed output to next input | (internal) |
| `*CollectLast` | Only last iteration | Single value |
| `*Reduce` | Maintain accumulator | Single value |

---

## Syntax Summary

### Marker Consistency

```polyglot
// Pipeline I/O
[r] |Pipeline
[|] <param << $var
[|] >result >> $var

// Unpack (collection → iterations)
[r] ~Unpack
[~] <collection << $var
[~] >item >> $var

// Pack (iterations → collection)
[V] *Pack
[*] <item << $var
[*] >collection >> $var
```

### Complete Loop Structure

```polyglot
[r/p/b] ~LoopPattern                  // Execution mode + unpack operator
[~] <input << $var                    // Unpack inputs (from main scope)
[~] >output >> $var                   // Unpack outputs (to iteration scope)

   // Iteration mini-pipeline scope (3-space indent)
   [r] // ... iteration work ...

   [V] *PackOperator                  // Pack operator (for r/p, not b)
   [*] <input << $var                 // Pack inputs (from iteration scope)
   [*] >output >> $var                // Pack outputs (to main scope)
```

---

**Status:** ✅ Complete and finalized design

**Key Achievement:** Clean, consistent operator system with mini-pipeline abstraction that makes loops first-class citizens alongside pipelines!
