---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: String Utilities (|U.String.*)
summary: API reference: String Utilities (|U.String.*)
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
# String Utilities (|U.String.*)

**String manipulation and processing**

---

## Pipelines Tree

**|U.String.\***
- [**|U.String.Concat**](./concat.md) - Concatenate strings
- [**|U.String.Split**](./split.md) - Split by delimiter
- [**|U.String.Upper**](./upper.md) - Convert to uppercase
- [**|U.String.Lower**](./lower.md) - Convert to lowercase
- [**|U.String.Trim**](./trim.md) - Remove whitespace
- [**|U.String.Length**](./length.md) - Get string length
- [**|U.String.Substring**](./substring.md) - Extract substring
- [**|U.String.Replace**](./replace.md) - Replace occurrences

---

## Overview

String utilities provide common string manipulation operations for text processing, formatting, and analysis.

**Total:** 8 string utility pipelines

---

## Quick Reference

| Pipeline | Purpose | Example | Result |
|----------|---------|---------|--------|
| `\|U.String.Concat` | Join strings | `\|U.String.Concat"{\"Hello\", \" \", \"World\"}"` | "Hello World" |
| `\|U.String.Split` | Split string | `\|U.String.Split"{\"a,b,c\", \",\"}"` | ["a","b","c"] |
| `\|U.String.Upper` | To uppercase | `\|U.String.Upper"{\"hello\"}"` | "HELLO" |
| `\|U.String.Lower` | To lowercase | `\|U.String.Lower"{\"HELLO\"}"` | "hello" |
| `\|U.String.Trim` | Remove whitespace | `\|U.String.Trim"{\" hello \"}"` | "hello" |
| `\|U.String.Length` | String length | `\|U.String.Length"{\"hello\"}"` | 5 |
| `\|U.String.Substring` | Extract portion | `\|U.String.Substring"{\"hello\", 1, 3}"` | "ell" |
| `\|U.String.Replace` | Replace text | `\|U.String.Replace"{\"hello\", \"l\", \"L\"}"` | "heLLo" |

---

## Common Patterns

### Text Cleaning
```polyglot
[r] $cleaned :pg.string << \|U.String.Trim"{$raw_input}"
[r] $normalized :pg.string << \|U.String.Lower"{$cleaned}"
```

### String Building
```polyglot
[r] $full_name :pg.string << \|U.String.Concat"{$first, \" \", $last}"
[r] $greeting :pg.string << \|U.String.Concat"{\"Hello, \", $full_name, \"!\"}"
```

### Parsing
```polyglot
[r] $parts :pg.array.pg.string << \|U.String.Split"{$csv_line, \",\"}"
[r] ~ForEach.Array
[~] <array << $parts
[~] >item >> $part
   [r] $trimmed :pg.string << \|U.String.Trim"{$part}"
   [v] *Into.Array
   [*] <item << $trimmed
   [*] >array >> $clean_parts
```

### Case Normalization
```polyglot
[r] $key :pg.string << \|U.String.Lower"{$user_input}"
// Use lowercase key for case-insensitive lookups
```

---

## Pipeline Composition

**String utilities work well in pipelines:**
```polyglot
[r] \|U.String.Trim \|> \|U.String.Lower              // Chain Trim → Lower
[|] <input:pg.string << $raw_input                   // Input to Trim
[|] >trimmed:pg.string >> <input                     // Trim output → Lower input
[|] \|> \|U.String.Replace                            // Chain Lower → Replace
[|] >lowered:pg.string >> <input                     // Lower output → Replace input
[|] <old:pg.string << " "                            // Replace parameter: old
[|] <new:pg.string << "_"                            // Replace parameter: new
[|] \|>                                               // End chain
[|] >result:pg.string >> $result                     // Capture Replace output
```

---

## Related Operators

- [*String.Concat](../../pack-operators/collection-building/string/string-concat.md) - Concatenate from iterations
- [*String.Lines](../../pack-operators/collection-building/string/string-lines.md) - Join with newlines

---

## See Also

- [Utilities Overview](../README.md)
- [Standard Library](../../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
