---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: ceiling
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Ceiling"
summary: "API reference: |U.Math.Ceiling"
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
# |U.Math.Ceiling

**Round up to integer**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Ceiling <value >result
```

**Inline:**
```polyglot
\|U.Math.Ceiling"{$value}"
```

**Alias:** `|U.Math.Ceil`

---

## Parameters

**Inputs:**
- `<value` :pg.float - Number to round up

**Outputs:**
- `>result` :pg.int - Smallest integer ≥ value

---

## Description

Rounds a floating-point number up to the smallest integer greater than or equal to the value.

**Always rounds up (toward positive infinity).**

---

## Examples

### Basic Usage

```polyglot
[r] $ceiled :pg.int << \|U.Math.Ceiling"{5.1}"
```

**Output:** `$ceiled = 6`

---

### Negative Numbers

```polyglot
[r] $ceiled :pg.int << \|U.Math.Ceiling"{-5.9}"
```

**Output:** `$ceiled = -5` (rounds toward positive infinity)

---

### Calculate Pages Needed

```polyglot
[r] $total_items :pg.int << 47
[r] $items_per_page :pg.int << 10
[r] $pages_needed :pg.int << \|U.Math.Ceiling"{\|U.Math.Divide\"{$total_items, $items_per_page}\"}"
```

**Output:** `$pages_needed = 5`

---

## Common Patterns

### Pattern 1: Pages/Chunks Needed
```polyglot
[r] $chunks :pg.int << \|U.Math.Ceiling"{\|U.Math.Divide\"{$total, $chunk_size}\"}"
```

---

## Related Pipelines

- [|U.Math.Round](./round.md) - Round to nearest
- [|U.Math.Floor](./floor.md) - Round down

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
