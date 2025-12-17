---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "join-nth"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Join.Nth"
summary: "API reference: *Join.Nth"
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# *Join.Nth

**Take Nth iteration result**

**Category:** Pack Operators > Collect
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Join.Nth
[*] <item
[*] <n :pg.uint
[*] >nth
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope
- `<n` :pg.uint - Zero-based index of iteration to collect

**Outputs:**
- `>nth` - Nth iteration result in main scope

---

## Description

Collects only the result from the **Nth iteration** (zero-based) and discards all others. In sequential loops, this is deterministic by position. In parallel loops, order is non-deterministic.

**Key characteristics:**
- **Zero-based indexing** - N=0 is first iteration
- **Single result** - Only Nth iteration is collected
- **Discards others** - All other iterations are ignored
- **Type inference** - Output type matches item type

**Use when:**
- Need specific position result
- Sampling from iterations
- Testing/debugging specific iteration
- Skip first/last N pattern

---

## Examples

### Basic Usage - Get 3rd Element

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|Process <input << $element

   [v] *Join.Nth
   [*] <item << $result
   [*] <n << 2
   [*] >nth >> $third_result
```

**Input:** `$items = ["a", "b", "c", "d", "e"]`
**Output:** `$third_result = Process("c")`

**Index 2 = 3rd element (zero-based).**

---

### Sample Every Nth Iteration

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $data
[~] >index >> $i
[~] >item >> $element

   [r] $remainder :pg.uint << \|U.Math.Mod"{$i, 5}"

   [y] $remainder == 0
      [v] *Join.Nth
      [*] <item << $element
      [*] <n << $i
      [*] >nth >> $sampled_item
```

**Collects every 5th element.**

---

### Get Specific Result by Position

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 100
[~] >index >> $i

   [r] $squared :pg.int << \|U.Math.Multiply"{$i, $i}"

   [v] *Join.Nth
   [*] <item << $squared
   [*] <n << 10
   [*] >nth >> $tenth_square
```

**Output:** `$tenth_square = 100` (10²)

---

### Skip First N, Take Next

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $element

   [y] $i >= 5
      [v] *Join.Nth
      [*] <item << $element
      [*] <n << 5
      [*] >nth >> $sixth_element
```

**Gets 6th element (index 5).**

---

### Dynamic N Selection

```polyglot
[r] $target_index :pg.uint << 7

[r] ~ForEach.Array
[~] <array << $results
[~] >item >> $result

   [v] *Join.Nth
   [*] <item << $result
   [*] <n << $target_index
   [*] >nth >> $selected_result
```

**N can be dynamically determined.**

---

## Out of Bounds Handling

**If N >= iteration count, no result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $small_array
[~] >item >> $element

   [v] *Join.Nth
   [*] <item << $element
   [*] <n << 100
   [*] >nth >> $result
```

**Input:** `$small_array = [1, 2, 3]`
**Behavior:** No value assigned to `$result` (variable undefined)

**Handle with default:**
```polyglot
[r] $result << <~ $default_value ~> $nth_result
```

---

## Zero-Based Indexing

| N | Iteration | Description |
|---|-----------|-------------|
| 0 | First | Same as *Join.First |
| 1 | Second | 2nd iteration |
| 2 | Third | 3rd iteration |
| ... | ... | ... |

**To get last element, use *Join.Last instead.**

---

## Type Inference

**Output type matches item type:**

| Item Type | Nth Type |
|-----------|----------|
| `:pg.int` | `:pg.int` |
| `:pg.string` | `:pg.string` |
| `:pg.serial` | `:pg.serial` |

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [v] *Join.Nth
   [*] <item << $element
   [*] <n << 3
   [*] >nth >> $fourth_item
```

**Result:** 4th element in array order (deterministic).

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|Process <input << $element

   [v] *Join.Nth
   [*] <item << $result
   [*] <n << 3
   [*] >nth >> $some_result
[v]
```

**Result:** Some iteration result (non-deterministic which one).

**Note:** In parallel mode, "Nth" has no guaranteed meaning since order is non-deterministic.

---

## Common Patterns

### Pattern 1: Get Specific Position
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [v] *Join.Nth
   [*] <item << $item
   [*] <n << 5
   [*] >nth >> $sixth_item
```

### Pattern 2: Sample Data
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $large_dataset
[~] >index >> $i
[~] >item >> $data
   [y] $i == 100
      [v] *Join.Nth
      [*] <item << $data
      [*] <n << 100
      [*] >nth >> $sample
```

### Pattern 3: Validate Specific Iteration
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 1000
[~] >index >> $i
   [r] $result << \|ComputeValue <i << $i
   [v] *Join.Nth
   [*] <item << $result
   [*] <n << 500
   [*] >nth >> $midpoint_value
```

### Pattern 4: Skip Header Rows
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $csv_rows
[~] >index >> $i
[~] >item >> $row
   [y] $i == 1  // Get first data row (skip header at 0)
      [v] *Join.Nth
      [*] <item << $row
      [*] <n << 1
      [*] >nth >> $first_data_row
```

---

## Performance

**Time Complexity:**
- Sequential: O(n) where n = N (must iterate to Nth position)
- Parallel: O(max(t₁, t₂, ..., tₙ)) - all iterations run

**Space Complexity:** O(1) - single result

**Note:** All iterations may run, but only Nth result is kept.

---

## Use Cases

### Good Use Cases

**Get specific element:**
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [v] *Join.Nth
   [*] <item << $item
   [*] <n << 5
   [*] >nth >> $sixth_item
```

**Sampling:**
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $data
[~] >index >> $i
[~] >item >> $item
   [r] $is_sample :pg.bool << $i == 10
   [y] $is_sample == #True
      [v] *Join.Nth
      [*] <item << $item
      [*] <n << 10
      [*] >nth >> $sample
```

### Alternative Approaches

**If you just want array element, use direct access:**
```polyglot
// Instead of this:
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [v] *Join.Nth
   [*] <item << $item
   [*] <n << 5
   [*] >nth >> $sixth_item

// Use this:
[r] $sixth_item << $items.5
```

---

## Comparison with Other Operators

| Operator | Collects | N Parameter | Use Case |
|----------|----------|-------------|----------|
| **\*Join.Nth** | Nth | Yes | Specific position |
| **\*Join.First** | First | No | First result |
| **\*Join.Last** | Last | No | Last result |
| **\*Into.Array** | All | No | All results |

**When to use \*Join.Nth:**
- Need specific position result
- Sampling iterations
- Testing specific case
- N is dynamic

**When to use direct access:**
- N is constant
- Source is array
- Simpler code

---

## Related Operators

- [*Join.First](./join-first.md) - Take first result
- [*Join.Last](./join-last.md) - Take last result
- [*Into.Array](../collection-building/into/into-array.md) - Collect all results

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
