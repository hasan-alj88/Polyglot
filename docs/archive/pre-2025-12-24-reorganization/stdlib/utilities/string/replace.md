---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: replace
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Replace"
summary: "API reference: |U.String.Replace"
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
# |U.String.Replace

**Replace all occurrences of substring**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Replace <string <old <new >result
```

**Inline:**
```polyglot
\|U.String.Replace"{$string, $old, $new}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - Source string
- `<old` :pg.string - Substring to find
- `<new` :pg.string - Replacement string

**Outputs:**
- `>result` :pg.string - String with replacements

---

## Description

Replaces all occurrences of a substring with another string.

**Replaces all matches**, not just the first one.

---

## Examples

### Basic Usage

```polyglot
[r] $result :pg.string << \|U.String.Replace"{\"hello world\", \"world\", \"there\"}"
```

**Output:** `$result = "hello there"`

---

### Replace Multiple Occurrences

```polyglot
[r] $result :pg.string << \|U.String.Replace"{\"hello hello hello\", \"hello\", \"hi\"}"
```

**Output:** `$result = "hi hi hi"`

---

### Replace Spaces with Underscores

```polyglot
[r] $slug :pg.string << \|U.String.Replace"{\"My Blog Post\", \" \", \"_\"}"
```

**Output:** `$slug = "My_Blog_Post"`

---

### Remove Characters

```polyglot
[r] $cleaned :pg.string << \|U.String.Replace"{\"a-b-c-d\", \"-\", \"\"}"
```

**Output:** `$cleaned = "abcd"`

---

### Normalize Line Endings

```polyglot
[r] $unix_style :pg.string << \|U.String.Replace"{$windows_text, \"\\r\\n\", \"\\n\"}"
```

---

## Case Sensitivity

**Replace is case-sensitive:**
```polyglot
[r] $result :pg.string << \|U.String.Replace"{\"Hello hello HELLO\", \"hello\", \"hi\"}"
```

**Output:** `$result = "Hello hi HELLO"` (only lowercase "hello" replaced)

**For case-insensitive, convert to lower first:**
```polyglot
[r] $lower :pg.string << \|U.String.Lower"{$text}"
[r] $replaced :pg.string << \|U.String.Replace"{$lower, \"hello\", \"hi\"}"
```

---

## Common Patterns

### Pattern 1: Create URL Slug
```polyglot
[r] \|U.String.Lower \|> \|U.String.Replace      // Chain Lower → Replace
[|] <input:pg.string << $title                  // Input to Lower
[|] >lowered:pg.string >> <input                // Lower output → Replace input
[|] <old:pg.string << \" \"                     // Replace parameter: old (space)
[|] <new:pg.string << \"-\"                     // Replace parameter: new (dash)
[|] \|>                                          // End chain
[|] >result:pg.string >> $slug                  // Capture Replace output
```

### Pattern 2: Remove Characters
```polyglot
[r] $no_dashes :pg.string << \|U.String.Replace"{$phone, \"-\", \"\"}"
```

### Pattern 3: Sanitize Input
```polyglot
[r] $safe :pg.string << \|U.String.Replace"{$input, \"<\", \"&lt;\"}"
[r] $safer :pg.string << \|U.String.Replace"{$safe, \">\", \"&gt;\"}"
```

---

## No Match

**If old string not found, returns original:**
```polyglot
[r] $result :pg.string << \|U.String.Replace"{\"hello\", \"xyz\", \"abc\"}"
```

**Output:** `$result = "hello"` (unchanged)

---

## Related Pipelines

- [|U.String.Split](./split.md) - Split string
- [|U.String.Trim](./trim.md) - Remove whitespace

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
