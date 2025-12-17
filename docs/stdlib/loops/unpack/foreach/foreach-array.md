---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-array"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.Array
summary: "API reference: ~ForEach.Array"
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
# ~ForEach.Array

**Iterate over array elements**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.Array
[~] <array :pg.array.*
[~] >item :*
```

---

## Parameters

**Inputs:**
- `<array` :pg.array.* - Array to iterate over

**Outputs:**
- `>item` :* - Current element (type matches array element type)

---

## Description

The **most common unpack operator**. Expands an array from the main scope into iteration scope, creating one iteration per array element. Each iteration receives one element through `>item`.

**Type inference:** Output type matches array element type.

**Order:**
- Sequential `[r]`: Elements processed in array order
- Parallel `[p]`: Elements processed concurrently (order non-deterministic)

---

## Examples

### Basic Usage - Sequential

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [r] $doubled :pg.int << \|U.Math.Multiply"{$num, 2}"

   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $doubled_numbers
```

**Input:** `$numbers = [1, 2, 3, 4, 5]`
**Output:** `$doubled_numbers = [2, 4, 6, 8, 10]`

---

### Basic Usage - Parallel

```polyglot
[p] ~ForEach.Array
[~] <array << $urls
[~] >item >> $url

   [r] $content :pg.string << \|HTTP.Get <url << $url

   [v] *Into.Array
   [*] <item << $content
   [*] >array >> $all_content
[v]
```

**Note:** All HTTP requests execute concurrently for better performance.

---

### Transform Array of Serials

```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [r] $email :pg.string << $user.email
   [r] $name :pg.string << $user.name

   [r] $summary :pg.string << \|U.String.Concat"{$name, \" <\", $email, \">\"}"

   [v] *Into.Array
   [*] <item << $summary
   [*] >array >> $email_list
```

**Input:** `$users = [{name: "Alice", email: "alice@example.com"}, ...]`
**Output:** `$email_list = ["Alice <alice@example.com>", ...]`

---

### Filter During Iteration

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [y] $num > 10
      [v] *Into.Array
      [*] <item << $num
      [*] >array >> $greater_than_ten
```

**Input:** `$numbers = [5, 12, 8, 15, 3, 20]`
**Output:** `$greater_than_ten = [12, 15, 20]`

---

### Nested ForEach

```polyglot
[r] ~ForEach.Array
[~] <array << $outer
[~] >item >> $inner_array

   [r] ~ForEach.Array
   [~] <array << $inner_array
   [~] >item >> $element

      [r] $processed << \|Process <input << $element

      [v] *Into.Array
      [*] <item << $processed
      [*] >array >> $flattened
```

**Input:** `$outer = [[1, 2], [3, 4], [5]]`
**Output:** `$flattened = [processed(1), processed(2), processed(3), processed(4), processed(5)]`

---

### Multiple Pack Operators

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total

   [v] *Math.Count
   [*] <item << $num
   [*] >count >> $count

   [v] *Into.Array
   [*] <item << $num
   [*] >array >> $all_numbers
```

**Multiple pack operators can collect different aggregations from the same iteration.**

---

## Empty Array Handling

**Empty array produces no iterations:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $element

   [v] *Into.Array
   [*] <item << $element
   [*] >array >> $result
```

**Input:** `$empty = []`
**Output:** `$result = []`

---

## Type Inference

**Output type matches array element type:**

| Input Type | Output Type |
|------------|-------------|
| `:pg.array.pg.int` | `:pg.int` |
| `:pg.array.pg.string` | `:pg.string` |
| `:pg.array.pg.serial` | `:pg.serial` |
| `:pg.array.pg.array.pg.int` | `:pg.array.pg.int` |

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   // Element 0 completes before element 1 starts
   // Predictable order
```

**Use when:**
- Order matters
- Operations have side effects
- Need deterministic results

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   // All elements process concurrently
   // Order non-deterministic
[v]
```

**Use when:**
- Independent operations
- Performance critical
- No order dependency
- Safe for concurrent execution

---

## Common Patterns

### Pattern 1: Extract Field
```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user
   [r] $name :pg.string << $user.name
   [v] *Into.Array
   [*] <item << $name
   [*] >array >> $names
```

### Pattern 2: Validation
```polyglot
[r] ~ForEach.Array
[~] <array << $emails
[~] >item >> $email
   [z] $validated :pg.bool << \|ValidateEmail <email << $email
      [!] !Validation.InvalidEmail
         // Skip invalid emails
   [v] *Into.Array
   [*] <item << $email
   [*] >array >> $valid_emails
```

### Pattern 3: Aggregation
```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $maximum
```

### Pattern 4: Building Serial Data
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $id :pg.string << $item.id
   [r] $path :pg.string << \|U.String.Concat"{\"items.\", $id}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $item
   [*] >serial >> $structured_data
```

---

## Performance

**Time Complexity:** O(n) where n = array length

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Optimization:**
- Use parallel `[p]` for independent operations
- Use sequential `[r]` for order-dependent operations
- Minimize work in iteration scope

---

## Comparison with Other Operators

| Operator | Index Output | Collection Type | Use Case |
|----------|--------------|-----------------|----------|
| **~ForEach.Array** | No | Array | Most common |
| **~ForEach.IndexedArray** | Yes | Array | Need position |
| **~ForEach.Set** | No | Set | Unique values |
| **~ForEach.Serial** | Path | Serial | Dynamic fields |

**When to use ~ForEach.Array:**
- Standard array iteration
- Don't need index
- Process each element

**When to use ~ForEach.IndexedArray:**
- Need element position
- Building indexed structures
- Position-based logic

---

## Related Operators

- [~ForEach.IndexedArray](./foreach-indexedarray.md) - Array iteration with index
- [~ForEach.Set](./foreach-set.md) - Set iteration
- [*Into.Array](../../pack-operators/collection-building/into/into-array.md) - Collect results

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Pack Operators](../../pack-operators/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
