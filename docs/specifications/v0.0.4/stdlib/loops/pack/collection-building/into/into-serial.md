---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "into-serial"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Into.Serial"
summary: "API reference: *Into.Serial"
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
# *Into.Serial

**Collect iteration items into serial data structure with field paths**

**Category:** Collection Building > Into
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Into.Serial
[*] <path :pg.string              // field.path.like.this
[*] <item
[*] >serial
```

---

## Parameters

**Inputs:**
- `<path` :pg.string - Field path for this item (e.g., "users.0.name")
- `<item` - Item from iteration scope to collect

**Outputs:**
- `>serial` :pg.serial - Collected serial data structure in main scope

---

## Description

Collects each iteration's `<item` value into a `:pg.serial` data structure at the specified `<path`. This allows building **structured serial data** with explicit field paths.

**Use when:**
- Building nested data structures
- Creating JSON/YAML with specific structure
- Need control over field placement

---

## Examples

### Basic Usage with Paths

```polyglot
[p] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $element

   [r] $path :pg.string << \|U.String.Concat"{\"items.\", $i}"

   [v] *Into.Serial
   [*] <path << $path              // "items.0", "items.1", "items.2"
   [*] <item << $element
   [*] >serial >> $serial_data
```

**Output structure:**
```json
{
  "items": {
    "0": <element0>,
    "1": <element1>,
    "2": <element2>
  }
}
```

---

### Build Nested User Records

```polyglot
[p] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [r] $user_id :pg.string << $user.id
   [r] $path :pg.string << \|U.String.Concat"{\"users.\", $user_id, \".profile\"}"

   [v] *Into.Serial
   [*] <path << $path              // "users.user123.profile"
   [*] <item << $user.profile
   [*] >serial >> $user_data
```

**Output structure:**
```json
{
  "users": {
    "user123": {
      "profile": {...}
    },
    "user456": {
      "profile": {...}
    }
  }
}
```

---

### Create Configuration Object

```polyglot
[p] ~ForEach.Array
[~] <array << $config_entries
[~] >item >> $entry

   [r] $section :pg.string << $entry.section
   [r] $key :pg.string << $entry.key
   [r] $path :pg.string << \|U.String.Concat"{$section, \".\", $key}"

   [v] *Into.Serial
   [*] <path << $path              // "database.host", "database.port"
   [*] <item << $entry.value
   [*] >serial >> $config
```

**Output structure:**
```json
{
  "database": {
    "host": "localhost",
    "port": 5432
  },
  "cache": {
    "enabled": true
  }
}
```

---

### Build API Response

```polyglot
[p] ~ForEach.IndexedArray
[~] <array << $products
[~] >index >> $i
[~] >item >> $product

   [r] $path :pg.string << \|U.String.Concat"{\"products.\", $i, \".name\"}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $product.name
   [*] >serial >> $response

// Export to JSON
[r] $json :pg.string << \|JSON.Dump"{$response}"
```

---

## Path Syntax

**Field paths use dot notation:**

| Path | Resulting Structure |
|------|---------------------|
| `"users.alice"` | `{ users: { alice: <item> } }` |
| `"config.db.host"` | `{ config: { db: { host: <item> } } }` |
| `"items.0.name"` | `{ items: { 0: { name: <item> } } }` |
| `"tags.urgent"` | `{ tags: { urgent: <item> } } }` |

**Array-like access:**
```polyglot
"items.0"     // First item
"items.1"     // Second item
"items.2"     // Third item
```

**Nested paths:**
```polyglot
"users.alice.profile.email"
// Creates: { users: { alice: { profile: { email: <item> } } } }
```

---

## Comparison with *Into.Array

| Feature | *Into.Serial | *Into.Array |
|---------|--------------|-------------|
| **Structure** | Nested with paths | Flat array |
| **Inputs** | `<path>`, `<item>` | `<item>` |
| **Output** | `:pg.serial` | `:pg.array.T` |
| **Use Case** | Structured data | Simple collection |
| **Access** | By path | By index |

**When to use *Into.Serial:**
- Need nested structure
- Building JSON/YAML output
- Specific field placement required

**When to use *Into.Array:**
- Simple list of items
- Order matters
- Don't need nesting

---

## Dynamic Structure Building

```polyglot
{|} \|BuildDynamicReport
[|] <data :pg.array.pg.serial
[|] >report :pg.string

[t] \|T.Call
[W] \|W.Polyglot.Scope

   [p] ~ForEach.IndexedArray
   [~] <array << $data
   [~] >index >> $i
   [~] >item >> $record

      [r] $category :pg.string << $record.category
      [r] $path :pg.string << \|U.String.Concat"{$category, \".items.\", $i}"

      [v] *Into.Serial
      [*] <path << $path
      [*] <item << $record
      [*] >serial >> $structured_data

   [r] $json :pg.string << \|JSON.Dump"{$structured_data}"
   [|] >report << $json

{x}
```

---

## Type Handling

**Mixed types at different paths:**
```polyglot
[v] *Into.Serial
[*] <path << "user.name"
[*] <item << "Alice"              // String

[v] *Into.Serial
[*] <path << "user.age"
[*] <item << 30                   // Int

[v] *Into.Serial
[*] <path << "user.active"
[*] <item << #True       // Bool

// Result: { user: { name: "Alice", age: 30, active: true } }
```

---

## Performance

**Time Complexity:** O(n * p) where:
- n = number of iterations
- p = average path depth

**Space Complexity:** O(total_structure_size)

**Path Processing:**
- Paths are parsed once per iteration
- Deep nesting adds overhead
- Shallow paths are more efficient

---

## Common Patterns

### Pattern 1: Indexed Collection

```polyglot
[p] ~ForEach.IndexedArray
[~] <array << $items
[~] >index >> $i
[~] >item >> $item

   [r] $path :pg.string << \|U.String.Concat"{\"data.\", $i}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $item
   [*] >serial >> $result
```

### Pattern 2: Grouped by Category

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $category :pg.string << $item.category
   [r] $id :pg.string << $item.id
   [r] $path :pg.string << \|U.String.Concat"{$category, \".\", $id}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $item
   [*] >serial >> $grouped
```

### Pattern 3: Multi-level Nesting

```polyglot
[p] ~ForEach.Array
[~] <array << $logs
[~] >item >> $log

   [r] $date :pg.string << $log.date
   [r] $time :pg.string << $log.time
   [r] $level :pg.string << $log.level
   [r] $path :pg.string << \|U.String.Concat"{$date, \".\", $time, \".\", $level}"

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $log.message
   [*] >serial >> $structured_logs
```

---

## Related Operators

- [*Into.Array](./into-array.md) - Collect into flat array
- [*Into.Set](./into-set.md) - Collect unique values

---

## See Also

- [Loop System](../../../../language/advanced/loop-system.md)
- [Serial Load Block](../../../../language/advanced/serial-load-block.md)
- [Data Utilities](../../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
