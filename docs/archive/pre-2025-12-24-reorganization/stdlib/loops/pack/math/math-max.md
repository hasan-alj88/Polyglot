---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-max"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Max"
summary: "API reference: *Math.Max"
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
# *Math.Max

**Find maximum value from iterations**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Max
[*] <item
[*] >max
```

---

## Parameters

**Inputs:**
- `<item` - Numeric value from iteration scope

**Outputs:**
- `>max` - Maximum value in main scope

---

## Description

Finds the largest numeric value from all iterations, producing a single maximum result.

**Key characteristics:**
- **Maximum selection** - Returns largest value
- **Empty = undefined** - Empty iteration produces no result
- **Type preservation** - Output type matches input type

**Use when:**
- Finding largest value
- Maximum price, score, temperature
- Upper bounds
- Peak detection

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $maximum
```

**Input:** `$numbers = [5, 2, 8, 1, 9]`
**Output:** `$maximum = 9`

---

### Find Highest Price

```polyglot
[r] ~ForEach.Array
[~] <array << $products
[~] >item >> $product

   [r] $price :pg.float << $product.price

   [v] *Math.Max
   [*] <item << $price
   [*] >max >> $highest_price
```

---

### Maximum Temperature

```polyglot
[r] ~ForEach.Array
[~] <array << $temperature_readings
[~] >item >> $reading

   [r] $temp :pg.float << $reading.celsius

   [v] *Math.Max
   [*] <item << $temp
   [*] >max >> $max_temp
```

---

### Find Latest Date

```polyglot
[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event

   [r] $timestamp :pg.int << $event.timestamp

   [v] *Math.Max
   [*] <item << $timestamp
   [*] >max >> $latest_timestamp
```

---

### Peak Value Detection

```polyglot
[r] ~ForEach.Array
[~] <array << $metrics
[~] >item >> $metric

   [r] $value :pg.int << $metric.value

   [v] *Math.Max
   [*] <item << $value
   [*] >max >> $peak_value
```

---

## Empty Array Handling

**Empty array produces undefined result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $num

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $maximum
```

**Input:** `$empty = []`
**Behavior:** No value assigned to `$maximum` (variable undefined)

**Handle with default:**
```polyglot
[r] $maximum << <~ $default_value ~> $max_result
```

---

## Type Handling

**Integer maximum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [5, 2, 8, 1]
[~] >item >> $num

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $max  // :pg.int = 8
```

**Float maximum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [5.5, 2.3, 8.1, 1.7]
[~] >item >> $num

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $max  // :pg.float = 8.1
```

---

## Negative Numbers

**Handles negative values correctly:**

```polyglot
[r] ~ForEach.Array
[~] <array << [-5, -2, -10, -3]
[~] >item >> $num

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $max
```

**Output:** `$max = -2`

---

## Common Patterns

### Pattern 1: Find Maximum with Object
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $value :pg.int << $item.value
   [v] *Math.Max
   [*] <item << $value
   [*] >max >> $max_value
```

### Pattern 2: Calculate Range
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

### Pattern 3: Top Score
```polyglot
[r] ~ForEach.Array
[~] <array << $player_scores
[~] >item >> $score
   [v] *Math.Max
   [*] <item << $score
   [*] >max >> $high_score
```

### Pattern 4: Peak Performance
```polyglot
[r] ~ForEach.Array
[~] <array << $daily_sales
[~] >item >> $day
   [r] $revenue :pg.float << $day.revenue
   [v] *Math.Max
   [*] <item << $revenue
   [*] >max >> $peak_revenue
```

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for tracking maximum

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Use Case |
|----------|-----------|--------------|----------|
| **\*Math.Max** | Maximum | Undefined | Largest value |
| **\*Math.Min** | Minimum | Undefined | Smallest value |
| **\*Math.Sum** | Addition | 0 | Total |
| **\*Math.Average** | Mean | Undefined | Average |

**When to use \*Math.Max:**
- Finding largest value
- Upper bounds
- Best score (if higher is better)
- Peak detection

**When to use \*Math.Min:**
- Finding smallest value
- Lower bounds

---

## Related Operators

- [*Math.Min](./math-min.md) - Find minimum
- [*Math.Average](./math-average.md) - Calculate average
- [*Join.Last](../collect/join-last.md) - Take last (not maximum)

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
