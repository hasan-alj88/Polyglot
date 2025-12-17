---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: subtract
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Subtract"
summary: "API reference: |U.Math.Subtract"
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
# |U.Math.Subtract

**Subtract one number from another**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Subtract <a <b >result
```

**Inline:**
```polyglot
\|U.Math.Subtract"{$a, $b}"
```

---

## Parameters

**Inputs:**
- `<a` - Number to subtract from
- `<b` - Number to subtract

**Outputs:**
- `>result` - Difference (a - b)

---

## Description

Subtracts the second number from the first and returns the difference.

**Operation:** `result = a - b`

---

## Examples

### Basic Usage

```polyglot
[r] $difference :pg.int << \|U.Math.Subtract"{10, 3}"
```

**Output:** `$difference = 7`

---

### Calculate Discount

```polyglot
[r] $original_price :pg.float << 50.00
[r] $discount :pg.float << 10.00
[r] $final_price :pg.float << \|U.Math.Subtract"{$original_price, $discount}"
```

**Output:** `$final_price = 40.00`

---

### Range Calculation

```polyglot
[r] $max :pg.int << 100
[r] $min :pg.int << 20
[r] $range :pg.int << \|U.Math.Subtract"{$max, $min}"
```

**Output:** `$range = 80`

---

### Negative Results

```polyglot
[r] $result :pg.int << \|U.Math.Subtract"{5, 10}"
```

**Output:** `$result = -5`

---

## Type Handling

**Integer - Integer:**
```polyglot
[r] $result :pg.int << \|U.Math.Subtract"{10, 3}"  // 7
```

**Float - Float:**
```polyglot
[r] $result :pg.float << \|U.Math.Subtract"{10.5, 3.2}"  // 7.3
```

**Mixed types:**
```polyglot
[r] $result :pg.float << \|U.Math.Subtract"{10, 3.5}"  // 6.5 (promotes to float)
```

---

## Common Patterns

### Pattern 1: Decrement
```polyglot
[r] $prev :pg.int << \|U.Math.Subtract"{$current, 1}"
```

### Pattern 2: Apply Discount
```polyglot
[r] $discounted :pg.float << \|U.Math.Subtract"{$price, $discount_amount}"
```

### Pattern 3: Calculate Remaining
```polyglot
[r] $remaining :pg.int << \|U.Math.Subtract"{$total, $used}"
```

---

## Related Pipelines

- [|U.Math.Add](./add.md) - Add numbers
- [|U.Math.Abs](./abs.md) - Absolute value (for absolute difference)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
