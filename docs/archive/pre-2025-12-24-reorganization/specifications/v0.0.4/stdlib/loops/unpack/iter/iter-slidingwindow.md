---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "iter-slidingwindow"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~Iter.SlidingWindow
summary: "API reference: ~Iter.SlidingWindow"
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
# ~Iter.SlidingWindow

**Iterate array with sliding overlapping windows**

**Category:** Unpack Operators > Iter
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~Iter.SlidingWindow
[~] <array :pg.array.*
[~] <window :pg.int
[~] >item :pg.array
```

---

## Parameters

**Inputs:**
- `<array` :pg.array.* - Array to iterate over
- `<window` :pg.int - Window size (number of elements per window)

**Outputs:**
- `>item` :pg.array - Sub-array of `<window>` size (overlapping windows)

---

## Description

Creates sliding overlapping windows over an array, where each iteration receives a sub-array of size `<window>`. The window "slides" one element at a time.

**Window semantics:**
- **Overlapping** - Consecutive windows share elements
- **Fixed size** - Each window contains exactly `<window>` elements
- **Stops early** - When remaining elements < window size

**Use when:**
- Pattern detection across elements
- Moving averages
- N-gram generation
- Context-aware processing
- Overlapping chunk analysis

---

## Examples

### Basic Usage - Window Size 3

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $numbers
[~] <window << 3
[~] >item >> $window_array

   [v] *Into.Array
   [*] <item << $window_array
   [*] >array >> $windows
```

**Input:** `$numbers = [1, 2, 3, 4, 5]`
**Output:**
```
$windows = [
  [1, 2, 3],
  [2, 3, 4],
  [3, 4, 5]
]
```

**Note:** 3 windows total, each overlapping with the next.

---

### Moving Average

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $values
[~] <window << 3
[~] >item >> $window_array

   // Sum the window
   [r] ~ForEach.Array
   [~] <array << $window_array
   [~] >item >> $val
      [v] *Math.Sum
      [*] <item << $val
      [*] >sum >> $window_sum

   // Calculate average
   [r] $avg :pg.float << \|U.Math.Divide"{$window_sum, 3}"

   [v] *Into.Array
   [*] <item << $avg
   [*] >array >> $moving_averages
```

**Input:** `$values = [10, 20, 30, 40, 50]`
**Output:** `$moving_averages = [20.0, 30.0, 40.0]`

---

### N-gram Generation

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $words
[~] <window << 2
[~] >item >> $bigram_array

   [r] $first :pg.string << $bigram_array.0
   [r] $second :pg.string << $bigram_array.1
   [r] $bigram :pg.string << \|U.String.Concat"{$first, \" \", $second}"

   [v] *Into.Array
   [*] <item << $bigram
   [*] >array >> $bigrams
```

**Input:** `$words = ["the", "quick", "brown", "fox"]`
**Output:**
```
$bigrams = [
  "the quick",
  "quick brown",
  "brown fox"
]
```

---

### Pattern Detection

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $sequence
[~] <window << 3
[~] >item >> $window_array

   // Check if window matches pattern [1, 2, 3]
   [f] $window_array.0 == 1 & $window_array.1 == 2 & $window_array.2 == 3
      [r] $found :pg.bool << #True
      [v] *Join.First
      [*] <item << $found
      [*] >first >> $pattern_found
```

---

### Window Size 2 - Pairs

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $items
[~] <window << 2
[~] >item >> $pair

   [r] $a << $pair.0
   [r] $b << $pair.1
   [r] $combined :pg.string << \|U.String.Concat"{$a, \"->\", $b}"

   [v] *String.Lines
   [*] <line << $combined
   [*] >lines >> $transitions
```

**Input:** `$items = ["A", "B", "C", "D"]`
**Output:**
```
A->B
B->C
C->D
```

---

### Maximum in Each Window

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $numbers
[~] <window << 3
[~] >item >> $window_array

   [r] ~ForEach.Array
   [~] <array << $window_array
   [~] >item >> $num
      [v] *Math.Max
      [*] <item << $num
      [*] >max >> $window_max

   [v] *Into.Array
   [*] <item << $window_max
   [*] >array >> $max_per_window
```

**Input:** `$numbers = [5, 2, 8, 1, 9]`
**Output:** `$max_per_window = [8, 8, 9]`

---

## Small Array Handling

**If array length < window size, no iterations occur:**

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $small
[~] <window << 5
[~] >item >> $window_array

   [v] *Into.Array
   [*] <item << $window_array
   [*] >array >> $result
```

**Input:** `$small = [1, 2]`
**Output:** `$result = []`

**No windows produced because array has fewer than 5 elements.**

---

## Window Size 1

**Window size 1 produces same as ~ForEach.Array but as nested arrays:**

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $items
[~] <window << 1
[~] >item >> $single_item_array

   [r] $item << $single_item_array.0

   [v] *Into.Array
   [*] <item << $item
   [*] >array >> $result
```

**Same as ~ForEach.Array but with extra wrapping/unwrapping.**

---

## Number of Windows

**For array of length N and window size W:**

**Number of windows = N - W + 1**

| Array Length | Window Size | Windows |
|--------------|-------------|---------|
| 5 | 3 | 3 |
| 10 | 2 | 9 |
| 7 | 4 | 4 |
| 3 | 5 | 0 (no windows) |

---

## Type Inference

**Output is always array type:**

```polyglot
[~] >item :pg.array
```

**Element type matches input array element type:**

| Input Array Type | Window Array Type |
|------------------|-------------------|
| `:pg.array.pg.int` | `:pg.array.pg.int` |
| `:pg.array.pg.string` | `:pg.array.pg.string` |

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $items
[~] <window << 3
[~] >item >> $window_array
   // Windows processed in order
```

**Use when:**
- Order matters
- Operations have dependencies
- Sequential analysis required

### Parallel [p]

```polyglot
[p] ~Iter.SlidingWindow
[~] <array << $items
[~] <window << 3
[~] >item >> $window_array
   // All windows processed concurrently
[v]
```

**Use when:**
- Independent window processing
- Performance critical
- Safe for concurrent execution

---

## Common Patterns

### Pattern 1: Detect Consecutive Patterns
```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $sequence
[~] <window << 3
[~] >item >> $triple
   [f] $triple.0 < $triple.1 & $triple.1 < $triple.2
      [v] *Into.Array
      [*] <item << $triple
      [*] >array >> $ascending_triples
```

### Pattern 2: Context-Aware Processing
```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $tokens
[~] <window << 3
[~] >item >> $context
   [r] $prev :pg.string << $context.0
   [r] $current :pg.string << $context.1
   [r] $next :pg.string << $context.2
   [r] $processed << \|ProcessWithContext <prev << $prev <curr << $current <next << $next
   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

### Pattern 3: Sum of Consecutive Elements
```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $numbers
[~] <window << 2
[~] >item >> $pair
   [r] $sum :pg.int << \|U.Math.Add"{$pair.0, $pair.1}"
   [v] *Into.Array
   [*] <item << $sum
   [*] >array >> $pairwise_sums
```

### Pattern 4: Trend Detection
```polyglot
[r] ~Iter.SlidingWindow
[~] <array << $prices
[~] <window << 3
[~] >item >> $window
   [r] $trend :pg.string << \|DetectTrend <prices << $window
   [v] *String.Lines
   [*] <line << $trend
   [*] >lines >> $trend_report
```

---

## Performance

**Time Complexity:** O(n - w + 1) where:
- n = array length
- w = window size

**Space Complexity:**
- Sequential: O(w) per iteration (window array)
- Parallel: O((n - w + 1) * w) for all windows

**Window creation:** O(w) per window (sub-array extraction)

---

## Comparison with Other Operators

| Operator | Overlap | Output | Use Case |
|----------|---------|--------|----------|
| **~Iter.SlidingWindow** | Yes | Overlapping sub-arrays | Pattern detection |
| **~ForEach.Array** | No | Individual elements | Standard iteration |
| **~Chunk** (hypothetical) | No | Non-overlapping chunks | Batch processing |

**When to use ~Iter.SlidingWindow:**
- Need overlapping windows
- Pattern detection
- Moving calculations
- Context-aware processing

**When to use ~ForEach.Array:**
- Process individual elements
- No overlap needed
- Standard iteration

---

## Related Operators

- [~ForEach.Array](../foreach/foreach-array.md) - Standard array iteration
- [~Iter.Range](./iter-range.md) - Numeric range iteration
- [*Math.Sum](../../pack-operators/math/math-sum.md) - Sum values

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Array Utilities](../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
