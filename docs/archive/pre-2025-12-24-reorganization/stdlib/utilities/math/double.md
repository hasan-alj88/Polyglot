---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: double
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Double"
summary: "API reference: |U.Math.Double"
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
# |U.Math.Double

**Multiply a number by 2**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Double <value >result
```

**Inline:**
```polyglot
\|U.Math.Double"{$value}"
```

---

## Parameters

**Inputs:**
- `<value` - Number to double

**Outputs:**
- `>result` - Value × 2

---

## Description

Multiplies the input value by 2. Convenience wrapper for `|U.Math.Multiply"{$value, 2}"`.

**Operation:** `result = value × 2`

---

## Examples

### Basic Usage

```polyglot
[r] $doubled :pg.int << \|U.Math.Double"{5}"
```

**Output:** `$doubled = 10`

---

### In Loop

```polyglot
[r] ~ForEach.Array
[~] <array << [1, 2, 3, 4, 5]
[~] >item >> $num
   [r] $doubled :pg.int << \|U.Math.Double"{$num}"
   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

**Output:** `$results = [2, 4, 6, 8, 10]`

---

## Common Patterns

### Pattern 1: Double Values
```polyglot
[r] $result :pg.float << \|U.Math.Double"{$input}"
```

### Pattern 2: Scale by 2
```polyglot
[r] $width :pg.int << 100
[r] $double_width :pg.int << \|U.Math.Double"{$width}"
```

---

## Equivalent To

```polyglot
// These are equivalent:
[r] $doubled :pg.int << \|U.Math.Double"{$value}"
[r] $doubled :pg.int << \|U.Math.Multiply"{$value, 2}"
```

---

## Related Pipelines

- [|U.Math.Multiply](./multiply.md) - General multiplication

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
