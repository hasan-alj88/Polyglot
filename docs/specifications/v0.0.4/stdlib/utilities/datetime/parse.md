---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: parse
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Parse"
summary: "API reference: |U.DT.Parse"
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
# |U.DT.Parse

**Parse string to datetime**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Parse <string <format >result
```

**Inline:**
```polyglot
\|U.DT.Parse"{$string, $format}"
```

---

## Parameters

**Inputs:**
- `<string` :pg.string - Date/time string to parse
- `<format` :pg.string - Format pattern

**Outputs:**
- `>result` :pg.datetime - Parsed datetime

---

## Description

Parses a string into a `:pg.datetime` value using the specified format pattern.

**Produces error `!DT.ParseFailed` if:**
- String doesn't match format
- Invalid date values (e.g., month 13, day 32)

---

## Format Tokens

**Date:**
- `YYYY` - 4-digit year (e.g., 2025)
- `YY` - 2-digit year (e.g., 25)
- `MM` - 2-digit month with leading zero (01-12)
- `M` - Month without leading zero (1-12)
- `DD` - 2-digit day with leading zero (01-31)
- `D` - Day without leading zero (1-31)

**Time:**
- `HH` - 2-digit hour with leading zero (00-23)
- `H` - Hour without leading zero (0-23)
- `mm` - 2-digit minute with leading zero (00-59)
- `m` - Minute without leading zero (0-59)
- `ss` - 2-digit second with leading zero (00-59)
- `s` - Second without leading zero (0-59)

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
```

**Output:** DateTime representing December 15, 2025

---

### Parse US Format

```polyglot
[r] $us_date :pg.string << "12/15/2025"
[r] $dt :pg.datetime << \|U.DT.Parse"{$us_date, \"MM/DD/YYYY\"}"
```

---

### Parse with Time

```polyglot
[r] $timestamp :pg.string << "2025-12-15 14:30:00"
[r] $dt :pg.datetime << \|U.DT.Parse"{$timestamp, \"YYYY-MM-DD HH:mm:ss\"}"
```

---

### Parse ISO 8601

```polyglot
[r] $iso :pg.string << "2025-12-15T14:30:00"
[r] $dt :pg.datetime << \|U.DT.Parse"{$iso, \"YYYY-MM-DDTHH:mm:ss\"}"
```

---

### Handle Parse Error

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$user_input, \"YYYY-MM-DD\"}"

[!] !DT.ParseFailed
   [r] !Validation.InvalidDate << "Please enter date in YYYY-MM-DD format"
```

---

## Common Formats

### ISO 8601
```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$str, \"YYYY-MM-DDTHH:mm:ss\"}"
```

### US Format
```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$str, \"MM/DD/YYYY\"}"
```

### European Format
```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$str, \"DD/MM/YYYY\"}"
```

### Time Only
```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$str, \"HH:mm:ss\"}"
```

---

## Common Patterns

### Pattern 1: Validate and Parse User Input

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$user_input, \"YYYY-MM-DD\"}"

[!] !DT.ParseFailed
   [r] !Validation.InvalidDateFormat << "Invalid date format"
```

### Pattern 2: Convert Between Formats

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$us_format, \"MM/DD/YYYY\"}"
[r] $iso :pg.string << \|U.DT.Format"{$dt, \"YYYY-MM-DD\"}"
```

### Pattern 3: Process CSV Date Fields

```polyglot
[r] ~ForEach.Array
[~] <array << $csv_rows
[~] >item >> $row
   [r] $date_field :pg.string << $row."date"
   [r] $parsed :pg.datetime << \|U.DT.Parse"{$date_field, \"MM/DD/YYYY\"}"

   [!] !DT.ParseFailed
      // Skip invalid rows
      [v] [r] [^]

   // Process valid date...
```

---

## Related Pipelines

- [|U.DT.Format](./format.md) - Format datetime to string
- [|U.DT.Now](./now.md) - Get current timestamp

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
