---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: second
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Second"
summary: "API reference: |U.DT.Second"
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
# |U.DT.Second

**Extract second component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Second <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Second"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Second (0-59)

---

## Description

Extracts the second component from a datetime value as an integer from 0 to 59.

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15 14:30:45\", \"YYYY-MM-DD HH:mm:ss\"}"
[r] $second :pg.int << \|U.DT.Second"{$dt}"
```

**Output:** `$second = 45`

---

### Current Second

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_second :pg.int << \|U.DT.Second"{$now}"
```

---

### Check for Top of Minute

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $second :pg.int << \|U.DT.Second"{$now}"

[f] $second == 0
   // Exactly on the minute
```

---

### Millisecond Approximation

```polyglot
[r] $start :pg.datetime << \|U.DT.Now
// ... operation ...
[r] $end :pg.datetime << \|U.DT.Now

[r] $elapsed_sec :pg.int << \|U.DT.Diff"{$start, $end, \"seconds\"}"
```

---

## Common Patterns

### Pattern 1: High-Precision Timing

```polyglot
[r] $start :pg.datetime << \|U.DT.Now
[r] $start_sec :pg.int << \|U.DT.Second"{$start}"

// ... operation ...

[r] $end :pg.datetime << \|U.DT.Now
[r] $end_sec :pg.int << \|U.DT.Second"{$end}"

[r] $diff :pg.int << \|U.Math.Subtract"{$end_sec, $start_sec}"
```

### Pattern 2: Full Time Display

```polyglot
[r] $hour :pg.int << \|U.DT.Hour"{$datetime}"
[r] $minute :pg.int << \|U.DT.Minute"{$datetime}"
[r] $second :pg.int << \|U.DT.Second"{$datetime}"

[r] $h_str :pg.string << \|U.String.Concat"{$hour}"
[r] $m_str :pg.string << \|U.String.Concat"{$minute}"
[r] $s_str :pg.string << \|U.String.Concat"{$second}"

[r] $time :pg.string << \|U.String.Concat"{$h_str, \":\", $m_str, \":\", $s_str}"
```

### Pattern 3: Rate Limiting Check

```polyglot
[r] $last_request_sec :pg.int << \|U.DT.Second"{$last_request_time}"
[r] $now_sec :pg.int << \|U.DT.Second"{$now}"

[r] $diff :pg.int << \|U.Math.Subtract"{$now_sec, $last_request_sec}"

[f] $diff < 1
   [r] !RateLimit << "Too many requests per second"
```

---

## Related Pipelines

- [|U.DT.Hour](./hour.md) - Extract hour component
- [|U.DT.Minute](./minute.md) - Extract minute component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
