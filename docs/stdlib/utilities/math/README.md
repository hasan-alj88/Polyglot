tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Math Utilities (|U.Math.*)
summary: "API reference: Math Utilities (|U.Math.*)"
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
complexity: low

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
 BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Math Utilities (|U.Math.*)
summary: API reference: Math Utilities (|U.Math.*)
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
complexity: low

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
# Math Utilities (|U.Math.*)

**Arithmetic and mathematical operations**

---

## Pipelines Tree

**|U.Math.\***
- [**|U.Math.Add**](./add.md) - Add two numbers
- [**|U.Math.Subtract**](./subtract.md) - Subtract numbers
- [**|U.Math.Multiply**](./multiply.md) - Multiply numbers
- [**|U.Math.Divide**](./divide.md) - Divide numbers
- [**|U.Math.Modulo**](./modulo.md) - Remainder of division
- [**|U.Math.Double**](./double.md) - Multiply by 2
- [**|U.Math.Round**](./round.md) - Round to nearest integer
- [**|U.Math.Floor**](./floor.md) - Round down
- [**|U.Math.Ceiling**](./ceiling.md) - Round up
- [**|U.Math.Abs**](./abs.md) - Absolute value

---

## Overview

Math utilities provide basic arithmetic and mathematical operations for use in pipelines and inline calculations.

**Total:** 10 math utility pipelines

---

## Quick Reference

| Pipeline | Operation | Example | Result |
|----------|-----------|---------|--------|
| `\|U.Math.Add` | a + b | `\|U.Math.Add"{5, 3}"` | 8 |
| `\|U.Math.Subtract` | a - b | `\|U.Math.Subtract"{10, 3}"` | 7 |
| `\|U.Math.Multiply` | a × b | `\|U.Math.Multiply"{4, 5}"` | 20 |
| `\|U.Math.Divide` | a ÷ b | `\|U.Math.Divide"{20, 4}"` | 5.0 |
| `\|U.Math.Modulo` | a % b | `\|U.Math.Modulo"{10, 3}"` | 1 |
| `\|U.Math.Double` | a × 2 | `\|U.Math.Double"{5}"` | 10 |
| `\|U.Math.Round` | round(a) | `\|U.Math.Round"{5.7}"` | 6 |
| `\|U.Math.Floor` | floor(a) | `\|U.Math.Floor"{5.7}"` | 5 |
| `\|U.Math.Ceiling` | ceil(a) | `\|U.Math.Ceiling"{5.2}"` | 6 |
| `\|U.Math.Abs` | \|a\| | `\|U.Math.Abs"{-5}"` | 5 |

---

## Common Patterns

### Basic Arithmetic
```polyglot
[r] $total :pg.float << \|U.Math.Add"{$price, $tax}"
[r] $discounted :pg.float << \|U.Math.Multiply"{$price, 0.9}"
[r] $remainder :pg.int << \|U.Math.Modulo"{$value, 10}"
```

### In Loops
```polyglot
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $doubled :pg.float << \|U.Math.Double"{$num}"
   [r] $rounded :pg.int << \|U.Math.Round"{$doubled}"
   [v] *Into.Array
   [*] <item << $rounded
   [*] >array >> $results
```

### Compound Calculations
```polyglot
[r] $subtotal :pg.float << \|U.Math.Multiply"{$quantity, $unit_price}"
[r] $tax :pg.float << \|U.Math.Multiply"{$subtotal, 0.08}"
[r] $total :pg.float << \|U.Math.Add"{$subtotal, $tax}"
```

### Rounding Patterns
```polyglot
// Round to nearest
[r] $rounded :pg.int << \|U.Math.Round"{$value}"

// Always round up
[r] $rounded_up :pg.int << \|U.Math.Ceiling"{$value}"

// Always round down
[r] $rounded_down :pg.int << \|U.Math.Floor"{$value}"
```

---

## Type Handling

**Integer operations:**
```polyglot
[r] $sum :pg.int << \|U.Math.Add"{5, 3}"          // 8
[r] $product :pg.int << \|U.Math.Multiply"{4, 5}" // 20
```

**Float operations:**
```polyglot
[r] $quotient :pg.float << \|U.Math.Divide"{10, 3}"  // 3.333...
[r] $result :pg.float << \|U.Math.Add"{5.5, 2.3}"   // 7.8
```

**Mixed types:**
```polyglot
[r] $result :pg.float << \|U.Math.Add"{5, 2.5}"  // 7.5 (promotes to float)
```

---

## Related Operators

- [*Math.Sum](../../pack-operators/math/math-sum.md) - Sum iteration values
- [*Math.Product](../../pack-operators/math/math-product.md) - Multiply iteration values
- [*Math.Average](../../pack-operators/math/math-average.md) - Calculate average

---

## See Also

- [Utilities Overview](../README.md)
- [Standard Library](../../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
