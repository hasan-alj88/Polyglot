---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "business-week"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Business.Week"
summary: "API reference: |U.DT.Business.Week"
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
# |U.DT.Business.Week

**Custom business week calculations with enum**

**Category:** Utilities > DateTime > Business Week
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Business.Week <datetime <week >result
```

**Inline:**
```polyglot
\|U.DT.Business.Week"{$datetime, $week}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Datetime to analyze
- `<week` :#DT.Business.Week - Business week profile enum

**Outputs:**
- `>result` :pg.serial - Business week information

---

## Description

Analyzes a datetime for business week information using a user-defined business week profile.

**Profile Enum Pattern:** `#;DT;Business;Week.*`

**Result Serial Structure:**
```polyglot
$result :pg.serial
   week_number :pg.int           // Week number in year
   year :pg.int                  // Year
   is_business_day :pg.bool      // True if within business days
   days_until_weekend :pg.int    // Days until weekend starts
   day_of_week :pg.int           // 0=Sunday, 6=Saturday
   week_profile :pg.string       // Name of business week profile
```

---

## User-Defined Enum

**Define custom business week profiles:**

```polyglot
// Sunday-Friday business week (Middle East)
[r] $sunfri :#DT.Business.Week << #;DT;Business;Week;SunFri

// Monday-Friday business week (Western)
[r] $monfri :#DT.Business.Week << #;DT;Business;Week;MonFri

// Monday-Saturday business week
[r] $monsat :#DT.Business.Week << #;DT;Business;Week;MonSat

// Custom 4-day week (Mon-Thu)
[r] $monthu :#DT.Business.Week << #;DT;Business;Week;Custom;MonThu
```

---

## Examples

### Basic Usage with SunFri Profile

```polyglot
[r] $date :pg.datetime << \|U.DT.Now
[r] $profile :#DT.Business.Week << #;DT;Business;Week;SunFri
[r] $week_info :pg.serial << \|U.DT.Business.Week"{$date, $profile}"
```

---

### Check Business Day with Custom Profile

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $profile :#DT.Business.Week << #;DT;Business;Week;MonFri
[r] $week_info :pg.serial << \|U.DT.Business.Week"{$now, $profile}"
[r] $is_business :pg.bool << $week_info."is_business_day"

[y] $is_business
   // Monday-Friday business day
[^]
   // Weekend (Saturday/Sunday)
```

---

### User-Specific Business Week

```polyglot
[r] $user_region :pg.string << $user."region"

[y] $user_region == "middle_east"
   [r] $profile :#DT.Business.Week << #;DT;Business;Week;SunFri
[&] $user_region == "western"
   [r] $profile :#DT.Business.Week << #;DT;Business;Week;MonFri
[^]
   [r] $profile :#DT.Business.Week << #;DT;Business;Week;MonSat

[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week"{$now, $profile}"
```

---

### Compare Different Business Weeks

```polyglot
[r] $date :pg.datetime << \|U.DT.Parse"{\"2025-12-20\", \"YYYY-MM-DD\"}"  // Saturday

[r] $sunfri :#DT.Business.Week << #;DT;Business;Week;SunFri
[r] $monfri :#DT.Business.Week << #;DT;Business;Week;MonFri

[r] $info_sunfri :pg.serial << \|U.DT.Business.Week"{$date, $sunfri}"
[r] $info_monfri :pg.serial << \|U.DT.Business.Week"{$date, $monfri}"

[r] $is_business_sunfri :pg.bool << $info_sunfri."is_business_day"  // false (weekend)
[r] $is_business_monfri :pg.bool << $info_monfri."is_business_day"  // false (weekend)
```

---

## Common Patterns

### Pattern 1: Regional Business Hours

```polyglot
[r] $user_profile :#DT.Business.Week << #;DT;Business;Week;SunFri
[r] $now :pg.datetime << \|U.DT.Now
[r] $week_info :pg.serial << \|U.DT.Business.Week"{$now, $user_profile}"
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $week_info."is_business_day" & $hour >= 9 & $hour < 17
   // Open for business
```

### Pattern 2: Calculate Business Days Between Dates

```polyglot
[r] $profile :#DT.Business.Week << #;DT;Business;Week;MonFri
[r] $days_diff :pg.int << \|U.DT.Diff"{$start_date, $end_date, \"days\"}"

[r] ~Iter.Range
[~] <from << 0
[~] <to << $days_diff
[~] >index >> $day_offset
   [r] $date :pg.datetime << \|U.DT.AddDays"{$start_date, $day_offset}"
   [r] $week_info :pg.serial << \|U.DT.Business.Week"{$date, $profile}"

   [y] $week_info."is_business_day"
      [v] *Math.Count
      [*] <item << 1
      [*] >count >> $business_days
```

### Pattern 3: Schedule Task on Next Business Day

```polyglot
[r] $profile :#DT.Business.Week << #;DT;Business;Week;MonFri
[r] $current :pg.datetime << \|U.DT.Now
[r] $next_day :pg.datetime << \|U.DT.AddDays"{$current, 1}"

[r] $week_info :pg.serial << \|U.DT.Business.Week"{$next_day, $profile}"

[y] $week_info."is_business_day"
   [r] $scheduled :pg.datetime << $next_day
[^]
   // Skip to following day if weekend
   [r] $day_after :pg.datetime << \|U.DT.AddDays"{$next_day, 1}"
   [r] $scheduled :pg.datetime << $day_after
```

---

## Predefined Profile Enums

**Common business week profiles:**

```polyglot
#;DT;Business;Week;SunFri    // Sunday-Friday (Middle East)
#;DT;Business;Week;MonFri    // Monday-Friday (Western)
#;DT;Business;Week;MonSat    // Monday-Saturday
#;DT;Business;Week;SunThu    // Sunday-Thursday
```

**Custom profiles (implementation-defined):**

```polyglot
#;DT;Business;Week;Custom;MonThu     // 4-day week (Mon-Thu)
#;DT;Business;Week;Custom;TueSat     // Tuesday-Saturday
#;DT;Business;Week;Custom;MyCompany  // Company-specific schedule
```

---

## Business Week Definitions

**Sunday-Friday (SunFri):**
- Business days: Sunday, Monday, Tuesday, Wednesday, Thursday, Friday
- Weekend: Saturday
- Common in: Middle East, North Africa

**Monday-Friday (MonFri):**
- Business days: Monday, Tuesday, Wednesday, Thursday, Friday
- Weekend: Saturday, Sunday
- Common in: North America, Europe, Asia-Pacific

**Monday-Saturday (MonSat):**
- Business days: Monday, Tuesday, Wednesday, Thursday, Friday, Saturday
- Weekend: Sunday
- Common in: Some Asian and Middle Eastern regions

**Sunday-Thursday (SunThu):**
- Business days: Sunday, Monday, Tuesday, Wednesday, Thursday
- Weekend: Friday, Saturday
- Common in: Some Middle Eastern countries

---

## Related Pipelines

- [|U.DT.Business.Week.SunFri](./business-week-sunfri.md) - Sunday-Friday (convenience)
- [|U.DT.Calendar.Gregorian](./calendar-gregorian.md) - Get weekday information

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
