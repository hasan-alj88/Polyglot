---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "calendar-hijri-standard"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Calendar.HijriStandard"
summary: "API reference: |U.DT.Calendar.HijriStandard"
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
# |U.DT.Calendar.HijriStandard

**Convert to Hijri (Islamic) calendar**

**Category:** Utilities > DateTime > Calendar
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Calendar.HijriStandard <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Calendar.HijriStandard"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Gregorian datetime to convert

**Outputs:**
- `>result` :pg.serial - Hijri calendar representation

---

## Description

Converts a Gregorian datetime to the Hijri (Islamic) calendar system using the standard calculation method.

**Result Serial Structure:**
```polyglot
$result :pg.serial
   year :pg.int           // Hijri year
   month :pg.int          // Hijri month (1-12)
   day :pg.int            // Day of Hijri month (1-29/30)
   weekday :pg.int        // Day of week (0=Sunday, 6=Saturday)
   calendar_name :pg.string  // "Hijri"
```

---

## Hijri Calendar

**Months (1-12):**
1. Muharram (29/30 days)
2. Safar (29/30 days)
3. Rabi' al-Awwal (29/30 days)
4. Rabi' al-Thani (29/30 days)
5. Jumada al-Awwal (29/30 days)
6. Jumada al-Thani (29/30 days)
7. Rajab (29/30 days)
8. Sha'ban (29/30 days)
9. Ramadan (29/30 days)
10. Shawwal (29/30 days)
11. Dhu al-Qi'dah (29/30 days)
12. Dhu al-Hijjah (29/30 days)

**Lunar calendar:** Based on moon phases, approximately 354 days per year.

---

## Examples

### Basic Usage

```polyglot
[r] $gregorian :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$gregorian}"
```

**Output:** Serial with Hijri date components

---

### Display Hijri Date

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$now}"

[r] $year :pg.int << $hijri."year"
[r] $month :pg.int << $hijri."month"
[r] $day :pg.int << $hijri."day"

[r] $year_str :pg.string << \|U.String.Concat"{$year}"
[r] $month_str :pg.string << \|U.String.Concat"{$month}"
[r] $day_str :pg.string << \|U.String.Concat"{$day}"

[r] $display :pg.string << \|U.String.Concat"{$year_str, \"-\", $month_str, \"-\", $day_str, \" AH\"}"
```

**Output:** `"1447-6-12 AH"` (example)

---

### Check Ramadan

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$now}"
[r] $month :pg.int << $hijri."month"

[f] $month == 9
   // Currently in Ramadan
```

---

### Group Events by Hijri Month

```polyglot
[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event
   [r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$event.\"date\"}"
   [r] $hijri_month :pg.int << $hijri."month"

   [v] *Into.Serial
   [*] <path :pg.string << \|U.String.Concat"{\"month_\", $hijri_month}"
   [*] <item << $event
   [*] >serial >> $events_by_hijri_month
```

---

## Common Patterns

### Pattern 1: Islamic Holiday Detection

```polyglot
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$date}"
[r] $month :pg.int << $hijri."month"
[r] $day :pg.int << $hijri."day"

[f] $month == 9 & $day == 1
   [r] $holiday :pg.string << "First of Ramadan"
[&] $month == 10 & $day == 1
   [r] $holiday :pg.string << "Eid al-Fitr"
[&] $month == 12 & ($day >= 10 & $day <= 13)
   [r] $holiday :pg.string << "Eid al-Adha"
```

### Pattern 2: Age Calculation in Hijri

```polyglot
[r] $birth_hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$birthdate}"
[r] $now_hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$now}"

[r] $birth_year :pg.int << $birth_hijri."year"
[r] $current_year :pg.int << $now_hijri."year"

[r] $age_hijri :pg.int << \|U.Math.Subtract"{$current_year, $birth_year}"
```

### Pattern 3: Format Hijri Date with Month Name

```polyglot
[r] $hijri :pg.serial << \|U.DT.Calendar.HijriStandard"{$date}"
[r] $month :pg.int << $hijri."month"

[f] $month == 1
   [r] $month_name :pg.string << "Muharram"
[&] $month == 2
   [r] $month_name :pg.string << "Safar"
[&] $month == 3
   [r] $month_name :pg.string << "Rabi' al-Awwal"
// ... etc for all 12 months
```

---

## Related Pipelines

- [|U.DT.Calendar.Gregorian](./calendar-gregorian.md) - Convert to Gregorian calendar
- [|U.DT.Calendar.Assyrian](./calendar-assyrian.md) - Convert to Assyrian calendar
- [|U.DT.Calendar](./calendar.md) - Convert using custom profile

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
