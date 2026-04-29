---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Core Component Types

<!-- @source:core-components -->

### #Date

```aljam3
{#} #Date
   [%] .description << "Calendar date with year, month, day"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "date"
   [.] .year#int
   [.] .month#int
   [.] .day#int
```

### #Time

```aljam3
{#} #Time
   [%] .description << "Time of day with nanosecond precision"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "time"
   [.] .hour#int
   [.] .minute#int
   [.] .second#int
   [.] .nano#int
```

### #Zone

```aljam3
{#} #Zone
   [%] .description << "Timezone with IANA identifier, offset, and DST flag"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "zone"
   [.] .iana#string
   [.] .offset#ZoneOffset
   [.] .dst#bool
```

### #ZoneOffset

```aljam3
{#} #ZoneOffset
   [%] .description << "UTC offset in hours and minutes"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "zoneoffset"
   [.] .hours#int
   [.] .minutes#int
```

### #Duration

```aljam3
{#} #Duration
   [%] .description << "Absolute time span in seconds and nanoseconds"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "duration"
   [.] .seconds#int
   [.] .nanos#int
```

### #Period

```aljam3
{#} #Period
   [%] .description << "Calendar-relative time span in years, months, weeks, days"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "period"
   [.] .years#int
   [.] .months#int
   [.] .weeks#int
   [.] .days#int
```

### #Interval

```aljam3
{#} #Interval
   [%] .description << "Start/end date-time range with inclusivity flags"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "interval"
   [.] .start#dt
   [.] .end#dt
   [.] .startInclusive#bool <~ #Boolean.True
   [.] .endInclusive#bool <~ #Boolean.False
```

### #Recurrence

```aljam3
{#} #Recurrence
   [%] .description << "Repeating date-time pattern with optional end"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "recurrence"
   [.] .pattern#RecurrencePattern
   [.] .count#int <~ 0
   [.] .until#dt
```

---

See also: [[main-type]], [[supporting-enums]], [[calendar-infrastructure]]
