# Polyglot DateTime System

**Version:** 0.0.2  
**Based on:** ICU4X International Components
**Last Updated:** 2025-12-02

---

## Overview

Polyglot's `:pg.dt` type represents **both DateTime and Duration** in a single unified type. All datetime operations use **inline pipeline syntax**: `DT.*"formatted_string"`.

---

## Core Principle

**Every DT literal is an inline pipeline call:**

```polyglot
|DT.Now""                           // Current instant
|DT.Minutes"5"                      // 5-minute duration
|DT.Gregorian"2025-12-01"           // Specific date
```

---

## Quick Reference

| Category         | Example                         | Returns            |
| ---------------- | ------------------------------- | ------------------ |
| **Current Time** | `\|DT.Now""`                    | `:pg.dt` instant   |
| **Date**         | `\|DT.Gregorian"2025-12-01"`    | `:pg.dt` instant   |
| **Time**         | `\|DT"15:00"`                   | `:pg.dt` instant   |
| **Duration**     | `\|DT.Minutes"5"`, `\|DT"2h 30m"` | `:pg.dt` duration  |
| **Relative**     | `|DT.Ago"2h"`, `|DT.From"30m"`    | `:pg.dt` instant   |
| **Recurrence**   | `|DT.Every"Sunday"`              | `:pg.dt` recurring |
| **Calendar**     | `|DT.Hijri"1447-09-01"`          | `:pg.dt` instant   |

---

## Time Formats

**CRITICAL:** Times MUST specify AM/PM or use 24-hour format!

### ✓ Valid

```polyglot
|DT"15:00"                          // 24-hour (unambiguous)
|DT"3:00PM"                         // 12-hour with PM
|DT"3:00AM"                         // 12-hour with AM
```

### ✗ Invalid

```polyglot
|DT"3:00"                           // ERROR: 3 AM or 3 PM?
```

---

## Multi-Calendar Support

### Gregorian (Default)

```polyglot
|DT.Gregorian"2025-12-01"
|DT"2025-12-01"                     // Shorthand
```

### Islamic Calendar

```polyglot
|DT.Hijri"1447-09-01"               // Algorithmic
|DT.Hijri.SaudiArabia"1447-09-01"   // Saudi profile
|DT.Hijri.UKMoonSighting"1447-09-01" // UK profile
```

### Other Calendars

```polyglot
|DT.Hebrew"5784-10-03"              // Hebrew
|DT.Chinese"4722-01-15"             // Chinese
|DT.Persian"1404-09-09"             // Persian (Solar Hijri)
|DT.Buddhist"2568-11-30"            // Buddhist
```

---

## Calendar Profiles

Profile-aware calendars (Hijri, Hebrew, Chinese support **custom observation profiles**:

```polyglot
// Use built-in profile
[t] |T.DT.Hijri.SaudiArabia.Yearly"09-01 3:00AM"

// Define custom profile
[#] #DT.Hijri.MyCompanyHR
[s] |U.YAML.Load"MyCompanyHRHijri.yaml"
[s][!] *                           // Expand all fields
[X]

// Use custom profile
[t] |T.DT.Hijri.MyCompanyHR.Yearly"09-01"
```

Profile priority:
1. **Manual overrides** (HR-entered dates
2. **API cache** (moon sighting services
3. **ICU4X calculated** (algorithmic fallback

---

## Duration Operations

### Simple Durations

```polyglot
|DT.Seconds"30"                     // 30 seconds
|DT.Minutes"5"                      // 5 minutes
|DT.Hours"2"                        // 2 hours
|DT.Days"7"                         // 7 days

// Shorthand
|DT"30s"
|DT"5m"
|DT"2h"
|DT"1d"
```

### Compound Durations

```polyglot
|DT"2h 30m"                         // 2 hours 30 minutes
|DT"1d 6h"                          // 1 day 6 hours
```

### Relative Time

```polyglot
|DT.Ago"2h"                         // 2 hours ago
|DT.From"30m"                       // 30 minutes from now
|DT.ToNow"{.start"                 // Duration from start to now
```

---

## Recurrence Patterns

### Simple Recurrence

```polyglot
|DT.Every"Sunday"                   // Every Sunday
|DT.Every"Friday 3:00PM"            // Every Friday at 3 PM
|DT.Daily"3:00AM"                   // Every day at 3 AM
```

### Yearly Recurrence

```polyglot
|DT.Yearly"12-25"                   // Every Dec 25 (Gregorian)
|DT.Hijri.Yearly"09-01"             // Every Ramadan 1 (Islamic)
|DT.Hebrew.Yearly"07-15"            // Every Sukkot
```

### Limited Recurrence

```polyglot
|DT.Every"4 Sun after 2026-01-01"   // Next 4 Sundays
|DT.Every"10 15:00 after {.start"  // Next 10 occurrences
```

---

## Triggers with DateTime

All `DT.*` patterns work with triggers:

```polyglot
[t] |T.DT.Daily"3:00AM"             // Daily at 3 AM
[t] |T.DT.Every"Friday 5:00PM"      // Every Friday at 5 PM
[t] |T.DT.Yearly"12-25"             // Every Christmas
[t] |T.DT.Hijri.SaudiArabia"09-01"  // Every Ramadan 1
```

**Trigger Semantics:** `|T.DT.*"..."` fires when `|DT.*"..."` equals `|DT.Now""`.

---

## Complete Examples

### Daily Report Trigger

```polyglot
[|] DailyReport
[i] !No.Input
[t] |T.DT.Daily"2:00AM"             // Every day at 2 AM
[W] |W.Polyglot.Scope
[r] |GenerateReport
[o] .report:pg.string
[X]
```

### Ramadan Notification

```polyglot
[|] RamadanNotification
[i] !No.Input
[t] |T.DT.Hijri.UKMoonSighting.Yearly"09-01 6:00AM"
[W] |W.Polyglot.Scope
[r] |SendNotification
[o] !No.Output
[X]
```

### Timeout Check

```polyglot
[r] .start:pg.dt << |DT.Now""
[r] .elapsed:pg.dt << |DT.ToNow"{.start"

[?] .elapsed >? |DT.Minutes"5"
[~][o] !TimeoutError
[~]
```

---

## See Also

- [DateTime String Literal Specification](/docs/technical/datetime-string-literal-specification.md
- [Triggers Catalog](/docs/user/standard-library/triggers-catalog.md

---

**Next:** [Parallel Execution →](parallel-execution.md
