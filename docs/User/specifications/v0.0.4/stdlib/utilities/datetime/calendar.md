---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: calendar
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Calendar"
summary: "API reference: |U.DT.Calendar"
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
# |U.DT.Calendar

**Convert to custom calendar using profile enum**

**Category:** Utilities > DateTime > Calendar
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Calendar <datetime <profile >result
```

**Inline:**
```polyglot
\|U.DT.Calendar"{$datetime, $profile}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Datetime to convert
- `<profile` :#DT.Profiles - Calendar profile enum

**Outputs:**
- `>result` :pg.serial - Calendar representation

---

## Description

Converts a datetime to a calendar system specified by a user-defined profile enum.

**Profile Enum Pattern:** `#DT.Profiles.*;*`

**Result Serial Structure:**
```polyglot
$result :pg.serial
   year :pg.int           // Year in target calendar
   month :pg.int          // Month (1-12 or calendar-specific)
   day :pg.int            // Day of month
   weekday :pg.int        // Day of week (0=Sunday, 6=Saturday)
   calendar_name :pg.string  // Name of calendar system
```

---

## User-Defined Enum

**Define custom calendar profiles:**

```polyglot
// Define built-in calendar profiles
[r] $hijri_profile :#DT.Profiles << #DT.Profiles;Hijri;Standard
[r] $gregorian_profile :#DT.Profiles << #DT.Profiles;Gregorian;Standard
[r] $assyrian_profile :#DT.Profiles << #DT.Profiles;Assyrian;Standard

// Define custom calendar profiles
[r] $persian_profile :#DT.Profiles << #DT.Profiles;Persian;Solar
[r] $hebrew_profile :#DT.Profiles << #DT.Profiles;Hebrew;Standard
[r] $chinese_profile :#DT.Profiles << #DT.Profiles;Chinese;Lunar
```

---

## Examples

### Basic Usage with Built-In Profiles

```polyglot
[r] $date :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $profile :#DT.Profiles << #DT.Profiles;Hijri;Standard
[r] $hijri :pg.serial << \|U.DT.Calendar"{$date, $profile}"
```

---

### Switch Between Calendars

```polyglot
[r] $date :pg.datetime << \|U.DT.Now

[f] $user_preference == "hijri"
   [r] $profile :#DT.Profiles << #DT.Profiles;Hijri;Standard
[&] $user_preference == "assyrian"
   [r] $profile :#DT.Profiles << #DT.Profiles;Assyrian;Standard
[^]
   [r] $profile :#DT.Profiles << #DT.Profiles;Gregorian;Standard

[r] $calendar :pg.serial << \|U.DT.Calendar"{$date, $profile}"
[r] $calendar_name :pg.string << $calendar."calendar_name"
```

---

### Custom Persian Calendar

```polyglot
[r] $date :pg.datetime << \|U.DT.Parse"{\"2025-03-21\", \"YYYY-MM-DD\"}"
[r] $persian_profile :#DT.Profiles << #DT.Profiles;Persian;Solar
[r] $persian :pg.serial << \|U.DT.Calendar"{$date, $persian_profile}"

[r] $year :pg.int << $persian."year"  // Persian year
[r] $month :pg.int << $persian."month"  // Farvardin = 1
```

---

### Convert Multiple Dates

```polyglot
[r] $profile :#DT.Profiles << #DT.Profiles;Hijri;Standard

[r] ~ForEach.Array
[~] <array << $dates
[~] >item >> $date
   [r] $cal :pg.serial << \|U.DT.Calendar"{$date, $profile}"
   [v] *Into.Array
   [*] <item << $cal
   [*] >array >> $converted_dates
```

---

## Common Patterns

### Pattern 1: Display Date in User's Preferred Calendar

```polyglot
[r] $user_cal_pref :pg.string << $user."calendar_preference"
[r] $date :pg.datetime << \|U.DT.Now

[f] $user_cal_pref == "hijri"
   [r] $profile :#DT.Profiles << #DT.Profiles;Hijri;Standard
[&] $user_cal_pref == "assyrian"
   [r] $profile :#DT.Profiles << #DT.Profiles;Assyrian;Standard
[&] $user_cal_pref == "persian"
   [r] $profile :#DT.Profiles << #DT.Profiles;Persian;Solar
[^]
   [r] $profile :#DT.Profiles << #DT.Profiles;Gregorian;Standard

[r] $cal :pg.serial << \|U.DT.Calendar"{$date, $profile}"
[r] $display_date :pg.string << \|U.String.Concat"{$cal.\"year\", \"-\", $cal.\"month\", \"-\", $cal.\"day\"}"
```

### Pattern 2: Calendar Comparison

```polyglot
[r] $date :pg.datetime << \|U.DT.Now

[r] $greg_profile :#DT.Profiles << #DT.Profiles;Gregorian;Standard
[r] $hijri_profile :#DT.Profiles << #DT.Profiles;Hijri;Standard

[r] $gregorian :pg.serial << \|U.DT.Calendar"{$date, $greg_profile}"
[r] $hijri :pg.serial << \|U.DT.Calendar"{$date, $hijri_profile}"

[r] $greg_year :pg.int << $gregorian."year"
[r] $hijri_year :pg.int << $hijri."year"
```

### Pattern 3: Holiday Detection with Custom Calendar

```polyglot
[r] $profile :#DT.Profiles << #DT.Profiles;Hijri;Standard
[r] $hijri :pg.serial << \|U.DT.Calendar"{$date, $profile}"

[r] $month :pg.int << $hijri."month"
[r] $day :pg.int << $hijri."day"

[f] $month == 9 & $day == 1
   [r] $holiday :pg.string << "First of Ramadan"
[&] $month == 10 & $day == 1
   [r] $holiday :pg.string << "Eid al-Fitr"
```

---

## Predefined Profile Enums

**Built-in calendars accessible via |U.DT.Calendar:**

```polyglot
#DT.Profiles;Hijri;Standard      // Islamic/Hijri calendar
#DT.Profiles;Gregorian;Standard  // Western/Gregorian calendar
#DT.Profiles;Assyrian;Standard   // Assyrian calendar
```

**Custom calendars (implementation-defined):**

```polyglot
#DT.Profiles;Persian;Solar       // Persian Solar Hijri
#DT.Profiles;Hebrew;Standard     // Hebrew calendar
#DT.Profiles;Chinese;Lunar       // Chinese lunar calendar
#DT.Profiles;Custom;MyCalendar   // User-defined calendar
```

---

## Related Pipelines

- [|U.DT.Calendar.HijriStandard](./calendar-hijri-standard.md) - Hijri calendar (convenience)
- [|U.DT.Calendar.Gregorian](./calendar-gregorian.md) - Gregorian calendar (convenience)
- [|U.DT.Calendar.Assyrian](./calendar-assyrian.md) - Assyrian calendar (convenience)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
