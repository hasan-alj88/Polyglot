---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "math-product"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Math.Product"
summary: "API reference: *Math.Product"
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
# *Math.Product

**Multiply all numeric values from iterations**

**Category:** Pack Operators > Math
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Math.Product
[*] <item
[*] >product
```

---

## Parameters

**Inputs:**
- `<item` - Numeric value from iteration scope

**Outputs:**
- `>product` - Product of all values in main scope

---

## Description

Multiplies all numeric values from iterations, producing a single product. This is the mathematical multiplication aggregation.

**Key characteristics:**
- **Multiplication** - Multiplies all input values
- **Empty = 1** - Empty iteration produces 1 (multiplicative identity)
- **Zero handling** - Any zero makes entire product zero
- **Type preservation** - Integer inputs → integer product, float inputs → float product

**Use when:**
- Calculating products
- Compound growth/decay
- Probability chains
- Scaling factors

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Product
   [*] <item << $num
   [*] >product >> $result
```

**Input:** `$numbers = [2, 3, 4]`
**Output:** `$result = 24`

---

### Calculate Factorial

```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << 6
[~] >index >> $i

   [v] *Math.Product
   [*] <item << $i
   [*] >product >> $factorial_5
```

**Output:** `$factorial_5 = 120` (5!)

---

### Compound Interest

```polyglot
[r] ~ForEach.Array
[~] <array << $growth_rates
[~] >item >> $rate

   [r] $multiplier :pg.float << \|U.Math.Add"{1.0, $rate}"

   [v] *Math.Product
   [*] <item << $multiplier
   [*] >product >> $compound_factor
```

**Input:** `$growth_rates = [0.05, 0.03, 0.04]` (5%, 3%, 4%)
**Output:** `$compound_factor = 1.124...` (12.4% total growth)

---

### Volume Calculation

```polyglot
[r] ~ForEach.Array
[~] <array << $dimensions
[~] >item >> $dimension

   [v] *Math.Product
   [*] <item << $dimension
   [*] >product >> $volume
```

**Input:** `$dimensions = [5, 3, 2]` (length, width, height)
**Output:** `$volume = 30`

---

### Probability Chain

```polyglot
[r] ~ForEach.Array
[~] <array << $step_probabilities
[~] >item >> $prob

   [v] *Math.Product
   [*] <item << $prob
   [*] >product >> $total_probability
```

**Input:** `$step_probabilities = [0.9, 0.8, 0.95]`
**Output:** `$total_probability = 0.684` (68.4% chance all succeed)

---

## Empty Array Handling

**Empty array produces 1:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $num

   [v] *Math.Product
   [*] <item << $num
   [*] >product >> $result
```

**Input:** `$empty = []`
**Output:** `$result = 1` (multiplicative identity)

---

## Zero Handling

**Any zero makes product zero:**

```polyglot
[r] ~ForEach.Array
[~] <array << [5, 0, 10]
[~] >item >> $num

   [v] *Math.Product
   [*] <item << $num
   [*] >product >> $result
```

**Output:** `$result = 0`

---

## Type Handling

**Integer product:**
```polyglot
[r] ~ForEach.Array
[~] <array << [2, 3, 4]
[~] >item >> $num

   [v] *Math.Product
   [*] <item << $num
   [*] >product >> $result  // :pg.int = 24
```

**Float product:**
```polyglot
[r] ~ForEach.Array
[~] <array << [1.5, 2.0, 3.0]
[~] >item >> $num

   [v] *Math.Product
   [*] <item << $num
   [*] >product >> $result  // :pg.float = 9.0
```

---

## Overflow Warning

**Large products can overflow:**

```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << 100
[~] >index >> $i

   [v] *Math.Product
   [*] <item << $i
   [*] >product >> $huge_number  // May overflow!
```

**Consider using logarithms for very large products.**

---

## Common Patterns

### Pattern 1: Factorial
```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << $n + 1
[~] >index >> $i
   [v] *Math.Product
   [*] <item << $i
   [*] >product >> $factorial_n
```

### Pattern 2: Scale by Factors
```polyglot
[r] ~ForEach.Array
[~] <array << $scale_factors
[~] >item >> $factor
   [v] *Math.Product
   [*] <item << $factor
   [*] >product >> $total_scale

[r] $scaled_value :pg.float << \|U.Math.Multiply"{$original_value, $total_scale}"
```

### Pattern 3: Combined Probabilities
```polyglot
[r] ~ForEach.Array
[~] <array << $success_rates
[~] >item >> $rate
   [v] *Math.Product
   [*] <item << $rate
   [*] >product >> $overall_success_rate
```

### Pattern 4: Geometric Mean Preparation
```polyglot
[r] ~ForEach.Array
[~] <array << $values
[~] >item >> $val
   [v] *Math.Product
   [*] <item << $val
   [*] >product >> $product

   [v] *Math.Count
   [*] <item << $val
   [*] >count >> $count

[r] $geometric_mean :pg.float << \|U.Math.Pow"{$product, \|U.Math.Divide\"{1.0, $count}\"}"
```

---

## Performance

**Time Complexity:** O(n) where n = number of iterations

**Space Complexity:** O(1) - constant space for accumulator

**Overflow:** Possible with large values or many iterations

---

## Comparison with Other Operators

| Operator | Operation | Empty Result | Use Case |
|----------|-----------|--------------|----------|
| **\*Math.Product** | Multiplication | 1 | Products |
| **\*Math.Sum** | Addition | 0 | Totals |
| **\*Math.Count** | Count | 0 | Count items |

**When to use \*Math.Product:**
- Calculating products
- Compound rates
- Probability chains
- Factorial/combinatorics

**When to use \*Math.Sum:**
- Need total, not product
- Adding values

---

## Related Operators

- [*Math.Sum](./math-sum.md) - Sum values
- [*Math.Count](./math-count.md) - Count items

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
