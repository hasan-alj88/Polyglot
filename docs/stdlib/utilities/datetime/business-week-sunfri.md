---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "business-week-sunfri"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Business.Week.SunFri"
summary: "API reference: |U.DT.Business.Week.SunFri"
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
# |U.DT.Business.Week.SunFri

**Sunday-Friday business week calculations**

**Category:** Utilities > DateTime > Business Week
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Business.Week.SunFri <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Business.Week.SunFri"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Datetime to analyze

**Outputs:**
- `>result` :pg.serial - Business week information

---

## Description

Analyzes a datetime for business week information using Sunday-Friday as the business week (common in Middle Eastern countries).

**Business Days:** Sunday through Friday
**Weekend:** Saturday

**Result Serial Structure:**
```polyglot
$result :pg.serial
   week_number :pg.int           // Week number in year
   year :pg.int                  // Year
   is_business_day :pg.bool      // True if Sunday-Friday
   days_until_weekend :pg.int    // Days until Saturday (0-5)
   day_of_week :pg.int           // 0=Sunday, 6=Saturday
```

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$now}"
```

---

### Check If Business Day

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$now}"
[r] $is_business :pg.bool << $week_info."is_business_day"

[y] $is_business
   // Process business day logic
[^]
   // Weekend (Saturday)
```

---

### Days Until Weekend

```polyglot
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$date}"
[r] $days_left :pg.int << $week_info."days_until_weekend"

[y] $days_left == 0
   [r] $message :pg.string << "Weekend!"
[&] $days_left == 1
   [r] $message :pg.string << "Last day before weekend"
[^]
   [r] $days_str :pg.string << \|U.String.Concat"{$days_left}"
   [r] $message :pg.string << \|U.String.Concat"{$days_str, \" days until weekend\"}"
```

---

### Filter Business Days

```polyglot
[r] ~ForEach.Array
[~] <array << $dates
[~] >item >> $date
   [r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$date}"
   [r] $is_business :pg.bool << $week_info."is_business_day"

   [y] $is_business
      [v] *Into.Array
      [*] <item << $date
      [*] >array >> $business_days
```

---

## Common Patterns

### Pattern 1: Business Hours Check

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$now}"
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $week_info."is_business_day" & $hour >= 9 & $hour < 17
   // Within business hours (9 AM - 5 PM, Sun-Fri)
   [r] $open :pg.bool << true
[^]
   [r] $open :pg.bool << false
```

### Pattern 2: Calculate Next Business Day

```polyglot
[r] $current :pg.datetime << $date
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$current}"
[r] $day_of_week :pg.int << $week_info."day_of_week"

[y] $day_of_week == 6
   // Saturday -> Next business day is Sunday (1 day)
   [r] $next_business :pg.datetime << \|U.DT.AddDays"{$current, 1}"
[^]
   // Any other day -> Next business day is tomorrow
   [r] $next_business :pg.datetime << \|U.DT.AddDays"{$current, 1}"
```

### Pattern 3: Count Business Days in Range

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 7
[~] >index >> $day_offset
   [r] $date :pg.datetime << \|U.DT.AddDays"{$start_date, $day_offset}"
   [r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri"{$date}"

   [y] $week_info."is_business_day"
      [v] *Math.Count
      [*] <item << 1
      [*] >count >> $business_day_count
```

---

## Week Numbering

**Week Number:** ISO 8601 week numbering
- Week 1 is the first week with at least 4 days in the new year
- Weeks start on Monday (for numbering purposes)

---

## Regional Context

**Sunday-Friday business week used in:**
- Middle Eastern countries
- Some North African countries
- Regions following Islamic calendar workweek

**Compare with:** [|U.DT.Business.Week](./business-week.md) for custom business week definitions

---

## Related Pipelines

- [|U.DT.Business.Week](./business-week.md) - Custom business week with enum
- [|U.DT.Calendar.Gregorian](./calendar-gregorian.md) - Get weekday information

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
