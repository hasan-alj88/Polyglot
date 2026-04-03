---
audience: pg-coder
type: reference
updated: 2026-04-03
---

# Supporting Enums

<!-- @source:supporting-enums -->

### #Precision

```polyglot
{#} #Precision
   [%] .description << "Date-time resolution level"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "precision"
   [.] .Year
   [.] .Month
   [.] .Day
   [.] .Hour
   [.] .Minute
   [.] .Second
   [.] .Millisecond
   [.] .Microsecond
   [.] .Nanosecond
```

### #RecurrencePattern

`#RecurrencePattern.Custom` has a nested `.rule#string` value sub-field for user-defined recurrence rules (cron-like or RRULE).

```polyglot
{#} #RecurrencePattern
   [%] .description << "Recurrence frequency pattern"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "recurrencepattern"
   [.] .Daily
   [.] .Weekly
   [.] .Monthly
   [.] .Yearly
   [.] .Custom
      [.] .rule#string
```

### #CalendarSystem

```polyglot
{#} #CalendarSystem
   [%] .description << "Supported calendar system identifier"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "calendarsystem"
   [.] .Gregorian
   [.] .Hijri
   [.] .Hebrew
   [.] .Chinese
   [.] .Persian
   [.] .Buddhist
   [.] .Hindu
   [.] .Japanese
   [.] .Ethiopian
   [.] .Coptic
   [.] .Custom
```

### #Weekday

```polyglot
{#} #Weekday
   [%] .description << "Day of the week"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "weekday"
   [.] .Monday
   [.] .Tuesday
   [.] .Wednesday
   [.] .Thursday
   [.] .Friday
   [.] .Saturday
   [.] .Sunday
```

### #Month

Gregorian month names. Other calendars define their own month enums.

```polyglot
{#} #Month
   [%] .description << "Gregorian calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "month"
   [.] .January
   [.] .February
   [.] .March
   [.] .April
   [.] .May
   [.] .June
   [.] .July
   [.] .August
   [.] .September
   [.] .October
   [.] .November
   [.] .December
```

### #DayBoundary

```polyglot
{#} #DayBoundary
   [%] .description << "When a calendar day begins"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "dayboundary"
   [.] .Midnight
   [.] .Sunset
   [.] .Sunrise
   [.] .Noon
```

### #WeekSystem

Each variant carries a `.config#BusinessWeek` sub-field that defines the week configuration for that system.

```polyglot
{#} #WeekSystem
   [%] .description << "Week system standard"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "weeksystem"
   [.] .ISO
      [.] .config#BusinessWeek
   [.] .US
      [.] .config#BusinessWeek
   [.] .MiddleEastern
      [.] .config#BusinessWeek
   [.] .Custom
      [.] .config#BusinessWeek
```

### #BusinessWeek

All fields must be explicitly set by the user. No defaults are assumed for regional norms.

```polyglot
{#} #BusinessWeek
   [%] .description << "Configurable work/off day schedule"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "businessweek"
   [.] .firstDay#Weekday
   [.] .workDays#array.Weekday
   [.] .offDays#array.Weekday
   [.] .hoursPerDay#int
```

### #MonthStructure

```polyglot
{#} #MonthStructure
   [%] .description << "How a calendar organizes month lengths"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "monthstructure"
   [.] .Fixed
   [.] .Mixed
   [.] .Lunisolar
   [.] .Lunar
```

---

See also: [[main-type]], [[core-components]], [[calendar-infrastructure]], [[cultural-types]]
