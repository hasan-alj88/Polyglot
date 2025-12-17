---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: add
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Add"
summary: "API reference: |U.Math.Add"
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
# |U.Math.Add

**Add two numbers**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Add <a <b >result
```

**Inline:**
```polyglot
\|U.Math.Add"{$a, $b}"
```

---

## Parameters

**Inputs:**
- `<a` - First number
- `<b` - Second number

**Outputs:**
- `>result` - Sum of a and b

---

## Description

Adds two numbers and returns their sum. Supports both integer and floating-point arithmetic.

**Operation:** `result = a + b`

---

## Examples

### Basic Usage

```polyglot
[r] $sum :pg.int << \|U.Math.Add"{5, 3}"
```

**Output:** `$sum = 8`

---

### With Variables

```polyglot
[r] $price :pg.float << 19.99
[r] $tax :pg.float << 1.60
[r] $total :pg.float << \|U.Math.Add"{$price, $tax}"
```

**Output:** `$total = 21.59`

---

### In Loop

```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $plus_ten :pg.int << \|U.Math.Add"{$num, 10}"
   [v] *Into.Array
   [*] <item << $plus_ten
   [*] >array >> $results
```

**Input:** `$numbers = [1, 2, 3]`
**Output:** `$results = [11, 12, 13]`

---

### Multiple Additions

```polyglot
[r] $subtotal :pg.float << 100.00
[r] $tax :pg.float << 8.00
[r] $shipping :pg.float << 5.00
[r] $with_tax :pg.float << \|U.Math.Add"{$subtotal, $tax}"
[r] $total :pg.float << \|U.Math.Add"{$with_tax, $shipping}"
```

**Output:** `$total = 113.00`

---

## Type Handling

**Integer + Integer:**
```polyglot
[r] $result :pg.int << \|U.Math.Add"{5, 3}"  // 8
```

**Float + Float:**
```polyglot
[r] $result :pg.float << \|U.Math.Add"{5.5, 2.3}"  // 7.8
```

**Integer + Float:**
```polyglot
[r] $result :pg.float << \|U.Math.Add"{5, 2.5}"  // 7.5 (promotes to float)
```

---

## Common Patterns

### Pattern 1: Calculate Total
```polyglot
[r] $total :pg.float << \|U.Math.Add"{$subtotal, $tax}"
```

### Pattern 2: Increment
```polyglot
[r] $next :pg.int << \|U.Math.Add"{$current, 1}"
```

### Pattern 3: Offset
```polyglot
[r] $adjusted :pg.int << \|U.Math.Add"{$base_value, $offset}"
```

---

## Related Pipelines

- [|U.Math.Subtract](./subtract.md) - Subtract numbers
- [|U.Math.Multiply](./multiply.md) - Multiply numbers
- [*Math.Sum](../../pack-operators/math/math-sum.md) - Sum iteration values

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
