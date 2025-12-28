# Loop System Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Prerequisites:** Basic pipeline syntax, understanding of arrays

---

## Overview

Polyglot's loop system uses **unpack** (`~`) and **pack** (`*`) operators to iterate over collections and collect results. Loops can run in parallel or sequentially, with explicit iteration and collection markers.

**Key Concepts:**
- `~` (tilde) = **Unpack operator** - Iterate over collections
- `*` (asterisk) = **Pack operator** - Collect iteration results
- `(~)` = Unpack operator I/O parameters
- `(*)` = Pack operator I/O parameters
- Loop body is **indented**

---

## Basic Syntax

### Unpack Operator `~`

```polyglot
[p] ~ForEach.Array          // [p] = parallel, ~ = unpack
(~) <array << $collection   // (~) = unpack operator I/O
(~) >item >> $element       // Output: current item
   // Loop body (indented)
   [r] |Process"{$element}"
```

### Pack Operator `*`

```polyglot
[*] *Into.Array            // [*] = pack marker, * = pack operator
(*) <item << $result       // (*) = pack operator I/O
```

**Result:** Collected into array in main pipeline

---

## Simple Example

### Iterate Over Files

```polyglot
{@} @Local:Examples.FileProcessing:0.0.0.1
{x}



{|} |ProcessFiles
[|] <files:pg.array.pg.path
[|] >results:pg.array.pg.string

[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |File.Read
   (|) <path << $file
   (|) >content >> $file_content

   [r] |String.Upper
   (|) <text << $file_content
   (|) >result >> $uppercased

[*] *Into.Array
(*) <item << $uppercased

// $uppercased is now array of all results
[r] $uppercased >> >results
{x}
```

**What happens:**
1. Unpack `$files` array
2. For each file, read content and uppercase it
3. Pack all uppercased results into array
4. Return as output

---

## Parallel vs Sequential

### Parallel Execution `[p]`

```polyglot
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] |SlowOperation"{$item}"
```

**Characteristics:**
- All iterations run **simultaneously**
- Non-deterministic completion order
- Faster for I/O-bound operations
- **Use when:** Operations are independent

### Sequential Execution `[r]`

```polyglot
[r] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] |OrderedOperation"{$item}"
```

**Characteristics:**
- Iterations run **one after another**
- Deterministic order
- Slower but safer
- **Use when:** Order matters or shared resources

---

## Loop Body Indentation

**Important:** Loop body MUST be indented!

```polyglot
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   // ← Indented block starts here
   [r] $processed << |Process"{$item}"
   [r] $validated << |Validate"{$processed}"
   // ← Indented block continues

[*] *Into.Array  // ← Back to original indentation
(*) <item << $validated
```

---

## Unpack Operators

### `~ForEach.Array`

Iterate over array elements:

```polyglot
[p] ~ForEach.Array
(~) <array:pg.array.T << $collection
(~) >item:T >> $element
   // Process $element
```

### `~ForEach.Set`

Iterate over set elements (unordered):

```polyglot
[p] ~ForEach.Set
(~) <set:pg.set.T << $unique_items
(~) >item:T >> $element
   // Process $element
```

### `~ForEach.IndexedArray`

Iterate with index:

```polyglot
[p] ~ForEach.IndexedArray
(~) <array:pg.array.T << $items
(~) >index:pg.int >> $i
(~) >item:T >> $element
   // Process $element with $i index
```

---

## Pack Operators

### `*Into.Array`

Collect results into array:

```polyglot
[*] *Into.Array
(*) <item:T << $result

// Main pipeline receives: pg.array.T
```

### `*Into.Set`

Collect unique results:

```polyglot
[*] *Into.Set
(*) <item:T << $result

// Main pipeline receives: pg.set.T (duplicates removed)
```

### `*Math.Sum`

Sum numeric results:

```polyglot
[p] ~ForEach.Array
(~) <array << $numbers
(~) >item >> $num
   [r] $squared :pg.int << |Math.Multiply"{$num}, {$num}"

[*] *Math.Sum
(*) <item << $squared

// Main pipeline receives: pg.int (sum of all squares)
```

### `*Math.Count`

Count iterations:

```polyglot
[*] *Math.Count

// Main pipeline receives: pg.int (number of iterations)
```

---

## Complete Example: Parallel File Processing

```polyglot
{@} @Local:Examples.LogAnalyzer:0.0.0.1
{x}



{|} |AnalyzeLogs
[%] %Doc << "Analyze log files in parallel, collect error counts"

[|] <log_folder:pg.path
[|] >total_errors:pg.int
[|] >error <~ !NoError

[r] |Folder.ListFiles
(|) <path << $log_folder
(|) <pattern << "*.log"
(|) >files >> $log_files

[p] ~ForEach.Array
(~) <array << $log_files
(~) >item >> $log_file
   [r] |File.Read
   (|) <path << $log_file
   (|) >content >> $log_content

   [r] |String.Count
   (|) <text << $log_content
   (|) <pattern << "ERROR"
   (|) >count >> $error_count

[*] *Math.Sum
(*) <item << $error_count

// $error_count is now total across all files
[r] $error_count >> >total_errors
{x}
```

**Data Flow:**
1. List all `.log` files in folder
2. **Parallel:** Read each file
3. **Parallel:** Count "ERROR" occurrences in each
4. **Pack:** Sum all error counts
5. Return total

---

## Nested Loops

### Pattern: Chained Pack Operators

```polyglot
[p] ~ForEach.Array
(~) <array << $outer_items
(~) >item >> $outer
   [p] ~ForEach.Array
   (~) <array << $inner_items
   (~) >item >> $inner
      [r] $result << |Process"{$outer}, {$inner}"

   [*] *Into.Array           // Packs to outer iteration
   (*) <item << $result

[*] *Into.Array              // Packs to main pipeline
(*) <item << $result

// Main pipeline receives: pg.array.pg.array.T (array of arrays)
```

**Important:**
- Inner pack (`[*]`) collects to outer iteration
- Outer pack (`[*]`) collects to main pipeline
- Creates nested structure

---

## Error Handling in Loops

### Pattern: Collect Success/Failure States

```polyglot
[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |RiskyOperation
   (|) <input << $file
      [!] $success:pg.bool << !
         [?] !NoError ? #True
         [?] !* ? #False

[*] *Into.Array
(*) <item << $success

// Check if all succeeded
[r] |U.Boolean.All
(|) <array << $success
(|) >result >> $all_ok

[f] $all_ok =? #False
   [r] >error << !SomeOperationsFailed
{x}
```

**Pattern:**
1. Each iteration has error block
2. Pack collects success/failure booleans
3. Check aggregate result
4. Set pipeline error based on aggregate

---

## Pipeline Composition in Loops

### Combining `|>` with Loops

```polyglot
[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |File.Load
   (|) <path << $file

   [|] |> |Data.Process
   (|) >content:pg.string >> <input

   [|] |> |Data.Validate
   (|) >processed:pg.string >> <data

   [|] |>
   (|) >is_valid:pg.bool >> $valid

[*] *Into.Array
(*) <item << $valid
```

**Benefit:** Clean multi-step processing per iteration.

---

## Common Patterns

### Pattern 1: Map (Transform Each Element)

```polyglot
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $transformed << |Transform"{$item}"

[*] *Into.Array
(*) <item << $transformed
```

### Pattern 2: Filter (Conditional Collection)

```polyglot
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $should_include :pg.bool << |CheckCondition"{$item}"

   [f] $should_include =? #True
      [r] $item >> $collected
   {x}

[*] *Into.Array
(*) <item << $collected

// Only items that passed condition
```

### Pattern 3: Reduce (Aggregate Values)

```polyglot
[r] ~ForEach.Array
(~) <array << $numbers
(~) >item >> $num
   [r] $squared :pg.int << |Math.Multiply"{$num}, {$num}"

[*] *Math.Sum
(*) <item << $squared

// Sum of squares
```

### Pattern 4: Count Matches

```polyglot
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $matches :pg.bool << |CheckMatch"{$item}"

[*] *Math.Count
(*) <item << $matches

// Count of matching items
```

---

## Loop Operators vs `(|)` Parameters

**CRITICAL DISTINCTION:**

```polyglot
// ❌ WRONG: Using (|) for loop operators
[p] ~ForEach.Array
(|) <array << $items        // ❌ Should be (~)
(|) >item >> $item          // ❌ Should be (~)

[*] *Into.Array
(|) <item << $result        // ❌ Should be (*)
```

```polyglot
// ✅ RIGHT: Using (~) and (*)
[p] ~ForEach.Array
(~) <array << $items        // ✅ Unpack I/O
(~) >item >> $item          // ✅ Unpack I/O

[*] *Into.Array
(*) <item << $result        // ✅ Pack I/O
```

**Rule:**
- `(|)` = Pipeline call parameters
- `(~)` = Unpack operator parameters
- `(*)` = Pack operator parameters

---

## Performance Considerations

### When to Use Parallel

**✅ Good for parallel:**
- Independent file operations
- API calls to different endpoints
- Data transformations without shared state
- I/O-bound operations

**❌ Bad for parallel:**
- Shared resource writes (same file)
- Order-dependent operations
- Very fast operations (overhead not worth it)
- Operations with side effects

### Example: Parallel vs Sequential

```polyglot
// ✅ GOOD: Parallel independent file reads
[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] $content << |File.Read"{$file}"

// ❌ BAD: Parallel writes to same file
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] |File.Append
   (|) <path << $shared_file    // Race condition!
   (|) <content << $item
```

**Solution for shared file:**

```polyglot
// ✅ GOOD: Collect first, then write sequentially
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $processed << |Process"{$item}"

[*] *Into.Array
(*) <item << $processed

[r] ~ForEach.Array          // Sequential write
(~) <array << $processed
(~) >item >> $line
   [r] |File.Append
   (|) <path << $output_file
   (|) <content << $line
```

---

## Troubleshooting

### Issue 1: Wrong Parameter Markers

**Error:** Using `(|)` instead of `(~)` or `(*)`

```polyglot
// ❌ WRONG
[p] ~ForEach.Array
(|) <array << $items    // Should be (~)
```

**Solution:** Use correct markers for loop operators:

```polyglot
// ✅ RIGHT
[p] ~ForEach.Array
(~) <array << $items
```

### Issue 2: Missing Indentation

**Error:** Loop body not indented

```polyglot
// ❌ WRONG: Not indented
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
[r] $result << |Process"{$item}"    // Missing indent!
```

**Solution:** Indent loop body:

```polyglot
// ✅ RIGHT: Properly indented
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $result << |Process"{$item}"    // Indented
```

### Issue 3: Pack Without Unpack

**Error:** Using pack operator without unpack

```polyglot
// ❌ WRONG: No unpack before pack
[*] *Into.Array
(*) <item << $value    // What items to collect?
```

**Solution:** Always pair pack with unpack:

```polyglot
// ✅ RIGHT: Unpack then pack
[p] ~ForEach.Array
(~) <array << $items
(~) >item >> $item
   [r] $processed << |Process"{$item}"

[*] *Into.Array
(*) <item << $processed
```

---

## Version Note

**Collector Marker Changed:**
- v0.0.3: `[v]` (vector marker)
- v0.0.4: `[*]` (collector/pack marker)

If migrating from v0.0.3, update all `[v]` to `[*]`.

---

## Quick Reference

```
┌──────────────────────────────────────────┐
│ LOOP SYSTEM                              │
├──────────────────────────────────────────┤
│                                          │
│  UNPACK (Iterate)                        │
│  [p] ~ForEach.Array    Parallel          │
│  [r] ~ForEach.Array    Sequential        │
│  (~) <array << $items                    │
│  (~) >item >> $element                   │
│     [r] // Loop body (indented)          │
│                                          │
│  PACK (Collect)                          │
│  [*] *Into.Array                         │
│  (*) <item << $result                    │
│                                          │
│  OPERATORS                               │
│  ~ = Unpack operator                     │
│  * = Pack operator                       │
│  (~) = Unpack I/O                        │
│  (*) = Pack I/O                          │
│  (|) = Pipeline I/O (NOT for loops!)     │
│                                          │
│  INDENTATION                             │
│  Loop body MUST be indented              │
│                                          │
└──────────────────────────────────────────┘
```

---

## See Also

- [Parallel Execution](./parallel-execution.md) - `[p]` marker details
- [Error Handling](../error-handling/basics.md) - Error blocks in loops
- [Pipeline Composition](../advanced/pipeline-composition.md) - `|>` in loops
- [Operators Reference](../syntax/operators.md) - Assignment operators

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-27
**Confidence:** ✅ Verified - All patterns human-validated
