# Loop System - v0.0.5

**Version:** 0.0.5
**Status:** Official Language Guide
**Last Updated:** 2026-01-04
**Audience:** Polyglot Developers

---

## Overview

Polyglot's loop system provides powerful iteration capabilities through two complementary operators:

- **Unpack (`~`)** - Iterate over collections, producing individual items
- **Pack (`*`)** - Collect results from iterations into aggregated output

This design enables:
- Parallel or sequential processing
- Accumulation and aggregation
- Nested iterations
- Type-safe collection operations

---

## Core Concepts

### Unpack Operator: `~`

Unpack operators **decompose collections** into individual items for processing.

```polyglot
[r] $items >> ~ForEach.Array
   %% Process each item in the loop body
   [r] $result << |Process"{$current}"
```

### Pack Operator: `*`

Pack operators **collect results** from loop iterations into a single output.

```polyglot
[*] *Into.Array
 *  <item.value:uint << $count
 *  >array >> $results
```

### Loop Body

The loop body is **indented** and contains the processing logic for each iteration.

---

## Unpack Operators

### `~ForEach.Array`

Iterate over array elements.

**Signature:**
```polyglot
[r] $array:array.{type} >> ~ForEach.Array
   %% Loop body - $current contains current element
   [r] $processed << |Transform"{$current}"
```

**Parameters:**
- Input: `$array` - Array to iterate over
- Output: `$current` - Current element (implicit variable)

**Example:**
```polyglot
[r] $numbers:array.uint << ( 1, 2, 3, 4, 5 )

[r] $numbers >> ~ForEach.Array
   [r] $doubled:uint << |U.Math.Multiply
    |  <a:uint << $current
    |  <b:uint << 2
    |  >result >> $doubled

   [*] *Into.Array
    *  <item.value:uint << $doubled
    *  >array >> $results
```

### `~ForEach.Set`

Iterate over set elements (unordered).

**Example:**
```polyglot
[r] $uniqueIds:set.string << { "id1", "id2", "id3" }

[r] $uniqueIds >> ~ForEach.Set
   [r] |ProcessId"{$current}"
```

### `~ForEach.Serial`

Iterate over serial fields.

**Example:**
```polyglot
[r] $config:serial << {:}  %% From some source

[r] $config >> ~ForEach.Serial
   %% $current.key and $current.value available
   [r] |Log"Key: {$current.key}, Value: {$current.value}"
```

### `~ForEach.IndexedArray`

Iterate with both index and value.

**Example:**
```polyglot
[r] $items >> ~ForEach.IndexedArray
   %% $current.index and $current.value available
   [r] |Log"[{$current.index}] = {$current.value}"
```

---

## Pack Operators

### Collection Building: `*Into.*`

#### `*Into.Array`

Collect iteration results into an array.

**Signature:**
```polyglot
[*] *Into.Array
 *  <item.{field}:{type} << $value
 *  >array >> $output
```

**Example:**
```polyglot
[r] $runtimeResults >> ~ForEach.Array
   [b] $current.success ?= -True
      [*] *Into.Array
       *  <item.runtime:string << $current.name
       *  <item.status:string << "passed"
       *  >array >> $passed
```

**Pattern: Dot Notation (Recommended)**
```polyglot
[*] *Into.Array
 *  <item.field1:type << $value1
 *  <item.field2:type << $value2
 *  >array >> $results
```

#### `*Into.Set`

Collect unique values into a set.

**Example:**
```polyglot
[r] $users >> ~ForEach.Array
   [*] *Into.Set
    *  <item.email:string << $current.email
    *  >set >> $uniqueEmails
```

#### `*Into.Serial`

Build a serial structure from iterations.

**Example:**
```polyglot
[r] $configs >> ~ForEach.Array
   [*] *Into.Serial
    *  <item.key:string << $current.name
    *  <item.value:string << $current.setting
    *  >serial >> $combinedConfig
```

### Aggregation: `*Aggregate.*`

#### `*Aggregate.Sum`

Sum numeric values across iterations.

**Example:**
```polyglot
[r] $transactions >> ~ForEach.Array
   [b] $current.type ?= #Transaction.Type.Credit
      [*] *Aggregate.Sum
       *  <value:uint << $current.amount
       *  >sum:uint >> $totalCredits

[r] $totalCredits:uint <~ 0  %% Default value
```

**Pattern: Default Values**
```polyglot
%% Declare with default BEFORE loop
[r] $total:uint <~ 0

%% Loop aggregates and overrides
[r] $items >> ~ForEach.Array
   [*] *Aggregate.Sum
    *  <value:uint << $current.value
    *  >sum:uint >> $total
```

#### `*Aggregate.Count`

Count iterations.

**Example:**
```polyglot
[r] $items >> ~ForEach.Array
   [b] $current.valid ?= -True
      [*] *Aggregate.Count
       *  >count:uint >> $validCount

[r] $validCount:uint <~ 0
```

#### `*Aggregate.Max` / `*Aggregate.Min`

Find maximum or minimum values.

**Example:**
```polyglot
[r] $scores >> ~ForEach.Array
   [*] *Aggregate.Max
    *  <value:uint << $current.score
    *  >max:uint >> $highScore

[r] $highScore:uint <~ 0
```

#### `*Aggregate.Average`

Calculate average of numeric values.

**Example:**
```polyglot
[r] $measurements >> ~ForEach.Array
   [*] *Aggregate.Average
    *  <value:float << $current.temperature
    *  >average:float >> $avgTemp
```

### String Building: `*String.*`

#### `*String.Concat`

Concatenate strings from iterations.

**Example:**
```polyglot
[r] $names >> ~ForEach.Array
   [*] *String.Concat
    *  <value:string << $current
    *  >result:string >> $allNames
```

#### `*String.Lines`

Build multi-line string from iterations.

**Example:**
```polyglot
[r] $logEntries >> ~ForEach.Array
   [*] *String.Lines
    *  <line:string << "[{$current.timestamp}] {$current.message}"
    *  >result:string >> $logText
```

---

## Parallel vs Sequential Loops

### Parallel Loops: `[p]`

Process items concurrently (order not guaranteed).

```polyglot
[p] $urls >> ~ForEach.Array
   [r] |HTTP.Get
    |  <url:string << $current
    |  >response >> $response

   [*] *Into.Array
    *  <item.url:string << $current
    *  <item.data:string << $response
    *  >array >> $results
```

**Use when:**
- Operations are independent
- Order doesn't matter
- Performance is critical

### Sequential Loops: `[r]`

Process items one after another (order preserved).

```polyglot
[r] $files >> ~ForEach.Array
   [r] |File.Append
    |  <path:path << $logFile
    |  <content:string << "Processing: {$current}\n"
```

**Use when:**
- Order matters
- Operations have side effects
- Sequential dependencies exist

---

## Complete Examples

### Example 1: Process Files with Error Handling

```polyglot
{|} |ProcessFiles
[t] |T.Cli"process-files"

[<] <input_dir:path
[>] >report#Processing.Report

%% Get all files in directory
[r] $files:array.path << |U.File.List
 |  <directory:path << $input_dir
 |  >files >> $files

%% Process each file
[r] $files >> ~ForEach.Array
   [r] |File.Read
    |  <path:path << $current
    |  >content:string >> $content
      [!] !File.Read.Error
         [*] *Into.Array
          *  <item.file:path << $current
          *  <item.success:bool << -False
          *  <item.error:string << "Read failed"
          *  >array >> $results
      [!] !*
         %% Successfully read, now process
         [r] $processed:string << |Transform"{$content}"

         [*] *Into.Array
          *  <item.file:path << $current
          *  <item.success:bool << -True
          *  <item.processed:string << $processed
          *  >array >> $results

%% Count successes and failures
[r] $results >> ~ForEach.Array
   [b] $current.success ?= -True
      [*] *Aggregate.Sum
       *  <value:uint << 1
       *  >sum:uint >> $successCount

   [b] $current.success ?= -False
      [*] *Aggregate.Sum
       *  <value:uint << 1
       *  >sum:uint >> $failureCount

[r] $successCount:uint <~ 0
[r] $failureCount:uint <~ 0

%% Build report
[>] >report
   [.] .total_files:uint << |U.Math.Add
    |  <a:uint << $successCount
    |  <b:uint << $failureCount
   [.] .successful:uint << $successCount
   [.] .failed:uint << $failureCount
   [.] .results:array.serial << $results
{x}
```

### Example 2: Data Aggregation Pipeline

```polyglot
{|} |SalesAnalysis
[t] |T.Cli"analyze-sales"

[<] <sales_data:array.serial
[>] >analysis#Sales.Analysis

%% Calculate total revenue
[r] $sales_data >> ~ForEach.Array
   [*] *Aggregate.Sum
    *  <value:uint << $current.amount
    *  >sum:uint >> $totalRevenue

%% Find highest sale
[r] $sales_data >> ~ForEach.Array
   [*] *Aggregate.Max
    *  <value:uint << $current.amount
    *  >max:uint >> $highestSale

%% Count by category
[r] $sales_data >> ~ForEach.Array
   [b] $current.category ?= "electronics"
      [*] *Aggregate.Sum
       *  <value:uint << $current.amount
       *  >sum:uint >> $electronicsRevenue

   [b] $current.category ?= "clothing"
      [*] *Aggregate.Sum
       *  <value:uint << $current.amount
       *  >sum:uint >> $clothingRevenue

%% Defaults
[r] $totalRevenue:uint <~ 0
[r] $highestSale:uint <~ 0
[r] $electronicsRevenue:uint <~ 0
[r] $clothingRevenue:uint <~ 0

%% Output
[>] >analysis
   [.] .total_revenue:uint << $totalRevenue
   [.] .highest_sale:uint << $highestSale
   [.] .electronics_revenue:uint << $electronicsRevenue
   [.] .clothing_revenue:uint << $clothingRevenue
{x}
```

### Example 3: Nested Loops

```polyglot
{|} |ProcessMatrix
[<] <matrix:array.array.uint
[>] >sums:array.uint

%% Outer loop: rows
[r] $matrix >> ~ForEach.Array
   %% $current is an array (one row)

   %% Inner loop: columns
   [r] $current >> ~ForEach.Array
      %% $current is now a single value
      [*] *Aggregate.Sum
       *  <value:uint << $current
       *  >sum:uint >> $rowSum

   [r] $rowSum:uint <~ 0

   %% Collect row sums
   [*] *Into.Array
    *  <item.sum:uint << $rowSum
    *  >array >> $sums
{x}
```

---

## Common Patterns

### Pattern 1: Filter and Collect

```polyglot
[r] $allItems >> ~ForEach.Array
   [b] $current.status ?= #Status.Active
      [*] *Into.Array
       *  <item.id:uint << $current.id
       *  <item.name:string << $current.name
       *  >array >> $activeItems
```

### Pattern 2: Transform and Aggregate

```polyglot
[r] $values >> ~ForEach.Array
   %% Transform
   [r] $doubled:uint << |U.Math.Multiply
    |  <a:uint << $current
    |  <b:uint << 2

   %% Aggregate
   [*] *Aggregate.Sum
    *  <value:uint << $doubled
    *  >sum:uint >> $total

[r] $total:uint <~ 0
```

### Pattern 3: Error Accumulation

```polyglot
[r] $operations >> ~ForEach.Array
   [r] |RiskyOperation
    |  <input:string << $current
      [!] !RiskyOperation.Error
         [*] *Into.Array
          *  <item.operation:string << $current
          *  <item.error:error << !RiskyOperation.Error
          *  >array >> $errors
      [!] !*
         [r] |U.Do.Nothing
```

### Pattern 4: Build Lookup Map

```polyglot
[r] $users >> ~ForEach.Array
   [*] *Into.Serial
    *  <item.key:string << $current.id
    *  <item.value:serial << $current
    *  >serial >> $userMap
```

---

## Best Practices

### 1. Declare Defaults Before Loops

**Good:**
```polyglot
[r] $total:uint <~ 0  %% Default declared first

[r] $items >> ~ForEach.Array
   [*] *Aggregate.Sum
    *  <value:uint << $current.value
    *  >sum:uint >> $total  %% Overrides default
```

**Bad:**
```polyglot
[r] $items >> ~ForEach.Array
   [*] *Aggregate.Sum
    *  <value:uint << $current.value
    *  >sum:uint >> $total

[r] $total:uint <~ 0  %% TOO LATE - won't work as default!
```

### 2. Use Dot Notation for Pack Items

**Recommended:**
```polyglot
[*] *Into.Array
 *  <item.field1:type << $value1
 *  <item.field2:type << $value2
 *  >array >> $results
```

**Alternative (verbose):**
```polyglot
[*] *Into.Array
 *  <item:serial
    [.] .field1:type << $value1
    [.] .field2:type << $value2
 *  >array >> $results
```

### 3. Choose Parallel vs Sequential Thoughtfully

**Parallel for independent operations:**
```polyglot
[p] $urls >> ~ForEach.Array  %% Parallel HTTP requests
   [r] |HTTP.Get"{$current}"
```

**Sequential for order-dependent operations:**
```polyglot
[r] $steps >> ~ForEach.Array  %% Sequential workflow steps
   [r] |ExecuteStep"{$current}"
```

### 4. Handle Errors in Loops

Always consider error handling within iterations:

```polyglot
[r] $items >> ~ForEach.Array
   [r] |ProcessItem"{$current}"
      [!] !ProcessItem.Error
         %% Handle error for this item
         [*] *Into.Array
          *  <item.failed:bool << -True
          *  >array >> $results
      [!] !*
         %% Success case
         [*] *Into.Array
          *  <item.success:bool << -True
          *  >array >> $results
```

### 5. Use Type-Safe Collection Literals

**Good:**
```polyglot
[r] $items:array.uint << ( 1, 2, 3, 4, 5 )
[r] $unique:set.string << { "a", "b", "c" }
```

**Bad:**
```polyglot
[r] $items:array << ( 1, 2, 3, 4, 5 )  %% Missing element type!
```

---

## Performance Considerations

### Parallel Loops

- Use `[p]` for I/O-bound operations (HTTP, file access, database queries)
- Polyglot runtime manages concurrency automatically
- Results may be in different order than input

### Memory Usage

- Large collections may consume significant memory
- Consider streaming or batching for very large datasets
- Pack operations accumulate all results in memory

### Early Termination

Currently, loops run to completion. For early termination:

```polyglot
%% Use fork to check condition before loop
[f] $shouldProcess ?= -True
   [r] $items >> ~ForEach.Array
      %% Process all items
```

---

## See Also

- [Variable Lifecycle](./variable-lifecycle.md) - Understanding default vs final states
- [Error Handling](./error-handling.md) - Error patterns in iterations
- [Standard Operators](../stdlib/standard-operators.yaml) - Complete operator reference
- [Hello World Multi-Runtime Example](../examples/hello-world-multi-runtime.pg) - Uses pack operations

---

## Summary

**Unpack operators** (`~`) decompose collections for iteration:
- `~ForEach.Array`, `~ForEach.Set`, `~ForEach.Serial`
- Access current element via `$current`

**Pack operators** (`*`) collect iteration results:
- `*Into.Array`, `*Into.Set`, `*Into.Serial` - Build collections
- `*Aggregate.Sum`, `*Aggregate.Count`, etc. - Calculate values
- `*String.Concat`, `*String.Lines` - Build strings

**Key principles:**
- Declare aggregation defaults BEFORE loops
- Use dot notation for pack items
- Choose parallel `[p]` or sequential `[r]` appropriately
- Handle errors within iterations
- Type safety throughout

---

**Status:** ✅ Official Language Guide
**Version:** 0.0.5
**Last Updated:** 2026-01-04
