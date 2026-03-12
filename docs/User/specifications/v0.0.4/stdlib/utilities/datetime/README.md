---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: DateTime Utilities
summary: API reference: DateTime Utilities
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
# DateTime Utilities

**Package:** `|U.DateTime.*` (alias: `|U.DT.*`)
**Category:** Utilities
**Since:** v0.0.1

---

## Overview

DateTime utilities provide comprehensive date and time manipulation capabilities including parsing, formatting, arithmetic operations, calendar systems, and business week calculations.

---

## Package Structure

**Core Operations**
- [**|U.DT.Now**](./now.md) - Get current timestamp
- [**|U.DT.Parse**](./parse.md) - Parse string to datetime
- [**|U.DT.Format**](./format.md) - Format datetime to string
- [**|U.DT.AddDays**](./add-days.md) - Add days to datetime
- [**|U.DT.AddHours**](./add-hours.md) - Add hours to datetime
- [**|U.DT.AddMinutes**](./add-minutes.md) - Add minutes to datetime
- [**|U.DT.AddSeconds**](./add-seconds.md) - Add seconds to datetime
- [**|U.DT.Diff**](./diff.md) - Calculate time difference
- [**|U.DT.Year**](./year.md) - Extract year component
- [**|U.DT.Month**](./month.md) - Extract month component
- [**|U.DT.Day**](./day.md) - Extract day component
- [**|U.DT.Hour**](./hour.md) - Extract hour component
- [**|U.DT.Minute**](./minute.md) - Extract minute component
- [**|U.DT.Second**](./second.md) - Extract second component

**Calendar Profiles**
- [**|U.DT.Calendar.HijriStandard**](./calendar-hijri-standard.md) - Hijri (Islamic) calendar
- [**|U.DT.Calendar.Gregorian**](./calendar-gregorian.md) - Gregorian (Western) calendar
- [**|U.DT.Calendar.Assyrian**](./calendar-assyrian.md) - Assyrian calendar
- [**|U.DT.Calendar**](./calendar.md) - Custom calendar with profile enum

**Business Weeks**
- [**|U.DT.Business.Week.SunFri**](./business-week-sunfri.md) - Sunday-Friday business week
- [**|U.DT.Business.Week**](./business-week.md) - Custom business week with enum

---

## Quick Reference

### Core Operations

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.DT.Now` | - | `>result :pg.datetime` | Current timestamp |
| `\|U.DT.Parse` | `<string :pg.string`<br>`<format :pg.string` | `>result :pg.datetime` | Parse string to datetime |
| `\|U.DT.Format` | `<datetime :pg.datetime`<br>`<format :pg.string` | `>result :pg.string` | Format datetime to string |
| `\|U.DT.AddDays` | `<datetime :pg.datetime`<br>`<days :pg.int` | `>result :pg.datetime` | Add/subtract days |
| `\|U.DT.AddHours` | `<datetime :pg.datetime`<br>`<hours :pg.int` | `>result :pg.datetime` | Add/subtract hours |
| `\|U.DT.AddMinutes` | `<datetime :pg.datetime`<br>`<minutes :pg.int` | `>result :pg.datetime` | Add/subtract minutes |
| `\|U.DT.AddSeconds` | `<datetime :pg.datetime`<br>`<seconds :pg.int` | `>result :pg.datetime` | Add/subtract seconds |
| `\|U.DT.Diff` | `<start :pg.datetime`<br>`<end :pg.datetime`<br>`<unit :pg.string` | `>result :pg.int` | Time difference |
| `\|U.DT.Year` | `<datetime :pg.datetime` | `>result :pg.int` | Extract year |
| `\|U.DT.Month` | `<datetime :pg.datetime` | `>result :pg.int` | Extract month (1-12) |
| `\|U.DT.Day` | `<datetime :pg.datetime` | `>result :pg.int` | Extract day (1-31) |
| `\|U.DT.Hour` | `<datetime :pg.datetime` | `>result :pg.int` | Extract hour (0-23) |
| `\|U.DT.Minute` | `<datetime :pg.datetime` | `>result :pg.int` | Extract minute (0-59) |
| `\|U.DT.Second` | `<datetime :pg.datetime` | `>result :pg.int` | Extract second (0-59) |

### Calendar Profiles

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.DT.Calendar.HijriStandard` | `<datetime :pg.datetime` | `>result :pg.serial` | Convert to Hijri calendar |
| `\|U.DT.Calendar.Gregorian` | `<datetime :pg.datetime` | `>result :pg.serial` | Convert to Gregorian calendar |
| `\|U.DT.Calendar.Assyrian` | `<datetime :pg.datetime` | `>result :pg.serial` | Convert to Assyrian calendar |
| `\|U.DT.Calendar` | `<datetime :pg.datetime`<br>`<profile :#DT.Profiles` | `>result :pg.serial` | Convert to custom calendar |

**Calendar Profile Enums:**
- User-defined enum: `#DT.Profiles.*;*`
- Examples: `#DT.Profiles;Hijri;Standard`, `#DT.Profiles;Custom;MyCalendar`

**Calendar Result Serial:**
```polyglot
$calendar :pg.serial
   year :pg.int
   month :pg.int
   day :pg.int
   weekday :pg.int
   calendar_name :pg.string
```

### Business Weeks

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.DT.Business.Week.SunFri` | `<datetime :pg.datetime` | `>result :pg.serial` | Sunday-Friday business week |
| `\|U.DT.Business.Week` | `<datetime :pg.datetime`<br>`<week :#DT.Business.Week` | `>result :pg.serial` | Custom business week |

**Business Week Enums:**
- User-defined enum: `#DT.Business;Week.*`
- Examples: `#DT.Business;Week;SunFri`, `#DT.Business;Week;Custom`

**Business Week Result Serial:**
```polyglot
$week_info :pg.serial
   week_number :pg.int
   year :pg.int
   is_business_day :pg.bool
   days_until_weekend :pg.int
```

---

## Common Patterns

### Pattern 1: Parse and Format
```polyglot
[r] $parsed :pg.datetime << \|U.DT.Parse"{$date_string, \"YYYY-MM-DD\"}"
[r] $formatted :pg.string << \|U.DT.Format"{$parsed, \"DD/MM/YYYY\"}"
```

### Pattern 2: Date Arithmetic
```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $tomorrow :pg.datetime << \|U.DT.AddDays"{$now, 1}"
[r] $next_week :pg.datetime << \|U.DT.AddDays"{$now, 7}"
```

### Pattern 3: Calculate Age
```polyglot
[r] $birthdate :pg.datetime << \|U.DT.Parse"{$birth_string, \"YYYY-MM-DD\"}"
[r] $today :pg.datetime << \|U.DT.Now
[r] $age :pg.int << \|U.DT.Diff"{$birthdate, $today, \"years\"}"
```

### Pattern 4: Business Week Check
```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week.SunFri <datetime << $now

[f] $week_info."is_business_day"
   // Process business day logic
[^]
   // Weekend handling
```

### Pattern 5: Calendar Conversion
```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard <datetime << $now
[r] $hijri_year :pg.int << $hijri."year"
[r] $hijri_month :pg.int << $hijri."month"
```

---

## Format Strings

**Supported format tokens:**
- `YYYY` - 4-digit year
- `YY` - 2-digit year
- `MM` - 2-digit month (01-12)
- `M` - Month without leading zero (1-12)
- `DD` - 2-digit day (01-31)
- `D` - Day without leading zero (1-31)
- `HH` - 2-digit hour (00-23)
- `H` - Hour without leading zero (0-23)
- `mm` - 2-digit minute (00-59)
- `m` - Minute without leading zero (0-59)
- `ss` - 2-digit second (00-59)
- `s` - Second without leading zero (0-59)

**Common formats:**
- ISO 8601: `YYYY-MM-DDTHH:mm:ss`
- US: `MM/DD/YYYY`
- EU: `DD/MM/YYYY`
- Time only: `HH:mm:ss`

---

## Time Difference Units

**Supported units for |U.DT.Diff:**
- `"seconds"` - Difference in seconds
- `"minutes"` - Difference in minutes
- `"hours"` - Difference in hours
- `"days"` - Difference in days
- `"weeks"` - Difference in weeks
- `"months"` - Approximate months (30 days)
- `"years"` - Approximate years (365 days)

---

## Related Packages

- [Math Utilities](../math/README.md) - For numeric operations on time differences
- [String Utilities](../string/README.md) - For datetime string manipulation

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
