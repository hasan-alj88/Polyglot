---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "zip-sets"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~Zip.Sets
summary: "API reference: ~Zip.Sets"
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
# ~Zip.Sets

**Combine multiple sets element-wise (unordered)**

**Category:** Unpack Operators > Zip
**Since:** v0.0.1

---

## Signature

### Full Syntax

```polyglot
[execution_marker] ~Zip.Sets
[~] <sets.0 :pg.set.*
[~] <sets.1 :pg.set.*
[~] <sets.N :pg.set.*
[~] >items.0 :*
[~] >items.1 :*
[~] >items.N :*
```

### Shortcut Syntax

```polyglot
[execution_marker] ~Zip.Sets
[~] <<< $set0
[~] <<< $set1
[~] <<< $setN
[~] >>> $item0
[~] >>> $item1
[~] >>> $itemN
```

---

## Parameters

**Inputs (variadic):**
- `<sets.0`, `<sets.1`, ... - Sets to zip together
- OR `<<<` - Shortcut for `<sets.N <<` (implied indexing)

**Outputs (variadic):**
- `>items.0`, `>items.1`, ... - Corresponding elements from each set
- OR `>>>` - Shortcut for `>items.N >>` (implied indexing)

**Type:** Input signature `:pg.set.serial`, output signature `:pg.set.serial`

---

## Description

Zips multiple sets together element-wise, creating iterations where each iteration receives corresponding elements from all input sets. Stops when the **smallest set is exhausted**.

**IMPORTANT:** Since sets are unordered, the pairing of elements is **non-deterministic**.

**Key characteristics:**
- **Variadic inputs** - Zip any number of sets (2 or more)
- **Non-deterministic pairing** - Order of element pairs is random
- **Smallest set wins** - Iteration stops at smallest set size
- **Shortcut syntax** - Use `<<<` and `>>>` for cleaner code
- **Unique elements** - Each set contains no duplicates

**Use when:**
- Combining sets of unique values
- Processing corresponding unique elements
- Order doesn't matter

---

## Examples

### Basic Usage - Two Sets (Shortcut Syntax)

```polyglot
[p] ~Zip.Sets
[~] <<< $set_a
[~] <<< $set_b
[~] >>> $a
[~] >>> $b

   [r] $pair :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $pair.a << $a
   [r] $pair.b << $b

   [v] *Into.Array
   [*] <item << $pair
   [*] >array >> $pairs
[v]
```

**Input:**
- `$set_a = {"apple", "banana", "cherry"}`
- `$set_b = {1, 2, 3}`

**Output (order non-deterministic):**
```json
[
  {"a": "banana", "b": 2},
  {"a": "cherry", "b": 1},
  {"a": "apple", "b": 3}
]
```

**Note:** Pairing is non-deterministic and may differ across runs.

---

### Full Syntax Example

```polyglot
[p] ~Zip.Sets
[~] <sets.0 << $tags_set
[~] <sets.1 << $priorities_set
[~] >items.0 >> $tag
[~] >items.1 >> $priority

   [r] $item :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $item.tag << $tag
   [r] $item.priority << $priority

   [v] *Into.Array
   [*] <item << $item
   [*] >array >> $tagged_priorities
[v]
```

---

### Three Sets - Unique Combinations

```polyglot
[p] ~Zip.Sets
[~] <<< $unique_names
[~] <<< $unique_colors
[~] <<< $unique_sizes
[~] >>> $name
[~] >>> $color
[~] >>> $size

   [r] $product :pg.string << \|U.String.Concat"{$name, \" - \", $color, \" - \", $size}"

   [v] *Into.Set
   [*] <item << $product
   [*] >set >> $product_variants
[v]
```

**Combines unique values from three sets (order non-deterministic).**

---

### Smallest Set Wins

```polyglot
[p] ~Zip.Sets
[~] <<< $small_set
[~] <<< $large_set
[~] >>> $small_item
[~] >>> $large_item

   [v] *Into.Array
   [*] <item << $small_item
   [*] >array >> $result
[v]
```

**Input:**
- `$small_set = {1, 2}`
- `$large_set = {10, 20, 30, 40, 50}`

**Only 2 iterations occur** (smallest set size).

---

### Build Validation Pairs

```polyglot
[p] ~Zip.Sets
[~] <<< $unique_ids_a
[~] <<< $unique_ids_b
[~] >>> $id_a
[~] >>> $id_b

   [r] $match :pg.bool << \|ValidatePair <a << $id_a <b << $id_b

   [y] $match == #False
      [r] $error :pg.string << \|U.String.Concat"{\"Mismatch: \", $id_a, \" vs \", $id_b}"
      [v] *Into.Array
      [*] <item << $error
      [*] >array >> $errors
[v]
```

---

## Non-Deterministic Pairing

**IMPORTANT:** The pairing of elements across sets is **always non-deterministic**.

```polyglot
[r] ~Zip.Sets
[~] <<< $set1
[~] <<< $set2
[~] >>> $item1
[~] >>> $item2
   // Which element from set1 pairs with which from set2 is random
```

**Even with sequential `[r]` marker, pairing is unpredictable.**

**If deterministic pairing is needed:**
```polyglot
// Convert sets to arrays first
[r] $array1 :pg.array.* << \|U.Data.SetToArray"{$set1}"
[r] $array2 :pg.array.* << \|U.Data.SetToArray"{$set2}"

[p] ~Zip.Arrays
[~] <<< $array1
[~] <<< $array2
[~] >>> $item1
[~] >>> $item2
   // Now pairing is deterministic
[v]
```

---

## Shortcut Syntax

**The `<<<` and `>>>` operators provide implied indexing:**

### Input Shortcut `<<<`

**Full syntax:**
```polyglot
[~] <sets.0 << $set0
[~] <sets.1 << $set1
[~] <sets.2 << $set2
```

**Shortcut:**
```polyglot
[~] <<< $set0
[~] <<< $set1
[~] <<< $set2
```

### Output Shortcut `>>>`

**Full syntax:**
```polyglot
[~] >items.0 >> $item0
[~] >items.1 >> $item1
[~] >items.2 >> $item2
```

**Shortcut:**
```polyglot
[~] >>> $item0
[~] >>> $item1
[~] >>> $item2
```

---

## Empty Set Handling

**If any set is empty, no iterations occur:**

```polyglot
[p] ~Zip.Sets
[~] <<< $empty
[~] <<< $other
[~] >>> $a
[~] >>> $b

   [v] *Into.Array
   [*] <item << $a
   [*] >array >> $result
[v]
```

**Input:**
- `$empty = {}`
- `$other = {1, 2, 3}`

**Output:**
- `$result = []`

---

## Type Inference

**Each output type matches corresponding set element type:**

```polyglot
[~] <<< $tags               // :pg.set.pg.string
[~] <<< $counts             // :pg.set.pg.int
[~] >>> $tag                // :pg.string
[~] >>> $count              // :pg.int
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~Zip.Sets
[~] <<< $set0
[~] <<< $set1
[~] >>> $item0
[~] >>> $item1
   // Element pairs processed one at a time
   // Pairing still non-deterministic
```

**Use when:**
- Operations have side effects
- Resource constraints
- Order doesn't matter but need sequential execution

### Parallel [p]

```polyglot
[p] ~Zip.Sets
[~] <<< $set0
[~] <<< $set1
[~] >>> $item0
[~] >>> $item1
   // All pairs processed concurrently
   // Pairing non-deterministic
[v]
```

**Use when:**
- Independent operations
- Performance critical
- Safe for concurrent execution

---

## Common Patterns

### Pattern 1: Process Unique Pairs
```polyglot
[p] ~Zip.Sets
[~] <<< $unique_users
[~] <<< $unique_roles
[~] >>> $user
[~] >>> $role
   [r] $assignment << \|AssignRole <user << $user <role << $role
   [v] *Into.Array
   [*] <item << $assignment
   [*] >array >> $assignments
[v]
```

### Pattern 2: Build Unique Combinations
```polyglot
[p] ~Zip.Sets
[~] <<< $colors
[~] <<< $sizes
[~] >>> $color
[~] >>> $size
   [r] $variant :pg.string << \|U.String.Concat"{$color, \"-\", $size}"
   [v] *Into.Set
   [*] <item << $variant
   [*] >set >> $product_variants
[v]
```

### Pattern 3: Validate Correspondences
```polyglot
[p] ~Zip.Sets
[~] <<< $expected_ids
[~] <<< $actual_ids
[~] >>> $expected
[~] >>> $actual
   [y] $expected != $actual
      [r] $error :pg.string << \|U.String.Concat"{\"ID mismatch\"}"
      [v] *Into.Array
      [*] <item << $error
      [*] >array >> $errors
[v]
```

---

## Performance

**Time Complexity:** O(min(s₁, s₂, ..., sₖ)) where sᵢ = size of set i

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(min_size) for concurrent execution

**Stops at smallest set** - Only processes as many elements as smallest set.

---

## Comparison with Other Operators

| Operator | Collection Type | Order | Pairing | Use Case |
|----------|-----------------|-------|---------|----------|
| **~Zip.Sets** | Sets | Non-deterministic | Non-deterministic | Unique value pairs |
| **~Zip.Arrays** | Arrays | Deterministic | Deterministic | Parallel arrays |
| **~ForEach.Set** | Single set | Non-deterministic | N/A | Single set iteration |

**When to use ~Zip.Sets:**
- Multiple sets of unique values
- Order doesn't matter
- Processing unique pairs

**When to use ~Zip.Arrays:**
- Deterministic pairing needed
- Ordered collections
- Parallel arrays

**When to use ~ForEach.Set:**
- Single set iteration
- No pairing needed

---

## Related Operators

- [~Zip.Arrays](./zip-arrays.md) - Zip multiple arrays (deterministic)
- [~ForEach.Set](../foreach/foreach-set.md) - Single set iteration
- [*Into.Set](../../pack-operators/collection-building/into/into-set.md) - Collect into set

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Data Utilities](../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
