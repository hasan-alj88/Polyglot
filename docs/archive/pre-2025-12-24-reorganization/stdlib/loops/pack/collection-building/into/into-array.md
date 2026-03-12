---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "into-array"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Into.Array"
summary: "API reference: *Into.Array"
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
# *Into.Array

**Collect iteration items into an array**

**Category:** Collection Building > Into
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Into.Array
[*] <item
[*] >array
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope to collect

**Outputs:**
- `>array` :pg.array.T - Collected array in main scope (T = type of item)

---

## Description

Collects each iteration's `<item` value into an array in the main scope. This is the **most common pack operator**.

**Order:**
- Sequential `[r]` loops: Array elements in same order as input
- Parallel `[p]` loops: Order non-deterministic (depends on completion time)

---

## Examples

### Basic Usage

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Transform <input << $element

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

**Input:** `$items = ["a", "b", "c"]`
**Output:** `$results = ["transformed_a", "transformed_b", "transformed_c"]`

---

### With Filtering

```polyglot
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [f] $num >? 0
      [v] *Into.Array
      [*] <item << $num
      [*] >array >> $positives
```

**Input:** `$numbers = [-1, 2, -3, 4, 5]`
**Output:** `$positives = [2, 4, 5]`

---

### Nested Transformation

```polyglot
[p] ~ForEach.Array
[~] <array << $user_ids
[~] >item >> $id

   [r] $user :pg.serial << \|Database.Users.Find <id << $id
   [r] $name :pg.string << $user.name

   [v] *Into.Array
   [*] <item << $name
   [*] >array >> $user_names
```

---

### Sequential vs Parallel

**Sequential (preserves order):**
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Process <input << $element

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
// $results in same order as $items
```

**Parallel (order non-deterministic):**
```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Process <input << $element

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
// $results order depends on which iteration completes first
```

---

## Type Inference

Output array type is inferred from `<item` type:

| Item Type | Output Array Type |
|-----------|-------------------|
| `:pg.string` | `:pg.array.pg.string` |
| `:pg.int` | `:pg.array.pg.int` |
| `:pg.serial` | `:pg.array.pg.serial` |
| Custom `#Enum` | `:pg.array.EnumName` |

---

## Common Patterns

### Pattern 1: Map Operation

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $doubled :pg.int << \|U.Math.Double"{$item}"

   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $doubled_items
```

### Pattern 2: Extract Field

```polyglot
[p] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [r] $email :pg.string << $user.email

   [v] *Into.Array
   [*] <item << $email
   [*] >array >> $all_emails
```

### Pattern 3: Complex Transformation Chain

```polyglot
[p] ~ForEach.Array
[~] <array << $raw_data
[~] >item >> $data

   [r] $cleaned :pg.string << \|U.String.Trim"{$data}"
   [r] $lowercase :pg.string << \|U.String.Lower"{$cleaned}"
   [r] $validated :pg.bool << \|Validate <input << $lowercase

   [f] $validated =? #True
      [v] *Into.Array
      [*] <item << $lowercase
      [*] >array >> $valid_items
```

---

## Performance

**Time Complexity:** O(n) where n = number of iterations
**Space Complexity:** O(n) - array grows with each iteration

**Parallel Performance:**
- All iterations execute concurrently
- Array construction is thread-safe
- Final array available when all iterations complete

---

## Related Operators

- [*Into.Set](./into-set.md) - Collect unique values
- [*Into.Serial](./into-serial.md) - Collect into serial data
- [*Math.Count](../../math/math-count.md) - Count items instead of collecting

---

## See Also

- [Loop System](../../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../../README.md)
- [~ForEach.Array](../../../unpack-operators/foreach-array.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
