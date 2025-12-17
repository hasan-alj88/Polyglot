---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "calendar-gregorian"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Calendar.Gregorian"
summary: "API reference: |U.DT.Calendar.Gregorian"
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
# |U.DT.Calendar.Gregorian

**Convert to Gregorian (Western) calendar**

**Category:** Utilities > DateTime > Calendar
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Calendar.Gregorian <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Calendar.Gregorian"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Datetime to convert

**Outputs:**
- `>result` :pg.serial - Gregorian calendar representation

---

## Description

Converts a datetime to explicit Gregorian calendar representation. Since `:pg.datetime` uses Gregorian internally, this primarily provides structured calendar data.

**Result Serial Structure:**
```polyglot
$result :pg.serial
   year :pg.int           // Gregorian year
   month :pg.int          // Month (1-12)
   day :pg.int            // Day of month (1-31)
   weekday :pg.int        // Day of week (0=Sunday, 6=Saturday)
   calendar_name :pg.string  // "Gregorian"
```

---

## Gregorian Calendar

**Months (1-12):**
1. January (31 days)
2. February (28/29 days)
3. March (31 days)
4. April (30 days)
5. May (31 days)
6. June (30 days)
7. July (31 days)
8. August (31 days)
9. September (30 days)
10. October (31 days)
11. November (30 days)
12. December (31 days)

**Leap year:** Every 4 years (except century years not divisible by 400).

---

## Examples

### Basic Usage

```polyglot
[r] $datetime :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $gregorian :pg.serial << \|U.DT.Calendar.Gregorian"{$datetime}"
```

**Output:** Serial with `year=2025, month=12, day=15`

---

### Get Weekday

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $cal :pg.serial << \|U.DT.Calendar.Gregorian"{$now}"
[r] $weekday :pg.int << $cal."weekday"

[y] $weekday == 0
   [r] $day_name :pg.string << "Sunday"
[&] $weekday == 1
   [r] $day_name :pg.string << "Monday"
[&] $weekday == 2
   [r] $day_name :pg.string << "Tuesday"
// ... etc
```

---

### Check Leap Year

```polyglot
[r] $cal :pg.serial << \|U.DT.Calendar.Gregorian"{$datetime}"
[r] $year :pg.int << $cal."year"
[r] $month :pg.int << $cal."month"

[y] $month == 2
   [r] $mod4 :pg.int << \|U.Math.Modulo"{$year, 4}"
   [r] $mod100 :pg.int << \|U.Math.Modulo"{$year, 100}"
   [r] $mod400 :pg.int << \|U.Math.Modulo"{$year, 400}"

   [y] $mod400 == 0
      [r] $is_leap :pg.bool << true
   [&] $mod100 == 0
      [r] $is_leap :pg.bool << false
   [&] $mod4 == 0
      [r] $is_leap :pg.bool << true
   [^]
      [r] $is_leap :pg.bool << false
```

---

## Common Patterns

### Pattern 1: Day of Week Name

```polyglot
[r] $cal :pg.serial << \|U.DT.Calendar.Gregorian"{$date}"
[r] $weekday :pg.int << $cal."weekday"

[y] $weekday == 0
   [r] $name :pg.string << "Sunday"
[&] $weekday == 1
   [r] $name :pg.string << "Monday"
[&] $weekday == 2
   [r] $name :pg.string << "Tuesday"
[&] $weekday == 3
   [r] $name :pg.string << "Wednesday"
[&] $weekday == 4
   [r] $name :pg.string << "Thursday"
[&] $weekday == 5
   [r] $name :pg.string << "Friday"
[^]
   [r] $name :pg.string << "Saturday"
```

### Pattern 2: Check Weekend

```polyglot
[r] $cal :pg.serial << \|U.DT.Calendar.Gregorian"{$date}"
[r] $weekday :pg.int << $cal."weekday"

[y] $weekday == 0 | $weekday == 6
   // Weekend (Saturday/Sunday)
   [r] $is_weekend :pg.bool << true
[^]
   [r] $is_weekend :pg.bool << false
```

### Pattern 3: Quarter Detection

```polyglot
[r] $cal :pg.serial << \|U.DT.Calendar.Gregorian"{$date}"
[r] $month :pg.int << $cal."month"

[y] $month >= 1 & $month <= 3
   [r] $quarter :pg.int << 1
[&] $month >= 4 & $month <= 6
   [r] $quarter :pg.int << 2
[&] $month >= 7 & $month <= 9
   [r] $quarter :pg.int << 3
[^]
   [r] $quarter :pg.int << 4
```

---

## Related Pipelines

- [|U.DT.Calendar.HijriStandard](./calendar-hijri-standard.md) - Convert to Hijri calendar
- [|U.DT.Calendar.Assyrian](./calendar-assyrian.md) - Convert to Assyrian calendar
- [|U.DT.Calendar](./calendar.md) - Convert using custom profile
- [|U.DT.Business.Week.SunFri](./business-week-sunfri.md) - Business week calculations

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
