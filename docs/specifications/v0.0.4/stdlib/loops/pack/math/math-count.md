---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-count"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Count"
summary: "API reference: *Math.Count"
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
# *Math.Count

**Count number of iterations**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Count
[*] <item
[*] >count :pg.uint
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope (value ignored, only counts iterations)

**Outputs:**
- `>count` :pg.uint - Number of iterations in main scope

---

## Description

Counts the number of iterations, producing a single count result. The value of `<item` is ignored; only the number of times the pack operator is reached matters.

**Key characteristics:**
- **Iteration counter** - Counts how many times reached
- **Value ignored** - Item value doesn't matter
- **Empty = 0** - Empty iteration produces 0
- **Type** - Always returns `:pg.uint`

**Use when:**
- Counting items
- Determining array/collection size
- Tracking iteration progress
- Validation checks

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [v] *Math.Count
   [*] <item << $item
   [*] >count >> $item_count
```

**Input:** `$items = ["a", "b", "c", "d", "e"]`
**Output:** `$item_count = 5`

---

### Count Matching Items

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [y] $num > 10
      [v] *Math.Count
      [*] <item << $num
      [*] >count >> $count_above_ten
```

**Counts only numbers greater than 10.**

---

### Count Active Users

```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [y] $user.active == #True
      [r] $placeholder :pg.int << 1
      [v] *Math.Count
      [*] <item << $placeholder
      [*] >count >> $active_user_count
```

---

### Count Successful Operations

```polyglot
[r] ~ForEach.Array
[~] <array << $operations
[~] >item >> $op

   [z] $result << \|Execute <op << $op
      [!] *!
         // Skip errors

   [v] *Math.Count
   [*] <item << $result
   [*] >count >> $success_count
```

**Counts only successful operations.**

---

### Count Items by Category

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [y] $item.category == "electronics"
      [r] $one :pg.int << 1
      [v] *Math.Count
      [*] <item << $one
      [*] >count >> $electronics_count

   [y] $item.category == "clothing"
      [r] $one :pg.int << 1
      [v] *Math.Count
      [*] <item << $one
      [*] >count >> $clothing_count
```

---

## Empty Array Handling

**Empty array produces 0:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $item

   [v] *Math.Count
   [*] <item << $item
   [*] >count >> $count
```

**Input:** `$empty = []`
**Output:** `$count = 0`

---

## Value is Ignored

**The item value doesn't affect the count:**

```polyglot
[r] ~ForEach.Array
[~] <array << $mixed
[~] >item >> $item

   [v] *Math.Count
   [*] <item << $item
   [*] >count >> $count
```

**Input:** `$mixed = [1, "hello", true, null, 5.5]`
**Output:** `$count = 5`

**Every iteration contributes 1 to count, regardless of value.**

---

## Type

**Always returns `:pg.uint`:**

```polyglot
[*] >count :pg.uint
```

---

## Common Patterns

### Pattern 1: Simple Count
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [v] *Math.Count
   [*] <item << $item
   [*] >count >> $total_items
```

### Pattern 2: Conditional Count
```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val
   [y] $val > 0
      [r] $dummy :pg.int << 1
      [v] *Math.Count
      [*] <item << $dummy
      [*] >count >> $positive_count
```

### Pattern 3: Count and Calculate Percentage
```polyglot
[r] ~ForEach.Array
[~] <array << $survey_responses
[~] >item >> $response
   [v] *Math.Count
   [*] <item << $response
   [*] >count >> $total_responses

   [y] $response.answer == "yes"
      [r] $dummy :pg.int << 1
      [v] *Math.Count
      [*] <item << $dummy
      [*] >count >> $yes_count

[r] $yes_percentage :pg.float << \|U.Math.Divide"{$yes_count, $total_responses}"
```

### Pattern 4: Validation Count
```polyglot
[r] ~ForEach.Array
[~] <array << $records
[~] >item >> $record
   [z] $validated << \|Validate <record << $record
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Math.Count
         [*] <item << $err
         [*] >count >> $validation_errors_count
```

---

## Alternative: Array Length

**For counting array elements, can use array length:**

```polyglot
// Instead of:
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [v] *Math.Count
   [*] <item << $item
   [*] >count >> $count

// Use:
[r] $count :pg.uint << $items.length
```

**But *Math.Count is useful for:**
- Conditional counting
- Counting after filtering
- Counting successful operations

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for counter

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Output Type |
|----------|-----------|--------------|-------------|
| **\*Math.Count** | Count | 0 | `:pg.uint` |
| **\*Math.Sum** | Addition | 0 | Numeric |
| **\*Into.Array** | Collect | `[]` | `:pg.array.*` |

**When to use \*Math.Count:**
- Need iteration count
- Counting filtered items
- Progress tracking
- Statistics

**When to use array.length:**
- Counting array elements directly
- Simpler/faster
- Don't need filtering

---

## Related Operators

- [*Math.Sum](./math-sum.md) - Sum values
- [*Math.Average](./math-average.md) - Calculate average (uses count internally)
- [*Into.Array](../collection-building/into/into-array.md) - Collect items

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
