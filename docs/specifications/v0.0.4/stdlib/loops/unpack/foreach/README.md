---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: ForEach Unpack Operators
summary: API reference: ForEach Unpack Operators
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
# ForEach Unpack Operators

**Iterate over collection elements**

---

## Operators Tree

**~ForEach.\***
- [**~ForEach.Array**](./foreach-array.md)
  - `<array :pg.array.*`
  - `>item :*`
- [**~ForEach.Set**](./foreach-set.md)
  - `<set :pg.set.*`
  - `>item :*`
- [**~ForEach.Serial**](./foreach-serial.md)
  - `<serial :pg.serial`
  - `>path :pg.string`
  - `>item :*`
- [**~ForEach.SerialArray**](./foreach-serialarray.md)
  - `<serial :pg.serial`
  - `<path :pg.string`
  - `>item :pg.serial`
- [**~ForEach.IndexedArray**](./foreach-indexedarray.md)
  - `<array :pg.array.*`
  - `>index :pg.uint`
  - `>item :*`
- [**~ForEach.IndexedSet**](./foreach-indexedset.md)
  - `<set :pg.set.*`
  - `>index :pg.uint`
  - `>item :*`

---

## Overview

ForEach operators are the **most common unpack operators**, used to iterate over collections and expand each element into the iteration scope.

**Common pattern:**
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   // Process $element in iteration scope
   [r] $result << \|Process <input << $element

   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

---

## Collection Operators

### Basic Iteration
- [~ForEach.Array](./foreach-array.md) - Most common, iterate array elements
- [~ForEach.Set](./foreach-set.md) - Iterate set elements (unordered)

### Serial Iteration
- [~ForEach.Serial](./foreach-serial.md) - Iterate all fields with paths
- [~ForEach.SerialArray](./foreach-serialarray.md) - Iterate array at specific path

### Indexed Iteration
- [~ForEach.IndexedArray](./foreach-indexedarray.md) - Array with index
- [~ForEach.IndexedSet](./foreach-indexedset.md) - Set with index

---

## Comparison Table

| Operator | Input | Outputs | Order | Use Case |
|----------|-------|---------|-------|----------|
| **~ForEach.Array** | Array | item | Sequential/Parallel | Most common |
| **~ForEach.Set** | Set | item | Unordered | Unique values |
| **~ForEach.Serial** | Serial | path, item | Field order | Dynamic structures |
| **~ForEach.SerialArray** | Serial + path | item | Array order | Nested arrays |
| **~ForEach.IndexedArray** | Array | index, item | Sequential/Parallel | Need position |
| **~ForEach.IndexedSet** | Set | index, item | Unordered | Need count |

---

## When to Use Each Operator

### Use ~ForEach.Array when:
- Processing arrays (most common case)
- Order matters or doesn't matter
- Don't need index

### Use ~ForEach.Set when:
- Processing unique values
- Order doesn't matter
- Set data structure

### Use ~ForEach.Serial when:
- Processing dynamic serial data
- Need field paths
- Structure is not fixed

### Use ~ForEach.SerialArray when:
- Array is nested in serial at specific path
- Need to iterate that array
- Know the path beforehand

### Use ~ForEach.IndexedArray when:
- Need element position
- Building indexed output
- Array processing with position

### Use ~ForEach.IndexedSet when:
- Need count of iterations
- Processing set with position tracking

---

## Common Patterns

### Pattern 1: Transform Array
```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $transformed << \|Transform <input << $element

   [v] *Into.Array
   [*] <item << $transformed
   [*] >array >> $results
```

### Pattern 2: Extract Field from Array of Serials
```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [r] $name :pg.string << $user.name

   [v] *Into.Array
   [*] <item << $name
   [*] >array >> $names
```

### Pattern 3: Indexed Processing
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item

   [r] $labeled :pg.string << \|U.String.Concat"{\"[\", $i, \"] \", $item}"

   [v] *String.Lines
   [*] <line << $labeled
   [*] >lines >> $output
```

### Pattern 4: Process Serial Fields
```polyglot
[r] ~ForEach.Serial
[~] <serial << $config
[~] >path >> $key
[~] >item >> $value

   [r] $line :pg.string << \|U.String.Concat"{$key, \" = \", $value}"

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $config_text
```

---

## Related Documentation

- [Unpack Operators Overview](../README.md)
- [Pack Operators](../../pack-operators/README.md)
- [Loop System](../../../language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
