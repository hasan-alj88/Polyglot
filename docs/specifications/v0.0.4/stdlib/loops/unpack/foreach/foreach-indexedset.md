---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-indexedset"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.IndexedSet
summary: "API reference: ~ForEach.IndexedSet"
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
# ~ForEach.IndexedSet

**Iterate set elements with iteration count**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.IndexedSet
[~] <set :pg.set.*
[~] >index :pg.uint
[~] >item :*
```

---

## Parameters

**Inputs:**
- `<set` :pg.set.* - Set to iterate over

**Outputs:**
- `>index` :pg.uint - Iteration counter (zero-based)
- `>item` :* - Current element (type matches set element type)

---

## Description

Iterates over a set while providing both an **iteration counter** and the **element value**. Similar to `~ForEach.Set` but includes an index output representing the iteration count.

**IMPORTANT:** Since sets are unordered, the index represents the **iteration count**, NOT the element's position in any particular order.

**Key characteristics:**
- **Zero-based counter** - First iteration has index 0
- **No guaranteed order** - Index assignment is non-deterministic
- **Unique elements** - Sets contain no duplicates

**Use when:**
- Need iteration count for sets
- Tracking progress through set
- Numbering unique items
- Set processing with counter

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $tags
[~] >index >> $i
[~] >item >> $tag

   [r] $num :pg.uint << \|U.Math.Add"{$i, 1}"
   [r] $labeled :pg.string << \|U.String.Concat"{$num, \". \", $tag}"

   [v] *String.Lines
   [*] <line << $labeled
   [*] >lines >> $numbered_tags
```

**Input:** `$tags = {"urgent", "bug", "feature"}`
**Output (order non-deterministic):**
```
1. bug
2. feature
3. urgent
```

---

### Count Set Elements

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $unique_ids
[~] >index >> $i
[~] >item >> $id

   [r] $count :pg.uint << \|U.Math.Add"{$i, 1}"

   [v] *Join.Last
   [*] <item << $count
   [*] >last >> $total_count
```

**Alternative to *Math.Count for demonstration.**

---

### Process First N Unique Items

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $all_tags
[~] >index >> $i
[~] >item >> $tag

   [y] $i < 5
      [r] $processed << \|ProcessTag <tag << $tag

      [v] *Into.Array
      [*] <item << $processed
      [*] >array >> $first_five_processed
```

**Processes first 5 unique tags encountered (order non-deterministic).**

---

### Track Progress Through Set

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $user_ids
[~] >index >> $i
[~] >item >> $user_id

   [r] $total :pg.uint << 1000  // Total known count
   [r] $percent :pg.float << \|U.Math.Divide"{$i, $total}"
   [r] $progress :pg.string << \|U.String.Concat"{\"Processing user \", $i, \" of \", $total}"

   [r] \|Log <message << $progress

   [r] $user << \|Database.Users.Find <id << $user_id

   [v] *Into.Array
   [*] <item << $user
   [*] >array >> $users
```

---

### Build Indexed Unique Items

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $unique_values
[~] >index >> $i
[~] >item >> $value

   [r] $index_str :pg.string << \|U.String.FromInt"{$i}"

   [v] *Into.Serial
   [*] <path << $index_str
   [*] <item << $value
   [*] >serial >> $indexed_unique
```

**Output (order non-deterministic):**
```json
{
  "0": "value_a",
  "1": "value_b",
  "2": "value_c"
}
```

---

## Index Semantics

**IMPORTANT:** The index represents **iteration count**, NOT element position.

**Zero-based counting:**

| Iteration | Index | Item |
|-----------|-------|------|
| First | 0 | (some element) |
| Second | 1 | (some element) |
| Third | 2 | (some element) |
| ... | ... | ... |

**Order is non-deterministic:**
- Same set may produce different index assignments across runs
- Index 0 might be assigned to different elements each time

---

## Empty Set Handling

**Empty set produces no iterations:**

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $empty
[~] >index >> $i
[~] >item >> $element

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $indices
```

**Input:** `$empty = {}`
**Output:** `$indices = []`

---

## Type Inference

**Index is always `:pg.uint`:**

```polyglot
[~] >index :pg.uint
```

**Item type matches set element type:**

| Input Type | Item Type |
|------------|-----------|
| `:pg.set.pg.int` | `:pg.int` |
| `:pg.set.pg.string` | `:pg.string` |
| `:pg.set.pg.serial` | `:pg.serial` |

---

## No Guaranteed Order

**Even with sequential `[r]` marker, order is non-deterministic:**

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $items
[~] >index >> $i
[~] >item >> $item
   // Index assignment is NOT predictable
   // Different runs may produce different orderings
```

**If deterministic order needed:**
```polyglot
// Convert set to array first
[r] $items_array :pg.array.* << \|U.Data.SetToArray"{$items}"

[r] ~ForEach.IndexedArray
[~] <array << $items_array
[~] >index >> $i
[~] >item >> $item
   // Now order is deterministic
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $items
[~] >index >> $i
[~] >item >> $element
   // Elements process one at a time
   // Order still non-deterministic
   // Index assignment consistent within single run
```

### Parallel [p]

```polyglot
[p] ~ForEach.IndexedSet
[~] <set << $items
[~] >index >> $i
[~] >item >> $element
   // All elements process concurrently
   // Order non-deterministic
   // Index assignment non-deterministic
[v]
```

---

## Common Patterns

### Pattern 1: Number Unique Items
```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $unique_tags
[~] >index >> $i
[~] >item >> $tag
   [r] $num :pg.uint << \|U.Math.Add"{$i, 1}"
   [r] $line :pg.string << \|U.String.Concat"{$num, \". \", $tag}"
   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $numbered_list
```

### Pattern 2: Limit Iterations
```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $large_set
[~] >index >> $i
[~] >item >> $item
   [y] $i < 100
      [r] $result << \|Process <input << $item
      [v] *Into.Array
      [*] <item << $result
      [*] >array >> $first_hundred
```

### Pattern 3: Progress Tracking
```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $items
[~] >index >> $i
[~] >item >> $item
   [y] $i % 10 == 0
      [r] $msg :pg.string << \|U.String.Concat"{\"Processed \", $i, \" items\"}"
      [r] \|Log <message << $msg

   [r] $result << \|Process <input << $item
   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

### Pattern 4: Build Index-Value Map
```polyglot
[r] ~ForEach.IndexedSet
[~] <set << $unique_values
[~] >index >> $i
[~] >item >> $value
   [r] $pair :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $pair.index << $i
   [r] $pair.value << $value
   [v] *Into.Array
   [*] <item << $pair
   [*] >array >> $indexed_unique
```

---

## Performance

**Time Complexity:** O(n) where n = set size

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Index assignment:** O(1) per element

---

## Comparison with Other Operators

| Operator | Index Output | Order | Duplicates | Use Case |
|----------|--------------|-------|------------|----------|
| **~ForEach.IndexedSet** | Yes (count) | Non-deterministic | No | Set with counter |
| **~ForEach.Set** | No | Non-deterministic | No | Simple set iteration |
| **~ForEach.IndexedArray** | Yes (position) | Deterministic | Possible | Array with position |

**When to use ~ForEach.IndexedSet:**
- Need iteration count
- Processing unique values
- Order doesn't matter
- Set data structure

**When to use ~ForEach.Set:**
- Don't need index
- Simple set iteration

**When to use ~ForEach.IndexedArray:**
- Need deterministic order
- Array data structure
- Position-based logic

---

## Related Operators

- [~ForEach.Set](./foreach-set.md) - Set iteration without index
- [~ForEach.IndexedArray](./foreach-indexedarray.md) - Array iteration with position
- [*Into.Set](../../pack-operators/collection-building/into/into-set.md) - Collect into set

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Data Utilities](../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
