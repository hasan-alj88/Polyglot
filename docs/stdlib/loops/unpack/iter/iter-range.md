---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "iter-range"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~Iter.Range
summary: "API reference: ~Iter.Range"
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
# ~Iter.Range

**Generate numeric range for iteration**

**Category:** Unpack Operators > Iter
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~Iter.Range
[~] <from :pg.int
[~] <to :pg.int
[~] >index :pg.int
```

---

## Parameters

**Inputs:**
- `<from` :pg.int - Start value (inclusive)
- `<to` :pg.int - End value (exclusive)

**Outputs:**
- `>index` :pg.int - Current index value

---

## Description

Generates a numeric range from `<from>` (inclusive) to `<to>` (exclusive), creating one iteration per integer in the range. This is the Polyglot equivalent of traditional for-loops like `for i in range(0, 10)`.

**Range semantics:**
- **Inclusive start** - `<from>` is included
- **Exclusive end** - `<to>` is NOT included
- **Step size** - Always 1 (use transformation for different steps)

**Use when:**
- Numeric loops
- Generate sequences
- Index-based iteration
- Fixed iteration count

---

## Examples

### Basic Usage - Zero to N

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 5
[~] >index >> $i

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $numbers
```

**Output:** `$numbers = [0, 1, 2, 3, 4]`

**Note:** `<to>` value (5) is **not** included.

---

### One to N

```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << 11
[~] >index >> $i

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $numbers
```

**Output:** `$numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`

---

### Generate Squares

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

**Output:** `$squares = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]`

---

### Build Indexed Serial Data

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 3
[~] >index >> $i

   [r] $index_str :pg.string << \|U.String.FromInt"{$i}"
   [r] $value :pg.int << \|U.Math.Multiply"{$i, 10}"

   [v] *Into.Serial
   [*] <path << $index_str
   [*] <item << $value
   [*] >serial >> $indexed_data
```

**Output:**
```json
{
  "0": 0,
  "1": 10,
  "2": 20
}
```

---

### Generate Labels

```polyglot
[r] ~Iter.Range
[~] <from << 1
[~] <to << 6
[~] >index >> $i

   [r] $label :pg.string << \|U.String.Concat"{\"Item \", $i}"

   [v] *String.Lines
   [*] <line << $label
   [*] >lines >> $labels
```

**Output:**
```
Item 1
Item 2
Item 3
Item 4
Item 5
```

---

### Step by N (Custom Step)

**Polyglot doesn't have native step parameter. Implement with filtering:**

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 20
[~] >index >> $i

   [r] $remainder :pg.int << \|U.Math.Mod"{$i, 3}"

   [y] $remainder == 0
      [v] *Into.Array
      [*] <item << $i
      [*] >array >> $multiples_of_three
```

**Output:** `$multiples_of_three = [0, 3, 6, 9, 12, 15, 18]`

---

### Reverse Range (Countdown)

**Polyglot ranges are always ascending. Implement reverse with transformation:**

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 5
[~] >index >> $i

   [r] $reversed :pg.int << \|U.Math.Subtract"{4, $i}"

   [v] *Into.Array
   [*] <item << $reversed
   [*] >array >> $countdown
```

**Output:** `$countdown = [4, 3, 2, 1, 0]`

---

## Empty Range Handling

**If `<from>` >= `<to>`, no iterations occur:**

```polyglot
[r] ~Iter.Range
[~] <from << 5
[~] <to << 5
[~] >index >> $i

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $result
```

**Output:** `$result = []`

---

## Negative Numbers

**Ranges support negative integers:**

```polyglot
[r] ~Iter.Range
[~] <from << -3
[~] <to << 3
[~] >index >> $i

   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $numbers
```

**Output:** `$numbers = [-3, -2, -1, 0, 1, 2]`

---

## Type

**Index is always `:pg.int`:**

```polyglot
[~] >index :pg.int
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $i
   // Iterations run in order: 0, 1, 2, ...
```

**Use when:**
- Order matters
- Operations have dependencies
- Sequential processing required

### Parallel [p]

```polyglot
[p] ~Iter.Range
[~] <from << 0
[~] <to << 100
[~] >index >> $i
   // All iterations run concurrently
[v]
```

**Use when:**
- Independent operations
- Performance critical
- Safe for concurrent execution

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
   [*] >array >> $sequence
```

### Pattern 2: Repeat Operation N Times
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $i
   [r] $result << \|ExpensiveOperation""
   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

### Pattern 3: Build Array of Fixed Size
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 5
[~] >index >> $i
   [r] $default :pg.int << 0
   [v] *Into.Array
   [*] <item << $default
   [*] >array >> $zeros  // [0, 0, 0, 0, 0]
```

### Pattern 4: Index-Based Data Access
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << $count
[~] >index >> $i
   [r] $item << \|Database.Items.GetByIndex <index << $i
   [v] *Into.Array
   [*] <item << $item
   [*] >array >> $items
```

### Pattern 5: Pagination
```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $page
   [r] $offset :pg.int << \|U.Math.Multiply"{$page, 100}"
   [r] $data << \|API.Fetch <offset << $offset <limit << 100
   [v] *Into.Array
   [*] <item << $data
   [*] >array >> $all_pages
```

---

## Performance

**Time Complexity:** O(n) where n = (`<to>` - `<from>`)

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Integer generation:** O(1) per iteration

---

## Comparison with Other Operators

| Operator | Input | Output | Use Case |
|----------|-------|--------|----------|
| **~Iter.Range** | from, to | index | Numeric loops |
| **~ForEach.Array** | array | item | Array iteration |
| **~ForEach.IndexedArray** | array | index, item | Array with index |

**When to use ~Iter.Range:**
- Fixed iteration count
- Numeric sequence generation
- Index-based operations
- Don't have existing collection

**When to use ~ForEach.Array:**
- Iterating existing array
- Processing collection elements

---

## Related Operators

- [~ForEach.IndexedArray](../foreach/foreach-indexedarray.md) - Array with index
- [~Iter.SlidingWindow](./iter-slidingwindow.md) - Overlapping windows
- [*Math.Sum](../../pack-operators/math/math-sum.md) - Sum numeric values

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Math Utilities](../../utilities/math/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
