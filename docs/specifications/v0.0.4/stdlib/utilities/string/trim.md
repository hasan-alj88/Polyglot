---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: trim
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Trim"
summary: "API reference: |U.String.Trim"
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
# |U.String.Trim

**Remove leading and trailing whitespace**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Trim <string >result
```

**Inline:**
```polyglot
\|U.String.Trim"{$string}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String to trim

**Outputs:**
- `>result` :pg.string - Trimmed string

---

## Description

Removes leading and trailing whitespace (spaces, tabs, newlines) from a string. Does not remove whitespace in the middle of the string.

---

## Examples

### Basic Usage

```polyglot
[r] $trimmed :pg.string << \|U.String.Trim"{\"  hello  \"}"
```

**Output:** `$trimmed = "hello"`

---

### Clean User Input

```polyglot
[r] $user_input :pg.string << "  john.doe@example.com  \\n"
[r] $email :pg.string << \|U.String.Trim"{$user_input}"
```

**Output:** `$email = "john.doe@example.com"`

---

### Process CSV Fields

```polyglot
[r] $csv :pg.string << "apple , banana , cherry"
[r] $parts :pg.array.pg.string << \|U.String.Split"{$csv, \",\"}"

[r] ~ForEach.Array
[~] <array << $parts
[~] >item >> $part
   [r] $trimmed :pg.string << \|U.String.Trim"{$part}"
   [v] *Into.Array
   [*] <item << $trimmed
   [*] >array >> $clean_parts
```

**Output:** `$clean_parts = ["apple", "banana", "cherry"]`

---

## Whitespace Removed

**Removes:**
- Spaces (` `)
- Tabs (`\t`)
- Newlines (`\n`)
- Carriage returns (`\r`)

**Keeps:**
- Internal whitespace between words

```polyglot
[r] $result :pg.string << \|U.String.Trim"{\"  hello world  \"}"
```

**Output:** `$result = "hello world"` (space between words preserved)

---

## Common Patterns

### Pattern 1: Clean Input
```polyglot
[r] $clean :pg.string << \|U.String.Trim"{$user_input}"
```

### Pattern 2: Normalize Before Comparison
```polyglot
[r] $input_clean :pg.string << \|U.String.Trim"{$input}"
[r] $expected_clean :pg.string << \|U.String.Trim"{$expected}"

[y] $input_clean == $expected_clean
   // Compare without whitespace
```

### Pattern 3: Pipeline with Other Operations
```polyglot
[r] \|U.String.Trim \|> \|U.String.Lower      // Chain Trim → Lower
[|] <input:pg.string << $raw_input           // Input to Trim
[|] >trimmed:pg.string >> <input             // Trim output → Lower input
[|] \|>                                       // End chain
[|] >result:pg.string >> $result             // Capture Lower output
```

---

## Related Pipelines

- [|U.String.Replace](./replace.md) - For more complex whitespace removal

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
