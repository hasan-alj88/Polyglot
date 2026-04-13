---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Hebrew Calendar Types

## #HebrewDate

```polyglot
{#} #HebrewDate
   [%] .description << "Hebrew calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "hebrewdate"
   [.] .year#int
   [.] .month#HebrewMonth
   [.] .day#int
```

## #HebrewMonth

```polyglot
{#} #HebrewMonth
   [%] .description << "Hebrew calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "hebrewmonth"
   [.] .Tishrei
   [.] .Cheshvan
   [.] .Kislev
   [.] .Tevet
   [.] .Shevat
   [.] .AdarI
   [.] .AdarII
   [.] .Nisan
   [.] .Iyyar
   [.] .Sivan
   [.] .Tammuz
   [.] .Av
   [.] .Elul
```
