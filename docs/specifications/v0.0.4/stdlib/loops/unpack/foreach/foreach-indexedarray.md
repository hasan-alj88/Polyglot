---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-indexedarray"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.IndexedArray
summary: "API reference: ~ForEach.IndexedArray"
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
# ~ForEach.IndexedArray

**Iterate array elements with index**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.IndexedArray
[~] <array :pg.array.*
[~] >index :pg.uint
[~] >item :*
```

---

## Parameters

**Inputs:**
- `<array` :pg.array.* - Array to iterate over

**Outputs:**
- `>index` :pg.uint - Zero-based index of current element
- `>item` :* - Current element (type matches array element type)

---

## Description

Iterates over an array while providing both the **element index** (position) and the **element value**. Similar to `~ForEach.Array` but includes index output.

**Key characteristics:**
- **Zero-based indexing** - First element has index 0
- **Type inference** - Item type matches array element type
- **Order preserved** - Sequential iteration respects array order

**Use when:**
- Need element position
- Building indexed structures
- Position-based logic
- Labeling or numbering items

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item

   [r] $labeled :pg.string << \|U.String.Concat"{\"[\", $i, \"] \", $item}"

   [v] *String.Lines
   [*] <line << $labeled
   [*] >lines >> $numbered_list
```

**Input:** `$items = ["Apple", "Banana", "Cherry"]`
**Output:**
```
[0] Apple
[1] Banana
[2] Cherry
```

---

### Build Indexed Serial Data

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $values
[~] >index >> $i
[~] >item >> $value

   [r] $index_str :pg.string << \|U.String.FromInt"{$i}"
   [r] $path :pg.string << \|U.String.Concat"{\"items.\", $index_str}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $value
   [*] >serial >> $indexed_data
```

**Input:** `$values = ["a", "b", "c"]`
**Output:**
```json
{
  "items": {
    "0": "a",
    "1": "b",
    "2": "c"
  }
}
```

---

### Skip First N Elements

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $all_items
[~] >index >> $i
[~] >item >> $item

   [y] $i >= 5
      [v] *Into.Array
      [*] <item << $item
      [*] >array >> $items_after_five
```

**Collects elements starting from index 5.**

---

### Process Even-Indexed Elements

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item

   [r] $remainder :pg.uint << \|U.Math.Mod"{$i, 2}"

   [y] $remainder == 0
      [v] *Into.Array
      [*] <item << $item
      [*] >array >> $even_indexed
```

**Collects items at indices 0, 2, 4, 6, ...**

---

### Build CSV with Row Numbers

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $records
[~] >index >> $i
[~] >item >> $record

   [r] $row_num :pg.uint << \|U.Math.Add"{$i, 1}"  // 1-based numbering
   [r] $csv_row :pg.string << \|U.String.Concat"{$row_num, \",\", $record.name, \",\", $record.value}"

   [v] *String.Lines
   [*] <line << $csv_row
   [*] >lines >> $csv_content
```

**Output:**
```
1,Alice,100
2,Bob,200
3,Charlie,300
```

---

### Create Lookup Map

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $tags
[~] >index >> $i
[~] >item >> $tag

   [r] $index_str :pg.string << \|U.String.FromInt"{$i}"

   [v] *Into.Serial
   [*] <path << $index_str
   [*] <item << $tag
   [*] >serial >> $tag_lookup
```

**Input:** `$tags = ["urgent", "bug", "feature"]`
**Output:**
```json
{
  "0": "urgent",
  "1": "bug",
  "2": "feature"
}
```

---

## Zero-Based Indexing

**Indices start at 0:**

| Element | Index |
|---------|-------|
| First element | 0 |
| Second element | 1 |
| Third element | 2 |
| ... | ... |
| Last element | length - 1 |

**Convert to 1-based if needed:**
```polyglot
[r] $one_based :pg.uint << \|U.Math.Add"{$i, 1}"
```

---

## Empty Array Handling

**Empty array produces no iterations:**

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $empty
[~] >index >> $i
[~] >item >> $element

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $indices
```

**Input:** `$empty = []`
**Output:** `$indices = []`

---

## Type Inference

**Index is always `:pg.uint`:**

```polyglot
[~] >index :pg.uint
```

**Item type matches array element type:**

| Input Type | Item Type |
|------------|-----------|
| `:pg.array.pg.int` | `:pg.int` |
| `:pg.array.pg.string` | `:pg.string` |
| `:pg.array.pg.serial` | `:pg.serial` |

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $element
   // Index 0 completes before index 1 starts
   // Predictable order
```

**Use when:**
- Order matters
- Position-dependent operations
- Sequential processing required

### Parallel [p]

```polyglot
[p] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $element
   // All indices process concurrently
   // Results may complete out of order
[v]
```

**Note:** Even with parallel execution, indices are assigned correctly, but iteration completion order is non-deterministic.

---

## Common Patterns

### Pattern 1: Numbered List
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item
   [r] $num :pg.uint << \|U.Math.Add"{$i, 1}"
   [r] $line :pg.string << \|U.String.Concat"{$num, \". \", $item}"
   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $numbered_list
```

### Pattern 2: Conditional by Position
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $values
[~] >index >> $i
[~] >item >> $value
   [y] $i < 10
      [v] *Into.Array
      [*] <item << $value
      [*] >array >> $first_ten
```

### Pattern 3: Build Index-Value Pairs
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $data
[~] >index >> $i
[~] >item >> $value
   [r] $pair :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $pair.index << $i
   [r] $pair.value << $value
   [v] *Into.Array
   [*] <item << $pair
   [*] >array >> $indexed_pairs
```

### Pattern 4: Track Progress
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $tasks
[~] >index >> $i
[~] >item >> $task
   [r] $progress :pg.string << \|U.String.Concat"{\"Processing \", $i, \" of \", $count}"
   [r] \|Log <message << $progress
   [r] $result << \|ProcessTask <task << $task
   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

---

## Performance

**Time Complexity:** O(n) where n = array length

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Index calculation:** O(1) per element

---

## Comparison with Other Operators

| Operator | Index Output | Collection Type | Order | Use Case |
|----------|--------------|-----------------|-------|----------|
| **~ForEach.IndexedArray** | Yes | Array | Deterministic | Need position |
| **~ForEach.Array** | No | Array | Deterministic | Don't need position |
| **~ForEach.IndexedSet** | Yes (count) | Set | Non-deterministic | Set with count |

**When to use ~ForEach.IndexedArray:**
- Need element position
- Building numbered lists
- Position-based filtering
- Indexed processing

**When to use ~ForEach.Array:**
- Don't need index
- Standard array iteration
- Simpler code

---

## Related Operators

- [~ForEach.Array](./foreach-array.md) - Array iteration without index
- [~ForEach.IndexedSet](./foreach-indexedset.md) - Set iteration with index
- [~Iter.Range](../iter/iter-range.md) - Generate index range

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
