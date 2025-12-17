---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: round
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Round"
summary: "API reference: |U.Math.Round"
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
# |U.Math.Round

**Round to nearest integer**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Round <value >result
```

**Inline:**
```polyglot
\|U.Math.Round"{$value}"
```

---

## Parameters

**Inputs:**
- `<value` :pg.float - Number to round

**Outputs:**
- `>result` :pg.int - Nearest integer

---

## Description

Rounds a floating-point number to the nearest integer using standard rounding rules (0.5 rounds up).

**Rounding rules:**
- 5.4 → 5
- 5.5 → 6
- 5.6 → 6
- -5.5 → -6

---

## Examples

### Basic Usage

```polyglot
[r] $rounded :pg.int << \|U.Math.Round"{5.7}"
```

**Output:** `$rounded = 6`

---

### Round Down

```polyglot
[r] $rounded :pg.int << \|U.Math.Round"{5.3}"
```

**Output:** `$rounded = 5`

---

### Exactly 0.5

```polyglot
[r] $rounded :pg.int << \|U.Math.Round"{5.5}"
```

**Output:** `$rounded = 6` (rounds up)

---

### Negative Numbers

```polyglot
[r] $rounded :pg.int << \|U.Math.Round"{-5.7}"
```

**Output:** `$rounded = -6`

---

## Common Patterns

### Pattern 1: Clean Float to Int
```polyglot
[r] $int_value :pg.int << \|U.Math.Round"{$float_value}"
```

### Pattern 2: Round Currency
```polyglot
[r] $cents :pg.float << 123.456
[r] $rounded_cents :pg.int << \|U.Math.Round"{$cents}"
```

---

## Related Pipelines

- [|U.Math.Floor](./floor.md) - Always round down
- [|U.Math.Ceiling](./ceiling.md) - Always round up

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
