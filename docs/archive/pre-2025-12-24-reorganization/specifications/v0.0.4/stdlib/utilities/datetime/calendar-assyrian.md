---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "calendar-assyrian"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Calendar.Assyrian"
summary: "API reference: |U.DT.Calendar.Assyrian"
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
# |U.DT.Calendar.Assyrian

**Convert to Assyrian calendar**

**Category:** Utilities > DateTime > Calendar
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Calendar.Assyrian <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Calendar.Assyrian"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Gregorian datetime to convert

**Outputs:**
- `>result` :pg.serial - Assyrian calendar representation

---

## Description

Converts a Gregorian datetime to the Assyrian calendar system, which begins in 4750 BCE.

**Conversion:** Assyrian Year = Gregorian Year + 4750

**Result Serial Structure:**
```polyglot
$result :pg.serial
   year :pg.int           // Assyrian year
   month :pg.int          // Month (1-12, same as Gregorian)
   day :pg.int            // Day of month (1-31)
   weekday :pg.int        // Day of week (0=Sunday, 6=Saturday)
   calendar_name :pg.string  // "Assyrian"
```

---

## Assyrian Calendar

The Assyrian calendar uses the same month structure as Gregorian but with different year numbering.

**Months (1-12):**
1. Nīsan (31 days) - April
2. 'Īyar (30 days) - May
3. Ḥzīran (31 days) - June
4. Tamūz (31 days) - July
5. Ṭabbāḥ (30 days) - August
6. 'Īlūl (31 days) - September
7. Tišrīn Qḏīm (30 days) - October
8. Tišrīn Ḥrāy (31 days) - November
9. Kānōn Qḏīm (30 days) - December
10. Kānōn Ḥrāy (31 days) - January
11. Šḇāṭ (28/29 days) - February
12. Āḏar (31 days) - March

**Note:** Year starts in April (Nīsan).

---

## Examples

### Basic Usage

```polyglot
[r] $gregorian :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $assyrian :pg.serial << \|U.DT.Calendar.Assyrian"{$gregorian}"
```

**Output:** Serial with `year=6775` (2025 + 4750)

---

### Display Assyrian Date

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $assyrian :pg.serial << \|U.DT.Calendar.Assyrian"{$now}"

[r] $year :pg.int << $assyrian."year"
[r] $month :pg.int << $assyrian."month"
[r] $day :pg.int << $assyrian."day"

[r] $year_str :pg.string << \|U.String.Concat"{$year}"
[r] $month_str :pg.string << \|U.String.Concat"{$month}"
[r] $day_str :pg.string << \|U.String.Concat"{$day}"

[r] $display :pg.string << \|U.String.Concat"{$year_str, \"-\", $month_str, \"-\", $day_str, \" AC\"}"
```

**Output:** `"6775-12-15 AC"` (Assyrian Calendar)

---

### Check Akitu (New Year)

```polyglot
[r] $assyrian :pg.serial << \|U.DT.Calendar.Assyrian"{$date}"
[r] $month :pg.int << $assyrian."month"
[r] $day :pg.int << $assyrian."day"

[f] $month == 1 & $day == 1
   // Akitu - Assyrian New Year (April 1st Gregorian)
```

---

## Common Patterns

### Pattern 1: Convert Between Gregorian and Assyrian Years

```polyglot
// Gregorian to Assyrian
[r] $greg_year :pg.int << \|U.DT.Year"{$gregorian_date}"
[r] $assyrian_year :pg.int << \|U.Math.Add"{$greg_year, 4750}"

// Assyrian to Gregorian
[r] $assyrian_year :pg.int << $assyrian_cal."year"
[r] $greg_year :pg.int << \|U.Math.Subtract"{$assyrian_year, 4750}"
```

### Pattern 2: Display Bilingual Dates

```polyglot
[r] $greg_year :pg.int << \|U.DT.Year"{$date}"
[r] $assyrian :pg.serial << \|U.DT.Calendar.Assyrian"{$date}"
[r] $assyr_year :pg.int << $assyrian."year"

[r] $greg_str :pg.string << \|U.String.Concat"{$greg_year}"
[r] $assyr_str :pg.string << \|U.String.Concat"{$assyr_year}"

[r] $display :pg.string << \|U.String.Concat"{$assyr_str, \" AC / \", $greg_str, \" CE\"}"
```

**Output:** `"6775 AC / 2025 CE"`

### Pattern 3: Age in Assyrian Years

```polyglot
[r] $birth_assyr :pg.serial << \|U.DT.Calendar.Assyrian"{$birthdate}"
[r] $now_assyr :pg.serial << \|U.DT.Calendar.Assyrian"{$now}"

[r] $birth_year :pg.int << $birth_assyr."year"
[r] $current_year :pg.int << $now_assyr."year"

[r] $age :pg.int << \|U.Math.Subtract"{$current_year, $birth_year}"
```

---

## Related Pipelines

- [|U.DT.Calendar.Gregorian](./calendar-gregorian.md) - Convert to Gregorian calendar
- [|U.DT.Calendar.HijriStandard](./calendar-hijri-standard.md) - Convert to Hijri calendar
- [|U.DT.Calendar](./calendar.md) - Convert using custom profile

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
