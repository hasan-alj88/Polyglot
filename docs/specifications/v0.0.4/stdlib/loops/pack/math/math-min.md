---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-min"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Min"
summary: "API reference: *Math.Min"
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
# *Math.Min

**Find minimum value from iterations**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Min
[*] <item
[*] >min
```

---

## Parameters

**Inputs:**
- `<item` - Numeric value from iteration scope

**Outputs:**
- `>min` - Minimum value in main scope

---

## Description

Finds the smallest numeric value from all iterations, producing a single minimum result.

**Key characteristics:**
- **Minimum selection** - Returns smallest value
- **Empty = undefined** - Empty iteration produces no result
- **Type preservation** - Output type matches input type

**Use when:**
- Finding smallest value
- Minimum price, score, temperature
- Lower bounds
- Data analysis

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $minimum
```

**Input:** `$numbers = [5, 2, 8, 1, 9]`
**Output:** `$minimum = 1`

---

### Find Lowest Price

```polyglot
[r] ~ForEach.Array
[~] <array << $products
[~] >item >> $product

   [r] $price :pg.float << $product.price

   [v] *Math.Min
   [*] <item << $price
   [*] >min >> $lowest_price
```

**Finds cheapest product.**

---

### Minimum Temperature

```polyglot
[r] ~ForEach.Array
[~] <array << $temperature_readings
[~] >item >> $reading

   [r] $temp :pg.float << $reading.celsius

   [v] *Math.Min
   [*] <item << $temp
   [*] >min >> $min_temp
```

---

### Find Earliest Date

```polyglot
[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event

   [r] $timestamp :pg.int << $event.timestamp

   [v] *Math.Min
   [*] <item << $timestamp
   [*] >min >> $earliest_timestamp
```

---

### Minimum Score

```polyglot
[r] ~ForEach.Array
[~] <array << $test_results
[~] >item >> $result

   [r] $score :pg.int << $result.score

   [v] *Math.Min
   [*] <item << $score
   [*] >min >> $lowest_score
```

---

## Empty Array Handling

**Empty array produces undefined result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $num

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $minimum
```

**Input:** `$empty = []`
**Behavior:** No value assigned to `$minimum` (variable undefined)

**Handle with default:**
```polyglot
[r] $minimum << <~ $default_value ~> $min_result
```

---

## Type Handling

**Integer minimum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [5, 2, 8, 1]
[~] >item >> $num

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $min  // :pg.int = 1
```

**Float minimum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [5.5, 2.3, 8.1, 1.7]
[~] >item >> $num

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $min  // :pg.float = 1.7
```

---

## Negative Numbers

**Handles negative values correctly:**

```polyglot
[r] ~ForEach.Array
[~] <array << [-5, 2, -10, 3]
[~] >item >> $num

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $min
```

**Output:** `$min = -10`

---

## Common Patterns

### Pattern 1: Find Minimum with Object
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $value :pg.int << $item.value
   [v] *Math.Min
   [*] <item << $value
   [*] >min >> $min_value
```

### Pattern 2: Minimum Non-Zero
```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [y] $num > 0
      [v] *Math.Min
      [*] <item << $num
      [*] >min >> $min_positive
```

### Pattern 3: Min and Max Together
```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val
   [v] *Math.Min
   [*] <item << $val
   [*] >min >> $minimum

   [v] *Math.Max
   [*] <item << $val
   [*] >max >> $maximum

[r] $range :pg.float << \|U.Math.Subtract"{$maximum, $minimum}"
```

### Pattern 4: Best Score (Lowest)
```polyglot
[r] ~ForEach.Array
[~] <array << $golf_scores
[~] >item >> $score
   [v] *Math.Min
   [*] <item << $score
   [*] >min >> $best_score  // Lower is better in golf
```

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for tracking minimum

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Use Case |
|----------|-----------|--------------|----------|
| **\*Math.Min** | Minimum | Undefined | Smallest value |
| **\*Math.Max** | Maximum | Undefined | Largest value |
| **\*Math.Sum** | Addition | 0 | Total |
| **\*Math.Average** | Mean | Undefined | Average |

**When to use \*Math.Min:**
- Finding smallest value
- Lower bounds
- Best score (if lower is better)
- Minimum threshold

**When to use \*Math.Max:**
- Finding largest value
- Upper bounds

---

## Related Operators

- [*Math.Max](./math-max.md) - Find maximum
- [*Math.Average](./math-average.md) - Calculate average
- [*Join.First](../collect/join-first.md) - Take first (not minimum)

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
