---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: modulo
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Math.Modulo"
summary: "API reference: |U.Math.Modulo"
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
# |U.Math.Modulo

**Calculate remainder of division**

**Category:** Utilities > Math
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Math.Modulo <a <b >result
```

**Inline:**
```polyglot
\|U.Math.Modulo"{$a, $b}"
```

**Alias:** `|U.Math.Mod`

---

## Parameters

**Inputs:**
- `<a` - Dividend
- `<b` - Divisor

**Outputs:**
- `>result` - Remainder (a % b)

---

## Description

Returns the remainder after dividing the first number by the second.

**Operation:** `result = a % b`

---

## Examples

### Basic Usage

```polyglot
[r] $remainder :pg.int << \|U.Math.Modulo"{10, 3}"
```

**Output:** `$remainder = 1`

---

### Check Even/Odd

```polyglot
[r] $is_even_check :pg.int << \|U.Math.Modulo"{$number, 2}"

[y] $is_even_check == 0
   // Number is even
[^]
   // Number is odd
```

---

### Cycle Through Values

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $i
   [r] $color_index :pg.int << \|U.Math.Modulo"{$i, 3}"
   // Cycles through 0, 1, 2, 0, 1, 2, ...
```

---

### Extract Last Digit

```polyglot
[r] $last_digit :pg.int << \|U.Math.Modulo"{$number, 10}"
```

**Input:** `$number = 12345`
**Output:** `$last_digit = 5`

---

## Common Patterns

### Pattern 1: Even/Odd Check
```polyglot
[r] $mod2 :pg.int << \|U.Math.Modulo"{$num, 2}"
[y] $mod2 == 0
   // Even
```

### Pattern 2: Every Nth Item
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item
   [r] $mod_n :pg.int << \|U.Math.Modulo"{$i, 5}"
   [y] $mod_n == 0
      // Process every 5th item
```

### Pattern 3: Wraparound Index
```polyglot
[r] $wrapped_index :pg.int << \|U.Math.Modulo"{$index, $array_length}"
```

---

## Related Pipelines

- [|U.Math.Divide](./divide.md) - Division operation
- [|U.Math.Floor](./floor.md) - Integer division (use with divide)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
