---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: format
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Format"
summary: "API reference: |U.DT.Format"
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
# |U.DT.Format

**Format datetime to string**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Format <datetime <format >result
```

**Inline:**
```polyglot
\|U.DT.Format"{$datetime, $format}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Datetime to format
- `<format` :pg.string - Format pattern

**Outputs:**
- `>result` :pg.string - Formatted string

---

## Description

Formats a `:pg.datetime` value into a string using the specified format pattern.

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

**Literals:**
- Any other characters are output as-is

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $formatted :pg.string << \|U.DT.Format"{$now, \"YYYY-MM-DD\"}"
```

**Output:** `"2025-12-15"`

---

### ISO 8601 Format

```polyglot
[r] $iso :pg.string << \|U.DT.Format"{$datetime, \"YYYY-MM-DDTHH:mm:ss\"}"
```

**Output:** `"2025-12-15T14:30:00"`

---

### US Format

```polyglot
[r] $us :pg.string << \|U.DT.Format"{$datetime, \"MM/DD/YYYY\"}"
```

**Output:** `"12/15/2025"`

---

### European Format

```polyglot
[r] $eu :pg.string << \|U.DT.Format"{$datetime, \"DD/MM/YYYY\"}"
```

**Output:** `"15/12/2025"`

---

### Time Only

```polyglot
[r] $time :pg.string << \|U.DT.Format"{$datetime, \"HH:mm:ss\"}"
```

**Output:** `"14:30:00"`

---

### Custom Format with Literals

```polyglot
[r] $custom :pg.string << \|U.DT.Format"{$datetime, \"Date: YYYY-MM-DD at HH:mm\"}"
```

**Output:** `"Date: 2025-12-15 at 14:30"`

---

## Common Formats

### ISO 8601
```polyglot
[r] $iso :pg.string << \|U.DT.Format"{$dt, \"YYYY-MM-DDTHH:mm:ss\"}"
```

### Filename-Safe
```polyglot
[r] $filename :pg.string << \|U.DT.Format"{$dt, \"YYYY-MM-DD_HH-mm-ss\"}"
```

### Log Format
```polyglot
[r] $log :pg.string << \|U.DT.Format"{$dt, \"YYYY-MM-DD HH:mm:ss\"}"
```

### Human Readable
```polyglot
[r] $readable :pg.string << \|U.DT.Format"{$dt, \"DD/MM/YYYY HH:mm\"}"
```

---

## Common Patterns

### Pattern 1: Log Entry

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $timestamp :pg.string << \|U.DT.Format"{$now, \"YYYY-MM-DD HH:mm:ss\"}"
[r] $log_line :pg.string << \|U.String.Concat"{$timestamp, \" - \", $message}"
```

### Pattern 2: Generate Filename

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $date_part :pg.string << \|U.DT.Format"{$now, \"YYYY-MM-DD_HH-mm-ss\"}"
[r] $filename :pg.string << \|U.String.Concat"{\"backup_\", $date_part, \".zip\"}"
```

### Pattern 3: Convert Between Formats

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{$us_date, \"MM/DD/YYYY\"}"
[r] $iso :pg.string << \|U.DT.Format"{$dt, \"YYYY-MM-DD\"}"
```

### Pattern 4: Display Relative Time

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $diff_days :pg.int << \|U.DT.Diff"{$event_date, $now, \"days\"}"

[f] $diff_days == 0
   [r] $display :pg.string << "Today"
[&] $diff_days == 1
   [r] $display :pg.string << "Tomorrow"
[^]
   [r] $display :pg.string << \|U.DT.Format"{$event_date, \"MM/DD/YYYY\"}"
```

---

## Related Pipelines

- [|U.DT.Parse](./parse.md) - Parse string to datetime
- [|U.DT.Now](./now.md) - Get current timestamp
- [|U.String.Concat](../string/concat.md) - Build formatted strings

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
