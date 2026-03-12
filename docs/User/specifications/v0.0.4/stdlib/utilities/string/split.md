---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: split
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Split"
summary: "API reference: |U.String.Split"
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
# |U.String.Split

**Split string by delimiter**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Split <string <delimiter >result
```

**Inline:**
```polyglot
\|U.String.Split"{$string, $delimiter}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String to split
- `<delimiter` :pg.string - Delimiter to split on

**Outputs:**
- `>result` :pg.array.pg.string - Array of string parts

---

## Description

Splits a string into an array of substrings using the specified delimiter.

**Operation:** Breaks string at each occurrence of delimiter

---

## Examples

### Basic Usage - CSV

```polyglot
[r] $csv :pg.string << "apple,banana,cherry"
[r] $parts :pg.array.pg.string << \|U.String.Split"{$csv, \",\"}"
```

**Output:** `$parts = ["apple", "banana", "cherry"]`

---

### Split by Space

```polyglot
[r] $sentence :pg.string << "Hello World Example"
[r] $words :pg.array.pg.string << \|U.String.Split"{$sentence, \" \"}"
```

**Output:** `$words = ["Hello", "World", "Example"]`

---

### Split Path

```polyglot
[r] $path :pg.string << "/home/user/documents"
[r] $parts :pg.array.pg.string << \|U.String.Split"{$path, \"/\"}"
```

**Output:** `$parts = ["", "home", "user", "documents"]`

**Note:** Leading delimiter creates empty first element.

---

### Split Lines

```polyglot
[r] $multi_line :pg.string << "Line 1\\nLine 2\\nLine 3"
[r] $lines :pg.array.pg.string << \|U.String.Split"{$multi_line, \"\\n\"}"
```

**Output:** `$lines = ["Line 1", "Line 2", "Line 3"]`

---

### Process Split Results

```polyglot
[r] $csv_line :pg.string << "John,Doe,30"
[r] $fields :pg.array.pg.string << \|U.String.Split"{$csv_line, \",\"}"

[r] $first_name :pg.string << $fields.0
[r] $last_name :pg.string << $fields.1
[r] $age :pg.string << $fields.2
```

---

## Common Patterns

### Pattern 1: Parse CSV
```polyglot
[r] $parts :pg.array.pg.string << \|U.String.Split"{$csv_line, \",\"}"
```

### Pattern 2: Extract Path Components
```polyglot
[r] $path_parts :pg.array.pg.string << \|U.String.Split"{$file_path, \"/\"}"
[r] $filename :pg.string << $path_parts.-1  // Last element
```

### Pattern 3: Process Each Part
```polyglot
[r] $parts :pg.array.pg.string << \|U.String.Split"{$data, \";\"}"
[r] ~ForEach.Array
[~] <array << $parts
[~] >item >> $part
   [r] $trimmed :pg.string << \|U.String.Trim"{$part}"
   [v] *Into.Array
   [*] <item << $trimmed
   [*] >array >> $clean_parts
```

---

## Empty Strings

**Consecutive delimiters create empty strings:**
```polyglot
[r] $result :pg.array.pg.string << \|U.String.Split"{\"a,,b\", \",\"}"
```

**Output:** `$result = ["a", "", "b"]`

---

## Related Pipelines

- [|U.String.Concat](./concat.md) - Join strings
- [|U.String.Trim](./trim.md) - Clean split results

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
