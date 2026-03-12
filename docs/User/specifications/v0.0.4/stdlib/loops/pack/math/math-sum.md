---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-sum"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Sum"
summary: "API reference: *Math.Sum"
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
# *Math.Sum

**Sum all numeric values from iterations**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Sum
[*] <item
[*] >sum
```

---

## Parameters

**Inputs:**
- `<item` - Numeric value from iteration scope

**Outputs:**
- `>sum` - Sum of all values in main scope

---

## Description

Sums all numeric values from iterations, producing a single total. This is the mathematical addition aggregation.

**Key characteristics:**
- **Addition** - Adds all input values
- **Empty = 0** - Empty iteration produces 0
- **Type preservation** - Integer inputs → integer sum, float inputs → float sum

**Use when:**
- Calculating totals
- Summing prices, quantities, scores
- Aggregating numeric data

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

**Input:** `$numbers = [1, 2, 3, 4, 5]`
**Output:** `$total = 15`

---

### Sum Prices

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $price :pg.float << $item.price

   [v] *Math.Sum
   [*] <item << $price
   [*] >sum >> $total_cost
```

**Input:**
```json
[
  {"name": "Apple", "price": 1.50},
  {"name": "Banana", "price": 0.75},
  {"name": "Cherry", "price": 2.00}
]
```
**Output:** `$total_cost = 4.25`

---

### Sum with Condition

```polyglot
[r] ~ForEach.Array
[~] <array << $transactions
[~] >item >> $txn

   [f] $txn.type == "income"
      [r] $amount :pg.float << $txn.amount
      [v] *Math.Sum
      [*] <item << $amount
      [*] >sum >> $total_income
```

**Only sums transactions where type is "income".**

---

### Calculate Total Hours

```polyglot
[r] ~ForEach.Array
[~] <array << $time_entries
[~] >item >> $entry

   [r] $hours :pg.float << $entry.hours

   [v] *Math.Sum
   [*] <item << $hours
   [*] >sum >> $total_hours
```

---

### Sum Nested Values

```polyglot
[r] ~ForEach.Array
[~] <array << $orders
[~] >item >> $order

   [r] ~ForEach.Array
   [~] <array << $order.line_items
   [~] >item >> $line_item

      [r] $subtotal :pg.float << $line_item.quantity * $line_item.price

      [v] *Math.Sum
      [*] <item << $subtotal
      [*] >sum >> $order_total
```

---

## Empty Array Handling

**Empty array produces 0:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

**Input:** `$empty = []`
**Output:** `$total = 0`

---

## Type Handling

**Integer sum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [1, 2, 3]
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total  // :pg.int = 6
```

**Float sum:**
```polyglot
[r] ~ForEach.Array
[~] <array << [1.5, 2.3, 3.7]
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total  // :pg.float = 7.5
```

**Mixed types:**
```polyglot
// If mixing int and float, result is float
[r] ~ForEach.Array
[~] <array << [1, 2.5, 3]
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total  // :pg.float = 6.5
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

**Result is deterministic.**

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
[v]
```

**Result is same (addition is commutative).**

---

## Common Patterns

### Pattern 1: Total Price
```polyglot
[r] ~ForEach.Array
[~] <array << $cart_items
[~] >item >> $item
   [r] $price :pg.float << $item.price
   [v] *Math.Sum
   [*] <item << $price
   [*] >sum >> $cart_total
```

### Pattern 2: Count True Values
```polyglot
[r] ~ForEach.Array
[~] <array << $flags
[~] >item >> $flag
   [f] $flag == #True
      [r] $one :pg.int << 1
      [v] *Math.Sum
      [*] <item << $one
      [*] >sum >> $true_count
```

### Pattern 3: Weighted Sum
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
```

### Pattern 4: Calculate Subtotal and Tax
```polyglot
[r] ~ForEach.Array
[~] <array << $line_items
[~] >item >> $item
   [r] $price :pg.float << $item.price
   [v] *Math.Sum
   [*] <item << $price
   [*] >sum >> $subtotal

[r] $tax_rate :pg.float << 0.08
[r] $tax :pg.float << \|U.Math.Multiply"{$subtotal, $tax_rate}"
[r] $total :pg.float << \|U.Math.Add"{$subtotal, $tax}"
```

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for accumulator

**Precision:** Floating point addition may have rounding errors for very large datasets.

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Use Case |
|----------|-----------|--------------|----------|
| **\*Math.Sum** | Addition | 0 | Totals |
| **\*Math.Product** | Multiplication | 1 | Products |
| **\*Math.Average** | Mean | Undefined | Average |
| **\*Math.Count** | Count | 0 | Count items |

**When to use \*Math.Sum:**
- Calculating totals
- Summing quantities
- Aggregating numeric values

**When to use \*Math.Average:**
- Need mean value
- Want average not total

---

## Related Operators

- [*Math.Average](./math-average.md) - Calculate average
- [*Math.Count](./math-count.md) - Count items
- [*Math.Product](./math-product.md) - Multiply values

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
