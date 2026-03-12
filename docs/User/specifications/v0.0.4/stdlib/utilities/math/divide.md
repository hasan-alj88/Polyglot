---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: divide
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Divide"
summary: "API reference: |U.Math.Divide"
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
# |U.Math.Divide

**Divide one number by another**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Divide <a <b >result
```

**Inline:**
```polyglot
\|U.Math.Divide"{$a, $b}"
```

---

## Parameters

**Inputs:**
- `<a` - Dividend (number to be divided)
- `<b` - Divisor (number to divide by)

**Outputs:**
- `>result` :pg.float - Quotient (a ÷ b)

---

## Description

Divides the first number by the second and returns the quotient. Always returns a floating-point result.

**Operation:** `result = a ÷ b`

**Note:** Division by zero produces an error.

---

## Examples

### Basic Usage

```polyglot
[r] $quotient :pg.float << \|U.Math.Divide"{20, 4}"
```

**Output:** `$quotient = 5.0`

---

### Calculate Average

```polyglot
[r] $sum :pg.int << 100
[r] $count :pg.int << 4
[r] $average :pg.float << \|U.Math.Divide"{$sum, $count}"
```

**Output:** `$average = 25.0`

---

### Percentage Calculation

```polyglot
[r] $part :pg.int << 45
[r] $total :pg.int << 150
[r] $fraction :pg.float << \|U.Math.Divide"{$part, $total}"
[r] $percentage :pg.float << \|U.Math.Multiply"{$fraction, 100}"
```

**Output:** `$percentage = 30.0`

---

### Unit Conversion

```polyglot
[r] $centimeters :pg.float << 250.0
[r] $meters :pg.float << \|U.Math.Divide"{$centimeters, 100}"
```

**Output:** `$meters = 2.5`

---

## Division by Zero

**Produces error:**
```polyglot
[z] $result :pg.float << \|U.Math.Divide"{10, 0}"
   [!] !Math.DivisionByZero
      // Handle error
```

---

## Type Handling

**Always returns float:**
```polyglot
[r] $result :pg.float << \|U.Math.Divide"{10, 3}"  // 3.333...
[r] $result :pg.float << \|U.Math.Divide"{20, 4}"  // 5.0
```

---

## Common Patterns

### Pattern 1: Calculate Average
```polyglot
[r] $avg :pg.float << \|U.Math.Divide"{$sum, $count}"
```

### Pattern 2: Split Equally
```polyglot
[r] $per_person :pg.float << \|U.Math.Divide"{$total, $people}"
```

### Pattern 3: Percentage
```polyglot
[r] $percent :pg.float << \|U.Math.Multiply"{\|U.Math.Divide\"{$part, $whole}\", 100}"
```

---

## Related Pipelines

- [|U.Math.Multiply](./multiply.md) - Multiply numbers
- [|U.Math.Modulo](./modulo.md) - Remainder of division
- [*Math.Average](../../pack-operators/math/math-average.md) - Calculate average from iterations

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
