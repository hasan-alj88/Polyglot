---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Supporting Enums

<!-- @source:supporting-enums -->

### #Precision

```aljam3
{#} #Precision
   [%] .description << "Date-time resolution level"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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

```aljam3
{#} #RecurrencePattern
   [%] .description << "Recurrence frequency pattern"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "recurrencepattern"
   [.] .Daily
   [.] .Weekly
   [.] .Monthly
   [.] .Yearly
   [.] .Custom
      [.] .rule#string
```

### #CalendarSystem

```aljam3
{#} #CalendarSystem
   [%] .description << "Supported calendar system identifier"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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

```aljam3
{#} #Weekday
   [%] .description << "Day of the week"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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

```aljam3
{#} #Month
   [%] .description << "Gregorian calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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

```aljam3
{#} #DayBoundary
   [%] .description << "When a calendar day begins"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "dayboundary"
   [.] .Midnight
   [.] .Sunset
   [.] .Sunrise
   [.] .Noon
```

### #WeekSystem

Each variant carries a `.config#BusinessWeek` sub-field that defines the week configuration for that system.

```aljam3
{#} #WeekSystem
   [%] .description << "Week system standard"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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

```aljam3
{#} #BusinessWeek
   [%] .description << "Configurable work/off day schedule"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "businessweek"
   [.] .firstDay#Weekday
   [.] .workDays#array.Weekday
   [.] .offDays#array.Weekday
   [.] .hoursPerDay#int
```

### #MonthStructure

```aljam3
{#} #MonthStructure
   [%] .description << "How a calendar organizes month lengths"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "monthstructure"
   [.] .Fixed
   [.] .Mixed
   [.] .Lunisolar
   [.] .Lunar
```

---

See also: [[main-type]], [[core-components]], [[calendar-infrastructure]], [[cultural-types]]
