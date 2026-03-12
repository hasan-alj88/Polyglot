---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Pack Operators (`**`)
summary: API reference: Pack Operators (`**`)
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
# Pack Operators (`**`)

**Aggregate data from iteration scope back to main scope**

**Markers:**
- `[v]` - Join marker (precedes pack operation)
- `[*]` - Pack marker (used with pack operators)

---

## Complete Pack Operators Tree

```
** (Pack Operators)
│
├── Collection Building
│   ├── *Into.*
│   │   ├── *Into.Array
│   │   │   ├── <item
│   │   │   └── >array
│   │   ├── *Into.Set
│   │   │   ├── <item
│   │   │   └── >set
│   │   └── *Into.Serial
│   │       ├── <item
│   │       └── >serial
│   └── *String.*
│       ├── *String.Concat
│       │   ├── <string :pg.string
│       │   └── >concatenated :pg.string
│       └── *String.Lines
│           ├── <line :pg.string
│           └── >lines :pg.string
│
├── Collect
│   ├── *Join.First
│   │   ├── <item
│   │   └── >first
│   ├── *Join.Last
│   │   ├── <item
│   │   └── >last
│   ├── *Join.Nth
│   │   ├── <item
│   │   ├── <n :pg.uint
│   │   └── >nth
│   └── *Collect.Errors
│       ├── <error :pg.error
│       └── >errors :pg.array.pg.error
│
└── *Math.*
    ├── *Math.Sum
    │   ├── <item
    │   └── >sum
    ├── *Math.Product
    │   ├── <item
    │   └── >product
    ├── *Math.Min
    │   ├── <item
    │   └── >min
    ├── *Math.Max
    │   ├── <item
    │   └── >max
    ├── *Math.Count
    │   ├── <item
    │   └── >count
    └── *Math.Average
        ├── <item
        └── >average
```

**Total: 13 pack operators**

---

## Quick Navigation

### Collection Building
Build collections from iteration results

**Into Operators:**
- [*Into.Array](./collection-building/into/into-array.md) - Collect into array
- [*Into.Set](./collection-building/into/into-set.md) - Collect into set (unique values)
- [*Into.Serial](./collection-building/into/into-serial.md) - Collect into serial data

**String Operators:**
- [*String.Concat](./collection-building/string/string-concat.md) - Concatenate strings
- [*String.Lines](./collection-building/string/string-lines.md) - Join with newlines

**See:** [Collection Building Package](./collection-building/README.md)

---

### Collect Operators
Select specific items from iterations

- [*Join.First](./collect/join-first.md) - Take first item
- [*Join.Last](./collect/join-last.md) - Take last item
- [*Join.Nth](./collect/join-nth.md) - Take Nth item
- [*Collect.Errors](./collect/collect-errors.md) - Collect all errors

**See:** [Collect Package](./collect/README.md)

---

### Math Operators
Aggregate numeric values

- [*Math.Sum](./math/math-sum.md) - Sum all values
- [*Math.Product](./math/math-product.md) - Multiply all values
- [*Math.Min](./math/math-min.md) - Find minimum
- [*Math.Max](./math/math-max.md) - Find maximum
- [*Math.Count](./math/math-count.md) - Count iterations
- [*Math.Average](./math/math-average.md) - Calculate average

**See:** [Math Package](./math/README.md)

---

## What Are Pack Operators?

**Pack operators** aggregate data from the **iteration scope** back to the **main scope**:

```
Iteration Scope               Main Scope
----------------              -----------
$item = 1  (iteration 1)
$item = 2  (iteration 2)   →  $results = [1, 2, 3]
$item = 3  (iteration 3)  **
```

**Direction:** Iteration → Main

**Markers:**
- `[v]` - Join (synchronization point, looks like funnel V)
- `[*]` - Pack (aggregation operation)

**See:** [Loop System](../../User/language/advanced/loop-system.md)

---

## Basic Usage

```polyglot
[execution_marker] ~UnpackOperator
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Transform <input << $element

   [v] *PackOperator              // Join marker + pack operator
   [*] <input_name << $processed  // Input from iteration scope
   [*] >output_name >> $result    // Output to main scope
```

**Key Point:** `[v]` join marker MUST precede pack operation

---

## Common Patterns

### Pattern 1: Collect to Array

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Transform <input << $element

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

### Pattern 2: Sum Values

```polyglot
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

### Pattern 3: Concatenate Strings

```polyglot
[r] ~ForEach.Array
[~] <array << $words
[~] >item >> $word

   [v] *String.Concat
   [*] <string << $word
   [*] >concatenated >> $sentence
```

### Pattern 4: Take First Result

```polyglot
[p] ~ForEach.Array
[~] <array << $api_endpoints
[~] >item >> $endpoint

   [r] $response << \|HTTP.Get <url << $endpoint

   [v] *Join.First
   [*] <item << $response
   [*] >first >> $fastest_response
```

---

## Related Documentation

- [Loop System](../../User/language/advanced/loop-system.md) - Complete loop documentation
- [Unpack Operators](../unpack-operators/README.md) - Expansion operations
- [Standard Library Overview](../README.md) - Complete package tree

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../README.md)
