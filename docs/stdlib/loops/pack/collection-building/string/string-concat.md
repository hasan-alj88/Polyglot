---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "string-concat"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*String.Concat"
summary: "API reference: *String.Concat"
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
# *String.Concat

**Concatenate strings from iterations**

**Category:** Collection Building > String
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *String.Concat
[*] <string :pg.string
[*] >concatenated :pg.string
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - String from iteration scope to concatenate

**Outputs:**
- `>concatenated` :pg.string - Concatenated result in main scope

---

## Description

Concatenates all iteration strings into a single string in the main scope. Strings are joined **without any separator** - for newline-separated output, use [*String.Lines](./string-lines.md).

**Order:**
- Sequential `[r]` loops: Concatenation in input order
- Parallel `[p]` loops: Order non-deterministic

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $words
[~] >item >> $word

   [v] *String.Concat
   [*] <string << $word
   [*] >concatenated >> $sentence
```

**Input:** `$words = ["Hello", " ", "World"]`
**Output:** `$sentence = "HelloWorld"` (no separator)

---

### Build Sentence

```polyglot
[r] ~ForEach.Array
[~] <array << $parts
[~] >item >> $part

   [v] *String.Concat
   [*] <string << $part
   [*] >concatenated >> $full_text
```

**Input:** `$parts = ["The", " ", "quick", " ", "brown", " ", "fox"]`
**Output:** `$full_text = "The quick brown fox"`

---

### Concatenate with Transformation

```polyglot
[r] ~ForEach.Array
[~] <array << $names
[~] >item >> $name

   [r] $uppercase :pg.string << \|U.String.Upper"{$name}"

   [v] *String.Concat
   [*] <string << $uppercase
   [*] >concatenated >> $all_caps
```

**Input:** `$names = ["alice", "bob", "charlie"]`
**Output:** `$all_caps = "ALICEBOBCHARLIE"`

---

### Build Path

```polyglot
[r] ~ForEach.Array
[~] <array << $path_parts
[~] >item >> $part

   [v] *String.Concat
   [*] <string << $part
   [*] >concatenated >> $full_path
```

**Input:** `$path_parts = ["/", "home", "/", "user", "/", "file.txt"]`
**Output:** `$full_path = "/home/user/file.txt"`

---

### Generate HTML

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $li :pg.string << \|U.String.Concat"{\"<li>\", $item, \"</li>\"}"

   [v] *String.Concat
   [*] <string << $li
   [*] >concatenated >> $html_list
```

**Input:** `$items = ["Item 1", "Item 2", "Item 3"]`
**Output:** `$html_list = "<li>Item 1</li><li>Item 2</li><li>Item 3</li>"`

---

## No Separator

**Important:** This operator concatenates **without** any separator.

**To add separators:**
```polyglot
[r] ~ForEach.Array
[~] <array << $words
[~] >item >> $word

   [r] $with_separator :pg.string << \|U.String.Concat"{$word, \", \"}"

   [v] *String.Concat
   [*] <string << $with_separator
   [*] >concatenated >> $result
```

**Input:** `$words = ["apple", "banana", "cherry"]`
**Output:** `$result = "apple, banana, cherry, "` (note trailing separator)

**Better approach for separated strings:** Use dedicated join utility or [*String.Lines](./string-lines.md) for newlines.

---

## Comparison with Other Operators

| Operator | Separator | Use Case |
|----------|-----------|----------|
| `*String.Concat` | None | Direct concatenation |
| `*tring.Lines` | Newline (`\n`) | Multi-line text |
| `\|U.String.Concat` | Custom | Join with any separator |

---

## Common Patterns

### Pattern 1: Build Text from Parts

```polyglot
[r] ~ForEach.Array
[~] <array << $text_parts
[~] >item >> $part

   [v] *String.Concat
   [*] <string << $part
   [*] >concatenated >> $full_text
```

### Pattern 2: Generate Code/Markup

```polyglot
[r] ~ForEach.Array
[~] <array << $code_lines
[~] >item >> $line

   [v] *String.Concat
   [*] <string << $line
   [*] >concatenated >> $generated_code
```

### Pattern 3: Flatten String Array

```polyglot
[r] ~ForEach.Array
[~] <array << $string_array
[~] >item >> $str

   [v] *String.Concat
   [*] <string << $str
   [*] >concatenated >> $flattened
```

---

## Performance

**Time Complexity:** O(n * m) where:
- n = number of iterations
- m = average string length

**Space Complexity:** O(total_length) where total_length = sum of all string lengths

**Sequential vs Parallel:**
- Sequential `[r]`: Predictable order, strings concatenated left-to-right
- Parallel `[p]`: Non-deterministic order, may produce different results each run

---

## Related Operators

- [*String.Lines](./string-lines.md) - Concatenate with newlines
- [*Into.Array](../into/into-array.md) - Collect strings into array

---

## See Also

- [Loop System](../../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../../README.md)
- [String Utilities](../../../utilities/string/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
