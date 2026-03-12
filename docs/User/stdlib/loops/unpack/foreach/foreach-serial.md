---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-serial"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.Serial
summary: "API reference: ~ForEach.Serial"
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
# ~ForEach.Serial

**Iterate over serial data fields with paths**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.Serial
[~] <serial :pg.serial
[~] >path :pg.string
[~] >item :*
```

---

## Parameters

**Inputs:**
- `<serial` :pg.serial - Serial data to iterate over

**Outputs:**
- `>path` :pg.string - Field path (e.g., `"users.alice.email"`)
- `>item` :* - Field value at that path

---

## Description

Iterates over all fields in a `:pg.serial` data structure, outputting both the **field path** and **field value** for each iteration. This allows processing dynamic or nested serial data where the structure is not known beforehand.

**Key characteristics:**
- **Outputs field paths** - Know where each value comes from
- **Dynamic iteration** - Works with any serial structure
- **Nested access** - Iterates all nested fields recursively

**Use when:**
- Processing configuration data
- Iterating JSON/YAML with unknown structure
- Need to know field paths
- Building key-value representations

---

## Examples

### Basic Usage

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

**Input:**
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

**Output:**
```
database.host = localhost
database.port = 5432
cache.enabled = true
```

---

### Filter by Path Pattern

```polyglot
[r] ~ForEach.Serial
[~] <serial << $data
[~] >path >> $field_path
[~] >item >> $value

   [r] $is_email :pg.bool << \|U.String.EndsWith"{$field_path, \".email\"}"

   [f] $is_email == #True
      [v] *Into.Array
      [*] <item << $value
      [*] >array >> $all_emails
```

**Collects all values where path ends with `.email`.**

---

### Convert Serial to Key-Value Array

```polyglot
[r] ~ForEach.Serial
[~] <serial << $settings
[~] >path >> $key
[~] >item >> $value

   [r] $kv :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $kv.key :pg.string << $key
   [r] $kv.value << $value

   [v] *Into.Array
   [*] <item << $kv
   [*] >array >> $key_value_pairs
```

**Output:**
```json
[
  {"key": "database.host", "value": "localhost"},
  {"key": "database.port", "value": 5432},
  {"key": "cache.enabled", "value": true}
]
```

---

### Build Environment Variables

```polyglot
[r] ~ForEach.Serial
[~] <serial << $env_config
[~] >path >> $key
[~] >item >> $value

   [r] $env_var :pg.string << \|U.String.Upper"{$key}"
   [r] $env_var_formatted :pg.string << \|U.String.Replace"{$env_var, \".\", \"_\"}"
   [r] $line :pg.string << \|U.String.Concat"{$env_var_formatted, \"=\", $value}"

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $env_file
```

**Input:**
```json
{
  "api": {
    "key": "abc123",
    "url": "https://api.example.com"
  }
}
```

**Output:**
```
API_KEY=abc123
API_URL=https://api.example.com
```

---

### Validate All Fields

```polyglot
[r] ~ForEach.Serial
[~] <serial << $user_input
[~] >path >> $field_path
[~] >item >> $value

   [z] $validated << \|ValidateField <path << $field_path <value << $value
      [!] !Validation.*
         [r] $error_msg :pg.string << \|U.String.Concat"{\"Invalid field: \", $field_path}"
         [v] *Into.Array
         [*] <item << $error_msg
         [*] >array >> $validation_errors
```

---

## Path Structure

**Paths use dot notation:**

| Serial Structure | Path Output |
|------------------|-------------|
| `{"name": "Alice"}` | `"name"` |
| `{"user": {"name": "Bob"}}` | `"user.name"` |
| `{"users": {"0": {"email": "..."}}}` | `"users.0.email"` |

**Nested fields:**
```polyglot
{
  "database": {
    "primary": {
      "host": "localhost"
    }
  }
}
```

**Iteration outputs:**
- Path: `"database.primary.host"`, Item: `"localhost"`

---

## Empty Serial Handling

**Empty serial produces no iterations:**

```polyglot
[r] ~ForEach.Serial
[~] <serial << $empty
[~] >path >> $key
[~] >item >> $value

   [v] *Into.Array
   [*] <item << $key
   [*] >array >> $result
```

**Input:** `$empty = {}`
**Output:** `$result = []`

---

## Type Inference

**Path is always `:pg.string`:**

```polyglot
[~] >path :pg.string
```

**Item type is dynamic:**

```polyglot
[~] >item :*          // Can be any type
```

---

## Nested Iteration

**All nested fields are iterated:**

```polyglot
[r] ~ForEach.Serial
[~] <serial << $nested
[~] >path >> $field_path
[~] >item >> $value
```

**Input:**
```json
{
  "a": {
    "b": {
      "c": 123
    }
  }
}
```

**Iterations:**
1. Path: `"a.b.c"`, Item: `123`

**Note:** Intermediate objects (`a`, `a.b`) are **not** iterated, only **leaf values**.

---

## Common Patterns

### Pattern 1: Configuration to Text
```polyglot
[r] ~ForEach.Serial
[~] <serial << $config
[~] >path >> $key
[~] >item >> $value
   [r] $line :pg.string << \|U.String.Concat"{$key, \" = \", $value}"
   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $config_file
```

### Pattern 2: Filter by Path Prefix
```polyglot
[r] ~ForEach.Serial
[~] <serial << $data
[~] >path >> $field_path
[~] >item >> $value

   [r] $starts_with_db :pg.bool << \|U.String.StartsWith"{$field_path, \"database.\"}"

   [f] $starts_with_db == #True
      [v] *Into.Array
      [*] <item << $value
      [*] >array >> $db_values
```

### Pattern 3: Build New Serial with Transform
```polyglot
[r] ~ForEach.Serial
[~] <serial << $source
[~] >path >> $field_path
[~] >item >> $value

   [r] $transformed << \|Transform <input << $value

   [v] *Into.Serial
   [*] <path << $field_path
   [*] <item << $transformed
   [*] >serial >> $transformed_data
```

### Pattern 4: Count Fields
```polyglot
[r] ~ForEach.Serial
[~] <serial << $data
[~] >path >> $field_path
[~] >item >> $value

   [r] $one :pg.int << 1

   [v] *Math.Sum
   [*] <item << $one
   [*] >sum >> $field_count
```

---

## Performance

**Time Complexity:** O(n) where n = total number of leaf fields

**Space Complexity:** O(d) where d = maximum depth for path construction

**Path construction:** Each path is built as string during iteration

---

## Comparison with Other Operators

| Operator | Input | Path Output | Use Case |
|----------|-------|-------------|----------|
| **~ForEach.Serial** | Serial | Yes (all fields) | Dynamic structure |
| **~ForEach.SerialArray** | Serial + path | No | Specific array |
| **~ForEach.Array** | Array | No | Fixed array |

**When to use ~ForEach.Serial:**
- Don't know structure beforehand
- Need field paths
- Processing all fields
- Configuration/settings data

**When to use ~ForEach.SerialArray:**
- Know specific array path
- Don't need path output
- Processing array within serial

---

## Related Operators

- [~ForEach.SerialArray](./foreach-serialarray.md) - Iterate array at specific path
- [~ForEach.Array](./foreach-array.md) - Standard array iteration
- [*Into.Serial](../../pack-operators/collection-building/into/into-serial.md) - Build serial with paths

---

## See Also

- [Serial Load Block](../../../User/language/advanced/serial-load-block.md)
- [Loop System](../../../User/language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
