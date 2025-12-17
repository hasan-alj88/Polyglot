---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: abs
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Abs"
summary: "API reference: |U.Math.Abs"
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
# |U.Math.Abs

**Calculate absolute value**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Abs <value >result
```

**Inline:**
```polyglot
\|U.Math.Abs"{$value}"
```

---

## Parameters

**Inputs:**
- `<value` - Number (can be positive or negative)

**Outputs:**
- `>result` - Absolute value (always positive or zero)

---

## Description

Returns the absolute value of a number, removing any negative sign.

**Operation:** `result = |value|`

---

## Examples

### Basic Usage

```polyglot
[r] $abs :pg.int << \|U.Math.Abs"{-5}"
```

**Output:** `$abs = 5`

---

### Positive Input

```polyglot
[r] $abs :pg.int << \|U.Math.Abs"{10}"
```

**Output:** `$abs = 10` (unchanged)

---

### Calculate Distance

```polyglot
[r] $point_a :pg.int << 10
[r] $point_b :pg.int << 25
[r] $diff :pg.int << \|U.Math.Subtract"{$point_a, $point_b}"
[r] $distance :pg.int << \|U.Math.Abs"{$diff}"
```

**Output:** `$distance = 15`

---

## Common Patterns

### Pattern 1: Ensure Positive
```polyglot
[r] $positive :pg.int << \|U.Math.Abs"{$value}"
```

### Pattern 2: Calculate Distance
```polyglot
[r] $distance :pg.int << \|U.Math.Abs"{\|U.Math.Subtract\"{$a, $b}\"}"
```

---

## Related Pipelines

- [|U.Math.Subtract](./subtract.md) - Used with Abs for distance calculations

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
