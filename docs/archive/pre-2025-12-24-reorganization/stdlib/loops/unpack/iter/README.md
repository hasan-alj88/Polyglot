tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Iter Unpack Operators
summary: "API reference: Iter Unpack Operators"
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
complexity: medium

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
 BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Iter Unpack Operators
summary: API reference: Iter Unpack Operators
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
complexity: medium

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
# Iter Unpack Operators

**Generate iteration sequences**

---

## Operators Tree

**~Iter.\***
- [**~Iter.Range**](./iter-range.md)
  - `<from :pg.int`
  - `<to :pg.int`
  - `>index :pg.int`
- [**~Iter.SlidingWindow**](./iter-slidingwindow.md)
  - `<array :pg.array.*`
  - `<window :pg.int`
  - `>item :pg.array`

---

## Overview

Iter operators generate iteration sequences programmatically, enabling numeric ranges and sliding window patterns over collections.

**Common pattern:**
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $i

   [r] $squared :pg.int << \|U.Math.Multiply"{$i, $i}"

   [v] *Into.Array
   [*] <item << $squared
   [*] >array >> $squares
```

---

## Range Operator

- [~Iter.Range](./iter-range.md) - Generate numeric range

**Use when:**
- Numeric loops (0 to N)
- Generate sequence of numbers
- Index-based iteration
- Traditional for-loop patterns

---

## Window Operator

- [~Iter.SlidingWindow](./iter-slidingwindow.md) - Iterate with sliding window

**Use when:**
- Processing overlapping chunks
- Pattern detection across elements
- N-gram generation
- Moving averages

---

## Comparison Table

| Operator | Input | Output | Use Case |
|----------|-------|--------|----------|
| **~Iter.Range** | from, to | index | Numeric loops |
| **~Iter.SlidingWindow** | array, window size | sub-array | Overlapping chunks |

---

## Common Patterns

### Pattern 1: Generate Sequence
```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << 11
[~] >index >> $i
   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $numbers  // [1, 2, 3, ..., 10]
```

### Pattern 2: Build Indexed Structure
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 5
[~] >index >> $i
   [r] $key :pg.string << \|U.String.FromInt"{$i}"
   [r] $value :pg.int << \|U.Math.Multiply"{$i, 10}"
   [v] *Into.Serial
   [*] <path << $key
   [*] <item << $value
   [*] >serial >> $indexed  // {"0": 0, "1": 10, "2": 20, ...}
```

### Pattern 3: Sliding Window Average
```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $values
[~] <window << 3
[~] >item >> $window_array
   [r] ~ForEach.Array
   [~] <array << $window_array
   [~] >item >> $val
      [v] *Math.Sum
      [*] <item << $val
      [*] >sum >> $window_sum

   [r] $avg :pg.float << \|U.Math.Divide"{$window_sum, 3}"
   [v] *Into.Array
   [*] <item << $avg
   [*] >array >> $moving_averages
```

---

## Related Documentation

- [Unpack Operators Overview](../README.md)
- [ForEach Operators](../foreach/README.md)
- [Loop System](../../../User/language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
