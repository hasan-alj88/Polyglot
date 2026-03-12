---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: length
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Length"
summary: "API reference: |U.String.Length"
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
# |U.String.Length

**Get string length**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Length <string >result
```

**Inline:**
```polyglot
\|U.String.Length"{$string}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String to measure

**Outputs:**
- `>result` :pg.uint - Number of characters

---

## Description

Returns the number of characters in a string.

---

## Examples

### Basic Usage

```polyglot
[r] $length :pg.uint << \|U.String.Length"{\"hello\"}"
```

**Output:** `$length = 5`

---

### Check Empty String

```polyglot
[r] $len :pg.uint << \|U.String.Length"{$input}"

[f] $len == 0
   // String is empty
[^]
   // String has content
```

---

### Validate Input Length

```polyglot
[r] $username_len :pg.uint << \|U.String.Length"{$username}"

[f] $username_len < 3
   [r] !Validation.UsernameTooShort << "Username must be at least 3 characters"
```

---

## Common Patterns

### Pattern 1: Validation
```polyglot
[r] $len :pg.uint << \|U.String.Length"{$password}"
[f] $len >= 8
   // Password meets minimum length
```

### Pattern 2: Empty Check
```polyglot
[r] $is_empty :pg.bool << \|U.String.Length"{$str}" == 0
```

---

## Related Pipelines

- [|U.String.Substring](./substring.md) - Extract portion by length

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
