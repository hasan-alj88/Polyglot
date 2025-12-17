# DateTime System

**Version:** 0.0.2
**Last Updated:** 2025-11-30
**Status:** Complete
**Related Documents:**
- [DateTime Extension Guide](../datetime-extension-guide.md)
- [DateTime Formatted String Grammar](../../technical/datetime-formatted-string-grammar.md)
- [DT Pipeline Tree](../../technical/dt-pipeline-tree.md)
- [DateTime String Literal Specification](../../technical/datetime-string-literal-specification.md)

---

## Overview

Polyglot's DateTime system provides comprehensive date and time handling with support for multiple calendar systems, timezone management, relative time expressions, and recurring patterns. The system is built on the `pg\dt` primitive type and uses the `DT.*` namespace for all datetime operations.

**Key Features:**
- **Multi-Calendar Support** - Gregorian, Islamic (Hijri), Hebrew, Chinese, Persian, and more
- **Profile-Based Calendars** - Community-driven calendar observations with manual overrides
- **DateTime Literals** - Concise `DT"..."` syntax for inline datetime values
- **Relative Time** - `DT.Ago`, `DT.From.Now` for dynamic dates
- **Recurring Patterns** - `DT.Daily`, `DT.Weekly`, `DT.Monthly` for schedules
- **Timezone Support** - Built-in and custom timezone definitions
- **Extension System** - User-definable timezones, profiles, aliases, and holidays
- **ICU4X Integration** - Battle-tested calendar algorithms and conversions

---

## Table of Contents

1. [The pg\dt Type](#the-pgdt-type)
2. [DateTime Structure](#datetime-structure)
3. [DateTime Literals](#datetime-literals)
4. [DT.* Pipeline Namespace](#dt-pipeline-namespace)
5. [Calendar Systems](#calendar-systems)
6. [Calendar Profiles](#calendar-profiles)
7. [Relative Date Patterns](#relative-date-patterns)
8. [Timezone System](#timezone-system)
9. [DateTime Operations](#datetime-operations)
10. [Recurring Patterns](#recurring-patterns)
11. [Extension System](#extension-system)
12. [Best Practices](#best-practices)

---

## The pg\dt Type

### Type Definition

**Type:** `pg\dt`

**Category:** Primitive Type

**Description:** The DateTime type represents dates, times, durations, and recurring patterns with full calendar and timezone support.

**Default Value:** No default (type must be explicitly initialized)

### Variable Declaration

```polyglot
[~] DateTime variables follow standard variable state machine
[<] .timestamp: pg\dt              [~] Declared state
[<] .scheduled: pg\dt <~ DT.Now""  [~] Default state
[<] .deadline: pg\dt << DT"2025-12-31 23:59:59"  [~] Ready state
```

### State Machine Integration

DateTime variables follow the standard 5-state variable state machine:

| State | Description | Example |
|-------|-------------|---------|
| **Declared** | Type defined, no value | `[<] .dt: pg\dt` |
| **Default** | Has default value | `[<] .dt: pg\dt <~ DT.Now""` |
| **Ready** | Final immutable value | `[<] .dt: pg\dt << DT"2025-11-30"` |
| **Error** | Operation failed | Error info accessible |
| **Close** | Out of scope | Memory cleared |

---

## DateTime Structure

### Internal Structure

Based on brainstorming session 2025-11-29, the `pg\dt` type has this internal structure:

```rust
pub struct DateTime {
    pattern: DateTimePattern,  // Once or Recurrence
    value: DateTimeValue,       // One of 7 variants
    type_: DateTimeType,        // Instant or Duration
}
```

### DateTimePattern

```rust
pub enum DateTimePattern {
    Once,
    Recurrence {
        count: Option<u32>,  // None = unlimited
    }
}
```

**Examples:**
- `DT"2025-11-30"` → `Once`
- `DT.Daily"9:00AM"` → `Recurrence { count: None }`
- `DT.Weekly"Friday 3:00PM"` → `Recurrence { count: None }`

### DateTimeValue (7 Variants)

The value component can be one of 7 variants:

```rust
pub enum DateTimeValue {
    TimeOnly(Time),                          // "3:00 PM"
    DateOnly(Date),                          // "2025-11-30"
    DayOfWeekOnly(DayOfWeek),                // "Friday"
    TimeAndDate { time, date },              // "3:00 PM on 2025-11-30"
    TimeAndDayOfWeek { time, day },          // "3:00 PM every Friday"
    DateAndDayOfWeek { date, day },          // "2025-11-30 (Sunday)"
    All { time, date, day },                 // "3:00 PM on 2025-11-30 (Sunday)"
}
```

**Validation:** `DateAndDayOfWeek` and `All` variants validate that the date actually falls on the specified day of week using ICU4X.

**Examples:**

```polyglot
[~] TimeOnly
[<] .lunch: pg\dt << DT.Time"12:00PM"

[~] DateOnly
[<] .birthday: pg\dt << DT"1990-05-15"

[~] DayOfWeekOnly
[<] .meeting: pg\dt << DT.Weekly"Monday"

[~] TimeAndDate
[<] .appointment: pg\dt << DT"2025-11-30 3:00PM"

[~] TimeAndDayOfWeek
[<] .standup: pg\dt << DT.Daily"Monday 9:00AM"

[~] DateAndDayOfWeek (validated)
[<] .event: pg\dt << DT"2025-11-30 Sunday"  [~] Validates: Nov 30, 2025 IS Sunday

[~] All (fully validated)
[<] .precise: pg\dt << DT"2025-11-30 Sunday 3:00PM"
```

### DateTimeType

```rust
pub enum DateTimeType {
    Instant,
    Duration { units: DurationUnits },
}
```

**Instant:** A specific point in time
**Duration:** A time interval

**Examples:**

```polyglot
[~] Instant
[<] .now: pg\dt << DT.Now""
[<] .deadline: pg\dt << DT"2025-12-31"

[~] Duration
[<] .timeout: pg\dt << DT"30m"
[<] .work_week: pg\dt << DT"5d"
[<] .project_length: pg\dt << DT"3mo 2w"
```

---

## DateTime Literals

### Basic Syntax

**Format:**
```polyglot
DT"formatted_string"
DT.{Calendar}"formatted_string"
DT.{Calendar}.{Profile}"formatted_string"
DT.{Operation}"formatted_string"
```

**String Literal Requirement:** DateTime literals REQUIRE formatted strings with mandatory AM/PM or 24-hour format to avoid ambiguity.

### Date Formats

```polyglot
[~] ISO 8601 date
[<] .date: pg\dt << DT"2025-11-30"

[~] Full day (00:00 to 23:59:59)
[<] .full_day: pg\dt << DT"2025-11-30:"

[~] Wildcard patterns (recurring)
[<] .every_nov_30: pg\dt << DT"*-11-30"          [~] Nov 30 every year
[<] .every_15th: pg\dt << DT"2025-*-15"          [~] 15th of every month in 2025
[<] .every_day_nov: pg\dt << DT"2025-11-*"       [~] Every day in Nov 2025
```

### Time Formats

```polyglot
[~] 24-hour format (MANDATORY for times without AM/PM)
[<] .afternoon: pg\dt << DT.Time"15:00"
[<] .precise: pg\dt << DT.Time"15:00:30"
[<] .milliseconds: pg\dt << DT.Time"15:00:30.500"

[~] 12-hour format (MANDATORY AM/PM)
[<] .morning: pg\dt << DT.Time"9:00AM"
[<] .evening: pg\dt << DT.Time"3:00PM"
[<] .hour_only: pg\dt << DT.Time"3PM"

[~] Midnight (4 valid forms)
[<] .midnight_1: pg\dt << DT.Time"00:00"        [~] ISO 8601 start of day
[<] .midnight_2: pg\dt << DT.Time"24:00"        [~] ISO 8601 end of day
[<] .midnight_3: pg\dt << DT.Time"12:00AM"      [~] 12-hour
[<] .midnight_4: pg\dt << DT.Time"12AM"         [~] Hour only

[~] ❌ INVALID: Ambiguous times
[~] [<] .bad: pg\dt << DT.Time"3:00"    [~] ERROR: Must specify AM/PM or use 24-hour
```

### DateTime Formats

```polyglot
[~] Date + Time (space separator)
[<] .meeting: pg\dt << DT"2025-11-30 15:00"

[~] ISO 8601 (T separator)
[<] .iso_dt: pg\dt << DT"2025-11-30T15:00:00"

[~] ISO 8601 UTC (Z suffix)
[<] .utc: pg\dt << DT"2025-11-30T15:00:00Z"

[~] With timezone
[<] .eastern: pg\dt << DT"2025-11-30 15:00 US/Eastern"
[<] .with_offset: pg\dt << DT"2025-11-30 15:00 +05:00"
```

### Duration Formats

**Syntax Rules:**
1. **Ordered units** - Must be descending order: `y > mo > w > d > h > m > s`
2. **No decimals** - Use `"2h 30m"` NOT `"2.5h"`
3. **No negatives** - Use `DT.Ago"2h"` instead of `"-2h"`
4. **Space-separated** - Multiple units separated by spaces

```polyglot
[~] Single unit
[<] .seconds: pg\dt << DT"30s"
[<] .minutes: pg\dt << DT"5m"
[<] .hours: pg\dt << DT"2h"
[<] .days: pg\dt << DT"1d"
[<] .weeks: pg\dt << DT"1w"
[<] .months: pg\dt << DT"1mo"
[<] .years: pg\dt << DT"1y"

[~] Compound durations (MUST be ordered)
[<] .timeout: pg\dt << DT"2h 30m"
[<] .sprint: pg\dt << DT"2w 3d"
[<] .project: pg\dt << DT"1y 3mo 2w"
[<] .precise: pg\dt << DT"2h 30m 15s"

[~] ❌ INVALID: Wrong order
[~] [<] .bad: pg\dt << DT"30m 2h"       [~] ERROR: m cannot come before h

[~] ❌ INVALID: Decimals
[~] [<] .bad: pg\dt << DT"2.5h"         [~] ERROR: Use "2h 30m"

[~] ❌ INVALID: Negatives
[~] [<] .bad: pg\dt << DT"-2h"          [~] ERROR: Use DT.Ago"2h"
```

### Day of Week Formats

```polyglot
[~] Full names
[<] .friday: pg\dt << DT"Friday"
[<] .monday: pg\dt << DT"Monday"

[~] Abbreviated (3 letters)
[<] .fri: pg\dt << DT"Fri"
[<] .mon: pg\dt << DT"Mon"
```

### Combined Formats

```polyglot
[~] Day + Time
[<] .meeting: pg\dt << DT"Friday 3:00PM"
[<] .standup: pg\dt << DT"Mon 9:00AM"

[~] Date + Day (VALIDATED)
[<] .event: pg\dt << DT"2025-11-30 Sunday"    [~] ICU4X validates match

[~] Date + Day + Time (ALL VALIDATED)
[<] .appointment: pg\dt << DT"2025-11-30 Sunday 3:00PM EST"
```

---

## DT.* Pipeline Namespace

The complete DT.* namespace hierarchy is extensive. Here are the major branches:

### Core Operations

```polyglot
DT""                                   [~] Parse formatted string
DT.Now""                               [~] Current instant
DT.Time"formatted_string"              [~] Time only
DT.Date"formatted_string"              [~] Date only
```

### Temporal Navigation

```polyglot
DT.Ago"duration"                       [~] Past time
DT.From.Now"duration"                  [~] Future time
DT.Add"duration"                       [~] Add duration to instant
DT.Subtract"duration"                  [~] Subtract duration from instant
```

### Recurring Patterns

```polyglot
DT.Daily"time"                         [~] Every day at time
DT.Weekly"day time"                    [~] Every week on day at time
DT.Monthly"day time"                   [~] Monthly on day at time
DT.Yearly"month day"                   [~] Annual on date

[~] Specific intervals
DT.Every.Second""
DT.Every.Minute""
DT.Every.Hour""
DT.Every.Day""
DT.Every.Week""
DT.Every.Month""
DT.Every.Year""
```

### Calendar Systems (Algorithmic)

```polyglot
DT.Gregorian"formatted_string"         [~] Gregorian (default)
DT.Julian"formatted_string"            [~] Julian calendar
DT.Assyrian"formatted_string"          [~] Assyrian/Syriac
DT.Persian"formatted_string"           [~] Persian solar
DT.Coptic"formatted_string"            [~] Coptic calendar
DT.Ethiopian"formatted_string"         [~] Ethiopian calendar
DT.Buddhist"formatted_string"          [~] Buddhist calendar
```

### Calendar Systems (Profile-Based)

Profile-based calendars support community-specific observations:

```polyglot
[~] Islamic calendar with profiles
DT.Hijri"formatted_string"                    [~] Algorithmic (no profile)
DT.Hijri.SaudiArabia"formatted_string"        [~] Umm al-Qura
DT.Hijri.UKMoonSighting"formatted_string"     [~] UK local sighting
DT.Hijri.Turkey"formatted_string"             [~] Turkey (Diyanet)
DT.Hijri.{UserDefined}"formatted_string"      [~] Custom profiles

[~] Hebrew calendar with profiles
DT.Hebrew"formatted_string"                   [~] Algorithmic
DT.Hebrew.Sephardic"formatted_string"         [~] Sephardic tradition
DT.Hebrew.Ashkenazi"formatted_string"         [~] Ashkenazi tradition
DT.Hebrew.{UserDefined}"formatted_string"     [~] Custom profiles

[~] Chinese calendar with profiles
DT.Chinese"formatted_string"                  [~] Algorithmic
DT.Chinese.Mainland"formatted_string"         [~] PRC observance
DT.Chinese.Taiwan"formatted_string"           [~] Taiwan observance
DT.Chinese.{UserDefined}"formatted_string"    [~] Custom profiles
```

### Timezone Namespace

```polyglot
[~] Built-in regional timezones
DT.TimeZone.US.Eastern"datetime_string"
DT.TimeZone.US.Central"datetime_string"
DT.TimeZone.US.Mountain"datetime_string"
DT.TimeZone.US.Pacific"datetime_string"
DT.TimeZone.Europe.London"datetime_string"
DT.TimeZone.Asia.Tokyo"datetime_string"
DT.TimeZone.Asia.Shanghai"datetime_string"

[~] User-defined timezones (extendable)
DT.TimeZone.{UserDefined}"datetime_string"
```

### Relative Date Patterns

Using dot hierarchy (no natural language parsing):

```polyglot
[~] First/Last weekday in month
DT.Gregorian.{Month}.First.{DayOfWeek}""
DT.Gregorian.{Month}.Second.{DayOfWeek}""
DT.Gregorian.{Month}.Third.{DayOfWeek}""
DT.Gregorian.{Month}.Fourth.{DayOfWeek}""
DT.Gregorian.{Month}.Fifth.{DayOfWeek}""
DT.Gregorian.{Month}.Last.{DayOfWeek}""

[~] Examples
DT.Gregorian.November.Fourth.Thursday""       [~] US Thanksgiving
DT.Gregorian.September.First.Monday""         [~] US Labor Day
DT.Hijri.SaudiArabia.Ramadan.Last.Friday""    [~] Last Friday in Ramadan
```

### Operations

```polyglot
[~] Comparison and membership
DT.IsBefore"other_datetime"
DT.IsAfter"other_datetime"
DT.IsBetween"start, end"
DT.Contains"instant"                   [~] Duration contains instant?

[~] Interval operations
DT.Overlaps"other_duration"
DT.Intersection"other_duration"
DT.Union"other_duration"
DT.Gap"other_duration"

[~] Extraction
DT.Year""
DT.Month""
DT.Day""
DT.Hour""
DT.Minute""
DT.Second""
DT.DayOfWeek""

[~] Conversion
DT.Convert.Hijri""
DT.Convert.Hebrew""
DT.Convert.Chinese""
DT.Convert.Gregorian""
DT.ToUTC""
DT.ToLocal""
```

### Extension Points

Users can extend these namespaces:

```polyglot
[~] Extendable reserved enumerations
#DT.TimeZone.{UserDefined}
#DT.Hijri.{UserProfile}
#DT.Hebrew.{UserProfile}
#DT.Chinese.{UserProfile}
#DT.Alias.{UserAlias}
#DT.Holiday.{UserHoliday}
#DT.Format.{UserFormat}
```

**See:** [DT Pipeline Tree](../../technical/dt-pipeline-tree.md) for complete namespace hierarchy (100+ pipelines)

---

## Calendar Systems

### Algorithmic Calendars (MVP)

These calendars use pure algorithmic calculations (ICU4X):

**Supported in MVP:**
- **Gregorian** - Standard Western calendar (default)
- **Julian** - Pre-Gregorian calendar
- **Assyrian/Syriac** - Julian + 311 year offset
- **Islamic Civil** - Algorithmic Islamic calendar (no moon sighting)

```polyglot
[~] Gregorian (default)
[<] .date: pg\dt << DT"2025-11-30"
[<] .explicit: pg\dt << DT.Gregorian"2025-11-30"

[~] Julian
[<] .julian: pg\dt << DT.Julian"2025-11-17"     [~] 13 days behind Gregorian

[~] Assyrian
[<] .assyrian: pg\dt << DT.Assyrian"2336-11-17"

[~] Islamic Civil (algorithmic only)
[<] .hijri: pg\dt << DT.Hijri"1447-09-01"
```

### Profile-Based Calendars (MVP + Post-MVP)

Profile-based calendars support community-specific observations that may differ from algorithmic calculations.

**MVP:** Basic profile structure
**Post-MVP:** Full API integration, marketplace

---

## Calendar Profiles

### Profile System Overview

Many calendars depend on community observations rather than pure algorithms:

**Example:** Islamic calendar dates vary by region due to moon sighting differences:
- Saudi Arabia follows Umm al-Qura calculation
- UK follows local moon sighting committees
- Turkey follows Diyanet (Turkish Religious Affairs)
- Egypt follows Dar al-Ifta

**Solution:** Profile-based system with 3-tier priority

### 3-Tier Priority System

```
Priority 1: Manual Overrides (HR-entered observed dates)
Priority 2: API Cache (moon sighting services) [POST-MVP]
Priority 3: ICU4X Calculated (algorithmic fallback)
```

### Profile Structure Example

**File:** `calendars/MyCompanyHijri.yaml`

```yaml
calendar:
  type: "Hijri"
  profile_name: "MyCompanyHR"
  description: "Company HR Hijri calendar"

  # Manual overrides (Priority 1)
  observed_dates:
    "2025-03-30": "1446-09-01"  # Ramadan start observed
    "2025-04-28": "1446-10-01"  # Shawwal start observed

  # Calculation method (Priority 3)
  calculation:
    method: "ICU4X"
    algorithm: "Umm al-Qura"
```

### Using Profiles

```polyglot
[~] Define profile as extendable enumeration
[#] DT.Hijri.MyCompanyHR
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\calendars\\MyCompanyHijri.yaml"
[<] .overrides: pg\yaml << .config.calendar.observed_dates
[s][!] *
[X]

[~] Use profile in datetime literals
[<] .ramadan_start: pg\dt << DT.Hijri.MyCompanyHR"1446-09-01"
[<] .current_hijri: pg\dt << DT.Hijri.MyCompanyHR.Now""
```

**See:** [DateTime Extension Guide](../datetime-extension-guide.md) for complete profile creation guide

---

## Relative Date Patterns

### Dot Hierarchy Pattern

Relative dates use explicit dot hierarchy (NO natural language parsing):

**Pattern:**
```
DT.{Calendar}.{Month}.{Occurrence}.{DayOfWeek}""
```

**Components:**
- **Calendar:** Gregorian, Hijri, Hebrew, Chinese, etc.
- **Month:** Month name or number
- **Occurrence:** First, Second, Third, Fourth, Fifth, Last
- **DayOfWeek:** Monday, Tuesday, ..., Sunday (or Mon, Tue, etc.)

### Examples

```polyglot
[~] US Thanksgiving (4th Thursday in November)
[<] .thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""

[~] US Labor Day (1st Monday in September)
[<] .labor_day: pg\dt << DT.Gregorian.September.First.Monday""

[~] Last Friday in Ramadan (with profile)
[<] .jumah: pg\dt << DT.Hijri.SaudiArabia.Ramadan.Last.Friday""

[~] First Monday of any month
[<] .monthly_meeting: pg\dt << DT.Gregorian.*.First.Monday""    [~] Wildcard month
```

### With Time Component

```polyglot
[~] Pattern includes time
DT.{Calendar}.{Month}.{Occurrence}.{DayOfWeek}"time"

[~] Examples
[<] .standup: pg\dt << DT.Gregorian.*.First.Monday"9:00AM"
[<] .prayer: pg\dt << DT.Hijri.SaudiArabia.Ramadan.Last.Friday"1:00PM"
```

### Recurring Relative Patterns

```polyglot
[~] Every year
[<] .annual_thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""

[~] With recurrence count
[<] .next_5_labor_days: pg\dt << DT.Gregorian.September.First.Monday""
    | DT.WithCount(5)
```

---

## Timezone System

### Built-In Timezones

Polyglot provides hierarchical timezone namespace:

```polyglot
[~] US Timezones
DT.TimeZone.US.Eastern"datetime_string"
DT.TimeZone.US.Central"datetime_string"
DT.TimeZone.US.Mountain"datetime_string"
DT.TimeZone.US.Pacific"datetime_string"

[~] European Timezones
DT.TimeZone.Europe.London"datetime_string"
DT.TimeZone.Europe.Paris"datetime_string"
DT.TimeZone.Europe.Berlin"datetime_string"

[~] Asian Timezones
DT.TimeZone.Asia.Tokyo"datetime_string"
DT.TimeZone.Asia.Shanghai"datetime_string"
DT.TimeZone.Asia.Dubai"datetime_string"
```

### Timezone in Literals

```polyglot
[~] UTC
[<] .utc: pg\dt << DT"2025-11-30T15:00:00Z"

[~] Named timezone
[<] .eastern: pg\dt << DT"2025-11-30 15:00 US/Eastern"

[~] Offset format
[<] .offset: pg\dt << DT"2025-11-30 15:00 +05:00"

[~] Using timezone namespace
[<] .tokyo: pg\dt << DT.TimeZone.Asia.Tokyo"2025-11-30 15:00"
```

### Custom Timezones

Users can define custom timezones with DST rules:

```polyglot
[#] DT.TimeZone.MyCompany
[<] .gmt_offset: pg\dt << DT.Ago"3h"
[<] .dst_offset: pg\dt << DT"1h"
[<] .dst_start: pg\dt << DT"*-03-01"    [~] March 1 every year
[<] .dst_end: pg\dt << DT"*-09-01"      [~] September 1
[s][!] *
[X]

[~] Use custom timezone
[<] .meeting: pg\dt << DT.TimeZone.MyCompany"2025-11-30 15:00"
```

**See:** [DateTime Extension Guide](../datetime-extension-guide.md) - Custom Timezones section

---

## DateTime Operations

### Equality and Comparison

**Equality Semantics:**
- **Instant**: Exact equality check
- **Duration**: Membership test (contains)

```polyglot
[~] Instant equality
[<] .a: pg\dt << DT"2025-11-30 15:00"
[<] .b: pg\dt << DT"2025-11-30 15:00"
[?] @.a =? @.b
  [o] "Equal!"                              [~] Exact match

[~] Duration membership (contains)
[<] .duration: pg\dt << DT"2h"              [~] 2-hour duration
[<] .point: pg\dt << DT.Ago"1h"             [~] 1 hour ago
[?] @.duration | DT.Contains(@.point)
  [o] "Point is inside duration!"

[~] Comparison operators (all use ? suffix)
[?] .date1 >? .date2                        [~] After
[?] .date1 <? .date2                        [~] Before
[?] .date1 =>? .date2                       [~] After or equal
[?] .date1 =<? .date2                       [~] Before or equal
[?] .date1 =? .date2                        [~] Equal
[?] .date1 =!? .date2                       [~] Not equal
```

### Arithmetic Operations

```polyglot
[~] Add duration to instant
[<] .start: pg\dt << DT"2025-11-30"
[<] .future: pg\dt << @.start | DT.Add"7d"  [~] Add 7 days

[~] Subtract duration
[<] .past: pg\dt << @.start | DT.Subtract"2d"

[~] Duration between instants
[<] .duration: pg\dt << @.start | DT.DurationUntil(@.future)
[o] @.duration                              [~] Output: "7d"
```

### Interval Operations

```polyglot
[~] Check overlap
[<] .meeting1: pg\dt << DT"2025-11-30 14:00" to DT"2025-11-30 15:00"
[<] .meeting2: pg\dt << DT"2025-11-30 14:30" to DT"2025-11-30 15:30"
[?] @.meeting1 | DT.Overlaps(@.meeting2)
  [o] "Meetings overlap!"

[~] Find intersection
[<] .overlap: pg\dt << @.meeting1 | DT.Intersection(@.meeting2)
[o] @.overlap                               [~] 14:30 to 15:00

[~] Check containment
[?] @.meeting1 | DT.ContainsInterval(@.shorter_meeting)
  [o] "Meeting 1 fully contains shorter meeting"

[~] Find gap between intervals
[<] .gap: pg\dt << @.meeting1 | DT.Gap(@.meeting2)
```

### Extraction Operations

```polyglot
[<] .dt: pg\dt << DT"2025-11-30 15:30:45"

[<] .year: pg\int << @.dt | DT.Year         [~] 2025
[<] .month: pg\int << @.dt | DT.Month       [~] 11
[<] .day: pg\int << @.dt | DT.Day           [~] 30
[<] .hour: pg\int << @.dt | DT.Hour         [~] 15
[<] .minute: pg\int << @.dt | DT.Minute     [~] 30
[<] .second: pg\int << @.dt | DT.Second     [~] 45
[<] .dow: pg\string << @.dt | DT.DayOfWeek  [~] "Sunday"
```

### Conversion Operations

```polyglot
[~] Calendar conversion
[<] .greg: pg\dt << DT"2025-11-30"
[<] .hijri: pg\dt << @.greg | DT.Convert.Hijri
[o] @.hijri                                 [~] "1447-05-29" (algorithmic)

[~] Timezone conversion
[<] .utc: pg\dt << DT"2025-11-30 15:00Z"
[<] .eastern: pg\dt << @.utc | DT.Convert.TimeZone("US/Eastern")
[o] @.eastern                               [~] "2025-11-30 10:00 EST"

[~] Convert to UTC
[<] .to_utc: pg\dt << @.eastern | DT.ToUTC

[~] Convert to system local
[<] .local: pg\dt << @.utc | DT.ToLocal
```

---

## Recurring Patterns

### Daily Recurrence

```polyglot
[~] Every day at specific time
[<] .daily_standup: pg\dt << DT.Daily"9:00AM"

[~] Every day at current time
[<] .daily_now: pg\dt << DT.Daily""
```

### Weekly Recurrence

```polyglot
[~] Every week on specific day and time
[<] .weekly_meeting: pg\dt << DT.Weekly"Friday 3:00PM"

[~] Every Monday
[<] .mondays: pg\dt << DT.Weekly"Monday"
```

### Monthly Recurrence

```polyglot
[~] First Monday of every month
[<] .monthly_meeting: pg\dt << DT.Monthly"First Monday 9:00AM"

[~] 15th of every month
[<] .mid_month: pg\dt << DT.Monthly"15"

[~] Last day of every month
[<] .month_end: pg\dt << DT.Monthly"Last"
```

### Yearly Recurrence

```polyglot
[~] Annual event
[<] .birthday: pg\dt << DT.Yearly"05-15"

[~] With specific day
[<] .thanksgiving: pg\dt << DT.Yearly.Gregorian.November.Fourth.Thursday""
```

### Custom Intervals

```polyglot
[~] Every N seconds/minutes/hours
[<] .every_30s: pg\dt << DT.Every"30s"
[<] .every_15m: pg\dt << DT.Every"15m"
[<] .every_6h: pg\dt << DT.Every"6h"

[~] Named intervals
[<] .every_second: pg\dt << DT.Every.Second""
[<] .every_minute: pg\dt << DT.Every.Minute""
[<] .every_hour: pg\dt << DT.Every.Hour""
[<] .every_day: pg\dt << DT.Every.Day""
[<] .every_week: pg\dt << DT.Every.Week""
```

### Recurrence with Count

```polyglot
[~] Limited recurrence
[<] .next_5_days: pg\dt << DT.Daily"9:00AM" | DT.WithCount(5)

[~] Unlimited recurrence (default)
[<] .forever: pg\dt << DT.Daily"9:00AM"     [~] count = None
```

### Trigger Integration

```polyglot
[|] ScheduledTask
[t] |T.DT.*                                 [~] Trigger pattern mirrors DT.*
[<] .pattern: pg\dt << DT.Daily"9:00AM"

[~] Task logic
[X]

[~] Alternative: Direct trigger syntax
[|] DailyReport
[t] |T.Daily"9:00AM"                        [~] T.DT.* = Trigger when DT.* is now
[~] Task logic
[X]
```

**Note:** `T.DT.*` triggers fire when `DT.*` equals `DT.Now""` (current instant)

---

## Extension System

### Overview

Polyglot's DateTime system is fully extensible through **extendable reserved enumerations**.

**What You Can Extend:**
- 🌍 Custom timezones with DST rules
- 📅 Calendar profiles with community observations
- 🔖 Datetime aliases (shortcuts)
- 🎉 Holiday definitions
- 📐 Custom format macros

### Extension Pattern

All DateTime extensions follow the same pattern:

```polyglot
[#] DT.{Namespace}.{YourName}
[~] Initialization logic
[s] |{pipeline that produces datetime value}
[s][!] *                                    [~] Export enumeration
[X]
```

### Alias Example

```polyglot
[~] Create datetime alias for end-of-day
[#] DT.Alias.EOD
[s] |DT.Time"23:59:59"
[s][!] *
[X]

[~] Use alias
[<] .deadline: pg\dt << DT.Alias.EOD
```

### Timezone Extension Example

```polyglot
[#] DT.TimeZone.MyCompany
[<] .gmt_offset: pg\dt << DT.Ago"3h"
[<] .dst_enabled: pg\bool << true
[<] .dst_offset: pg\dt << DT"1h"
[<] .dst_start: pg\dt << DT"*-03-01"
[<] .dst_end: pg\dt << DT"*-09-01"
[s][!] *
[X]

[~] Use custom timezone
[<] .meeting: pg\dt << DT.TimeZone.MyCompany"2025-11-30 15:00"
```

### Holiday Extension Example

```polyglot
[#] DT.Holiday.Company
[<] .founders_day: pg\dt << DT"2025-03-15"
[<] .summer_shutdown: pg\dt << DT"2025-07-01" to DT"2025-07-15"
[<] .thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""
[s][!] *
[X]

[~] Check if today is holiday
[?] DT.Now"" =? DT.Holiday.Company.founders_day
  [o] "Happy Founders Day!"
```

**See:** [DateTime Extension Guide](../datetime-extension-guide.md) for comprehensive extension documentation

---

## Best Practices

### 1. Always Specify AM/PM or Use 24-Hour Format

```polyglot
[~] ✅ CORRECT
[<] .time1: pg\dt << DT.Time"15:00"         [~] 24-hour unambiguous
[<] .time2: pg\dt << DT.Time"3:00PM"        [~] 12-hour with AM/PM
[<] .time3: pg\dt << DT.Time"3PM"           [~] Hour only with PM

[~] ❌ WRONG
[~] [<] .bad: pg\dt << DT.Time"3:00"        [~] ERROR: Ambiguous!
```

### 2. Use Appropriate Calendar for Context

```polyglot
[~] ✅ CORRECT - Use calendar matching cultural context
[<] .ramadan: pg\dt << DT.Hijri.SaudiArabia"1447-09-01"
[<] .passover: pg\dt << DT.Hebrew.Sephardic"Nisan 15"
[<] .christmas: pg\dt << DT.Gregorian"2025-12-25"

[~] ❌ WRONG - Calendar mismatch
[~] [<] .ramadan: pg\dt << DT"2025-03-30"   [~] Should use Hijri
```

### 3. Use Profiles for Community Calendars

```polyglot
[~] ✅ CORRECT - Use profiles for observable calendars
[<] .eid: pg\dt << DT.Hijri.MyCompanyHR"1447-10-01"

[~] ❌ RISKY - Algorithmic may differ from observed
[~] [<] .eid: pg\dt << DT.Hijri"1447-10-01" [~] May not match community
```

### 4. Store UTC, Display Local

```polyglot
[~] ✅ CORRECT - Store in UTC
[<] .stored: pg\dt << DT"2025-11-30T15:00:00Z"

[~] Convert for display
[<] .display: pg\dt << @.stored | DT.ToLocal
```

### 5. Use Relative Time for Dynamic Dates

```polyglot
[~] ✅ CORRECT - Dynamic
[<] .cutoff: pg\dt << DT.Ago"7d"            [~] Always "7 days ago"

[~] ❌ WRONG - Static, becomes outdated
[~] [<] .cutoff: pg\dt << DT"2025-11-23"
```

### 6. Validate Date/Day Combinations

```polyglot
[~] ✅ CORRECT - ICU4X validates automatically
[<] .event: pg\dt << DT"2025-11-30 Sunday"  [~] Validates: IS Sunday

[~] ❌ WRONG - Validation will fail
[~] [<] .bad: pg\dt << DT"2025-11-30 Monday" [~] ERROR: Nov 30 is NOT Monday
```

### 7. Use Dot Hierarchy for Relative Dates

```polyglot
[~] ✅ CORRECT - Explicit dot hierarchy
[<] .thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""

[~] ❌ WRONG - Natural language not supported
[~] [<] .bad: pg\dt << DT"Fourth Thursday in November"
```

### 8. Respect Duration Ordering

```polyglot
[~] ✅ CORRECT - Descending order
[<] .duration: pg\dt << DT"2h 30m 15s"

[~] ❌ WRONG - Out of order
[~] [<] .bad: pg\dt << DT"30m 2h"           [~] ERROR
[~] [<] .bad: pg\dt << DT"2.5h"             [~] ERROR: No decimals
```

### 9. Use Extensions for Organization-Specific Needs

```polyglot
[~] ✅ CORRECT - Define reusable extensions
[#] DT.Alias.WorkDay
[s] |DT.Weekly"Monday-Friday 9:00AM-5:00PM"
[s][!] *
[X]

[<] .schedule: pg\dt << DT.Alias.WorkDay
```

### 10. Handle Timezones Explicitly

```polyglot
[~] ✅ CORRECT - Explicit timezone
[<] .meeting: pg\dt << DT.TimeZone.US.Eastern"2025-11-30 15:00"

[~] ❌ RISKY - Ambiguous timezone
[~] [<] .meeting: pg\dt << DT"2025-11-30 15:00"
```

---

## MVP Scope & Limitations

### What's Supported (MVP)

✅ **Core DateTime Functionality:**
- `pg\dt` primitive type with state machine integration
- DateTime literals with formatted strings
- 7 DateTimeValue variants (TimeOnly, DateOnly, etc.)
- Duration type with ordered units

✅ **Calendar Systems:**
- Gregorian (default)
- Julian
- Assyrian/Syriac
- Islamic Civil (algorithmic)

✅ **Basic Operations:**
- Equality and comparison
- Arithmetic (add, subtract)
- Extraction (year, month, day, etc.)
- Basic conversion

✅ **Recurring Patterns:**
- Daily, Weekly, Monthly, Yearly
- Custom intervals

✅ **Extension System:**
- Custom timezones (basic)
- Calendar profiles (basic structure)
- Aliases, holidays, formats

✅ **Validation:**
- Date/DayOfWeek matching (ICU4X)
- Duration unit ordering
- AM/PM or 24-hour requirement

### What's Deferred (Post-MVP)

🔜 **Performance Optimizations:**
- Profile loading performance optimization
- Lazy loading for large profile collections
- Profile caching strategies

🔜 **Robustness:**
- API failure handling (timeouts, retries, circuit breakers)
- YAML security validation and sanitization
- Path traversal protection

🔜 **Advanced Features:**
- Full API integration (Priority 2 in profile system)
- Profile marketplace and community distribution
- Profile versioning and migration
- Natural language datetime parsing

🔜 **Additional Calendars:**
- Hebrew (with rabbinical profiles)
- Chinese (with regional profiles)
- Buddhist (with regional variations)
- Hindu (with Panchang integration)
- Coptic, Ethiopian, Persian

### MVP Deployment Best Practices

For MVP deployments:

1. **Use Local Profiles** - Avoid API dependencies
2. **Pre-validate YAML** - Check syntax before deployment
3. **Keep Profiles Simple** - Minimize manual override lists
4. **Document Changes** - Comment profile modifications
5. **Test Timezone Conversions** - Verify DST handling
6. **Explicit Timezones** - Always specify timezone in literals

---

## See Also

### Related Documentation

- **[DateTime Extension Guide](../datetime-extension-guide.md)** - Complete guide to extending the DateTime system
- **[DT Pipeline Tree](../../technical/dt-pipeline-tree.md)** - Full DT.* namespace hierarchy (100+ pipelines)
- **[DateTime Formatted String Grammar](../../technical/datetime-formatted-string-grammar.md)** - Complete BNF grammar specification
- **[DateTime String Literal Specification](../../technical/datetime-string-literal-specification.md)** - Detailed syntax reference

### Language Specification

- **[Type System](type-system.md)** - `pg\dt` type details and state machine
- **[Variables User Guide](variables-user-guide.md)** - Variable state management
- **[Operators](operators.md)** - Comparison operators with `?` suffix

### Standard Library

- **[Triggers](../../standard-library/triggers.md)** - Time-based triggers (`T.DT.*`)
- **[Reserved Enumerations](../../standard-library/reserved-enumerations.md)** - `#DT.*` enumerations

### Examples

- **[Scheduled Tasks](../../examples/scheduled-tasks.md)** - DateTime trigger examples
- **[Multi-Calendar Workflows](../../examples/multi-calendar.md)** - Calendar conversion examples

### Design Documentation

- **[Brainstorming Session 2025-11-29](../../project/brainstorming-session-results-2025-11-29.md)** - DateTime system design session

---

**End of DateTime System Reference**
