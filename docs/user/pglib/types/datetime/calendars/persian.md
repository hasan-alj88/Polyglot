---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Persian Calendar Types

## #PersianDate

```polyglot
{#} #PersianDate
   [%] .description << "Persian (Solar Hijri) calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "persiandate"
   [.] .year#int
   [.] .month#PersianMonth
   [.] .day#int
```

## #PersianMonth

```polyglot
{#} #PersianMonth
   [%] .description << "Persian calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "persianmonth"
   [.] .Farvardin
   [.] .Ordibehesht
   [.] .Khordad
   [.] .Tir
   [.] .Mordad
   [.] .Shahrivar
   [.] .Mehr
   [.] .Aban
   [.] .Azar
   [.] .Dey
   [.] .Bahman
   [.] .Esfand
```
