---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: floor
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Floor"
summary: "API reference: |U.Math.Floor"
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
# |U.Math.Floor

**Round down to integer**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Floor <value >result
```

**Inline:**
```polyglot
\|U.Math.Floor"{$value}"
```

---

## Parameters

**Inputs:**
- `<value` :pg.float - Number to round down

**Outputs:**
- `>result` :pg.int - Largest integer ≤ value

---

## Description

Rounds a floating-point number down to the largest integer less than or equal to the value.

**Always rounds down (toward negative infinity).**

---

## Examples

### Basic Usage

```polyglot
[r] $floored :pg.int << \|U.Math.Floor"{5.9}"
```

**Output:** `$floored = 5`

---

### Negative Numbers

```polyglot
[r] $floored :pg.int << \|U.Math.Floor"{-5.1}"
```

**Output:** `$floored = -6` (rounds toward negative infinity)

---

## Common Patterns

### Pattern 1: Integer Division
```polyglot
[r] $int_div :pg.int << \|U.Math.Floor"{\|U.Math.Divide\"{$a, $b}\"}"
```

---

## Related Pipelines

- [|U.Math.Round](./round.md) - Round to nearest
- [|U.Math.Ceiling](./ceiling.md) - Round up

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
