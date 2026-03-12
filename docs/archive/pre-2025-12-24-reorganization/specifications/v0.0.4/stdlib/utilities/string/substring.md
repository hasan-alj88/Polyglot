---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: substring
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Substring"
summary: "API reference: |U.String.Substring"
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
# |U.String.Substring

**Extract portion of string**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Substring <string <start <length >result
```

**Inline:**
```polyglot
\|U.String.Substring"{$string, $start, $length}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - Source string
- `<start` :pg.uint - Starting index (zero-based)
- `<length` :pg.uint - Number of characters to extract

**Outputs:**
- `>result` :pg.string - Extracted substring

---

## Description

Extracts a portion of a string starting at the specified index for the specified length.

**Zero-based indexing:** First character is at index 0.

---

## Examples

### Basic Usage

```polyglot
[r] $sub :pg.string << \|U.String.Substring"{\"hello world\", 0, 5}"
```

**Output:** `$sub = "hello"`

---

### Extract Middle

```polyglot
[r] $middle :pg.string << \|U.String.Substring"{\"hello world\", 6, 5}"
```

**Output:** `$middle = "world"`

---

### Extract Single Character

```polyglot
[r] $char :pg.string << \|U.String.Substring"{\"hello\", 1, 1}"
```

**Output:** `$char = "e"`

---

### Extract File Extension

```polyglot
[r] $filename :pg.string << "document.pdf"
[r] $len :pg.uint << \|U.String.Length"{$filename}"
[r] $ext_start :pg.uint << \|U.Math.Subtract"{$len, 3}"
[r] $extension :pg.string << \|U.String.Substring"{$filename, $ext_start, 3}"
```

**Output:** `$extension = "pdf"`

---

## Index Rules

**Zero-based:**
- Index 0 = first character
- Index 1 = second character

**Out of bounds:**
- If start >= string length, returns empty string
- If start + length > string length, returns from start to end

---

## Common Patterns

### Pattern 1: Extract Prefix
```polyglot
[r] $prefix :pg.string << \|U.String.Substring"{$string, 0, 3}"
```

### Pattern 2: Skip Prefix
```polyglot
[r] $without_prefix :pg.string << \|U.String.Substring"{$string, 3, $remaining_len}"
```

---

## Related Pipelines

- [|U.String.Length](./length.md) - Get string length
- [|U.String.Split](./split.md) - Split by delimiter

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
