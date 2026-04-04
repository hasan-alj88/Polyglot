---
audience: pg-coder
type: specification
updated: 2026-04-01
status: complete
---

# =DT.* DateTime Pipelines

<!-- @pipelines -->

DateTime pipelines for construction, calendar conversion, arithmetic, comparison, extraction, zone management, formatting, and business day operations. No `[@]` import needed.

All `=DT.*` pipelines are native definitions (`{N}` blocks). They operate on `#dt` (alias for `#DateTime`).

**Inline notation:** `=DT"..."` and `=DateTime"..."` are sugar for `=DT.From.ISO`. This follows the `=Path"..."` precedent.

```polyglot
[ ] These three are equivalent:
[r] $deadline#dt << =DateTime"2026-03-20T12:00:00Z"
[r] $deadline#dt << =DT"2026-03-20T12:00:00Z"
[r] $deadline#dt
   [r] =DT.From.ISO
      [=] <iso << "2026-03-20T12:00:00Z"
      [=] >dt >> $deadline
```

## Permissions

<!-- @permissions -->

All `=DT.*` pipelines require no permissions (`[_] _None`) except `=DT.Now`, which reads the system clock. See [[permissions]].

| Pipeline | Permission | Reason |
|----------|-----------|--------|
| `=DT.Now` | `_IO.Read` | Reads system clock |
| All others | `_None` | Pure computation |

---

## Construction

### =DT.Now

Returns the current instant from the system clock.

```polyglot
{N} =DT.Now
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtNow"
   [%] .description << "Current instant from system clock"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] >dt#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _IO.Read
```

### =DT.From.Epoch

Converts epoch seconds to a `#dt` value.

```polyglot
{N} =DT.From.Epoch
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromEpoch"
   [%] .description << "Epoch seconds to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <epoch#int
   [=] >dt#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.From.ISO

Parses an ISO-8601 string into a `#dt` value. Called implicitly by `=DT"..."` inline notation.

```polyglot
{N} =DT.From.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromIso"
   [%] .description << "ISO-8601 string to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <iso#string
   [=] >dt#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.From.Parts

Constructs a `#dt` from explicit date-time components. Hour, minute, second, nanosecond default to `0`; zone defaults to `"UTC"`.

```polyglot
{N} =DT.From.Parts
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromParts"
   [%] .description << "Explicit components to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <year#int
   [=] <month#int
   [=] <day#int
   [=] <hour#int <~ 0
   [=] <minute#int <~ 0
   [=] <second#int <~ 0
   [=] <nano#int <~ 0
   [=] <zone#string <~ "UTC"
   [=] >dt#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Calendar Conversion

All `=DT.To.*` pipelines project a `#dt` value into a calendar-specific date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

### =DT.To.Gregorian

```polyglot
{N} =DT.To.Gregorian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToGregorian"
   [%] .description << "DateTime to Gregorian date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >gregorian#GregorianDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Hijri

The `<authority` input selects the Hijri calendar authority. Defaults to `#HijriAuthority.UmmAlQura`.

```polyglot
{N} =DT.To.Hijri
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHijri"
   [%] .description << "DateTime to Hijri date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <authority#HijriAuthority <~ #HijriAuthority.UmmAlQura
   [=] >hijri#HijriDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Hebrew

```polyglot
{N} =DT.To.Hebrew
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHebrew"
   [%] .description << "DateTime to Hebrew date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >hebrew#HebrewDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Chinese

```polyglot
{N} =DT.To.Chinese
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToChinese"
   [%] .description << "DateTime to Chinese date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >chinese#ChineseDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Persian

```polyglot
{N} =DT.To.Persian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToPersian"
   [%] .description << "DateTime to Persian date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >persian#PersianDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Buddhist

```polyglot
{N} =DT.To.Buddhist
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToBuddhist"
   [%] .description << "DateTime to Buddhist date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >buddhist#BuddhistDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Hindu

The `<era` input selects the Hindu era. Defaults to `#HinduEra.VikramSamvat`.

```polyglot
{N} =DT.To.Hindu
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHindu"
   [%] .description << "DateTime to Hindu date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <era#HinduEra <~ #HinduEra.VikramSamvat
   [=] >hindu#HinduDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Japanese

```polyglot
{N} =DT.To.Japanese
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToJapanese"
   [%] .description << "DateTime to Japanese date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >japanese#JapaneseDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Ethiopian

```polyglot
{N} =DT.To.Ethiopian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToEthiopian"
   [%] .description << "DateTime to Ethiopian date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >ethiopian#EthiopianDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Coptic

```polyglot
{N} =DT.To.Coptic
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToCoptic"
   [%] .description << "DateTime to Coptic date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >coptic#CopticDate
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.Custom

Projects a `#dt` into a user-supplied custom calendar.

```polyglot
{N} =DT.To.Custom
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToCustom"
   [%] .description << "DateTime to custom calendar date"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <calendar#CustomCalendar
   [=] >date#Date
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Time Unit Conversion

These pipelines convert a `#dt` into culture-specific time representations.

### =DT.To.ChineseTime

Converts to Chinese traditional time units (shichen/ke/fen).

```polyglot
{N} =DT.To.ChineseTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToChineseTime"
   [%] .description << "DateTime to Chinese traditional time"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >time#ChineseTime
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.HinduTime

Converts to Hindu traditional time units (prahara/muhurta).

```polyglot
{N} =DT.To.HinduTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHinduTime"
   [%] .description << "DateTime to Hindu traditional time"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >time#HinduTime
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.To.DecimalTime

Converts to French Republican decimal time.

```polyglot
{N} =DT.To.DecimalTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToDecimalTime"
   [%] .description << "DateTime to decimal time"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >time#DecimalTime
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Arithmetic

### =DT.Add.Duration

Adds a `#Duration` (fixed time span) to a `#dt`.

```polyglot
{N} =DT.Add.Duration
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddDuration"
   [%] .description << "Add duration to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <duration#Duration
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Add.Period

Adds a `#Period` (calendar-aware span such as "1 month") to a `#dt`.

```polyglot
{N} =DT.Add.Period
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddPeriod"
   [%] .description << "Add calendar period to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <period#Period
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Sub

Subtracts two `#dt` values and returns the `#Duration` between them.

```polyglot
{N} =DT.Sub
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtSub"
   [%] .description << "Subtract two DateTimes to get Duration"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <a#dt
   [=] <b#dt
   [=] >result#Duration
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Comparison

### =DT.Compare

Returns `-1`, `0`, or `1` as an `#int`.

```polyglot
{N} =DT.Compare
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtCompare"
   [%] .description << "Compare two DateTimes"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <a#dt
   [=] <b#dt
   [=] >result#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.IsBefore

Returns `#bool` -- true when `a` is earlier than `b`.

```polyglot
{N} =DT.IsBefore
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtIsBefore"
   [%] .description << "Check if DateTime is before another"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <a#dt
   [=] <b#dt
   [=] >result#bool
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.IsAfter

Returns `#bool` -- true when `a` is later than `b`.

```polyglot
{N} =DT.IsAfter
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtIsAfter"
   [%] .description << "Check if DateTime is after another"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <a#dt
   [=] <b#dt
   [=] >result#bool
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.InInterval

Returns `#bool` -- true when `source` falls within the given `#Interval`.

```polyglot
{N} =DT.InInterval
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtInInterval"
   [%] .description << "Check if DateTime is within interval"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <interval#Interval
   [=] >result#bool
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Extraction

These pipelines extract individual components from a `#dt` value.

### =DT.Get.Year

```polyglot
{N} =DT.Get.Year
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetYear"
   [%] .description << "Extract year from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >year#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.Month

```polyglot
{N} =DT.Get.Month
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetMonth"
   [%] .description << "Extract month from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >month#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.Day

```polyglot
{N} =DT.Get.Day
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetDay"
   [%] .description << "Extract day from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >day#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.Weekday

Returns a `#Weekday` enum value.

```polyglot
{N} =DT.Get.Weekday
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetWeekday"
   [%] .description << "Extract weekday from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >weekday#Weekday
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.WeekNumber

Returns the week-of-year as `#int`. The `<system` input selects the week numbering system; defaults to `#WeekSystem.ISO`.

```polyglot
{N} =DT.Get.WeekNumber
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetWeekNumber"
   [%] .description << "Extract week number from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <system#WeekSystem <~ #WeekSystem.ISO
   [=] >week#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.Epoch

Returns epoch seconds as `#int`.

```polyglot
{N} =DT.Get.Epoch
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetEpoch"
   [%] .description << "Extract epoch seconds from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >epoch#int
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Get.Zone

Returns the `#Zone` attached to a `#dt` value.

```polyglot
{N} =DT.Get.Zone
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetZone"
   [%] .description << "Extract timezone from DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >zone#Zone
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Zone

### =DT.Zone.Set

Replaces the timezone label on a `#dt` without converting the instant. The wall-clock reading stays the same; the underlying instant changes.

```polyglot
{N} =DT.Zone.Set
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtZoneSet"
   [%] .description << "Set timezone on DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <iana#string
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Zone.Convert

Converts a `#dt` to a different timezone. The underlying instant stays the same; the wall-clock reading changes.

```polyglot
{N} =DT.Zone.Convert
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtZoneConvert"
   [%] .description << "Convert DateTime to different timezone"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <iana#string
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Formatting

### =DT.Format

Formats a `#dt` using a pattern string (e.g. `"YYYY-MM-DD HH:mm:ss"`).

```polyglot
{N} =DT.Format
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormat"
   [%] .description << "Format DateTime with pattern"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <pattern#string
   [=] >text#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Format.ISO

Formats a `#dt` as an ISO-8601 string.

```polyglot
{N} =DT.Format.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatIso"
   [%] .description << "Format DateTime as ISO-8601"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] >text#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Format.Calendar

Formats a `#dt` as a string in a specific calendar system's conventional format.

```polyglot
{N} =DT.Format.Calendar
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatCalendar"
   [%] .description << "Format DateTime in calendar system format"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <system#CalendarSystem
   [=] >text#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Business

Business day pipelines require an explicit `#BusinessWeek` configuration. No regional defaults are assumed -- all fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must be set by the user.

### =DT.Business.IsWorkDay

Returns `#bool` -- true when `source` falls on a work day.

```polyglot
{N} =DT.Business.IsWorkDay
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessIsWorkDay"
   [%] .description << "Check if DateTime is a work day"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <week#BusinessWeek
   [=] >result#bool
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Business.NextWorkDay

Returns the next work day as a `#dt`.

```polyglot
{N} =DT.Business.NextWorkDay
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessNextWorkDay"
   [%] .description << "Find next work day"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <week#BusinessWeek
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

### =DT.Business.AddWorkDays

Adds `N` work days to a `#dt`, skipping non-work days.

```polyglot
{N} =DT.Business.AddWorkDays
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessAddWorkDays"
   [%] .description << "Add work days to DateTime"
   [%] .version << "1.0.0"
   [T] =T.Manual
   [=] <source#dt
   [=] <days#int
   [=] <week#BusinessWeek
   [=] >result#dt
   [Q] =Q.Default
   [W] =W.Polyglot
   [_] _None
```

---

## Related

- [[stdlib/types/datetime|DateTime types]] -- `#dt`, `#Duration`, `#Period`, `#Interval`, calendar date structs
- [[stdlib/pipelines/INDEX|Pipelines index]] -- full stdlib pipeline listing
- [[permissions]] -- permission system for `[_]` declarations
