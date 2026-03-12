---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Zip Unpack Operators
summary: API reference: Zip Unpack Operators
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
# Zip Unpack Operators

**Combine multiple collections element-wise**

---

## Operators Tree

**~Zip.\***
- [**~Zip.Arrays**](./zip-arrays.md)
  - `<arrays :pg.array.serial`
  - `>items :pg.array.serial`
- [**~Zip.Sets**](./zip-sets.md)
  - `<sets :pg.set.serial`
  - `>items :pg.set.serial`

---

## Overview

Zip operators combine multiple collections element-wise, creating iterations where each iteration receives corresponding elements from all input collections.

**Common pattern:**
```polyglot
[p] ~Zip.Arrays
[~] <<< $names
[~] <<< $ages
[~] >>> $name
[~] >>> $age

   // Process paired values
   [r] $person :pg.string << \|U.String.Concat"{$name, \" is \", $age}"

   [v] *Into.Array
   [*] <item << $person
   [*] >array >> $descriptions
[v]
```

---

## Array Zipping

- [~Zip.Arrays](./zip-arrays.md) - Zip multiple arrays together

**Use when:**
- Combining parallel arrays
- Processing corresponding elements
- Building paired data

---

## Set Zipping

- [~Zip.Sets](./zip-sets.md) - Zip multiple sets together

**Use when:**
- Combining sets element-wise
- Processing unique value pairs
- Order doesn't matter

---

## Variadic Syntax

Both Zip operators support **variadic inputs/outputs** with shortcut syntax:

**Full syntax:**
```polyglot
[p] ~Zip.Arrays
[~] <arrays.0 << $array0
[~] <arrays.1 << $array1
[~] >items.0 >> $item0
[~] >items.1 >> $item1
[v]
```

**Shortcut syntax:**
```polyglot
[p] ~Zip.Arrays
[~] <<< $array0
[~] <<< $array1
[~] >>> $item0
[~] >>> $item1
[v]
```

**Operators:**
- `<<<` - Implied input (replaces `<arrays.N <<`)
- `>>>` - Implied output (replaces `>items.N >>`)

---

## Comparison Table

| Operator | Collection Type | Order | Use Case |
|----------|-----------------|-------|----------|
| **~Zip.Arrays** | Arrays | Deterministic | Parallel arrays |
| **~Zip.Sets** | Sets | Non-deterministic | Unique value pairs |

---

## Common Patterns

### Pattern 1: Combine Two Arrays
```polyglot
[p] ~Zip.Arrays
[~] <<< $usernames
[~] <<< $passwords
[~] >>> $username
[~] >>> $password
   [r] $credential << \|CreateCredential <user << $username <pass << $password
   [v] *Into.Array
   [*] <item << $credential
   [*] >array >> $credentials
[v]
```

### Pattern 2: Build Key-Value Pairs
```polyglot
[p] ~Zip.Arrays
[~] <<< $keys
[~] <<< $values
[~] >>> $key
[~] >>> $value
   [r] $kv :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $kv.key << $key
   [r] $kv.value << $value
   [v] *Into.Array
   [*] <item << $kv
   [*] >array >> $pairs
[v]
```

### Pattern 3: Multi-Array Zip
```polyglot
[p] ~Zip.Arrays
[~] <<< $names
[~] <<< $ages
[~] <<< $cities
[~] >>> $name
[~] >>> $age
[~] >>> $city
   [r] $info :pg.string << \|U.String.Concat"{$name, \", age \", $age, \", from \", $city}"
   [v] *String.Lines
   [*] <line << $info
   [*] >lines >> $directory
[v]
```

---

## Related Documentation

- [Unpack Operators Overview](../README.md)
- [ForEach Operators](../foreach/README.md)
- [Loop System](../../../User/language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
