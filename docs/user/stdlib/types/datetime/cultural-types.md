---
audience: pg-coder
type: reference
updated: 2026-04-03
---

# Cultural Types

<!-- @source:cultural-types -->

### #Holiday

```polyglot
{#} #Holiday
   [%] .description << "Named holiday with optional recurrence"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "holiday"
   [.] .name#string
   [.] .date#Date
   [.] .recurring#bool <~ #Boolean.True
   [.] .calendar#CalendarSystem <~ #CalendarSystem.Gregorian
   [:] :extra
```

### #Observance

```polyglot
{#} #Observance
   [%] .description << "Named observance with type classification"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "observance"
   [.] .name#string
   [.] .date#Date
   [.] .type#ObservanceType
   [.] .recurring#bool <~ #Boolean.True
   [:] :extra
```

### #ObservanceType

```polyglot
{#} #ObservanceType
   [%] .description << "Classification of observance"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "observancetype"
   [.] .Religious
   [.] .National
   [.] .Cultural
   [.] .Personal
```

### #Season

```polyglot
{#} #Season
   [%] .description << "Named season with start and end dates"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "season"
   [.] .name#string
   [.] .start#Date
   [.] .end#Date
   [.] .type#SeasonType
   [:] :extra
```

### #SeasonType

```polyglot
{#} #SeasonType
   [%] .description << "Classification of season"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "seasontype"
   [.] .Astronomical
   [.] .Meteorological
   [.] .Cultural
   [.] .Agricultural
   [.] .Custom
```

---

See also: [[supporting-enums]], [[calendar-date-types]], [[non-standard-time]], [[main-type]]
