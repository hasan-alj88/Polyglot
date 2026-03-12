---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: minute
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Minute"
summary: "API reference: |U.DT.Minute"
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
# |U.DT.Minute

**Extract minute component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Minute <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Minute"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Minute (0-59)

---

## Description

Extracts the minute component from a datetime value as an integer from 0 to 59.

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15 14:30:45\", \"YYYY-MM-DD HH:mm:ss\"}"
[r] $minute :pg.int << \|U.DT.Minute"{$dt}"
```

**Output:** `$minute = 30`

---

### Current Minute

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_minute :pg.int << \|U.DT.Minute"{$now}"
```

---

### Check for Top of Hour

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $minute :pg.int << \|U.DT.Minute"{$now}"

[f] $minute == 0
   // Exactly on the hour
```

---

### Round to Nearest 15 Minutes

```polyglot
[r] $minute :pg.int << \|U.DT.Minute"{$datetime}"

[f] $minute < 8
   [r] $rounded :pg.int << 0
[&] $minute < 23
   [r] $rounded :pg.int << 15
[&] $minute < 38
   [r] $rounded :pg.int << 30
[&] $minute < 53
   [r] $rounded :pg.int << 45
[^]
   [r] $rounded :pg.int << 0
   // Also need to add hour
```

---

## Common Patterns

### Pattern 1: Scheduled Minute Tasks

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $minute :pg.int << \|U.DT.Minute"{$now}"

[f] $minute == 0 | $minute == 30
   // Run task every 30 minutes
```

### Pattern 2: Time Formatting

```polyglot
[r] $hour :pg.int << \|U.DT.Hour"{$datetime}"
[r] $minute :pg.int << \|U.DT.Minute"{$datetime}"

[r] $hour_str :pg.string << \|U.String.Concat"{$hour}"
[r] $minute_str :pg.string << \|U.String.Concat"{$minute}"

[f] $minute < 10
   [r] $minute_padded :pg.string << \|U.String.Concat"{\"0\", $minute_str}"
[^]
   [r] $minute_padded :pg.string << $minute_str

[r] $time :pg.string << \|U.String.Concat"{$hour_str, \":\", $minute_padded}"
```

### Pattern 3: Grouping by 5-Minute Intervals

```polyglot
[r] $minute :pg.int << \|U.DT.Minute"{$timestamp}"
[r] $remainder :pg.int << \|U.Math.Modulo"{$minute, 5}"
[r] $bucket :pg.int << \|U.Math.Subtract"{$minute, $remainder}"

// $bucket will be 0, 5, 10, 15, ..., 55
```

---

## Related Pipelines

- [|U.DT.Hour](./hour.md) - Extract hour component
- [|U.DT.Second](./second.md) - Extract second component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
