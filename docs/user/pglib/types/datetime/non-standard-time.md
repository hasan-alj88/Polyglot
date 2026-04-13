---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Non-Standard Time Units

<!-- @source:non-standard-time -->

Some calendars divide the day differently than hours/minutes/seconds. These all map back to epoch seconds -- they are alternative projections of `.Instant`.

### #ChineseTime

```polyglot
{#} #ChineseTime
   [%] .description << "Chinese traditional time divisions"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "chinesetime"
   [.] .shichen#ChineseShichen
   [.] .ke#int
   [.] .fen#int
   [.] .yeGeng#int
```

### #ChineseShichen

The twelve double-hours (shichen) that divide the Chinese day.

```polyglot
{#} #ChineseShichen
   [%] .description << "Chinese double-hour period"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "chineseshichen"
   [.] .Zi
   [.] .Chou
   [.] .Yin
   [.] .Mao
   [.] .Chen
   [.] .Si
   [.] .Wu
   [.] .Wei
   [.] .Shen
   [.] .You
   [.] .Xu
   [.] .Hai
```

### #HinduTime

```polyglot
{#} #HinduTime
   [%] .description << "Hindu traditional time divisions"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "hindutime"
   [.] .prahara#int
   [.] .muhurta#int
   [.] .ghati#int
   [.] .pala#int
   [.] .vipala#int
```

### #DecimalTime

French Republican decimal time system (10 hours/day, 100 minutes/hour, 100 seconds/minute).

```polyglot
{#} #DecimalTime
   [%] .description << "French Republican decimal time"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "decimaltime"
   [.] .hour#int
   [.] .minute#int
   [.] .second#int
```

### #CustomTimeUnit

User-defined time division. Fixed fields define the basic unit; flexible fields allow user-defined subdivisions and epoch-to-unit conversion mapping.

```polyglot
{#} #CustomTimeUnit
   [%] .description << "User-defined time division"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "customtimeunit"
   [.] .name#string
   [.] .unitsPerDay#int
   [:] :subdivisions
   [:] :mapping
```

---

See also: [[calendar-date-types]], [[main-type]], [[cultural-types]]
