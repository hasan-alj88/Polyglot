---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Cultural Types

<!-- @source:cultural-types -->

### #Holiday

```aljam3
{#} #Holiday
   [%] .description << "Named holiday with optional recurrence"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "holiday"
   [.] .name#string
   [.] .date#Date
   [.] .recurring#bool <~ #Boolean.True
   [.] .calendar#CalendarSystem <~ #CalendarSystem.Gregorian
   [:] :extra
```

### #Observance

```aljam3
{#} #Observance
   [%] .description << "Named observance with type classification"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "observance"
   [.] .name#string
   [.] .date#Date
   [.] .type#ObservanceType
   [.] .recurring#bool <~ #Boolean.True
   [:] :extra
```

### #ObservanceType

```aljam3
{#} #ObservanceType
   [%] .description << "Classification of observance"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "observancetype"
   [.] .Religious
   [.] .National
   [.] .Cultural
   [.] .Personal
```

### #Season

```aljam3
{#} #Season
   [%] .description << "Named season with start and end dates"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "season"
   [.] .name#string
   [.] .start#Date
   [.] .end#Date
   [.] .type#SeasonType
   [:] :extra
```

### #SeasonType

```aljam3
{#} #SeasonType
   [%] .description << "Classification of season"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "seasontype"
   [.] .Astronomical
   [.] .Meteorological
   [.] .Cultural
   [.] .Agricultural
   [.] .Custom
```

---

See also: [[supporting-enums]], [[calendar-date-types]], [[non-standard-time]], [[main-type]]
