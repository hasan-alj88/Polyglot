---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $DT Constructor

<!-- @u:syntax/constructors -->
<!-- @c:pglib/types/datetime/main-type -->
<!-- @c:pglib/types/datetime/core-components -->

The `$DT` constructor produces `#DateTime`, `#Date`, and `#Time` values from string literals and keywords. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overloads

### ISO-8601 Full DateTime

```aljam3
{$} $DT"{year}-{month}-{day}T{hour}:{min}:{sec}Z"
   ($) <year.re << "[0-9]{4}"
   ($) <month.re << "[0-1][0-9]"
   ($) <day.re << "[0-3][0-9]"
   ($) <hour.re << "[0-2][0-9]"
   ($) <min.re << "[0-5][0-9]"
   ($) <sec.re << "[0-5][0-9]"
   [$] #DateTime
   [.] .Civil.date.year << <year
   [.] .Civil.date.month << <month
   [.] .Civil.date.day << <day
   [.] .Civil.time.hour << <hour
   [.] .Civil.time.minute << <min
   [.] .Civil.time.second << <sec
```

Matches ISO-8601 datetime strings with `T` separator and `Z` UTC suffix. Captures map to `#DateTime.Civil.date` and `#DateTime.Civil.time` subfields.

### Date-Only

```aljam3
{$} $DT"{year}-{month}-{day}"
   ($) <year.re << "[0-9]{4}"
   ($) <month.re << "[0-1][0-9]"
   ($) <day.re << "[0-3][0-9]"
   [$] #Date
   [.] .year << <year
   [.] .month << <month
   [.] .day << <day
```

Matches date strings in `YYYY-MM-DD` format. Produces a `#Date` value directly.

### Time-Only

```aljam3
{$} $DT"{hour}:{min}:{sec}"
   ($) <hour.re << "[0-2][0-9]"
   ($) <min.re << "[0-5][0-9]"
   ($) <sec.re << "[0-5][0-9]"
   [$] #Time
   [.] .hour << <hour
   [.] .minute << <min
   [.] .second << <sec
```

Matches time strings in `HH:MM:SS` format. Produces a `#Time` value directly.

## Keyword Overloads

### Today

```aljam3
{$} $DT"Today"
   [$] #Date
   [.] .year << %Runtime.Date.Year
   [.] .month << %Runtime.Date.Month
   [.] .day << %Runtime.Date.Day
```

Produces today's date from the runtime environment. The compiler knows all possible runtime dates are valid `#Date` trees.

### Yesterday

```aljam3
{$} $DT"Yesterday"
   [$] #Date
   [.] .year << %Runtime.Date.Yesterday.Year
   [.] .month << %Runtime.Date.Yesterday.Month
   [.] .day << %Runtime.Date.Yesterday.Day
```

Produces yesterday's date. The runtime resolves calendar rollover (month/year boundaries).

### Tomorrow

```aljam3
{$} $DT"Tomorrow"
   [$] #Date
   [.] .year << %Runtime.Date.Tomorrow.Year
   [.] .month << %Runtime.Date.Tomorrow.Month
   [.] .day << %Runtime.Date.Tomorrow.Day
```

Produces tomorrow's date. The runtime resolves calendar rollover (month/year boundaries).

### Now

```aljam3
{$} $DT"Now"
   [-] -DT.Now
      (-) >dt >> $now
   [$] #DateTime
   [.] << $now
```

Produces the current instant from the system clock. This is the **native pipeline overload** pattern — only pglib constructors may use `[-]` calls inside `{$}`. The `-DT.Now` pipeline is a `{N}` native operation guaranteed infallible by the runtime.

The `[.] << $now` syntax assigns the entire tree from the pipeline result — no field-by-field mapping needed because `-DT.Now` already produces a complete `#DateTime`.

## Overload Resolution

The three string-parsing overloads have non-overlapping regex patterns:

| Overload | Distinguishing Feature |
|---|---|
| ISO-8601 full | Contains `T` separator between date and time |
| Date-only | Contains `-` separators, no `T` or `:` |
| Time-only | Contains `:` separators, no `-` or `T` |

Keywords (`Today`, `Yesterday`, `Tomorrow`, `Now`) are exact-match — no regex ambiguity with string-parsing overloads.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $deadline << $DT"2026-04-22"
[-] $meetingTime << $DT"14:30:00"
[-] $launch << $DT"2026-04-22T14:30:00Z"
[-] $today << $DT"Today"
[-] $current << $DT"Now"

[ ] for dynamic strings, use -DT.Parse with error handling
[-] $parsed#dt << -DT.Parse
   (<) <raw#string << $userInput
   [!] !Parse.DateTime.InvalidFormat
      [-] $parsed << $DT"Today"
```

## Related

- [[constructors/INDEX|pglib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[pglib/pipelines/DT/INDEX|-DT.* pipelines]] -- DateTime pipelines
- [[pglib/pipelines/DT/Parse|-DT.Parse]] -- runtime DateTime parsing
- [[pglib/types/datetime/main-type|#DateTime type]] -- type hierarchy
- [[pglib/types/datetime/core-components|core components]] -- `#Date`, `#Time`, `#Zone` fields
