---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-set"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.Set
summary: "API reference: ~ForEach.Set"
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
# ~ForEach.Set

**Iterate over set elements (unordered)**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.Set
[~] <set :pg.set.*
[~] >item :*
```

---

## Parameters

**Inputs:**
- `<set` :pg.set.* - Set to iterate over

**Outputs:**
- `>item` :* - Current element (type matches set element type)

---

## Description

Expands a set from the main scope into iteration scope, creating one iteration per unique element. Each iteration receives one element through `>item`.

**Key characteristics:**
- **No guaranteed order** - Even with `[r]` sequential marker, order is non-deterministic
- **Unique elements** - Sets contain no duplicates
- **Type inference** - Output type matches set element type

**Use when:**
- Processing unique values
- Order doesn't matter
- Working with set data structures

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Set
[~] <set << $unique_ids
[~] >item >> $id

   [r] $user << \|Database.Users.Find <id << $id

   [v] *Into.Array
   [*] <item << $user
   [*] >array >> $users
```

**Note:** Even with `[r]` marker, iteration order is **not guaranteed**.

---

### Process Unique Values

```polyglot
[p] ~ForEach.Set
[~] <set << $unique_emails
[~] >item >> $email

   [r] $sent :pg.bool << \|Email.Send <to << $email <subject << "Notice"

   [v] *Into.Set
   [*] <item << $email
   [*] >set >> $processed_emails
[v]
```

**Ensures each email is sent only once, no duplicates.**

---

### Convert Set to Array

```polyglot
[r] ~ForEach.Set
[~] <set << $tags_set
[~] >item >> $tag

   [v] *Into.Array
   [*] <item << $tag
   [*] >array >> $tags_array
```

**Converts set to array (order non-deterministic).**

---

### Filter Set Elements

```polyglot
[r] ~ForEach.Set
[~] <set << $all_tags
[~] >item >> $tag

   [y] $tag != "deprecated"
      [v] *Into.Set
      [*] <item << $tag
      [*] >set >> $active_tags
```

---

### Aggregate Set Values

```polyglot
[r] ~ForEach.Set
[~] <set << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total

   [v] *Math.Count
   [*] <item << $num
   [*] >count >> $unique_count
```

---

## Empty Set Handling

**Empty set produces no iterations:**

```polyglot
[r] ~ForEach.Set
[~] <set << $empty
[~] >item >> $element

   [v] *Into.Set
   [*] <item << $element
   [*] >set >> $result
```

**Input:** `$empty = {}`
**Output:** `$result = {}`

---

## Type Inference

**Output type matches set element type:**

| Input Type | Output Type |
|------------|-------------|
| `:pg.set.pg.int` | `:pg.int` |
| `:pg.set.pg.string` | `:pg.string` |
| `:pg.set.pg.serial` | `:pg.serial` |

---

## No Guaranteed Order

**IMPORTANT:** Set iteration order is **always non-deterministic**, even with sequential `[r]` marker.

```polyglot
[r] ~ForEach.Set
[~] <set << $items
[~] >item >> $item
   // Order is NOT guaranteed
```

**If order matters, use ~ForEach.Array instead:**
```polyglot
[r] $items_array :pg.array.* << \|U.Data.SetToArray"{$items}"
[r] ~ForEach.Array
[~] <array << $items_array
[~] >item >> $item
   // Order is now deterministic
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Set
[~] <set << $items
[~] >item >> $element
   // Elements process one at a time
   // Order still non-deterministic
```

**Use when:**
- Operations have side effects
- Resource constraints
- Order doesn't matter but need sequential execution

### Parallel [p]

```polyglot
[p] ~ForEach.Set
[~] <set << $items
[~] >item >> $element
   // All elements process concurrently
   // Order non-deterministic
[v]
```

**Use when:**
- Independent operations
- Performance critical
- Safe for concurrent execution

---

## Common Patterns

### Pattern 1: Deduplicate and Process
```polyglot
// First, convert array to set (deduplicates)
[r] $unique_items :pg.set.* << \|U.Data.ArrayToSet"{$items}"

// Then iterate unique values
[r] ~ForEach.Set
[~] <set << $unique_items
[~] >item >> $item
   [r] $processed << \|Process <input << $item
   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

### Pattern 2: Set Operations
```polyglot
[r] ~ForEach.Set
[~] <set << $set_a
[~] >item >> $item
   // Check if item in set_b
   [r] $in_b :pg.bool << \|U.Data.SetContains"{$set_b, $item}"
   [y] $in_b == #True
      [v] *Into.Set
      [*] <item << $item
      [*] >set >> $intersection
```

### Pattern 3: Build String from Set
```polyglot
[r] ~ForEach.Set
[~] <set << $tags
[~] >item >> $tag
   [v] *String.Concat
   [*] <string << $tag
   [*] >concatenated >> $tags_string
// Note: Order non-deterministic
```

### Pattern 4: Aggregate Unique Values
```polyglot
[r] ~ForEach.Set
[~] <set << $unique_amounts
[~] >item >> $amount
   [v] *Math.Sum
   [*] <item << $amount
   [*] >sum >> $total_unique
```

---

## Performance

**Time Complexity:** O(n) where n = set size

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Set access:** O(1) average case for element access

---

## Comparison with Other Operators

| Operator | Order | Duplicates | Collection Type |
|----------|-------|------------|-----------------|
| **~ForEach.Set** | Non-deterministic | No | Set |
| **~ForEach.Array** | Deterministic | Possible | Array |
| **~ForEach.IndexedSet** | Non-deterministic | No | Set (with index) |

**When to use ~ForEach.Set:**
- Processing unique values
- Order doesn't matter
- Set data structure

**When to use ~ForEach.Array:**
- Order matters
- May have duplicates
- Array data structure

**When to use ~ForEach.IndexedSet:**
- Need iteration count
- Set with position tracking

---

## Related Operators

- [~ForEach.Array](./foreach-array.md) - Array iteration (ordered)
- [~ForEach.IndexedSet](./foreach-indexedset.md) - Set iteration with index
- [*Into.Set](../../pack-operators/collection-building/into/into-set.md) - Collect into set

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Data Utilities](../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
