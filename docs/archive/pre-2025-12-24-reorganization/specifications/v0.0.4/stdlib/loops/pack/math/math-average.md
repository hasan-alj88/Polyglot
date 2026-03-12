---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-average"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Average"
summary: "API reference: *Math.Average"
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
# *Math.Average

**Calculate average (mean) of numeric values**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Average
[*] <item
[*] >average :pg.float
```

---

## Parameters

**Inputs:**
- `<item` - Numeric value from iteration scope

**Outputs:**
- `>average` :pg.float - Average (arithmetic mean) of all values in main scope

---

## Description

Calculates the arithmetic mean of all numeric values from iterations, producing a single average result. This is computed as sum / count.

**Key characteristics:**
- **Arithmetic mean** - (sum of values) / (count of values)
- **Empty = undefined** - Empty iteration produces no result
- **Type** - Always returns `:pg.float`
- **Precision** - Floating point arithmetic

**Use when:**
- Calculating averages
- Statistical analysis
- Performance metrics
- Grade/score calculations

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Average
   [*] <item << $num
   [*] >average >> $mean
```

**Input:** `$numbers = [10, 20, 30, 40, 50]`
**Output:** `$mean = 30.0`

---

### Calculate Average Grade

```polyglot
[r] ~ForEach.Array
[~] <array << $test_scores
[~] >item >> $score

   [r] $grade :pg.int << $score.grade

   [v] *Math.Average
   [*] <item << $grade
   [*] >average >> $average_grade
```

---

### Average Response Time

```polyglot
[r] ~ForEach.Array
[~] <array << $api_calls
[~] >item >> $call

   [r] $duration :pg.float << $call.response_time_ms

   [v] *Math.Average
   [*] <item << $duration
   [*] >average >> $avg_response_time
```

---

### Average Price

```polyglot
[r] ~ForEach.Array
[~] <array << $products
[~] >item >> $product

   [r] $price :pg.float << $product.price

   [v] *Math.Average
   [*] <item << $price
   [*] >average >> $average_price
```

---

### Average with Filtering

```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val

   [f] $val > 0
      [v] *Math.Average
      [*] <item << $val
      [*] >average >> $avg_positive
```

**Only averages positive values.**

---

## Empty Array Handling

**Empty array produces undefined result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $num

   [v] *Math.Average
   [*] <item << $num
   [*] >average >> $avg
```

**Input:** `$empty = []`
**Behavior:** No value assigned to `$avg` (variable undefined)

**Handle with default:**
```polyglot
[r] $avg << <~ 0.0 ~> $average_result
```

---

## Type Handling

**Always returns float:**

```polyglot
[r] ~ForEach.Array
[~] <array << [1, 2, 3, 4, 5]
[~] >item >> $num

   [v] *Math.Average
   [*] <item << $num
   [*] >average >> $avg  // :pg.float = 3.0
```

**Even with integer inputs, result is float.**

---

## Manual Average Calculation

**Equivalent to:**

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $sum

   [v] *Math.Count
   [*] <item << $num
   [*] >count >> $count

[r] $average :pg.float << \|U.Math.Divide"{$sum, $count}"
```

**But *Math.Average is more concise.**

---

## Common Patterns

### Pattern 1: Simple Average
```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val
   [v] *Math.Average
   [*] <item << $val
   [*] >average >> $mean
```

### Pattern 2: Average Score
```polyglot
[r] ~ForEach.Array
[~] <array << $students
[~] >item >> $student
   [r] $score :pg.int << $student.final_score
   [v] *Math.Average
   [*] <item << $score
   [*] >average >> $class_average
```

### Pattern 3: Performance Metrics
```polyglot
[r] ~ForEach.Array
[~] <array << $requests
[~] >item >> $req
   [r] $latency :pg.float << $req.latency_ms
   [v] *Math.Average
   [*] <item << $latency
   [*] >average >> $avg_latency

[r] $report :pg.string << \|U.String.Concat"{\"Average latency: \", $avg_latency, \" ms\"}"
```

### Pattern 4: Statistics Summary
```polyglot
[r] ~ForEach.Array
[~] <array << $data
[~] >item >> $val
   [v] *Math.Sum
   [*] <item << $val
   [*] >sum >> $sum

   [v] *Math.Count
   [*] <item << $val
   [*] >count >> $count

   [v] *Math.Average
   [*] <item << $val
   [*] >average >> $mean

   [v] *Math.Min
   [*] <item << $val
   [*] >min >> $min

   [v] *Math.Max
   [*] <item << $val
   [*] >max >> $max
```

---

## Floating Point Precision

**Average uses floating point arithmetic:**

```polyglot
[r] ~ForEach.Array
[~] <array << [1, 2, 3]
[~] >item >> $num

   [v] *Math.Average
   [*] <item << $num
   [*] >average >> $avg  // 2.0 (exact)
```

```polyglot
[r] ~ForEach.Array
[~] <array << [1, 2]
[~] >item >> $num

   [v] *Math.Average
   [*] <item << $num
   [*] >average >> $avg  // 1.5 (exact)
```

**May have rounding errors with very large datasets.**

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for sum and count accumulators

**Precision:** Subject to floating point rounding

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Output Type |
|----------|-----------|--------------|-------------|
| **\*Math.Average** | Mean | Undefined | `:pg.float` |
| **\*Math.Sum** | Total | 0 | Numeric |
| **\*Math.Count** | Count | 0 | `:pg.uint` |
| **\*Math.Min** | Minimum | Undefined | Numeric |
| **\*Math.Max** | Maximum | Undefined | Numeric |

**When to use \*Math.Average:**
- Need arithmetic mean
- Statistical analysis
- Performance metrics
- Grade calculations

**When to use \*Math.Sum:**
- Need total, not average
- Summing values

**Manual calculation:**
- Need median (not mean)
- Need weighted average
- Custom calculation needed

---

## Weighted Average

**For weighted average, use manual calculation:**

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $value :pg.float << $item.value
   [r] $weight :pg.float << $item.weight
   [r] $weighted :pg.float << \|U.Math.Multiply"{$value, $weight}"

   [v] *Math.Sum
   [*] <item << $weighted
   [*] >sum >> $weighted_sum

   [v] *Math.Sum
   [*] <item << $weight
   [*] >sum >> $total_weight

[r] $weighted_average :pg.float << \|U.Math.Divide"{$weighted_sum, $total_weight}"
```

---

## Related Operators

- [*Math.Sum](./math-sum.md) - Sum values
- [*Math.Count](./math-count.md) - Count items
- [*Math.Min](./math-min.md) - Minimum value
- [*Math.Max](./math-max.md) - Maximum value

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
