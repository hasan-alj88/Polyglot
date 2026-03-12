---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: diff
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Diff"
summary: "API reference: |U.DT.Diff"
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
# |U.DT.Diff

**Calculate time difference**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Diff <start <end <unit >result
```

**Inline:**
```polyglot
\|U.DT.Diff"{$start, $end, $unit}"
```

---

## Parameters

**Inputs:**
- `<start` :pg.datetime - Start datetime
- `<end` :pg.datetime - End datetime
- `<unit` :pg.string - Unit for difference calculation

**Outputs:**
- `>result` :pg.int - Difference in specified units

---

## Description

Calculates the difference between two datetime values in the specified unit.

**Result is positive if end > start, negative if end < start.**

---

## Supported Units

**Time Units:**
- `"seconds"` - Difference in seconds
- `"minutes"` - Difference in minutes
- `"hours"` - Difference in hours

**Date Units:**
- `"days"` - Difference in days
- `"weeks"` - Difference in weeks (7 days)
- `"months"` - Approximate months (assumes 30 days)
- `"years"` - Approximate years (assumes 365 days)

**Produces error `!DT.InvalidUnit` if unit is not recognized.**

---

## Examples

### Basic Usage

```polyglot
[r] $start :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $end :pg.datetime << \|U.DT.Parse"{\"2025-12-20\", \"YYYY-MM-DD\"}"
[r] $days :pg.int << \|U.DT.Diff"{$start, $end, \"days\"}"
```

**Output:** `$days = 5`

---

### Calculate Age

```polyglot
[r] $birthdate :pg.datetime << \|U.DT.Parse"{$birth_string, \"YYYY-MM-DD\"}"
[r] $today :pg.datetime << \|U.DT.Now
[r] $age :pg.int << \|U.DT.Diff"{$birthdate, $today, \"years\"}"
```

---

### Measure Elapsed Time

```polyglot
[r] $start :pg.datetime << \|U.DT.Now

// ... perform operation ...

[r] $end :pg.datetime << \|U.DT.Now
[r] $elapsed_seconds :pg.int << \|U.DT.Diff"{$start, $end, \"seconds\"}"
```

---

### Check Time Until Event

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $event :pg.datetime << \|U.DT.Parse"{$event_str, \"YYYY-MM-DD HH:mm:ss\"}"
[r] $hours_until :pg.int << \|U.DT.Diff"{$now, $event, \"hours\"}"

[f] $hours_until <= 0
   // Event has passed or is now
[&] $hours_until <= 24
   // Event within 24 hours
[^]
   // Event more than 24 hours away
```

---

### Negative Differences

```polyglot
[r] $future :pg.datetime << \|U.DT.Parse"{\"2025-12-20\", \"YYYY-MM-DD\"}"
[r] $past :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $diff :pg.int << \|U.DT.Diff"{$future, $past, \"days\"}"
```

**Output:** `$diff = -5` (start is after end)

---

## Common Patterns

### Pattern 1: Check Expiration

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $diff_days :pg.int << \|U.DT.Diff"{$now, $expires_at, \"days\"}"

[f] $diff_days < 0
   [r] !Expired << "Item has expired"
[&] $diff_days <= 7
   // Expiring soon warning
```

### Pattern 2: Performance Logging

```polyglot
[r] $start :pg.datetime << \|U.DT.Now
// ... operation ...
[r] $end :pg.datetime << \|U.DT.Now
[r] $ms :pg.int << \|U.DT.Diff"{$start, $end, \"seconds\"}"
[r] $ms_total :pg.int << \|U.Math.Multiply"{$ms, 1000}"
```

### Pattern 3: Time-Based Pricing

```polyglot
[r] $rental_start :pg.datetime << $booking."start_time"
[r] $rental_end :pg.datetime << $booking."end_time"
[r] $hours :pg.int << \|U.DT.Diff"{$rental_start, $rental_end, \"hours\"}"
[r] $cost :pg.int << \|U.Math.Multiply"{$hours, $hourly_rate}"
```

### Pattern 4: Filter by Recency

```polyglot
[r] $now :pg.datetime << \|U.DT.Now

[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $item_date :pg.datetime << $item."created_at"
   [r] $days_old :pg.int << \|U.DT.Diff"{$item_date, $now, \"days\"}"

   [f] $days_old <= 7
      // Item from last 7 days
      [v] *Into.Array
      [*] <item << $item
      [*] >array >> $recent_items
```

---

## Month and Year Approximations

**Note:** `"months"` and `"years"` units are approximate:
- 1 month = 30 days
- 1 year = 365 days

For precise calendar calculations, use [|U.DT.Calendar](./calendar.md) pipelines.

---

## Related Pipelines

- [|U.DT.AddDays](./add-days.md) - Add days to datetime
- [|U.DT.AddHours](./add-hours.md) - Add hours to datetime
- [|U.DT.Now](./now.md) - Get current timestamp

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
