---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: multiply
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Multiply"
summary: "API reference: |U.Math.Multiply"
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
# |U.Math.Multiply

**Multiply two numbers**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Multiply <a <b >result
```

**Inline:**
```polyglot
\|U.Math.Multiply"{$a, $b}"
```

---

## Parameters

**Inputs:**
- `<a` - First number
- `<b` - Second number

**Outputs:**
- `>result` - Product (a × b)

---

## Description

Multiplies two numbers and returns their product.

**Operation:** `result = a × b`

---

## Examples

### Basic Usage

```polyglot
[r] $product :pg.int << \|U.Math.Multiply"{4, 5}"
```

**Output:** `$product = 20`

---

### Calculate Total Price

```polyglot
[r] $quantity :pg.int << 3
[r] $unit_price :pg.float << 15.99
[r] $total :pg.float << \|U.Math.Multiply"{$quantity, $unit_price}"
```

**Output:** `$total = 47.97`

---

### Apply Percentage

```polyglot
[r] $price :pg.float << 100.00
[r] $tax_rate :pg.float << 0.08
[r] $tax :pg.float << \|U.Math.Multiply"{$price, $tax_rate}"
```

**Output:** `$tax = 8.00`

---

### Scale Value

```polyglot
[r] $base :pg.int << 10
[r] $scale_factor :pg.float << 2.5
[r] $scaled :pg.float << \|U.Math.Multiply"{$base, $scale_factor}"
```

**Output:** `$scaled = 25.0`

---

## Common Patterns

### Pattern 1: Quantity × Price
```polyglot
[r] $total :pg.float << \|U.Math.Multiply"{$qty, $price}"
```

### Pattern 2: Calculate Percentage
```polyglot
[r] $percent_value :pg.float << \|U.Math.Multiply"{$total, 0.15}"
```

### Pattern 3: Scale/Convert Units
```polyglot
[r] $meters :pg.float << 5.0
[r] $centimeters :pg.float << \|U.Math.Multiply"{$meters, 100}"
```

---

## Related Pipelines

- [|U.Math.Divide](./divide.md) - Divide numbers
- [|U.Math.Double](./double.md) - Multiply by 2
- [*Math.Product](../../pack-operators/math/math-product.md) - Multiply iteration values

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
