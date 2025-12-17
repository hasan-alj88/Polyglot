---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/language/07-datetime-system.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# DateTime System

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot's DateTime system provides comprehensive date and time handling with support for multiple calendar systems, relative time expressions, and recurring patterns. The system uses the `DT` prefix for all datetime operations.

**Key Features:**
- Multiple calendar systems (Gregorian, Hijri, Chinese, Hebrew, Persian)
- DateTime literals with `|DT"..."` syntax
- Relative time expressions (`|DT.Ago`, `|DT.From.Now`)
- Recurring patterns (`|DT.Every`)
- Timezone support
- Syntax sugar that compiles to pipeline operations

---

## Table of Contents

1. [DateTime Type Basics](#datetime-type-basics)
2. [DateTime Literals](#datetime-literals)
3. [Calendar Systems](#calendar-systems)
4. [Relative Time](#relative-time)
5. [Recurring Patterns](#recurring-patterns)
6. [DateTime Format Strings](#datetime-format-strings)
7. [Syntax Sugar Compilation](#syntax-sugar-compilation)
8. [DateTime Operations](#datetime-operations)
9. [Timezone Handling](#timezone-handling)
10. [Best Practices](#best-practices)

---

## DateTime Type Basics

### The `:pg.dt` Type

**Type:** `:pg.dt`

**Description:** DateTime type with calendar and timezone support

**Default Value:** Implementation-specific (likely current time or epoch)

---

### Variable Declaration

```polyglot
// DateTime variable
[r] .timestamp:pg.dt << |DT"2024-01-15 14:30:00"

// DateTime input
[i] .scheduled_time:pg.dt

// DateTime with default
[i] Default .start_time:pg.dt << |DT"09:00:"
```

---

### Why `DT` Not `T`?

**Decision:** DateTime uses `DT` prefix, not `T`

**Rationale:**
- `T` could be confused with triggers (`|T.*`)
- `DT` is unambiguous and clearly indicates DateTime
- Consistent with namespace conventions

```polyglot
// ✓ CORRECT
|DT"2024-01-15"
|DT.Ago"2 days"
|DT.Every"1 hour"

// ✗ WRONG
|T"2024-01-15"
|T.Ago"2 days"
|T.Every"1 hour"
```

---

## DateTime Literals

### Basic DateTime Literal Syntax

**Format:**
```polyglot
|DT"datetime_string"
```

**Note:** DateTime literals are **syntax sugar** that compile to underlying pipeline operations.

---

### Full DateTime

```polyglot
// Date and time
[r] .now:pg.dt << |DT"2024-01-15 14:30:00"

// With timezone
[r] .utc_time:pg.dt << |DT"2024-01-15 14:30:00 UTC"
[r] .local_time:pg.dt << |DT"2024-01-15 14:30:00 EST"
```

---

### Date Only

```polyglot
// Just the date
[r] .date:pg.dt << |DT"2024-01-15"

// Midnight implied
[r] .start_of_day:pg.dt << |DT"2024-01-15"
```

---

### Time Only

```polyglot
// Just the time (colon at end indicates time-only)
[r] .time:pg.dt << |DT"14:30:"
[r] .morning:pg.dt << |DT"09:00:"
[r] .midnight:pg.dt << |DT"00:00:"
```

**Format:** Time literals end with `:` to distinguish from date strings.

---

### DateTime Format Variations

```polyglot
// ISO 8601 format
[r] .iso:pg.dt << |DT"2024-01-15T14:30:00Z"

// Date with slashes
[r] .us_date:pg.dt << |DT"01/15/2024"

// Date with dashes
[r] .std_date:pg.dt << |DT"2024-01-15"

// With milliseconds
[r] .precise:pg.dt << |DT"2024-01-15 14:30:00.123"
```

---

## Calendar Systems

### Supported Calendars

Polyglot supports multiple calendar systems beyond Gregorian:

| Calendar | Prefix | Description |
|----------|--------|-------------|
| Gregorian | `DT` (default) | Standard Western calendar |
| Hijri | `|DT.Hijri` | Islamic lunar calendar |
| Chinese | `|DT.Chinese` | Chinese lunisolar calendar |
| Hebrew | `|DT.Hebrew` | Hebrew lunisolar calendar |
| Persian | `|DT.Persian` | Persian solar calendar |

---

### Gregorian Calendar (Default)

```polyglot
// Default calendar
[r] .date:pg.dt << |DT"2024-01-15"

// Explicit Gregorian
[r] .greg_date:pg.dt << |DT.Gregorian"2024-01-15"
```

---

### Hijri (Islamic) Calendar

```polyglot
// Hijri date
[r] .hijri_date:pg.dt << |DT.Hijri"1445-07-15"

// Hijri with time
[r] .hijri_time:pg.dt << |DT.Hijri"1445-07-15 14:30:00"

// Current Hijri date
[r] .now_hijri:pg.dt << |DT.Hijri.Now
```

---

### Chinese Calendar

```polyglot
// Chinese date
[r] .chinese_date:pg.dt << |DT.Chinese"2024-01-15"

// Chinese New Year
[r] .cny:pg.dt << |DT.Chinese"2024-01-01"
```

---

### Hebrew Calendar

```polyglot
// Hebrew date
[r] .hebrew_date:pg.dt << |DT.Hebrew"5784-10-15"

// Hebrew with time
[r] .hebrew_time:pg.dt << |DT.Hebrew"5784-10-15 14:30:00"
```

---

### Persian Calendar

```polyglot
// Persian date
[r] .persian_date:pg.dt << |DT.Persian"1402-10-24"

// Nowruz (Persian New Year)
[r] .nowruz:pg.dt << |DT.Persian"1403-01-01"
```

---

### Calendar Conversion

```polyglot
// Define date in one calendar
[r] .gregorian:pg.dt << |DT"2024-01-15"

// Convert to another calendar (using standard library)
[r] |DT.Convert.ToHijri
[<] .date:pg.dt << .gregorian
[>] .hijri_date:pg.dt >> .hijri

// Format in specific calendar
[r] |DT.Format.Hijri
[<] .date:pg.dt << .gregorian
[<] .format:pg.string << "YYYY-MM-DD"
[>] .formatted:pg.string >> hijri_string
```

---

## Relative Time

### Relative Time Expressions

Polyglot supports relative time expressions for past and future dates.

---

### Past Time: `|DT.Ago`

**Format:**
```polyglot
|DT.Ago"time_expression"
```

**Examples:**
```polyglot
// Time in the past
[r] .two_days_ago:pg.dt << |DT.Ago"2 days"
[r] .one_hour_ago:pg.dt << |DT.Ago"1 hour"
[r] .thirty_minutes_ago:pg.dt << |DT.Ago"30 minutes"
[r] .one_week_ago:pg.dt << |DT.Ago"1 week"
[r] .six_months_ago:pg.dt << |DT.Ago"6 months"
[r] .one_year_ago:pg.dt << |DT.Ago"1 year"
```

---

### Future Time: `|DT.From.Now`

**Format:**
```polyglot
|DT.From.Now"time_expression"
```

**Examples:**
```polyglot
// Time in the future
[r] .in_two_days:pg.dt << |DT.From.Now"2 days"
[r] .in_one_hour:pg.dt << |DT.From.Now"1 hour"
[r] .in_thirty_minutes:pg.dt << |DT.From.Now"30 minutes"
[r] .in_one_week:pg.dt << |DT.From.Now"1 week"
[r] .in_six_months:pg.dt << |DT.From.Now"6 months"
[r] .in_one_year:pg.dt << |DT.From.Now"1 year"
```

---

### Current Time: `|DT.Now`

```polyglot
// Current timestamp
[r] .now:pg.dt << |DT.Now

// Current time in specific calendar
[r] .now_hijri:pg.dt << |DT.Hijri.Now
[r] .now_chinese:pg.dt << |DT.Chinese.Now
```

---

### Time Units

Supported time units for relative expressions:

| Unit | Singular | Plural |
|------|----------|--------|
| Second | `1 second` | `30 seconds` |
| Minute | `1 minute` | `45 minutes` |
| Hour | `1 hour` | `24 hours` |
| Day | `1 day` | `7 days` |
| Week | `1 week` | `4 weeks` |
| Month | `1 month` | `6 months` |
| Year | `1 year` | `5 years` |

---

## Recurring Patterns

### Recurring Time Expressions

Use `|DT.Every` for recurring patterns (commonly used with triggers).

---

### Every Time Unit

```polyglot
// Every second
|DT.Every"1 second"
|DT.Every.Second

// Every minute
|DT.Every"1 minute"
|DT.Every.Minute

// Every hour
|DT.Every"1 hour"
|DT.Every.Hour

// Every day
|DT.Every"1 day"
|DT.Every.Day

// Every week
|DT.Every"1 week"
|DT.Every.Week

// Custom intervals
|DT.Every"30 seconds"
|DT.Every"15 minutes"
|DT.Every"6 hours"
```

---

### Recurring Pattern Usage

**With Triggers:**
```polyglot
[|] ScheduledTask
// Trigger every hour
[t] |T.Every.Hour
[<] .minute:pg.int << 0  // At the top of the hour

[r] |DoWork
[X]
```

```polyglot
[|] FrequentTask
// Trigger every 30 seconds
[t] |T.Every.Seconds
[<] .interval:pg.int << 30

[r] |CheckStatus
[X]
```

---

### Daily at Specific Time

```polyglot
[|] DailyReport
// Every day at 9:00 AM
[t] |T.Daily
[<] .time:pg.dt << |DT"09:00:"

[r] |GenerateReport
[X]
```

---

### Weekly Patterns

```polyglot
[|] WeeklyBackup
// Every Monday at 2:00 AM
[t] |T.Weekly
[<] .day:pg.string << "Monday"
[<] .time:pg.dt << |DT"02:00:"

[r] |BackupDatabase
[X]
```

---

### Monthly Patterns

```polyglot
[|] MonthlyInvoice
// First day of every month at midnight
[t] |T.Monthly
[<] .day:pg.int << 1
[<] .time:pg.dt << |DT"00:00:"

[r] |GenerateInvoices
[X]
```

---

## DateTime Format Strings

### Format String Syntax

Polyglot uses format strings for custom datetime formatting:

| Token | Meaning | Example |
|-------|---------|---------|
| `YYYY` | 4-digit year | 2024 |
| `YY` | 2-digit year | 24 |
| `MM` | 2-digit month | 01-12 |
| `M` | Month without padding | 1-12 |
| `DD` | 2-digit day | 01-31 |
| `D` | Day without padding | 1-31 |
| `HH` | 2-digit hour (24h) | 00-23 |
| `hh` | 2-digit hour (12h) | 01-12 |
| `mm` | 2-digit minute | 00-59 |
| `ss` | 2-digit second | 00-59 |
| `SSS` | Milliseconds | 000-999 |
| `A` | AM/PM | AM or PM |
| `Z` | Timezone offset | +05:30 |

---

### Formatting Examples

```polyglot
// Format current time
[r] |DT.Format
[<] .datetime:pg.dt << |DT.Now
[<] .format:pg.string << "YYYY-MM-DD HH:mm:ss"
[>] .formatted:pg.string >> timestamp

// US date format
[r] |DT.Format
[<] .datetime:pg.dt << |DT"2024-01-15"
[<] .format:pg.string << "MM/DD/YYYY"
[>] .formatted:pg.string >> us_date  // "01/15/2024"

// 12-hour time with AM/PM
[r] |DT.Format
[<] .datetime:pg.dt << |DT"14:30:"
[<] .format:pg.string << "hh:mm A"
[>] .formatted:pg.string >> time_12h  // "02:30 PM"
```

---

### Parsing Custom Formats

```polyglot
// Parse custom date format
[r] |DT.Parse
[<] .input:pg.string << "15-01-2024"
[<] .format:pg.string << "DD-MM-YYYY"
[>] .datetime:pg.dt >> parsed_date

// Parse 12-hour time
[r] |DT.Parse
[<] .input:pg.string << "02:30 PM"
[<] .format:pg.string << "hh:mm A"
[>] .datetime:pg.dt >> parsed_time
```

---

## Syntax Sugar Compilation

### DateTime Literals are Syntax Sugar

**Important:** `|DT"..."` literals are syntax sugar that compile to underlying pipeline operations.

---

### Literal Compilation Example

**This literal:**
```polyglot
[r] .time:pg.dt << |DT"2024-01-15 14:30:00"
```

**Compiles to something like:**
```polyglot
[r] |pg.DateTime.Parse
[<] .input:pg.string << "2024-01-15 14:30:00"
[<] .format:pg.string << "YYYY-MM-DD HH:mm:ss"
[>] .result:pg.dt >> .time
```

---

### Relative Time Compilation

**This literal:**
```polyglot
[r] .past:pg.dt << |DT.Ago"2 days"
```

**Compiles to something like:**
```polyglot
[r] |pg.DateTime.Now
[>] .now:pg.dt >> current_time

[r] |pg.DateTime.Subtract
[<] .datetime:pg.dt << current_time
[<] .duration:pg.string << "2 days"
[>] .result:pg.dt >> .past
```

---

### Why Syntax Sugar?

**Benefits:**
1. **Concise syntax** - More readable and writable
2. **Pipeline consistency** - Everything is pipelines underneath
3. **Extensibility** - Can add new formats without language changes
4. **Flexibility** - Users can bypass sugar and use pipelines directly

**User choice:**
```polyglot
// Use sugar for convenience
[r] .time:pg.dt << |DT"14:30:"

// Or use pipelines directly for more control
[r] |pg.DateTime.Parse
[<] .input:pg.string << "14:30:"
[<] .format:pg.string << "HH:mm:"
[<] .timezone:pg.string << "UTC"
[>] .time:pg.dt >> .parsed_time
```

---

## DateTime Operations

### Arithmetic Operations

```polyglot
// Add duration
[r] |DT.Add
[<] .datetime:pg.dt << |DT"2024-01-15"
[<] .duration:pg.string << "7 days"
[>] .result:pg.dt >> .future_date

// Subtract duration
[r] |DT.Subtract
[<] .datetime:pg.dt << |DT"2024-01-15"
[<] .duration:pg.string << "2 days"
[>] .result:pg.dt >> .past_date

// Difference between dates
[r] |DT.Difference
[<] .start:pg.dt << |DT"2024-01-01"
[<] .end:pg.dt << |DT"2024-01-15"
[>] .duration:pg.string >> .days_between  // "14 days"
```

---

### Comparison Operations

**DateTime Comparison Operators:**

Polyglot uses comparison operators with the `?` suffix pattern for datetime comparisons.

```polyglot
// Compare dates using comparison operators
[r] .reference:pg.dt << |DT"2024-01-15"

// Check if date is after reference (>)
[?] .date1 >? .reference
[~][r] |HandleNewerDate

// Check if date is after or equal (>=)
[?] .date1 =>? .reference
[~][r] |HandleNewerOrSameDate

// Check if date equals reference
[?] .date1 =? .reference
[~][r] |HandleSameDate

// Check if date is before reference (<)
[?] .date1 <? .reference
[~][r] |HandleOlderDate

// Check if date is in range (between inclusive)
[?] .date1 ?[.start_date, .end_date]
[~][r] |HandleInRange

// Example with specific dates
[r] .start_date:pg.dt << |DT"2024-01-01"
[r] .end_date:pg.dt << |DT"2024-01-31"
[?] .date1 ?[.start_date, .end_date]
[~][r] |DateIsInJanuary
```

---

### Extraction Operations

```polyglot
// Extract components
[r] |DT.Year
[<] .datetime:pg.dt << |DT"2024-01-15"
[>] .year:pg.int >> year_value  // 2024

[r] |DT.Month
[<] .datetime:pg.dt << |DT"2024-01-15"
[>] .month:pg.int >> month_value  // 1

[r] |DT.Day
[<] .datetime:pg.dt << |DT"2024-01-15"
[>] .day:pg.int >> day_value  // 15

[r] |DT.Hour
[<] .datetime:pg.dt << |DT"14:30:00"
[>] .hour:pg.int >> hour_value  // 14

[r] |DT.DayOfWeek
[<] .datetime:pg.dt << |DT"2024-01-15"
[>] .day_name:pg.string >> weekday  // "Monday"
```

---

### Construction Operations

```polyglot
// Build datetime from components
[r] |DT.FromComponents
[<] .year:pg.int << 2024
[<] .month:pg.int << 1
[<] .day:pg.int << 15
[<] .hour:pg.int << 14
[<] .minute:pg.int << 30
[<] .second:pg.int << 0
[>] .datetime:pg.dt >> constructed_time
```

---

## Timezone Handling

### Timezone Support

Polyglot supports timezone-aware datetime operations.

---

### Specifying Timezone

```polyglot
// UTC time
[r] .utc:pg.dt << |DT"2024-01-15 14:30:00 UTC"

// Specific timezone
[r] .est:pg.dt << |DT"2024-01-15 14:30:00 EST"
[r] .pst:pg.dt << |DT"2024-01-15 14:30:00 PST"
[r] .cet:pg.dt << |DT"2024-01-15 14:30:00 CET"

// Offset format
[r] .plus_five:pg.dt << |DT"2024-01-15 14:30:00 +05:00"
[r] .minus_eight:pg.dt << |DT"2024-01-15 14:30:00 -08:00"
```

---

### Timezone Conversion

```polyglot
// Convert between timezones
[r] |DT.ToTimezone
[<] .datetime:pg.dt << |DT"2024-01-15 14:30:00 UTC"
[<] .timezone:pg.string << "EST"
[>] .converted:pg.dt >> est_time

// Convert to UTC
[r] |DT.ToUTC
[<] .datetime:pg.dt << |DT"2024-01-15 14:30:00 EST"
[>] .utc_time:pg.dt >> utc_converted

// Get local time
[r] |DT.ToLocal
[<] .datetime:pg.dt << |DT"2024-01-15 14:30:00 UTC"
[>] .local_time:pg.dt >> local_converted
```

---

### System Timezone

```polyglot
// Get system timezone
[r] |DT.GetSystemTimezone
[>] .timezone:pg.string >> system_tz

// Current time in system timezone
[r] .now_local:pg.dt << |DT.Now.Local
```

---

## Best Practices

### 1. Use Appropriate Calendar

```polyglot
// ✓ CORRECT - Use appropriate calendar for context
[r] .hijri_date:pg.dt << |DT.Hijri"1445-07-15"  // Islamic context
[r] .gregorian_date:pg.dt << |DT"2024-01-15"    // Western context

// ✗ AVOID - Wrong calendar for context
[r] .islamic_event:pg.dt << |DT"2024-01-15"  // Should use Hijri
```

---

### 2. Be Explicit with Timezones

```polyglot
// ✓ CORRECT - Explicit timezone
[r] .scheduled:pg.dt << |DT"2024-01-15 14:30:00 UTC"

// ✗ RISKY - Ambiguous timezone
[r] .scheduled:pg.dt << |DT"2024-01-15 14:30:00"  // What timezone?
```

---

### 3. Use Relative Time for Dynamic Dates

```polyglot
// ✓ CORRECT - Dynamic, always current
[r] .cutoff:pg.dt << |DT.Ago"7 days"

// ✗ AVOID - Hard-coded, becomes outdated
[r] .cutoff:pg.dt << |DT"2024-01-08"
```

---

### 4. Store UTC, Display Local

```polyglot
// ✓ CORRECT - Store in UTC
[r] .stored_time:pg.dt << |DT"2024-01-15 14:30:00 UTC"

// Convert to local for display
[r] |DT.ToLocal
[<] .datetime:pg.dt << .stored_time
[>] .display_time:pg.dt >> local_time

[r] |DT.Format
[<] .datetime:pg.dt << local_time
[<] .format:pg.string << "YYYY-MM-DD hh:mm A"
[>] .formatted:pg.string >> display_string
```

---

### 5. Use ISO 8601 for Interchange

```polyglot
// ✓ CORRECT - ISO 8601 format for APIs/interchange
[r] |DT.Format
[<] .datetime:pg.dt << .time
[<] .format:pg.string << "YYYY-MM-DDTHH:mm:ssZ"
[>] .iso:pg.string >> iso_string

// Parse ISO 8601
[r] |DT.Parse
[<] .input:pg.string << "2024-01-15T14:30:00Z"
[<] .format:pg.string << "YYYY-MM-DDTHH:mm:ssZ"
[>] .datetime:pg.dt >> parsed
```

---

### 6. Handle Daylight Saving Time

```polyglot
// Be aware of DST when scheduling
[t] |T.Daily
[<] .time:pg.dt << |DT"02:30:"  // May not exist during DST transition
[<] .timezone:pg.string << "EST"

// Use UTC for consistent scheduling
[t] |T.Daily
[<] .time:pg.dt << |DT"07:30: UTC"  // 02:30 EST, unaffected by DST
```

---

### 7. Validate Date Ranges

```polyglot
// Validate date is reasonable
[?] .input_date < |DT"1900-01-01"
[~][r] |ThrowError
[~][<] .message:pg.string << "Date too far in past"

[?] .input_date > |DT.From.Now"100 years"
[~][r] |ThrowError
[~][<] .message:pg.string << "Date too far in future"
```

---

### 8. Use Appropriate Precision

```polyglot
// ✓ CORRECT - Appropriate precision
[r] .log_time:pg.dt << |DT"2024-01-15 14:30:00.123"  // Milliseconds for logs
[r] .schedule:pg.dt << |DT"2024-01-15 14:30:00"       // Seconds for schedules
[r] .birthday:pg.dt << |DT"2024-01-15"                // Day for birthdays

// ✗ UNNECESSARY - Too much precision
[r] .birthday:pg.dt << |DT"2024-01-15 00:00:00.000000"  // Overkill
```

---

## See Also

### Language Specification
- [Type System](02-type-system.md) - `:pg.dt` type details
- [Complete Syntax Reference](01-syntax-complete.md) - DateTime syntax overview

### Standard Library
- [Triggers](../standard-library/04-triggers.md) - Time-based triggers
- [Reserved Enumerations](../standard-library/06-reserved-enumerations.md) - `#|DT.*` enumerations

### Examples
- [Complete Workflows](../examples/complete-workflows.md) - Scheduled task examples

### Planning
- [Decision Log](../decision-log.md) - DateTime decisions (#3)

---

**End of DateTime System Reference**