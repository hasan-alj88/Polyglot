# DateTime String Literal Specification

**Document Status**: Draft
**Last Updated**: 2025-11-30
**Related**: Brainstorming Session 2025-11-29, Story 2.1 (IR Type Definitions)

---

## Overview

All `pg\dt` (DateTime) values in Polyglot are created using **inline pipeline syntax** with string literals following the pattern `DT.*"formatted string"`.

**Core Principle**: Every `DT.*` pattern corresponds to a standard library pipeline that returns `pg\dt`.

---

## Existing Usage Patterns (Documented)

### Found in Current Docs

**Basic DateTime Operations:**
```polyglot
DT.Now""                      // Current instant (empty params mandatory)
DT.Minutes"5"                 // Duration: 5 minutes from now
DT.ToNow"{.start}"            // Duration from timestamp to now
DT.Ago"2h"                    // Relative instant: 2 hours in past
```

**Calendar-Specific Dates:**
```polyglot
DT.Gregorian"2024-01-15"      // Gregorian calendar date
DT.Hijri"1447-09-01:"         // Islamic calendar date (colon = full day)
DT.Chinese"4722-01-15"        // Chinese calendar date
DT.Hebrew"5784-10-03"         // Hebrew calendar date
```

**Recurring Patterns:**
```polyglot
DT.Every"Sun"                 // Every Sunday
DT.Hijri.Yearly"09-01:"       // Every Ramadan 1st (recurring yearly)
```

**Shorthand Formats:**
```polyglot
DT"Mon"                       // Day of week (calendar inferred)
DT"2025-11-07:"               // Date (calendar inferred, colon = full day)
DT"5m"                        // Duration shorthand (5 minutes)
DT"30s"                       // Duration shorthand (30 seconds)
```

**Trigger Usage:**
```polyglot
T.DT.Daily"3:00AM"            // Trigger: Daily at 3:00 AM
```

---

## Complete String Literal Format Specification

Based on the brainstorming session's **7 DateTimeValue variants** and existing patterns.

### Category 1: Instant (Point in Time)

#### 1.1 Current Time
```polyglot
DT.Now""                      // Current instant (UTC)
DT.Now.Local""                // Current instant (local timezone)
```

**Returns**: `DateTime { pattern: Once, value: TimeAndDate, type: Instant }`

---

#### 1.2 Specific Date (DateOnly variant)

**Gregorian Calendar:**
```polyglot
DT.Gregorian"2025-11-30"      // ISO 8601 date
DT.Gregorian"2025-11-30 EST"  // With timezone
DT"2025-11-30"                // Shorthand (Gregorian assumed)
```

**Islamic Calendar:**
```polyglot
DT.Hijri"1447-09-01"                      // Islamic date (algorithmic - no profile)
DT.Hijri"1447-09-01 EST"                  // With timezone
DT.Hijri.SaudiArabia"1447-09-01"          // Built-in Saudi Arabia profile
DT.Hijri.UKMoonSighting"1447-09-01"       // Built-in UK moon sighting profile
DT.Hijri.Turkey"1447-09-01"               // Built-in Turkey profile
DT.Hijri.MyCompanyHR"1447-09-01"          // User-defined custom profile
```

**Other Calendars:**
```polyglot
DT.Julian"2025-11-17"                     // Julian calendar (algorithmic)
DT.Assyrian"2336-11-30"                   // Assyrian/Syriac calendar (algorithmic)
DT.Chinese"4722-01-15"                    // Chinese calendar (algorithmic)
DT.Chinese.HongKong"4722-01-15"           // Chinese with HongKong profile
DT.Hebrew"5784-10-03"                     // Hebrew calendar (algorithmic)
DT.Hebrew.Sephardic"5784-10-03"           // Hebrew with Sephardic authority
DT.Buddhist"2568-11-30"                   // Buddhist calendar (algorithmic)
DT.Persian"1404-09-09"                    // Persian (Solar Hijri) (algorithmic)
```

**Returns**: `DateTime { pattern: Once, value: DateOnly(Date), type: Instant }`

---

#### 1.2.1 Calendar Profile Syntax (Extendable Reserved Enumerations)

**Profile-aware calendars** use **extendable reserved enumerations** (`#DT.Hijri.*`, `#DT.Hebrew.*`, etc.).

### Pattern: `DT.{Calendar}.{ProfileName}"Formatted String"`

**General Form**: `DT.{Calendar}.{Profile}"..."`

**Syntax**:
```polyglot
// Algorithmic (no profile)
DT.Hijri"1447-09-01"                      // Default algorithmic calculation

// Built-in profiles
DT.Hijri.SaudiArabia"1447-09-01"          // Saudi Arabia (Umm al-Qura)
DT.Hijri.UKMoonSighting"1447-09-01"       // UK local moon sighting
DT.Hijri.Turkey"1447-09-01"               // Turkey (Diyanet)
DT.Hijri.Egypt"1447-09-01"                // Egypt (Dar al-Ifta)

// User-defined custom profiles
DT.Hijri.MyCompanyHR"1447-09-01"          // Custom company profile
```

### Defining Custom Profiles

**Users define profiles as extendable enumerations** that load YAML configuration:

```polyglot
// Define custom Hijri profile for company HR
[#] DT.Hijri.MyCompanyHR
[s] |U.YAML.Load"\\FileDir\\MyCompanyHRHijri.yaml"
[s][!] *                                  // Expand all fields
[X]

// Later: Use custom profile in triggers
[|] EidAlFitrNotification
[t] |T.DT.Hijri.MyCompanyHR.Yearly"10-01 3:00AM"  // Eid al-Fitr
[o] .notify: pg\string
[X]
```

**Profile YAML Format** (`MyCompanyHRHijri.yaml`):
```yaml
# Hijri Calendar Profile
profile_id: "mycompany-hr"
name: "MyCompany HR Islamic Calendar"
region: "Global"
authority: "Company HR Department"
editable: true

# Manual overrides (Priority 1)
manual_overrides:
  - year: 1447
    month: 9
    day: 1
    gregorian_date: "2025-02-28"
    source: "HR Announcement 2025-02-20"
    notes: "Ramadan 1447 starts Friday, Feb 28"

  - year: 1447
    month: 10
    day: 1
    gregorian_date: "2025-03-30"
    source: "HR Announcement 2025-03-25"
    notes: "Eid al-Fitr 1447 on Sunday, Mar 30"

# API integration (Priority 2 - optional)
api_config:
  enabled: false
  source: null
  cache_duration_days: 365

# Default calculation method (Priority 3 - fallback)
default_method: "UmmAlQura"
```

### Built-in Profiles (Available by Default)

**Islamic Calendar** (`#DT.Hijri.*`):
- `DT.Hijri` - Algorithmic (no profile)
- `DT.Hijri.SaudiArabia` - Umm al-Qura calculation
- `DT.Hijri.UKMoonSighting` - UK local moon sighting
- `DT.Hijri.Turkey` - Diyanet (Turkey)
- `DT.Hijri.Egypt` - Dar al-Ifta (Egypt)
- `DT.Hijri.ISNA` - Islamic Society of North America
- `DT.Hijri.MWL` - Muslim World League

**Hebrew Calendar** (`#DT.Hebrew.*`):
- `DT.Hebrew` - Algorithmic (no profile)
- `DT.Hebrew.Sephardic` - Sephardic rabbinical authority
- `DT.Hebrew.Ashkenazi` - Ashkenazi rabbinical authority
- `DT.Hebrew.Israel` - Chief Rabbinate of Israel

**Chinese Calendar** (`#DT.Chinese.*`):
- `DT.Chinese` - Algorithmic (no profile)
- `DT.Chinese.Mainland` - Mainland China variant
- `DT.Chinese.HongKong` - Hong Kong variant
- `DT.Chinese.Taiwan` - Taiwan variant

### Profile Priority Hierarchy

Each profile uses a **3-tier priority system**:

1. **Manual Overrides** (P1) - HR entered observed dates (highest priority)
2. **API Cache** (P2) - Moon sighting service data
3. **ICU4X Calculated** (P3) - Algorithmic fallback (lowest priority)

**Example**:
```polyglot
// Query: What date is Ramadan 1, 1447?
DT.Hijri.MyCompanyHR"1447-09-01"

// Resolution:
// 1. Check manual_overrides → Found: "2025-02-28" ✅ RETURN
// 2. Check API cache → (skipped, P1 found)
// 3. Calculate with ICU4X → (skipped, P1 found)
```

### Namespace Pattern

**Extendable Enumeration Structure**:
```
#DT.{Calendar}.*                  // Extendable enumeration root
  ├─ DT.{Calendar}                // Algorithmic (no profile)
  ├─ .{BuiltInProfile1}           // Built-in profile 1
  ├─ .{BuiltInProfile2}           // Built-in profile 2
  └─ .{UserDefinedProfile}        // User-defined custom profile
```

**Applies to all profile-aware calendars**:
- `#DT.Hijri.*` - Islamic moon sighting variations
- `#DT.Hebrew.*` - Rabbinical authority variations
- `#DT.Chinese.*` - Regional variations
- `#DT.Buddhist.*` - Regional variations (if needed)

---

#### 1.3 Specific Time (TimeOnly variant)

**IMPORTANT**: Times MUST specify AM/PM (12-hour) OR use 24-hour format (Hrs) to avoid confusion.

```polyglot
// ✅ Valid: 24-hour format (Hrs)
DT.Time"15:00"                // 3:00 PM in 24-hour
DT.Time"15:00:30"             // With seconds
DT"15:00"                     // Shorthand (24-hour)
DT"09:00"                     // 9:00 AM in 24-hour

// ✅ Valid: 12-hour format with AM/PM
DT.Time"3:00PM"               // 12-hour with PM
DT.Time"3:00AM"               // 12-hour with AM
DT.Time"3:00:30PM EST"        // With seconds + timezone
DT"3:00PM"                    // Shorthand (12-hour)

// ❌ INVALID: Ambiguous time (missing AM/PM or 24-hour)
DT.Time"3:00"                 // ERROR: Is this 3 AM or 3 PM?
DT"3:00"                      // ERROR: Ambiguous
```

**Returns**: `DateTime { pattern: Once, value: TimeOnly(Time), type: Instant }`

---

#### 1.4 DateTime (TimeAndDate variant)

**IMPORTANT**: Time component MUST specify AM/PM or use 24-hour format.

```polyglot
// ✅ Valid: 24-hour format
DT.Gregorian"2025-11-30 15:00"           // ISO 8601 datetime (24-hour)
DT.Gregorian"2025-11-30T15:00:00Z"       // ISO 8601 with seconds + UTC
DT"2025-11-30 15:00"                     // Shorthand (24-hour)
DT.Hijri"1447-09-01 15:00"               // Islamic date + 24-hour time

// ✅ Valid: 12-hour with AM/PM
DT.Gregorian"2025-11-30 3:00PM EST"      // 12-hour + timezone
DT.Hijri"1447-09-01 3:00PM"              // Islamic date + 12-hour time
DT"2025-11-30 3:00PM"                    // Shorthand (12-hour)

// ❌ INVALID: Ambiguous time
DT"2025-11-30 3:00"                      // ERROR: 3 AM or 3 PM?
```

**Returns**: `DateTime { pattern: Once, value: TimeAndDate { time, date }, type: Instant }`

---

#### 1.5 Day of Week Only (DayOfWeekOnly variant)

```polyglot
DT.Day"Monday"                // Full name
DT.Day"Mon"                   // Abbreviated
DT"Mon"                       // Shorthand
DT"Friday"                    // Shorthand
```

**Returns**: `DateTime { pattern: Once, value: DayOfWeekOnly(DayOfWeek), type: Instant }`

---

#### 1.6 Time + Day of Week (TimeAndDayOfWeek variant)

**IMPORTANT**: Time MUST specify AM/PM or use 24-hour format.

```polyglot
// ✅ Valid: 12-hour with AM/PM
DT"Friday 3:00PM"             // Friday at 3:00 PM
DT"Mon 9:00AM"                // Monday at 9:00 AM
DT.Day"Friday at 3:00PM"      // Explicit format

// ✅ Valid: 24-hour format
DT"Friday 15:00"              // Friday at 15:00 (3 PM)
DT"Mon 09:00"                 // Monday at 09:00 (9 AM)

// ❌ INVALID: Ambiguous time
DT"Friday 3:00"               // ERROR: 3 AM or 3 PM?
```

**Returns**: `DateTime { pattern: Once, value: TimeAndDayOfWeek { time, day }, type: Instant }`

---

#### 1.7 Date + Day of Week with Validation (DateAndDayOfWeek variant)

```polyglot
DT.Gregorian"2025-11-30 Sunday"          // Validates: 2025-11-30 IS Sunday
DT.Hijri"1447-09-01 Saturday"            // Validates: Ramadan 1 IS Saturday
DT"2025-12-25 Wednesday"                 // Christmas 2025 validation
```

**Behavior**: Compile error if date doesn't fall on specified day of week.

**Returns**: `DateTime { pattern: Once, value: DateAndDayOfWeek { date, day }, type: Instant }`

---

#### 1.8 Full Specification (All variant)

**IMPORTANT**: Time MUST specify AM/PM or use 24-hour format.

```polyglot
// ✅ Valid: 24-hour format
DT.Gregorian"2025-11-30 Sunday 15:00"    // Date + DayOfWeek + Time (24-hour)
DT.Hijri"1447-09-01 Saturday 15:00"      // Islamic date with 24-hour time

// ✅ Valid: 12-hour with AM/PM
DT.Gregorian"2025-11-30 Sunday 3:00PM"   // Date + DayOfWeek + Time (12-hour)
DT.Hijri"1447-09-01 Saturday 3:00PM"     // Islamic date with 12-hour time

// ❌ INVALID: Ambiguous time
DT"2025-11-30 Sunday 3:00"               // ERROR: 3 AM or 3 PM?
```

**Behavior**: Compile error if date doesn't fall on specified day of week.

**Returns**: `DateTime { pattern: Once, value: All { time, date, day }, type: Instant }`

---

### Category 2: Duration (Time Span)

#### 2.1 Duration with Units

**Supported Units**: `s` (seconds), `m` (minutes), `h` (hours), `d` (days), `w` (weeks), `mo` (months), `y` (years)

```polyglot
DT.Seconds"30"                // 30 seconds
DT.Minutes"5"                 // 5 minutes
DT.Hours"2"                   // 2 hours
DT.Days"7"                    // 7 days
DT.Weeks"2"                   // 2 weeks
DT.Months"3"                  // 3 months (calendar-aware)
DT.Years"1"                   // 1 year (calendar-aware)

// Shorthand forms
DT"30s"                       // 30 seconds
DT"5m"                        // 5 minutes
DT"2h"                        // 2 hours
DT"1d"                        // 1 day
DT"1w"                        // 1 week
DT"1mo"                       // 1 month
DT"1y"                        // 1 year
```

**Returns**: `DateTime { pattern: Once, value: DateOnly(Date), type: Duration { unit } }`

---

#### 2.2 Compound Durations

```polyglot
DT"2h 30m"                    // 2 hours 30 minutes
DT"1d 6h"                     // 1 day 6 hours
DT"3mo 2w"                    // 3 months 2 weeks (calendar-aware)
```

**Returns**: `DateTime { pattern: Once, value: ..., type: Duration { compound } }`

---

#### 2.3 Relative Time (Past/Future)

```polyglot
DT.Ago"2h"                    // 2 hours ago
DT.Ago"3d"                    // 3 days ago
DT.From"2h"                   // 2 hours from now (alias: DT.Future)
DT.From"30m"                  // 30 minutes from now
```

**Returns**: `DateTime { pattern: Once, value: ..., type: Instant }` (computed instant)

---

#### 2.4 Duration Between

```polyglot
DT.ToNow"{.start}"                    // Duration from .start to now
DT.Between"{.start} to {.end}"        // Duration between two instants (single string with range)
DT.Since"{.event}"                    // Duration since .event
DT.Until"{.deadline}"                 // Duration until .deadline
```

**Note:** `DT.Between` takes a single formatted string with both start and end times using "to" as separator.

**Returns**: `DateTime { pattern: Once, value: ..., type: Duration }`

---

### Category 3: Recurrence (Repeating Patterns)

#### 3.1 Simple Recurrence

**IMPORTANT**: Times MUST specify AM/PM or use 24-hour format.

```polyglot
// ✅ Valid: Day only (all day)
DT.Every"Sun"                 // Every Sunday (all day)
DT.Every"Friday"              // Every Friday (all day)

// ✅ Valid: 24-hour format
DT.Every"Friday 15:00"        // Every Friday at 15:00 (3 PM)
DT.Every"15:00"               // Every day at 15:00
DT.Daily"03:00"               // Every day at 03:00 (3 AM)

// ✅ Valid: 12-hour with AM/PM
DT.Every"Friday 3:00PM"       // Every Friday at 3:00 PM
DT.Daily"3:00AM"              // Every day at 3:00 AM

// ❌ INVALID: Ambiguous time
DT.Every"15:00"               // ✅ OK: 24-hour is unambiguous
DT.Every"3:00"                // ❌ ERROR: 3 AM or 3 PM?
```

**Returns**: `DateTime { pattern: Recurrence { unlimited }, value: ..., type: Instant }`

---

#### 3.2 Interval Recurrence

```polyglot
DT.Every"5m"                  // Every 5 minutes
DT.Every"2h"                  // Every 2 hours
DT.Every"1d"                  // Every day (midnight)
DT.Every"1w"                  // Every week (Sunday midnight)
```

**Returns**: `DateTime { pattern: Recurrence { unlimited }, value: ..., type: Instant }`

---

#### 3.3 Limited Recurrence

```polyglot
DT.Every"4 Sun after {.start_date}"   // Next 4 Sundays after start date
DT.Every"10 15:00 after 2026-01-01"   // Next 10 occurrences of 15:00 after Jan 1
DT.Every"5m until {.deadline}"        // Every 5 minutes until deadline
```

**Pattern:** All parameters (count, pattern, reference date, limits) go in the formatted string.

**Returns**: `DateTime { pattern: Recurrence { count: 4 }, value: ..., type: Instant }`

---

#### 3.4 Yearly Recurrence (Calendar-Specific)

```polyglot
DT.Yearly"12-25"                          // Every December 25 (Gregorian)
DT.Hijri.Yearly"09-01"                    // Every Ramadan 1 (Islamic, algorithmic)
DT.Hijri.SaudiArabia.Yearly"09-01"        // Every Ramadan 1 (Saudi profile)
DT.Hijri.UKMoonSighting.Yearly"09-01"     // Every Ramadan 1 (UK profile)
DT.Hebrew.Sephardic.Yearly"07-15"         // Every Sukkot (15th Tishrei, Sephardic)
```

**Returns**: `DateTime { pattern: Recurrence { unlimited }, value: ..., type: Instant }`

---

### Category 4: Relative Date Patterns (Advanced)

#### 4.1 Nth Weekday in Month

```polyglot
DT.First"Monday in November"          // 1st Monday in November (annual)
DT.Second"Sunday in June"             // 2nd Sunday in June (annual)
DT.Fourth"Thursday in November"       // 4th Thursday in November (Thanksgiving)
DT.Last"Friday in Ramadan"            // Last Friday in Ramadan (Islamic calendar)
```

**Returns**: `DateTime { pattern: Once/Recurrence, value: RelativeDate(...), type: Instant }`

---

#### 4.2 Specific Year Relative Patterns

```polyglot
DT.First"Monday in November 2025"     // Specific year
DT.Last"Friday in Ramadan 1447"       // Specific Islamic year
```

**Returns**: `DateTime { pattern: Once, value: RelativeDate(...), type: Instant }`

---

### Category 5: Special Formats

#### 5.1 Ranges (Duration type)

```polyglot
DT.Range"2025-11-30 to 2025-12-05"            // Date range
DT.Range"2025-11-30 15:00 to 2025-11-30 17:00" // Time range
DT.Range"09:00 to 17:00"                      // Daily time range
```

**Returns**: `DateTime { pattern: Once, value: ..., type: Duration }`

---

#### 5.2 Business Days

```polyglot
DT.Business.Days"5"                   // 5 business days from now
DT.Business.Day"next"                 // Next business day
DT.Business.Week.Start""              // Start of business week (profile-aware)
```

**Returns**: Depends on context (Instant or Duration)

---

#### 5.3 ISO 8601 Extended

```polyglot
DT.ISO"2025-11-30T15:00:00Z"          // ISO 8601 with timezone
DT.ISO"2025-W48-6"                    // ISO week date (Week 48, Saturday)
DT.ISO"2025-334"                      // ISO ordinal date (day 334 of year)
```

**Returns**: `DateTime { pattern: Once, value: TimeAndDate, type: Instant }`

---

## T.DT.* Trigger Semantics

### Pattern: T.DT.* = "Trigger when DT.* is now"

**Core Principle**: Every `T.DT.*` trigger pattern means **"trigger when the corresponding `DT.*` evaluates to now"**.

**Semantic Mapping**:
```
T.DT.{Pattern}"{params}" = Trigger when DT.{Pattern}"{params}" == DT.Now""
```

### Examples

#### Daily Trigger
```polyglot
[t] |T.DT.Daily"3:00AM"
```
**Semantics**: Trigger when `DT.Daily"3:00AM"` equals `DT.Now""`
**Meaning**: Trigger every day when current time is 3:00 AM

#### Weekly Trigger
```polyglot
[t] |T.DT.Every"Friday 3:00PM"
```
**Semantics**: Trigger when `DT.Every"Friday 3:00PM"` equals `DT.Now""`
**Meaning**: Trigger every Friday when current time is 3:00 PM

#### Islamic Calendar Trigger
```polyglot
[t] |T.DT.Hijri.SaudiArabia.Yearly"09-01 3:00AM"
```
**Semantics**: Trigger when `DT.Hijri.SaudiArabia.Yearly"09-01 3:00AM"` equals `DT.Now""`
**Meaning**: Trigger every Ramadan 1st at 3:00 AM (using Saudi Arabia moon sighting profile)

#### Relative Pattern Trigger
```polyglot
[t] |T.DT.Hijri.UKMoonSighting.Last"Friday in Ramadan"
```
**Semantics**: Trigger when `DT.Hijri.UKMoonSighting.Last"Friday in Ramadan"` equals `DT.Now""`
**Meaning**: Trigger on the last Friday of Ramadan (using UK moon sighting profile)

### Time Resolution

**Trigger evaluation checks occur every minute** by default.

**Time matching**:
- `"3:00AM"` → Triggers when current time is 3:00:00 AM to 3:00:59 AM
- `"3:00:30AM"` → Triggers when current time is 3:00:30 AM (exact second)
- `"15:00"` → Triggers when current time is 15:00:00 to 15:00:59 (24-hour)

### Profile-Aware Triggers

Triggers respect calendar profiles at **evaluation time**:

```polyglot
// Saudi Arabia profile determines Ramadan 1, 1447 = 2025-02-28
[t] |T.DT.Hijri.SaudiArabia"1447-09-01 3:00AM"

// UK profile determines Ramadan 1, 1447 = 2025-03-01
[t] |T.DT.Hijri.UKMoonSighting"1447-09-01 3:00AM"
// → Different profiles = different Gregorian dates
```

### Implementation Notes

**Trigger Monitor Service**:
1. Load all active triggers with `[t]` marker
2. Evaluate each trigger condition every minute (or specified resolution)
3. For `T.DT.*` triggers:
   - Execute corresponding `DT.*` pipeline
   - Compare result with `DT.Now""`
   - If match: Fire trigger (create pipeline instance)

**Recurrence Handling**:
- `T.DT.Every"Sun"` → Fires every Sunday (unlimited recurrence)
- `T.DT.Every"4 Sun after 2026-01-01"` → Fires next 4 Sundays after Jan 1, 2026 (limited recurrence)
- After limited recurrence completes: Trigger becomes inactive

**Error Handling**:
- Profile unavailable: Use fallback (ICU4X calculated)
- Invalid date (e.g., `DT"2025-02-30"`): Trigger never fires (logged as error)
- Calendar conversion failure: Use configured error policy

---

## Complete Format Summary Table

| Category | Example | Variant | Pattern | Type |
|----------|---------|---------|---------|------|
| **Current Time** | `DT.Now""` | TimeAndDate | Once | Instant |
| **Date Only** | `DT.Gregorian"2025-11-30"` | DateOnly | Once | Instant |
| **Time Only** | `DT"15:00"` | TimeOnly | Once | Instant |
| **DateTime** | `DT"2025-11-30 15:00"` | TimeAndDate | Once | Instant |
| **Day of Week** | `DT"Monday"` | DayOfWeekOnly | Once | Instant |
| **Time + DayOfWeek** | `DT"Friday 3:00PM"` | TimeAndDayOfWeek | Once | Instant |
| **Date + DayOfWeek** | `DT"2025-11-30 Sunday"` | DateAndDayOfWeek | Once | Instant |
| **Full Spec** | `DT"2025-11-30 Sunday 15:00"` | All | Once | Instant |
| **Duration** | `DT"5m"`, `DT.Minutes"5"` | DateOnly | Once | Duration |
| **Relative Time** | `DT.Ago"2h"` | TimeAndDate | Once | Instant |
| **Simple Recurrence** | `DT.Every"Sun"` | DayOfWeekOnly | Recurrence | Instant |
| **Interval Recurrence** | `DT.Every"5m"` | TimeOnly | Recurrence | Instant |
| **Yearly Recurrence** | `DT.Yearly"12-25"` | DateOnly | Recurrence | Instant |
| **Relative Pattern** | `DT.Last"Friday in Ramadan"` | RelativeDate | Once | Instant |
| **Range** | `DT.Range"09:00 to 17:00"` | TimeAndDate | Once | Duration |

---

## Calendar-Specific Namespaces

All calendar-specific formats use namespaced pipelines:

```polyglot
DT.Gregorian.*         // Gregorian calendar (default)
DT.Hijri.*             // Islamic calendar (algorithmic + profiles)
DT.Hijri.{Profile}.*   // Islamic calendar (profile-aware: SaudiArabia, UKMoonSighting, etc.)
DT.Julian.*            // Julian calendar
DT.Assyrian.*          // Assyrian/Syriac calendar
DT.Chinese.*           // Chinese calendar (algorithmic + profiles)
DT.Hebrew.*            // Hebrew calendar
DT.Buddhist.*          // Buddhist calendar
DT.Persian.*           // Persian (Solar Hijri) calendar
```

**Profile-Aware Calendars**: `DT.Hijri.*`, `DT.Hebrew.*`, `DT.Chinese.*` support both algorithmic and profile-based dates.

---

## Format Conventions

### 1. Empty Parameters
**Always require `""`** even with no parameters:
```polyglot
DT.Now""              // ✅ Correct
DT.Now                // ❌ ERROR: Missing string literal
```

### 2. Time Formats
- **24-hour**: `"15:00"`, `"15:00:30"`
- **12-hour**: `"3:00PM"`, `"3:00:30AM"`
- **Separator**: Colon `:` (not `.` or `-`)

### 3. Date Formats
- **ISO 8601**: `"2025-11-30"` (YYYY-MM-DD)
- **Calendar-specific**: Follow calendar conventions
- **Trailing colon**: `"2025-11-30:"` = full day (00:00 to 23:59:59)

### 4. Duration Suffixes
- **Seconds**: `s` → `"30s"`
- **Minutes**: `m` → `"5m"`
- **Hours**: `h` → `"2h"`
- **Days**: `d` → `"1d"`
- **Weeks**: `w` → `"1w"`
- **Months**: `mo` → `"1mo"` (NOT `"1m"` - that's minutes!)
- **Years**: `y` → `"1y"`

### 5. Timezone Notation
- **Abbreviation**: `"EST"`, `"PST"`, `"UTC"`
- **Offset**: `"+05:00"`, `"-08:00"`
- **Z suffix**: `"Z"` = UTC (ISO 8601)

### 6. Interpolation Support
All formats support variable interpolation:
```polyglot
DT.ToNow"{.start}"                    // Variable interpolation
DT.Between"{.event1} to {.event2}"    // Multiple interpolations in single string
DT.Gregorian"{.year}-{.month}-{.day}" // Computed date
```

**Note:** When multiple values are needed, combine them in a single formatted string with appropriate separators (e.g., "to", "-", etc.).

---

## Implementation Notes

### Standard Library Pipelines
Each `DT.*` pattern corresponds to a standard library pipeline:

```polyglot
// Example: DT.Now"" implementation
[|] DT.Now
[i] !No.Input
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\dt               // Returns current instant
[X]

// Example: DT.Minutes"5" implementation
[|] DT.Minutes
[i] #Pipeline.String
[<] .count: pg\int               // Parse "5" to int
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .duration: pg\dt << /* compute duration */
[o] .result: pg\dt               // Returns duration
[X]
```

### Parsing Strategy
1. **Lexer**: Tokenize `DT.Now""` as inline pipeline identifier + string literal
2. **Parser**: Recognize as pipeline call with string parameter
3. **Type System**: Validate return type is `pg\dt`
4. **Runtime**: Execute pipeline, return `DateTime` value

### Error Handling
Invalid formats produce compile-time errors:
```polyglot
DT"invalid"                      // ERROR: Unrecognized format
DT.Gregorian"2025-02-30"         // ERROR: Invalid date (Feb 30)
DT"2025-11-30 Monday"            // ERROR: 2025-11-30 is Sunday, not Monday
DT.Hijri"invalid-date"           // ERROR: Invalid Hijri date format
```

---

## Open Questions

1. **Default Calendar**: Is `DT"2025-11-30"` Gregorian by default? Or configurable?
2. **Timezone Default**: What timezone for `DT"15:00"` without explicit TZ? (UTC? Local? Configurable?)
3. ~~**Profile Selection**~~: ✅ **ANSWERED** - Use extendable enumerations: `DT.{Calendar}.{ProfileName}"..."`
4. **Recurrence Syntax**: Is `"count:4"` the right syntax or use separate parameter?
5. **Range Operator**: Should ranges use `to`, `-`, or `..`? (`"09:00 to 17:00"` vs `"09:00-17:00"`)
6. **Profile YAML Loading**: Is `|U.YAML.Load` the right utility pipeline? Or different loader?
7. **Relative Pattern Namespacing**: Is `DT.Last"Friday in Ramadan"` or `DT.Hijri.Last"Friday in Ramadan"`? (Which calendar context?)

---

## Next Steps

1. **Lexer Extension**: Add tokenization for all `DT.*` patterns (Story 1.3 or new story)
2. **Standard Library**: Implement all `DT.*` pipelines (Epic 7 or new epic)
3. **Parser Validation**: Add semantic checks for format validation
4. **Calendar Integration**: Integrate ICU4X for calendar operations (Story 2.1)
5. **Profile System**: Implement profile loading and management (Post-MVP)

---

**Related Documents**:
- Brainstorming Session Results: `docs/project/brainstorming-session-results-2025-11-29.md`
- String Literals Update Summary: `docs/STRING-LITERALS-UPDATE-SUMMARY.md`
- Triggers Catalog: `docs/user/standard-library/triggers-catalog.md`
