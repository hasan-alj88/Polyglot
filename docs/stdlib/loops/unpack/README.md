tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Unpack Operators (~*)
summary: "API reference: Unpack Operators (~*)"
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
topic: Unpack Operators (~*)
summary: API reference: Unpack Operators (~*)
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
# Unpack Operators (~*)

**Expand collections and ranges into iteration scope**

---

## Operators Tree

**~\* (Unpack Operators)**
- **~ForEach.\***
  - [**~ForEach.Array**](./foreach/foreach-array.md)
    - `<array :pg.array.*`
    - `>item :*`
  - [**~ForEach.Set**](./foreach/foreach-set.md)
    - `<set :pg.set.*`
    - `>item :*`
  - [**~ForEach.Serial**](./foreach/foreach-serial.md)
    - `<serial :pg.serial`
    - `>path :pg.string`
    - `>item :*`
  - [**~ForEach.SerialArray**](./foreach/foreach-serialarray.md)
    - `<serial :pg.serial`
    - `<path :pg.string`
    - `>item :pg.serial`
  - [**~ForEach.IndexedArray**](./foreach/foreach-indexedarray.md)
    - `<array :pg.array.*`
    - `>index :pg.uint`
    - `>item :*`
  - [**~ForEach.IndexedSet**](./foreach/foreach-indexedset.md)
    - `<set :pg.set.*`
    - `>index :pg.uint`
    - `>item :*`
- **~Zip.\***
  - [**~Zip.Arrays**](./zip/zip-arrays.md)
    - `<arrays :pg.array.serial`
    - `>items :pg.array.serial`
  - [**~Zip.Sets**](./zip/zip-sets.md)
    - `<sets :pg.set.serial`
    - `>items :pg.set.serial`
- **~Iter.\***
  - [**~Iter.Range**](./iter/iter-range.md)
    - `<from :pg.int`
    - `<to :pg.int`
    - `>index :pg.int`
  - [**~Iter.SlidingWindow**](./iter/iter-slidingwindow.md)
    - `<array :pg.array.*`
    - `<window :pg.int`
    - `>item :pg.array`

---

## Overview

Unpack operators (`~*`) expand collections and ranges from the **main scope** into **iteration scope**, enabling loops and parallel processing. Each iteration receives unpacked values through the `>` outputs.

**Total operators:** 10

---

## ForEach Operators

Iterate over collection elements.

- [~ForEach.Array](./foreach/foreach-array.md) - Iterate array elements
- [~ForEach.Set](./foreach/foreach-set.md) - Iterate set elements
- [~ForEach.Serial](./foreach/foreach-serial.md) - Iterate serial fields with paths
- [~ForEach.SerialArray](./foreach/foreach-serialarray.md) - Iterate serial array at path
- [~ForEach.IndexedArray](./foreach/foreach-indexedarray.md) - Iterate array with index
- [~ForEach.IndexedSet](./foreach/foreach-indexedset.md) - Iterate set with index

**See:** [ForEach Package](./foreach/README.md)

---

## Zip Operators

Combine multiple collections element-wise.

- [~Zip.Arrays](./zip/zip-arrays.md) - Zip multiple arrays together
- [~Zip.Sets](./zip/zip-sets.md) - Zip multiple sets together

**See:** [Zip Package](./zip/README.md)

---

## Iter Operators

Generate iteration sequences.

- [~Iter.Range](./iter/iter-range.md) - Iterate numeric range
- [~Iter.SlidingWindow](./iter/iter-slidingwindow.md) - Iterate with sliding window

**See:** [Iter Package](./iter/README.md)

---

## Common Pattern

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   // Process each $element in iteration scope
   [r] $result << \|ProcessItem <input << $element

   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results
```

---

## Unpack Marker

All unpack operators use the `[~]` marker for parameters:

```polyglot
[r] ~ForEach.Array
[~] <array << $collection      // Input from main scope
[~] >item >> $element          // Output to iteration scope
```

---

## Sequential vs Parallel

**Sequential unpacking:**
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   // Iterations run in order
```

**Parallel unpacking:**
```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   // Iterations run concurrently
[v]                           // Join point
```

---

## Use Cases

### Data Transformation
```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user
   [r] $email :pg.string << $user.email
   [v] *Into.Array
   [*] <item << $email
   [*] >array >> $emails
```

### Filtering
```polyglot
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [y] $num > 10
      [v] *Into.Array
      [*] <item << $num
      [*] >array >> $filtered
```

### Indexed Processing
```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item
   [r] $labeled :pg.string << \|U.String.Concat"{$i, \": \", $item}"
   [v] *String.Lines
   [*] <line << $labeled
   [*] >lines >> $output
```

### Combining Collections
```polyglot
[p] ~Zip.Arrays
[~] <<< $names
[~] <<< $ages
[~] >>> $name
[~] >>> $age
   // Process paired values
[v]
```

---

## Related Documentation

- [Pack Operators](../pack-operators/README.md)
- [Loop System](../../language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../README.md)
