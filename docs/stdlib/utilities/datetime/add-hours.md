---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "add-hours"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.AddHours"
summary: "API reference: |U.DT.AddHours"
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
# |U.DT.AddHours

**Add or subtract hours**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.AddHours <datetime <hours >result
```

**Inline:**
```polyglot
\|U.DT.AddHours"{$datetime, $hours}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime
- `<hours` :pg.int - Number of hours to add (negative to subtract)

**Outputs:**
- `>result` :pg.datetime - Modified datetime

---

## Description

Adds or subtracts the specified number of hours to/from a datetime value.

**Use negative values to subtract hours.**

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $later :pg.datetime << \|U.DT.AddHours"{$now, 2}"
```

---

### Add One Day (24 hours)

```polyglot
[r] $tomorrow_same_time :pg.datetime << \|U.DT.AddHours"{$now, 24}"
```

---

### Subtract Hours

```polyglot
[r] $two_hours_ago :pg.datetime << \|U.DT.AddHours"{$now, -2}"
```

---

### Schedule Task

```polyglot
[r] $created :pg.datetime << \|U.DT.Now
[r] $scheduled :pg.datetime << \|U.DT.AddHours"{$created, 4}"
```

---

### Calculate Timezone Offset

```polyglot
// Convert UTC to EST (UTC-5)
[r] $utc :pg.datetime << \|U.DT.Now
[r] $est :pg.datetime << \|U.DT.AddHours"{$utc, -5}"
```

---

## Common Patterns

### Pattern 1: Cache Expiration

```polyglot
[r] $cached_at :pg.datetime << \|U.DT.Now
[r] $expires_at :pg.datetime << \|U.DT.AddHours"{$cached_at, 1}"
```

### Pattern 2: Check If Within Time Window

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $window_end :pg.datetime << \|U.DT.AddHours"{$event_start, 2}"

[r] $diff :pg.int << \|U.DT.Diff"{$now, $window_end, \"hours\"}"

[y] $diff <= 0
   // Within 2-hour window
```

### Pattern 3: Business Hours Calculation

```polyglot
[r] $start :pg.datetime << \|U.DT.Parse"{\"2025-12-15 09:00:00\", \"YYYY-MM-DD HH:mm:ss\"}"
[r] $end :pg.datetime << \|U.DT.AddHours"{$start, 8}"
// End of 8-hour workday
```

---

## Day Boundaries

**Automatically handles:**
- Day boundaries (e.g., 23:00 + 2 hours = 01:00 next day)
- Month boundaries
- Year boundaries

---

## Related Pipelines

- [|U.DT.AddDays](./add-days.md) - Add days
- [|U.DT.AddMinutes](./add-minutes.md) - Add minutes
- [|U.DT.Diff](./diff.md) - Calculate difference in hours

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
