---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: day
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Day"
summary: "API reference: |U.DT.Day"
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
# |U.DT.Day

**Extract day component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Day <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Day"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Day of month (1-31)

---

## Description

Extracts the day of the month component from a datetime value as an integer from 1 to 31.

**Range depends on month:**
- Most months: 1-30 or 1-31
- February: 1-28 or 1-29 (leap year)

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $day :pg.int << \|U.DT.Day"{$dt}"
```

**Output:** `$day = 15`

---

### Current Day

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_day :pg.int << \|U.DT.Day"{$now}"
```

---

### Filter by Day of Month

```polyglot
[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event
   [r] $day :pg.int << \|U.DT.Day"{$event.\"date\"}"

   [y] $day == 1
      // Events on the 1st of any month
      [v] *Into.Array
      [*] <item << $event
      [*] >array >> $first_of_month_events
```

---

### Check Same Date

```polyglot
[r] $year1 :pg.int << \|U.DT.Year"{$date1}"
[r] $year2 :pg.int << \|U.DT.Year"{$date2}"
[r] $month1 :pg.int << \|U.DT.Month"{$date1}"
[r] $month2 :pg.int << \|U.DT.Month"{$date2}"
[r] $day1 :pg.int << \|U.DT.Day"{$date1}"
[r] $day2 :pg.int << \|U.DT.Day"{$date2}"

[y] $year1 == $year2 & $month1 == $month2 & $day1 == $day2
   // Same date
```

---

## Common Patterns

### Pattern 1: Check Month Start/End

```polyglot
[r] $day :pg.int << \|U.DT.Day"{$date}"

[y] $day == 1
   // First day of month
[&] $day >= 28
   // Likely near end of month
```

### Pattern 2: Reminder for Monthly Tasks

```polyglot
[r] $today_day :pg.int << \|U.DT.Day"{$now}"

[y] $today_day == 15
   // Mid-month reminder (15th)
```

### Pattern 3: Birthday Check

```polyglot
[r] $today_month :pg.int << \|U.DT.Month"{$today}"
[r] $today_day :pg.int << \|U.DT.Day"{$today}"
[r] $birth_month :pg.int << \|U.DT.Month"{$birthdate}"
[r] $birth_day :pg.int << \|U.DT.Day"{$birthdate}"

[y] $today_month == $birth_month & $today_day == $birth_day
   // Today is birthday
```

---

## Related Pipelines

- [|U.DT.Year](./year.md) - Extract year component
- [|U.DT.Month](./month.md) - Extract month component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
