---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "add-days"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.AddDays"
summary: "API reference: |U.DT.AddDays"
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
# |U.DT.AddDays

**Add or subtract days**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.AddDays <datetime <days >result
```

**Inline:**
```polyglot
\|U.DT.AddDays"{$datetime, $days}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime
- `<days` :pg.int - Number of days to add (negative to subtract)

**Outputs:**
- `>result` :pg.datetime - Modified datetime

---

## Description

Adds or subtracts the specified number of days to/from a datetime value.

**Use negative values to subtract days.**

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $tomorrow :pg.datetime << \|U.DT.AddDays"{$now, 1}"
```

---

### Add One Week

```polyglot
[r] $next_week :pg.datetime << \|U.DT.AddDays"{$today, 7}"
```

---

### Subtract Days (Go Back in Time)

```polyglot
[r] $yesterday :pg.datetime << \|U.DT.AddDays"{$now, -1}"
[r] $last_week :pg.datetime << \|U.DT.AddDays"{$now, -7}"
```

---

### Calculate Due Date

```polyglot
[r] $created :pg.datetime << \|U.DT.Now
[r] $due_date :pg.datetime << \|U.DT.AddDays"{$created, 30}"
```

---

### Date Range Generation

```polyglot
[r] $start :pg.datetime << \|U.DT.Parse"{\"2025-01-01\", \"YYYY-MM-DD\"}"

[r] ~Iter.Range
[~] <from << 0
[~] <to << 7
[~] >index >> $day_offset
   [r] $date :pg.datetime << \|U.DT.AddDays"{$start, $day_offset}"
   [r] $formatted :pg.string << \|U.DT.Format"{$date, \"YYYY-MM-DD\"}"
   [v] *Into.Array
   [*] <item << $formatted
   [*] >array >> $week_dates
```

**Output:** `["2025-01-01", "2025-01-02", ..., "2025-01-06"]`

---

## Common Patterns

### Pattern 1: Calculate Expiration

```polyglot
[r] $issued_at :pg.datetime << \|U.DT.Now
[r] $expires_at :pg.datetime << \|U.DT.AddDays"{$issued_at, 30}"
```

### Pattern 2: Check If Date is in Past

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $diff :pg.int << \|U.DT.Diff"{$event_date, $now, \"days\"}"

[y] $diff < 0
   // Event is in the past
```

### Pattern 3: Business Days (Approximate)

```polyglot
// Add 5 business days (approximately 7 calendar days)
[r] $deadline :pg.datetime << \|U.DT.AddDays"{$start, 7}"
```

### Pattern 4: Generate Date Series

```polyglot
[r] ~Iter.Range <from << 0 <to << 30 >index >> $day
   [r] $date :pg.datetime << \|U.DT.AddDays"{$start_date, $day}"
   // Process each day...
```

---

## Month and Year Boundaries

**Automatically handles:**
- Month boundaries (e.g., Jan 31 + 1 day = Feb 1)
- Year boundaries (e.g., Dec 31 + 1 day = Jan 1 next year)
- Leap years (e.g., Feb 28, 2024 + 1 day = Feb 29, 2024)

---

## Related Pipelines

- [|U.DT.AddHours](./add-hours.md) - Add hours
- [|U.DT.AddMinutes](./add-minutes.md) - Add minutes
- [|U.DT.Diff](./diff.md) - Calculate difference in days

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
