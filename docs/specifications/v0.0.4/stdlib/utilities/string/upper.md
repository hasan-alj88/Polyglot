---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: upper
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Upper"
summary: "API reference: |U.String.Upper"
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
# |U.String.Upper

**Convert string to uppercase**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Upper <string >result
```

**Inline:**
```polyglot
\|U.String.Upper"{$string}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String to convert

**Outputs:**
- `>result` :pg.string - Uppercase string

---

## Description

Converts all characters in a string to uppercase.

---

## Examples

### Basic Usage

```polyglot
[r] $upper :pg.string << \|U.String.Upper"{\"hello world\"}"
```

**Output:** `$upper = "HELLO WORLD"`

---

### Normalize Key

```polyglot
[r] $user_input :pg.string << "Product_Name"
[r] $key :pg.string << \|U.String.Upper"{$user_input}"
```

**Output:** `$key = "PRODUCT_NAME"`

---

### Case-Insensitive Comparison

```polyglot
[r] $input1_upper :pg.string << \|U.String.Upper"{$input1}"
[r] $input2_upper :pg.string << \|U.String.Upper"{$input2}"

[y] $input1_upper == $input2_upper
   // Strings match (case-insensitive)
```

---

## Common Patterns

### Pattern 1: Normalize for Comparison
```polyglot
[r] $normalized :pg.string << \|U.String.Upper"{$user_input}"
```

### Pattern 2: Format Headers
```polyglot
[r] $header :pg.string << \|U.String.Upper"{$title}"
```

---

## Related Pipelines

- [|U.String.Lower](./lower.md) - Convert to lowercase

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
