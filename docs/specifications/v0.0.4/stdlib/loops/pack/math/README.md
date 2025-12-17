---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Math Pack Operators
summary: API reference: Math Pack Operators
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
# Math Pack Operators

**Aggregate numeric values from iterations**

---

## Operators Tree

**\*Math.\***
- [**\*Math.Sum**](./math-sum.md)
  - `<item`
  - `>sum`
- [**\*Math.Product**](./math-product.md)
  - `<item`
  - `>product`
- [**\*Math.Min**](./math-min.md)
  - `<item`
  - `>min`
- [**\*Math.Max**](./math-max.md)
  - `<item`
  - `>max`
- [**\*Math.Count**](./math-count.md)
  - `<item`
  - `>count`
- [**\*Math.Average**](./math-average.md)
  - `<item`
  - `>average`

---

## Overview

Math pack operators perform mathematical aggregations over iteration values, reducing multiple numeric values to a single result.

**Common pattern:**
```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

---

## Aggregation Operators

- [*Math.Sum](./math-sum.md) - Sum all values
- [*Math.Product](./math-product.md) - Multiply all values
- [*Math.Min](./math-min.md) - Find minimum value
- [*Math.Max](./math-max.md) - Find maximum value
- [*Math.Count](./math-count.md) - Count iterations
- [*Math.Average](./math-average.md) - Calculate average

---

## Comparison Table

| Operator | Operation | Empty Result | Type |
|----------|-----------|--------------|------|
| **\*Math.Sum** | Addition | 0 | Numeric |
| **\*Math.Product** | Multiplication | 1 | Numeric |
| **\*Math.Min** | Minimum | Undefined | Numeric |
| **\*Math.Max** | Maximum | Undefined | Numeric |
| **\*Math.Count** | Count | 0 | `:pg.uint` |
| **\*Math.Average** | Mean | Undefined | `:pg.float` |

---

## Multiple Aggregations

**Can use multiple math operators in same loop:**

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

   [v] *Math.Min
   [*] <item << $num
   [*] >min >> $minimum

   [v] *Math.Max
   [*] <item << $num
   [*] >max >> $maximum
```

**All aggregations computed in single pass.**

---

## Common Patterns

### Pattern 1: Basic Sum
```polyglot
[r] ~ForEach.Array
[~] <array << $prices
[~] >item >> $price
   [v] *Math.Sum
   [*] <item << $price
   [*] >sum >> $total_price
```

### Pattern 2: Statistics
```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val
   [v] *Math.Count
   [*] <item << $val
   [*] >count >> $count

   [v] *Math.Sum
   [*] <item << $val
   [*] >sum >> $sum

   [v] *Math.Min
   [*] <item << $val
   [*] >min >> $min

   [v] *Math.Max
   [*] <item << $val
   [*] >max >> $max

   [v] *Math.Average
   [*] <item << $val
   [*] >average >> $avg
```

### Pattern 3: Conditional Aggregation
```polyglot
[r] ~ForEach.Array
[~] <array << $transactions
[~] >item >> $txn
   [r] $amount :pg.float << $txn.amount

   [y] $amount > 0
      [v] *Math.Sum
      [*] <item << $amount
      [*] >sum >> $total_income

   [y] $amount < 0
      [r] $abs_amount :pg.float << \|U.Math.Abs"{$amount}"
      [v] *Math.Sum
      [*] <item << $abs_amount
      [*] >sum >> $total_expenses
```

### Pattern 4: Count with Filter
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [y] $item.active == #True
      [r] $one :pg.int << 1
      [v] *Math.Sum
      [*] <item << $one
      [*] >sum >> $active_count
```

---

## Related Documentation

- [Pack Operators Overview](../README.md)
- [Unpack Operators](../../unpack-operators/README.md)
- [Loop System](../../../language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
