---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: lower
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Lower"
summary: "API reference: |U.String.Lower"
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
# |U.String.Lower

**Convert string to lowercase**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Lower <string >result
```

**Inline:**
```polyglot
\|U.String.Lower"{$string}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String to convert

**Outputs:**
- `>result` :pg.string - Lowercase string

---

## Description

Converts all characters in a string to lowercase.

---

## Examples

### Basic Usage

```polyglot
[r] $lower :pg.string << \|U.String.Lower"{\"HELLO WORLD\"}"
```

**Output:** `$lower = "hello world"`

---

### Normalize for Lookup

```polyglot
[r] $user_input :pg.string << "UserName"
[r] $key :pg.string << \|U.String.Lower"{$user_input}"
// Use as case-insensitive key
```

**Output:** `$key = "username"`

---

## Common Patterns

### Pattern 1: Case-Insensitive Comparison
```polyglot
[r] $input_lower :pg.string << \|U.String.Lower"{$user_input}"
[r] $expected_lower :pg.string << \|U.String.Lower"{$expected}"

[y] $input_lower == $expected_lower
   // Match regardless of case
```

### Pattern 2: Normalize Keys
```polyglot
[r] $key :pg.string << \|U.String.Lower"{$field_name}"
```

---

## Related Pipelines

- [|U.String.Upper](./upper.md) - Convert to uppercase

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
